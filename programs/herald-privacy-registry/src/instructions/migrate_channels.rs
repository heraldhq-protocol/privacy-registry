use anchor_lang::prelude::*;

use crate::state::IdentityAccount;

/// Lazy migration for existing IdentityAccount PDAs.
///
/// After the program upgrade, existing accounts have channel_email = false
/// (zero-initialized) even though they have encrypted_email data.
/// This instruction sets channel_email = true for those accounts.
///
/// TODO(#security): Require identity owner as signer to prevent frontrunning.
/// Currently permissionless because enabling email channel when encrypted_email
/// is present is always correct behavior, but this allows front-running user
/// preference selection. Add `pub owner: Signer<'info>` and verify PDA seeds
/// when ready to prevent mempool frontrunning.
#[derive(Accounts)]
pub struct MigrateIdentityChannels<'info> {
    #[account(mut)]
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
