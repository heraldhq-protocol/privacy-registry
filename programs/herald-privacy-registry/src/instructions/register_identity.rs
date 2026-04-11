use anchor_lang::prelude::*;

use crate::constants::MAX_ENCRYPTED_EMAIL_LEN;
use crate::errors::HeraldError;
use crate::events::IdentityRegistered;
use crate::state::IdentityAccount;

/// Accounts required for `register_identity`.
#[derive(Accounts)]
pub struct RegisterIdentity<'info> {
    /// The wallet registering its identity; pays rent.
    #[account(mut)]
    pub owner: Signer<'info>,

    /// Identity PDA – created and owned by this program.
    /// Uses IdentityAccount::SPACE to accommodate all channel fields.
    #[account(
        init,
        payer = owner,
        space = IdentityAccount::SPACE,
        seeds = [b"identity", owner.key().as_ref()],
        bump,
    )]
    pub identity_account: Account<'info, IdentityAccount>,

    pub system_program: Program<'info, System>,
}

/// Register a new user identity with encrypted email and opt-in preferences.
pub fn handler(
    ctx: Context<RegisterIdentity>,
    encrypted_email: Vec<u8>,
    email_hash: [u8; 32],
    nonce: [u8; 24],
    opt_in_all: bool,
    opt_in_defi: bool,
    opt_in_governance: bool,
    opt_in_marketing: bool,
    digest_mode: bool,
) -> Result<()> {
    // ── Input validation ──
    require!(!encrypted_email.is_empty(), HeraldError::EmailEmpty);
    require!(
        encrypted_email.len() <= MAX_ENCRYPTED_EMAIL_LEN,
        HeraldError::EmailTooLong
    );

    let identity = &mut ctx.accounts.identity_account;
    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;

    identity.owner = ctx.accounts.owner.key();
    identity.encrypted_email = encrypted_email;
    identity.email_hash = email_hash;
    identity.nonce = nonce;
    identity.registered_at = now;
    identity.opt_in_all = opt_in_all;
    identity.opt_in_defi = opt_in_defi;
    identity.opt_in_governance = opt_in_governance;
    identity.opt_in_marketing = opt_in_marketing;
    identity.digest_mode = digest_mode;
    identity.bump = ctx.bumps.identity_account;

    // ── Channel defaults for new registrations ──
    // Email is the primary channel — enabled by default.
    // Telegram and SMS start disabled (no data registered yet).
    identity.channel_email = true;
    identity.channel_telegram = false;
    identity.channel_sms = false;
    // Channel data fields are zero-initialized by Anchor (empty vecs, zeroed arrays).

    emit!(IdentityRegistered {
        wallet: ctx.accounts.owner.key(),
        email_hash,
        opt_in_all,
        opt_in_defi,
        opt_in_governance,
        opt_in_marketing,
        digest_mode,
        timestamp: now,
    });

    Ok(())
}
