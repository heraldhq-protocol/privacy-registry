use anchor_lang::prelude::*;

use crate::constants::HERALD_AUTHORITY;
use crate::errors::HeraldError;
use crate::events::PeriodReset;
use crate::state::ProtocolRegistryAccount;

/// Accounts for `reset_protocol_sends`.
#[derive(Accounts)]
pub struct ResetProtocolSends<'info> {
    #[account(
        constraint = authority.key() == HERALD_AUTHORITY @ HeraldError::Unauthorized,
    )]
    pub authority: Signer<'info>,

    /// The protocol to reset. The authority constraint ensures this is
    /// a legitimate Herald-owned operation. No additional constraint is
    /// needed on the protocol account itself beyond that it exists.
    #[account(mut)]
    pub protocol_account: Account<'info, ProtocolRegistryAccount>,
}

/// Reset a protocol's sends counter at the end of a billing period.
///
/// # Security
/// Only callable by `HERALD_AUTHORITY`. Emits `PeriodReset` with the
/// last period's send count preserved in the event log for off-chain auditing.
pub fn handler(ctx: Context<ResetProtocolSends>) -> Result<()> {
    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;

    let protocol = &mut ctx.accounts.protocol_account;
    let sends_last_period = protocol.sends_this_period;

    protocol.sends_this_period = 0;

    emit!(PeriodReset {
        protocol: protocol.owner,
        sends_last_period,
        tier: protocol.tier,
        timestamp: now,
    });

    Ok(())
}
