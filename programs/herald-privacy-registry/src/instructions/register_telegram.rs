use anchor_lang::prelude::*;

use crate::constants::{IDENTITY_SEED, MAX_ENCRYPTED_TELEGRAM_ID_LEN};
use crate::errors::HeraldError;
use crate::events::TelegramRegistered;
use crate::state::IdentityAccount;

/// Accounts required for `register_telegram`.
///
/// Registers or updates the Telegram channel for an existing identity.
/// The owner calls this AFTER:
///   1. Opening a chat with @HeraldBot in Telegram
///   2. Receiving their telegram_chat_id from the bot
///   3. Encrypting the chat_id client-side with their wallet key
///
/// This is a separate instruction from register_identity so:
///   - Users can add Telegram to an existing email-only registration
///   - Channel additions don't require re-encrypting the email
///   - Telegram can be added/removed independently
#[derive(Accounts)]
pub struct RegisterTelegram<'info> {
    #[account(mut)]
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
    ctx: Context<RegisterTelegram>,
    encrypted_telegram_id: Vec<u8>,
    telegram_id_hash: [u8; 32],
    nonce_telegram: [u8; 24],
) -> Result<()> {
    require!(
        !encrypted_telegram_id.is_empty(),
        HeraldError::TelegramIdEmpty
    );
    require!(
        encrypted_telegram_id.len() <= MAX_ENCRYPTED_TELEGRAM_ID_LEN,
        HeraldError::TelegramIdTooLong
    );

    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;
    let account = &mut ctx.accounts.identity_account;

    account.encrypted_telegram_id = encrypted_telegram_id;
    account.telegram_id_hash = telegram_id_hash;
    account.nonce_telegram = nonce_telegram;
    account.channel_telegram = true;

    emit!(TelegramRegistered {
        wallet: account.owner,
        telegram_id_hash,
        timestamp: now,
    });

    Ok(())
}
