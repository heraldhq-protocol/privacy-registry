use anchor_lang::prelude::*;

use crate::errors::HeraldError;
use crate::events::IdentityDeleted;
use crate::state::IdentityAccount;

/// Accounts required for `delete_identity`.
#[derive(Accounts)]
pub struct DeleteIdentity<'info> {
    /// The wallet that owns this identity; receives rent refund.
    #[account(mut)]
    pub owner: Signer<'info>,

    /// Identity PDA – closed and rent returned to `owner`.
    #[account(
        mut,
        seeds = [b"identity", owner.key().as_ref()],
        bump = identity_account.bump,
        close = owner,
    )]
    pub identity_account: Account<'info, IdentityAccount>,
}

/// Delete (close) a user identity account. Rent is refunded to the owner.
pub fn handler(ctx: Context<DeleteIdentity>) -> Result<()> {
    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;

    emit!(IdentityDeleted {
        wallet: ctx.accounts.owner.key(),
        timestamp: now,
    });

    Ok(())
}
