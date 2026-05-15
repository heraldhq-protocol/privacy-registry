use anchor_lang::prelude::*;

use crate::constants::IDENTITY_SEED;
use crate::errors::HeraldError;
use crate::state::IdentityAccount;

/// Lazy migration for existing IdentityAccount PDAs.
///
/// After the program upgrade, existing accounts have channel_email = false
/// (zero-initialized) even though they have encrypted_email data.
/// This instruction sets channel_email = true for those accounts.
///
/// The identity owner must sign to prevent front-running or impersonation.
#[derive(Accounts)]
pub struct MigrateIdentityChannels<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [IDENTITY_SEED, owner.key().as_ref()],
        bump = identity_account.bump,
        constraint = identity_account.owner == owner.key() @ HeraldError::OwnerMismatch,
    )]
    pub identity_account: Account<'info, IdentityAccount>,
}

pub fn handler(ctx: Context<MigrateIdentityChannels>) -> Result<()> {
    let account = &mut ctx.accounts.identity_account;

    // Only migrate if not already migrated:
    // channel_email is false (zero-initialized) AND encrypted_email is present
    if !account.channel_email && !account.encrypted_email.is_empty() {
        account.channel_email = true;
    }

    Ok(())
}
