use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{self, Mint, Token, TokenAccount, Transfer},
};

use crate::constants::{
    HERALD_AUTHORITY, SUBSCRIPTION_PERIOD_SECS, USDC_MINT, USDT_MINT, VAULT_SEED,
};
use crate::errors::HeraldError;
use crate::events::{PaymentReceived, SubscriptionRenewed};
use crate::state::{ProtocolRegistryAccount, SubscriptionVaultAccount};

/// Accounts for `pay_subscription` — Phase 2 native USDC/USDT payment.
///
/// Protocol admin calls this directly. USDC/USDT transferred from
/// protocol's ATA → Herald vault ATA. Subscription renewed atomically.
#[derive(Accounts)]
pub struct PaySubscription<'info> {
    /// Protocol admin wallet — payer and authority.
    #[account(mut)]
    pub payer: Signer<'info>,

    /// Protocol registry account being renewed.
    #[account(
        mut,
        constraint = protocol_account.owner == payer.key() @ HeraldError::Unauthorized,
        constraint = !protocol_account.is_suspended @ HeraldError::ProtocolSuspended,
    )]
    pub protocol_account: Account<'info, ProtocolRegistryAccount>,

    /// Payment token mint — must be USDC or USDT.
    pub payment_mint: Account<'info, Mint>,

    /// Payer's ATA for the payment token.
    #[account(
        mut,
        associated_token::mint      = payment_mint,
        associated_token::authority = payer,
    )]
    pub payer_token_account: Account<'info, TokenAccount>,

    /// Herald's vault ATA for the payment token.
    #[account(
        mut,
        associated_token::mint      = payment_mint,
        associated_token::authority = vault_account,
    )]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// Herald treasury vault PDA.
    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump   = vault_account.bump,
    )]
    pub vault_account: Account<'info, SubscriptionVaultAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

/// Pay for a protocol subscription with USDC or USDT.
///
/// # Arguments
/// * `months` — Number of billing periods to pay for (1–12).
///
/// # Behaviour
/// 1. Validates payment mint is USDC or USDT.
/// 2. Rejects dev tier (tier 0 is free).
/// 3. Transfers `tier_price × months` from payer ATA → vault ATA.
/// 4. Atomically updates subscription expiry on-chain.
/// 5. Emits `PaymentReceived` + `SubscriptionRenewed`.
pub fn handler(ctx: Context<PaySubscription>, months: u8) -> Result<()> {
    let protocol = &mut ctx.accounts.protocol_account;

    // Dev tier is free — reject payment
    require!(protocol.tier > 0, HeraldError::DevTierNoPayment);
    require!(months >= 1 && months <= 12, HeraldError::Overflow);

    // Validate payment token
    let mint_key = ctx.accounts.payment_mint.key();
    require!(
        mint_key == USDC_MINT || mint_key == USDT_MINT,
        HeraldError::UnsupportedPaymentToken
    );

    // Compute total amount
    let price_per_period = protocol.period_price_usdc();
    let total_amount = price_per_period
        .checked_mul(months as u64)
        .ok_or_else(|| error!(HeraldError::Overflow))?;

    // Transfer tokens: payer ATA → vault ATA
    let transfer_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.payer_token_account.to_account_info(),
            to: ctx.accounts.vault_token_account.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        },
    );
    token::transfer(transfer_ctx, total_amount)?;

    // Update vault tracking
    let vault = &mut ctx.accounts.vault_account;
    if mint_key == USDC_MINT {
        vault.total_usdc_collected = vault
            .total_usdc_collected
            .checked_add(total_amount)
            .ok_or_else(|| error!(HeraldError::Overflow))?;
    } else {
        vault.total_usdt_collected = vault
            .total_usdt_collected
            .checked_add(total_amount)
            .ok_or_else(|| error!(HeraldError::Overflow))?;
    }

    // Renew subscription on-chain
    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;

    let period_secs = SUBSCRIPTION_PERIOD_SECS
        .checked_mul(months as i64)
        .ok_or_else(|| error!(HeraldError::Overflow))?;

    let base = if protocol.subscription_is_current(now) {
        protocol.subscription_expires_at
    } else {
        now
    };
    let new_expiry = base
        .checked_add(period_secs)
        .ok_or_else(|| error!(HeraldError::Overflow))?;

    protocol.subscription_expires_at = new_expiry;
    protocol.last_renewed_at = now;
    protocol.periods_paid = protocol
        .periods_paid
        .checked_add(months as u32)
        .ok_or_else(|| error!(HeraldError::Overflow))?;
    protocol.lifetime_usdc_paid = protocol
        .lifetime_usdc_paid
        .checked_add(total_amount)
        .ok_or_else(|| error!(HeraldError::Overflow))?;
    protocol.last_payment_mint = mint_key;
    protocol.is_active = true;

    // payment_source tag
    let mut payment_source = [0u8; 20];
    let tag: &[u8] = if mint_key == USDC_MINT {
        b"on_chain_usdc"
    } else {
        b"on_chain_usdt"
    };
    payment_source[..tag.len()].copy_from_slice(tag);

    emit!(PaymentReceived {
        protocol: protocol.owner,
        amount_usdc: total_amount,
        token_mint: mint_key,
        tier: protocol.tier,
        months,
        new_expiry,
        timestamp: now,
    });

    emit!(SubscriptionRenewed {
        protocol: protocol.owner,
        tier: protocol.tier,
        new_expiry,
        periods_paid: protocol.periods_paid,
        usdc_paid: total_amount,
        payment_source,
        timestamp: now,
    });

    Ok(())
}
