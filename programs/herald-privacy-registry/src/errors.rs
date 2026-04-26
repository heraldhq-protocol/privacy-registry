use anchor_lang::prelude::*;

#[error_code]
pub enum HeraldError {
    // ── Identity Errors (6000–6004) ────────────────────────────
    #[msg("Encrypted email exceeds maximum length of 200 bytes")]
    EmailTooLong, // 6000

    #[msg("Encrypted email must not be empty")]
    EmailEmpty, // 6001

    #[msg("Email hash must be exactly 32 bytes (SHA-256)")]
    InvalidEmailHash, // 6002

    #[msg("Nonce must be exactly 24 bytes")]
    InvalidNonce, // 6003

    #[msg("Update must modify at least one field")]
    EmptyUpdate, // 6004

    // ── Authorization Errors (6005–6006) ────────────────────────
    #[msg("Unauthorized: signer does not match required authority")]
    Unauthorized, // 6005

    #[msg("Unauthorized: signer does not own this identity account")]
    OwnerMismatch, // 6006

    // ── Protocol Lifecycle Errors (6007–6011) ───────────────────
    #[msg("Invalid tier: must be 0 (dev), 1 (growth), 2 (scale), or 3 (enterprise)")]
    InvalidTier, // 6007

    #[msg("Protocol is not active")]
    ProtocolInactive, // 6008

    #[msg("Protocol is already deactivated")]
    ProtocolAlreadyDeactivated, // 6009

    #[msg("Protocol has been suspended by Herald and cannot send notifications")]
    ProtocolSuspended, // 6010

    #[msg("Protocol is already active; no need to reactivate")]
    ProtocolAlreadyActive, // 6011

    // ── Subscription / Billing Errors (6012–6018) ───────────────
    #[msg("Protocol subscription has expired; renew to continue sending")]
    SubscriptionExpired, // 6012

    #[msg("Protocol has not yet subscribed; subscription_expires_at is zero")]
    SubscriptionNotActive, // 6013

    #[msg("Protocol has reached the maximum sends for this billing period")]
    SendsLimitExceeded, // 6014

    #[msg("Protocol sends counter would overflow")]
    SendsOverflow, // 6015

    #[msg("New subscription expiry must be in the future")]
    InvalidSubscriptionExpiry, // 6016

    #[msg("Developer tier is free; payment not required")]
    DevTierNoPayment, // 6017

    #[msg("Unsupported payment token; must be USDC or USDT")]
    UnsupportedPaymentToken, // 6018

    // ── Receipt / Notification Errors (6019–6021) ───────────────
    #[msg("Invalid category: must be 0 (DeFi), 1 (Governance), 2 (Marketing), or 3 (Other)")]
    InvalidCategory, // 6019

    #[msg("Recipient hash must be exactly 32 bytes (SHA-256)")]
    InvalidRecipientHash, // 6020

    #[msg("Notification ID must be exactly 16 bytes (UUID v4)")]
    InvalidNotificationId, // 6021

    // ── Light Protocol / CPI Errors (6022–6024) ─────────────────
    #[msg("Failed to initialise Light Protocol CPI accounts")]
    LightCpiAccountsError, // 6022

    #[msg("Failed to attach compressed account to Light CPI")]
    LightAccountError, // 6023

    #[msg("Light Protocol CPI invocation failed")]
    LightCpiInvocationError, // 6024

    // ── General (6025–6026) ─────────────────────────────────────
    #[msg("Arithmetic overflow")]
    Overflow, // 6025

    #[msg("Clock sysvar unavailable")]
    ClockUnavailable, // 6026

    // ── Telegram errors (6027–6029) ──────────────────────────────
    #[msg("Encrypted Telegram ID must not be empty")]
    TelegramIdEmpty, // 6027

    #[msg("Encrypted Telegram ID exceeds maximum length of 80 bytes")]
    TelegramIdTooLong, // 6028

    #[msg("Telegram channel not registered; add Telegram before enabling it")]
    TelegramNotRegistered, // 6029

    // ── SMS errors (6030–6032) ───────────────────────────────────
    #[msg("Encrypted phone number must not be empty")]
    PhoneEmpty, // 6030

    #[msg("Encrypted phone number exceeds maximum length of 65 bytes")]
    PhoneTooLong, // 6031

    #[msg("SMS channel not registered; add phone before enabling it")]
    SmsNotRegistered, // 6032

    // ── General channel errors (6033–6034) ──────────────────────
    #[msg("At least one delivery channel must remain active")]
    NoActiveChannels, // 6033

    #[msg("Invalid channel type: must be 0 (Telegram) or 1 (SMS)")]
    InvalidChannelType, // 6034

    // ── Notification Key Errors (6035–6041) ──────────────────────
    #[msg("Sealed pubkey cannot be all zeros — invalid key material")]
    ZeroSealedPubkey, // 6035

    #[msg("Notification nonce cannot be all zeros — replay attack risk")]
    ZeroNotificationNonce, // 6036

    #[msg("Unsupported notification key version")]
    UnsupportedNotificationKeyVersion, // 6037

    #[msg("Max notification key rotations reached — revoke and re-register")]
    MaxNotificationKeyRotationsExceeded, // 6038

    #[msg("Notification nonce must differ from current nonce on rotation")]
    NotificationNonceReuse, // 6039

    #[msg("No notification key registered — cannot rotate or revoke")]
    NotificationKeyNotRegistered, // 6040

    #[msg("Identity account must be registered before adding a notification key")]
    IdentityNotRegistered, // 6041
}
