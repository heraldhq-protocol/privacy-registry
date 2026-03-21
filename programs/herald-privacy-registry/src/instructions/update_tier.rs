use anchor_lang::prelude::*;

use crate::constants::HERALD_AUTHORITY;
use crate::errors::HeraldError;
use crate::events::ProtocolTierUpdated;
use crate::state::ProtocolRegistryAccount;

/// Accounts for `update_protocol_tier`.
#[derive(Accounts)]
pub struct UpdateProtocolTier<'info> {
    #[account(
        constraint = authority.key() == HERALD_AUTHORITY @ HeraldError::Unauthorized,
    )]
    pub authority: Signer<'info>,

    #[account(mut)]
    pub protocol_account: Account<'info, ProtocolRegistryAccount>,
}

/// Update a protocol's tier level. Only callable by the Herald authority.
pub fn handler(ctx: Context<UpdateProtocolTier>, new_tier: u8) -> Result<()> {
    require!(new_tier <= 3, HeraldError::InvalidTier);

    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;

    let protocol = &mut ctx.accounts.protocol_account;
    let old_tier = protocol.tier;
    protocol.tier = new_tier;

    emit!(ProtocolTierUpdated {
        protocol: protocol.owner,
        old_tier,
        new_tier,
        timestamp: now,
    });

    Ok(())
}
