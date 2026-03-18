use anchor_lang::prelude::*;

use crate::constants::HERALD_AUTHORITY;
use crate::errors::HeraldError;
use crate::events::ProtocolReactivated;
use crate::state::ProtocolRegistryAccount;

/// Accounts for `reactivate_protocol`.
#[derive(Accounts)]
pub struct ReactivateProtocol<'info> {
    #[account(
        constraint = authority.key() == HERALD_AUTHORITY @ HeraldError::Unauthorized,
    )]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = !protocol_account.is_active @ HeraldError::ProtocolAlreadyActive,
        constraint = !protocol_account.is_suspended @ HeraldError::ProtocolSuspended,
    )]
    pub protocol_account: Account<'info, ProtocolRegistryAccount>,
}

/// Reactivate a deactivated (but not suspended) protocol.
///
/// Note: this only sets `is_active = true`. It does NOT renew the subscription.
/// Reactivating without a valid subscription will immediately fail on `write_receipt`
/// due to the subscription check there.
pub fn handler(ctx: Context<ReactivateProtocol>) -> Result<()> {
    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;

    let protocol = &mut ctx.accounts.protocol_account;
    protocol.is_active = true;

    emit!(ProtocolReactivated {
        protocol: protocol.owner,
        timestamp: now,
    });

    Ok(())
}
