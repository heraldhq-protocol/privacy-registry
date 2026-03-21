use anchor_lang::prelude::*;

use crate::constants::HERALD_AUTHORITY;
use crate::errors::HeraldError;
use crate::events::ProtocolRegistered;
use crate::state::ProtocolRegistryAccount;

/// Accounts required for `register_protocol`.
#[derive(Accounts)]
pub struct RegisterProtocol<'info> {
    /// Herald backend authority – pays rent and must match `HERALD_AUTHORITY`.
    #[account(
        mut,
        constraint = authority.key() == HERALD_AUTHORITY @ HeraldError::Unauthorized,
    )]
    pub authority: Signer<'info>,

    /// Protocol registry PDA – seeded by the protocol's wallet address.
    #[account(
        init,
        payer = authority,
        space = 8 + ProtocolRegistryAccount::INIT_SPACE,
        seeds = [b"protocol", protocol_pubkey.key().as_ref()],
        bump,
    )]
    pub protocol_account: Account<'info, ProtocolRegistryAccount>,

    /// The protocol's wallet address (not required to sign).
    /// CHECK: Validated only as a PDA seed; the protocol admin does not need to sign.
    pub protocol_pubkey: UncheckedAccount<'info>,

    pub system_program: Program<'info, System>,
}

/// Register a new DeFi protocol. Only callable by the Herald authority.
pub fn handler(ctx: Context<RegisterProtocol>, name_hash: [u8; 32], tier: u8) -> Result<()> {
    require!(tier <= 3, HeraldError::InvalidTier);

    let protocol = &mut ctx.accounts.protocol_account;
    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;

    protocol.owner = ctx.accounts.protocol_pubkey.key();
    protocol.name_hash = name_hash;
    protocol.tier = tier;
    protocol.sends_this_period = 0;
    // Protocol starts inactive; `renew_subscription` activates it after payment.
    protocol.is_active = false;
    protocol.is_suspended = false;
    protocol.subscription_expires_at = 0; // not yet subscribed
    protocol.last_renewed_at = 0;
    protocol.periods_paid = 0;
    protocol.lifetime_usdc_paid = 0;
    protocol.last_payment_mint = Pubkey::default();
    protocol.registered_at = now;
    protocol.bump = ctx.bumps.protocol_account;

    emit!(ProtocolRegistered {
        protocol: ctx.accounts.protocol_pubkey.key(),
        name_hash,
        tier,
        timestamp: now,
    });

    Ok(())
}
