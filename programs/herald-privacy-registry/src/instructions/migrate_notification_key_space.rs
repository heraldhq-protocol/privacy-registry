use anchor_lang::prelude::*;

use crate::constants::IDENTITY_SEED;
use crate::errors::HeraldError;
use crate::state::IdentityAccount;

/// Lazy migration for existing IdentityAccount PDAs.
///
/// After the program upgrade, existing accounts are smaller than the new
/// SPACE (which includes notification key fields). This instruction
/// reallocates the account to the new size, zero-initializing the new bytes.
///
/// The owner must be the signer and payer for the additional rent.
/// No data fields are modified — only the account size grows.
///
/// After this migration, the user can call `register_notification_key`.
#[derive(Accounts)]
pub struct MigrateNotificationKeySpace<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds  = [IDENTITY_SEED, owner.key().as_ref()],
        bump   = identity_account.bump,
        constraint = identity_account.owner == owner.key() @ HeraldError::OwnerMismatch,
        realloc = IdentityAccount::SPACE,
        realloc::payer = owner,
        realloc::zero = true,
    )]
    pub identity_account: Account<'info, IdentityAccount>,

    pub system_program: Program<'info, System>,
}

pub fn handler(_ctx: Context<MigrateNotificationKeySpace>) -> Result<()> {
    // No data changes needed — the realloc constraint handles account growth.
    // New fields are zero-initialized by `realloc::zero = true`.
    Ok(())
}
