use anchor_lang::prelude::*;

use crate::constants::HERALD_AUTHORITY;
use crate::errors::HeraldError;
use crate::events::ProtocolDeactivated;
use crate::state::ProtocolRegistryAccount;

#[derive(Accounts)]
pub struct DeactivateProtocol<'info> {
    #[account(
        constraint = authority.key() == HERALD_AUTHORITY @ HeraldError::Unauthorized,
    )]
    pub authority: Signer<'info>,

    #[account(
        mut,
        constraint = protocol_account.is_active @ HeraldError::ProtocolAlreadyDeactivated,
    )]
    pub protocol_account: Account<'info, ProtocolRegistryAccount>,
}

pub fn handler(ctx: Context<DeactivateProtocol>) -> Result<()> {
    let protocol = &mut ctx.accounts.protocol_account;
    protocol.is_active = false;

    emit!(ProtocolDeactivated {
        protocol: protocol.owner,
        timestamp: Clock::get()
            .map_err(|_| error!(HeraldError::ClockUnavailable))?
            .unix_timestamp,
    });

    Ok(())
}
