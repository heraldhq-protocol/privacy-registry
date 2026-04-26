use anchor_lang::prelude::*;

use crate::constants::{IDENTITY_SEED, NOTIFICATION_KEY_VERSION};
use crate::errors::HeraldError;
use crate::events::NotificationKeyRegistered;
use crate::state::IdentityAccount;

/// Accounts required for `register_notification_key`.
///
/// Registers or replaces the sealed X25519 notification key on an existing
/// IdentityAccount PDA. The sealed blob is only decryptable by the Herald
/// Nitro Enclave.
///
/// This follows the same pattern as `register_telegram`: the owner must
/// already have a registered identity (email), and this instruction adds
/// the notification encryption key to the same PDA.
#[derive(Accounts)]
pub struct RegisterNotificationKey<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds  = [IDENTITY_SEED, owner.key().as_ref()],
        bump   = identity_account.bump,
        constraint = identity_account.owner == owner.key() @ HeraldError::OwnerMismatch,
    )]
    pub identity_account: Account<'info, IdentityAccount>,
}

pub fn handler(
    ctx: Context<RegisterNotificationKey>,
    sealed_x25519_pubkey: [u8; 48],
    sender_x25519_pubkey: [u8; 32],
    nonce: [u8; 24],
    version: u8,
) -> Result<()> {
    // ── Input validation ──
    require!(
        sealed_x25519_pubkey != [0u8; 48],
        HeraldError::ZeroSealedPubkey
    );
    require!(nonce != [0u8; 24], HeraldError::ZeroNotificationNonce);
    require!(
        version == NOTIFICATION_KEY_VERSION,
        HeraldError::UnsupportedNotificationKeyVersion
    );

    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;

    let account = &mut ctx.accounts.identity_account;

    account.sealed_x25519_pubkey = sealed_x25519_pubkey;
    account.sender_x25519_pubkey = sender_x25519_pubkey;
    account.notification_nonce = nonce;
    account.notification_key_version = version;
    account.notification_key_updated_at = now;
    account.notification_key_rotation_count = 0;

    emit!(NotificationKeyRegistered {
        wallet: account.owner,
        version,
        registered_at: now,
    });

    Ok(())
}
