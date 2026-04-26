use anchor_lang::prelude::*;

use crate::constants::{IDENTITY_SEED, MAX_NOTIFICATION_KEY_ROTATIONS, NOTIFICATION_KEY_VERSION};
use crate::errors::HeraldError;
use crate::events::NotificationKeyRotated;
use crate::state::IdentityAccount;

/// Accounts required for `rotate_notification_key`.
///
/// Rotates the sealed X25519 notification key on an existing IdentityAccount.
/// The user must already have a registered notification key.
/// Rotation count is incremented and capped at MAX_NOTIFICATION_KEY_ROTATIONS.
#[derive(Accounts)]
pub struct RotateNotificationKey<'info> {
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
    ctx: Context<RotateNotificationKey>,
    new_sealed_x25519_pubkey: [u8; 48],
    new_sender_x25519_pubkey: [u8; 32],
    new_nonce: [u8; 24],
    version: u8,
) -> Result<()> {
    let account = &mut ctx.accounts.identity_account;

    // ── Pre-conditions ──
    require!(
        account.has_notification_key(),
        HeraldError::NotificationKeyNotRegistered
    );

    // ── Input validation ──
    require!(
        new_sealed_x25519_pubkey != [0u8; 48],
        HeraldError::ZeroSealedPubkey
    );
    require!(new_nonce != [0u8; 24], HeraldError::ZeroNotificationNonce);
    require!(
        version == NOTIFICATION_KEY_VERSION,
        HeraldError::UnsupportedNotificationKeyVersion
    );

    // Prevent nonce reuse (replay attack mitigation)
    require!(
        new_nonce != account.notification_nonce,
        HeraldError::NotificationNonceReuse
    );

    // Cap rotations to prevent abuse
    require!(
        account.notification_key_rotation_count < MAX_NOTIFICATION_KEY_ROTATIONS,
        HeraldError::MaxNotificationKeyRotationsExceeded
    );

    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;

    // ── Atomic overwrite ──
    account.sealed_x25519_pubkey = new_sealed_x25519_pubkey;
    account.sender_x25519_pubkey = new_sender_x25519_pubkey;
    account.notification_nonce = new_nonce;
    account.notification_key_version = version;
    account.notification_key_updated_at = now;
    account.notification_key_rotation_count = account
        .notification_key_rotation_count
        .checked_add(1)
        .ok_or(error!(HeraldError::Overflow))?;

    emit!(NotificationKeyRotated {
        wallet: account.owner,
        version,
        rotation_count: account.notification_key_rotation_count,
        rotated_at: now,
    });

    Ok(())
}
