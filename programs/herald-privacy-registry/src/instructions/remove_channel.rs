use anchor_lang::prelude::*;

use crate::constants::IDENTITY_SEED;
use crate::errors::HeraldError;
use crate::events::ChannelRemoved;
use crate::state::IdentityAccount;

/// Channel type enum for per-channel removal (GDPR erasure).
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy)]
pub enum ChannelType {
    Telegram = 0,
    Sms = 1,
}

/// Permanently removes encrypted channel data.
/// Zeroes all encrypted fields for the specified channel.
/// Enforces at least one channel remains active.
#[derive(Accounts)]
pub struct RemoveChannel<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds  = [IDENTITY_SEED, owner.key().as_ref()],
        bump   = identity_account.bump,
        constraint = identity_account.owner == owner.key() @ HeraldError::OwnerMismatch,
    )]
    pub identity_account: Account<'info, IdentityAccount>,
}

pub fn handler(ctx: Context<RemoveChannel>, channel: ChannelType) -> Result<()> {
    let account = &mut ctx.accounts.identity_account;

    match channel {
        ChannelType::Telegram => {
            account.encrypted_telegram_id = vec![];
            account.telegram_id_hash = [0u8; 32];
            account.nonce_telegram = [0u8; 24];
            account.channel_telegram = false;
        }
        ChannelType::Sms => {
            account.encrypted_phone = vec![];
            account.phone_hash = [0u8; 32];
            account.nonce_sms = [0u8; 24];
            account.channel_sms = false;
        }
    }

    // Must have at least one active channel after removal
    require!(account.has_any_channel(), HeraldError::NoActiveChannels);

    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;

    emit!(ChannelRemoved {
        wallet: account.owner,
        channel: channel as u8,
        timestamp: now,
    });

    Ok(())
}
