use anchor_lang::prelude::*;

use light_sdk::{
    account::LightAccount,
    cpi::v1::{CpiAccounts, InvokeLightSystemProgram, LightSystemProgramCpi},
    instruction::ValidityProof,
};

use crate::constants::HERALD_AUTHORITY;
use crate::errors::HeraldError;
use crate::events::{NotificationDelivered, ProtocolSendRecorded};
use crate::state::{DeliveryReceipt, ProtocolRegistryAccount};

/// Accounts required for `write_receipt`.
///
/// Light Protocol system accounts are passed via `remaining_accounts`
/// and parsed by `CpiAccounts::new(…)`.
#[derive(Accounts)]
pub struct WriteReceipt<'info> {
    /// Herald backend authority – pays for the compressed account write.
    #[account(
        mut,
        constraint = authority.key() == HERALD_AUTHORITY @ HeraldError::Unauthorized,
    )]
    pub authority: Signer<'info>,

    /// The protocol that triggered this notification.
    /// Must be active, not suspended, and within subscription/tier limits.
    /// Marked `mut` so the sends counter can be incremented.
    #[account(mut)]
    pub protocol_account: Account<'info, ProtocolRegistryAccount>,
}

/// Write a ZK-compressed delivery receipt via Light Protocol CPI.
///
/// # Security checks performed
/// 1. Authority must be `HERALD_AUTHORITY`.
/// 2. Protocol must be `is_active` and not `is_suspended`.
/// 3. Protocol subscription must not be expired.
/// 4. Sends this period must not exceed the tier limit.
/// 5. Sends counter increment must not overflow.
/// 6. Category must be 0–3.
///
/// # Arguments
/// * `proof`             – validity proof obtained from the Light RPC
/// * `output_tree_index` – index of the output state tree
/// * `recipient_hash`    – SHA-256 of the recipient's wallet pubkey
/// * `notification_id`   – UUID v4 (16 bytes)
/// * `category`          – 0-3 notification category
pub fn handler<'info>(
    ctx: Context<'_, '_, '_, 'info, WriteReceipt<'info>>,
    proof: ValidityProof,
    output_tree_index: u8,
    recipient_hash: [u8; 32],
    notification_id: [u8; 16],
    category: u8,
) -> Result<()> {
    // ── 1. Input validation ──
    require!(category <= 3, HeraldError::InvalidCategory);

    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;

    let protocol = &mut ctx.accounts.protocol_account;

    // ── 2. Protocol state checks ──
    require!(protocol.is_active, HeraldError::ProtocolInactive);
    require!(!protocol.is_suspended, HeraldError::ProtocolSuspended);

    // ── 3. Subscription check ──
    require!(
        protocol.subscription_expires_at > 0,
        HeraldError::SubscriptionNotActive
    );
    require!(
        protocol.subscription_is_current(now),
        HeraldError::SubscriptionExpired
    );

    // ── 4. Tier quota check ──
    let sends_limit = protocol.sends_limit();
    require!(
        protocol.sends_this_period < sends_limit,
        HeraldError::SendsLimitExceeded
    );

    // ── 5. Increment sends counter (checked adds) ──
    protocol.sends_this_period = protocol
        .sends_this_period
        .checked_add(1)
        .ok_or(error!(HeraldError::SendsOverflow))?;

    // ── 6. Build the Light CPI accounts from remaining_accounts ──
    // CpiAccounts::new returns CpiAccounts directly (not a Result) in light-sdk 0.23.
    let light_cpi_accounts = CpiAccounts::new(
        ctx.accounts.authority.as_ref(),
        ctx.remaining_accounts,
        crate::constants::LIGHT_CPI_SIGNER,
    );

    // ── 7. Construct the compressed receipt ──
    let mut receipt = LightAccount::<DeliveryReceipt>::new_init(
        &crate::ID,
        None, // receipts are append-only; no unique address required
        output_tree_index,
    );

    receipt.protocol_pubkey = protocol.owner;
    receipt.recipient_hash = recipient_hash;
    receipt.notification_id = notification_id;
    receipt.timestamp = now;
    receipt.delivered = true;
    receipt.category = category;

    // ── 8. CPI into Light System Program ──
    // InvokeLightSystemProgram trait must be in scope for .new_cpi() and .invoke().
    LightSystemProgramCpi::new_cpi(crate::constants::LIGHT_CPI_SIGNER, proof)
        .with_light_account(receipt)
        .map_err(|_| error!(HeraldError::LightAccountError))?
        .invoke(light_cpi_accounts)
        .map_err(|_| error!(HeraldError::LightCpiInvocationError))?;

    // ── 9. Emit events ──
    emit!(NotificationDelivered {
        protocol: protocol.owner,
        recipient_hash,
        notification_id,
        category,
        sends_this_period: protocol.sends_this_period,
        timestamp: now,
    });

    emit!(ProtocolSendRecorded {
        protocol: protocol.owner,
        sends_this_period: protocol.sends_this_period,
        sends_limit,
        timestamp: now,
    });

    Ok(())
}
