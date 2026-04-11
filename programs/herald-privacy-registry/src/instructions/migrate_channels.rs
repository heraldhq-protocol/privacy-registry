use anchor_lang::prelude::*;

use crate::state::IdentityAccount;

/// Lazy migration for existing IdentityAccount PDAs.
///
/// After the program upgrade, existing accounts have channel_email = false
/// (zero-initialized) even though they have encrypted_email data.
/// This instruction sets channel_email = true for those accounts.
///
/// Permissionless — anyone can call, because setting channel_email = true
/// when encrypted_email is present is always correct behavior.
/// No security risk: it only enables the email channel that was already
/// the sole delivery channel before multi-channel support.
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
