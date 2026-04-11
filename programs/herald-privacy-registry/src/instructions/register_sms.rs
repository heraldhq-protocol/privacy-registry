use anchor_lang::prelude::*;

use crate::constants::{IDENTITY_SEED, MAX_ENCRYPTED_PHONE_LEN};
use crate::errors::HeraldError;
use crate::events::SmsRegistered;
use crate::state::IdentityAccount;

/// Accounts required for `register_sms`.
///
/// Registers or updates the SMS channel for an existing identity.
/// The owner calls this AFTER:
///   1. Verifying phone ownership via OTP (Admin API)
///   2. Encrypting the E.164 phone number client-side with their wallet key
///
/// SMS channel is used by verified protocols to deliver important
/// messages and OTPs to users via Herald.
#[derive(Accounts)]
pub struct RegisterSms<'info> {
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
    ctx: Context<RegisterSms>,
    encrypted_phone: Vec<u8>,
    phone_hash: [u8; 32],
    nonce_sms: [u8; 24],
) -> Result<()> {
    require!(!encrypted_phone.is_empty(), HeraldError::PhoneEmpty);
    require!(
        encrypted_phone.len() <= MAX_ENCRYPTED_PHONE_LEN,
        HeraldError::PhoneTooLong
    );

    let now = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;
    let account = &mut ctx.accounts.identity_account;

    account.encrypted_phone = encrypted_phone;
    account.phone_hash = phone_hash;
    account.nonce_sms = nonce_sms;
    account.channel_sms = true;

    emit!(SmsRegistered {
        wallet: account.owner,
        phone_hash,
        timestamp: now,
    });

    Ok(())
}
