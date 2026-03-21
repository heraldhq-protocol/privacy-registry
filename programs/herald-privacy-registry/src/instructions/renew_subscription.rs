use anchor_lang::prelude::*;

use crate::constants::HERALD_AUTHORITY;
use crate::errors::HeraldError;
use crate::events::SubscriptionRenewed;
use crate::state::ProtocolRegistryAccount;

/// Accounts for `renew_subscription`.
#[derive(Accounts)]
pub struct RenewSubscription<'info> {
    #[account(
        constraint = authority.key() == HERALD_AUTHORITY @ HeraldError::Unauthorized,
    )]
    pub authority: Signer<'info>,

    /// The protocol whose subscription is being renewed.
    /// Marked `mut` – we update billing timestamps and the active flag.
    #[account(mut)]
    pub protocol_account: Account<'info, ProtocolRegistryAccount>,
}

/// Renew (or initially activate) a protocol's monthly subscription.
///
/// Called exclusively by the Herald backend after off-chain payment confirmation (Helio).
///
/// Behaviour:
/// - If the current subscription is still active, the new expiry is calculated
///   from `subscription_expires_at + SUBSCRIPTION_PERIOD_SECS` (extending forward).
/// - If the subscription has already lapsed, the expiry is `now + SUBSCRIPTION_PERIOD_SECS`.
/// - Sets `is_active = true` (reactivates a lapsed protocol without a separate call).
/// - Increments `periods_paid`.
pub fn handler(ctx: Context<RenewSubscription>) -> Result<()> {
    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;

    let protocol = &mut ctx.accounts.protocol_account;

    // Refuse to renew a suspended protocol through this instruction.
    require!(!protocol.is_suspended, HeraldError::ProtocolSuspended);

    let new_expiry = protocol.compute_new_expiry(now)?;

    protocol.subscription_expires_at = new_expiry;
    protocol.last_renewed_at = now;
    protocol.periods_paid = protocol
        .periods_paid
        .checked_add(1)
        .ok_or_else(|| error!(HeraldError::Overflow))?;

    // Renewing also reactivates the protocol if it was deactivated due to lapse.
    protocol.is_active = true;

    // payment_source tag: "helio_webhook" padded to 20 bytes
    let mut payment_source = [0u8; 20];
    let tag = b"helio_webhook";
    payment_source[..tag.len()].copy_from_slice(tag);

    emit!(SubscriptionRenewed {
        protocol: protocol.owner,
        tier: protocol.tier,
        new_expiry,
        periods_paid: protocol.periods_paid,
        usdc_paid: 0, // off-chain payment; amount not known here
        payment_source,
        timestamp: now,
    });

    Ok(())
}
