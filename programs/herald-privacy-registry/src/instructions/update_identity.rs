use anchor_lang::prelude::*;

use crate::constants::MAX_ENCRYPTED_EMAIL_LEN;
use crate::errors::HeraldError;
use crate::events::{IdentityUpdated, PreferencesUpdated};
use crate::state::IdentityAccount;

/// Accounts required for `update_identity`.
#[derive(Accounts)]
pub struct UpdateIdentity<'info> {
    /// The wallet that owns this identity; must be a signer.
    #[account(mut)]
    pub owner: Signer<'info>,

    /// Existing identity PDA – only the owner can mutate it.
    #[account(
        mut,
        seeds = [b"identity", owner.key().as_ref()],
        bump = identity_account.bump,
        constraint = identity_account.owner == owner.key() @ HeraldError::OwnerMismatch,
    )]
    pub identity_account: Account<'info, IdentityAccount>,
}

/// Partially update an existing user identity.
/// Only provided `Some(…)` fields are overwritten; `None` fields remain unchanged.
pub fn handler(
    ctx: Context<UpdateIdentity>,
    encrypted_email: Option<Vec<u8>>,
    email_hash: Option<[u8; 32]>,
    nonce: Option<[u8; 24]>,
    opt_in_all: Option<bool>,
    opt_in_defi: Option<bool>,
    opt_in_governance: Option<bool>,
    opt_in_marketing: Option<bool>,
    digest_mode: Option<bool>,
) -> Result<()> {
    // ── Require at least one field to update ──
    let has_update = encrypted_email.is_some()
        || email_hash.is_some()
        || nonce.is_some()
        || opt_in_all.is_some()
        || opt_in_defi.is_some()
        || opt_in_governance.is_some()
        || opt_in_marketing.is_some()
        || digest_mode.is_some();
    require!(has_update, HeraldError::EmptyUpdate);

    let identity = &mut ctx.accounts.identity_account;
    let mut email_changed = false;
    let mut preferences_changed = false;

    // ── Email fields ──
    if let Some(email) = encrypted_email {
        require!(!email.is_empty(), HeraldError::EmailEmpty);
        require!(
            email.len() <= MAX_ENCRYPTED_EMAIL_LEN,
            HeraldError::EmailTooLong
        );
        identity.encrypted_email = email;
        email_changed = true;
    }
    if let Some(hash) = email_hash {
        identity.email_hash = hash;
        email_changed = true;
    }
    if let Some(n) = nonce {
        identity.nonce = n;
        email_changed = true;
    }

    // ── Preference fields ──
    if let Some(v) = opt_in_all {
        identity.opt_in_all = v;
        preferences_changed = true;
    }
    if let Some(v) = opt_in_defi {
        identity.opt_in_defi = v;
        preferences_changed = true;
    }
    if let Some(v) = opt_in_governance {
        identity.opt_in_governance = v;
        preferences_changed = true;
    }
    if let Some(v) = opt_in_marketing {
        identity.opt_in_marketing = v;
        preferences_changed = true;
    }
    if let Some(v) = digest_mode {
        identity.digest_mode = v;
        preferences_changed = true;
    }

    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;

    // ── Emit general update event ──
    emit!(IdentityUpdated {
        wallet: ctx.accounts.owner.key(),
        email_changed,
        preferences_changed,
        timestamp: now,
    });

    // ── Emit granular preferences event if changed ──
    if preferences_changed {
        emit!(PreferencesUpdated {
            wallet: ctx.accounts.owner.key(),
            opt_in_all: identity.opt_in_all,
            opt_in_defi: identity.opt_in_defi,
            opt_in_governance: identity.opt_in_governance,
            opt_in_marketing: identity.opt_in_marketing,
            digest_mode: identity.digest_mode,
            timestamp: now,
        });
    }

    Ok(())
}
