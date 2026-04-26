use anchor_lang::prelude::*;

use crate::constants::IDENTITY_SEED;
use crate::errors::HeraldError;
use crate::events::NotificationKeyRevoked;
use crate::state::IdentityAccount;

/// Accounts required for `revoke_notification_key`.
///
/// Zeroes out all notification key fields on the IdentityAccount.
/// Does NOT close the PDA — other identity data (email, channels, preferences)
/// remain intact. The user can re-register a new notification key afterwards.
#[derive(Accounts)]
pub struct RevokeNotificationKey<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds  = [IDENTITY_SEED, owner.key().as_ref()],
        bump   = identity_account.bump,
        constraint = identity_account.owner == owner.key() @ HeraldError::OwnerMismatch,
    )]
    pub identity_account: Account<'info, IdentityAccount>,
}

pub fn handler(ctx: Context<RevokeNotificationKey>) -> Result<()> {
    let account = &mut ctx.accounts.identity_account;

    // Must have a key to revoke
    require!(
        account.has_notification_key(),
        HeraldError::NotificationKeyNotRegistered
    );

    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;

    // ── Zero out all notification key fields ──
    account.sealed_x25519_pubkey = [0u8; 48];
    account.sender_x25519_pubkey = [0u8; 32];
    account.notification_nonce = [0u8; 24];
    account.notification_key_version = 0;
    account.notification_key_updated_at = 0;
    account.notification_key_rotation_count = 0;

    emit!(NotificationKeyRevoked {
        wallet: account.owner,
        revoked_at: now,
    });

    Ok(())
}
