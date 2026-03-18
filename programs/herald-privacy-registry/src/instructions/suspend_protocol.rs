use anchor_lang::prelude::*;

use crate::constants::HERALD_AUTHORITY;
use crate::errors::HeraldError;
use crate::events::ProtocolSuspended as ProtocolSuspendedEvent;
use crate::state::ProtocolRegistryAccount;

/// Accounts for `suspend_protocol`.
#[derive(Accounts)]
pub struct SuspendProtocol<'info> {
    #[account(
        constraint = authority.key() == HERALD_AUTHORITY @ HeraldError::Unauthorized,
    )]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub protocol_account: Account<'info, ProtocolRegistryAccount>,
}

/// Hard-suspend a protocol (e.g. for ToS violation, fraud, or AML).
/// Unlike deactivation (which is a soft state), suspension blocks `renew_subscription`
/// and requires a separate `unsuspend_protocol` call.
pub fn handler(ctx: Context<SuspendProtocol>) -> Result<()> {
    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;

    let protocol = &mut ctx.accounts.protocol_account;
    protocol.is_suspended = true;
    protocol.is_active = false;

    emit!(ProtocolSuspendedEvent {
        protocol: protocol.owner,
        timestamp: now,
    });

    Ok(())
}
