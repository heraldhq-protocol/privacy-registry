use anchor_lang::prelude::*;

use crate::constants::IDENTITY_SEED;
use crate::errors::HeraldError;
use crate::events::ChannelSettingsUpdated;
use crate::state::IdentityAccount;

/// Toggle individual channels on/off without modifying the encrypted data.
/// Use case: user wants to pause SMS but keep the phone stored.
#[derive(Accounts)]
pub struct UpdateChannelSettings<'info> {
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds  = [IDENTITY_SEED, owner.key().as_ref()],
        bump   = identity_account.bump,
        constraint = identity_account.owner == owner.key() @ HeraldError::OwnerMismatch,
    )]
    pub identity_account: Account<'info, IdentityAccount>,
}

pub fn handler(
    ctx: Context<UpdateChannelSettings>,
    channel_email: Option<bool>,
    channel_telegram: Option<bool>,
    channel_sms: Option<bool>,
) -> Result<()> {
    let account = &mut ctx.accounts.identity_account;

    if let Some(v) = channel_email {
        account.channel_email = v;
    }
    if let Some(v) = channel_telegram {
        // Can only enable telegram if encrypted_telegram_id is present
        if v {
            require!(
                !account.encrypted_telegram_id.is_empty(),
                HeraldError::TelegramNotRegistered
            );
        }
        account.channel_telegram = v;
    }
    if let Some(v) = channel_sms {
        // Can only enable sms if encrypted_phone is present
        if v {
            require!(
                !account.encrypted_phone.is_empty(),
                HeraldError::SmsNotRegistered
            );
        }
        account.channel_sms = v;
    }

    // Must have at least one active channel
    require!(account.has_any_channel(), HeraldError::NoActiveChannels);

    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;

    emit!(ChannelSettingsUpdated {
        wallet: account.owner,
        channel_email: account.channel_email,
        channel_telegram: account.channel_telegram,
        channel_sms: account.channel_sms,
        timestamp: now,
    });

    Ok(())
}
