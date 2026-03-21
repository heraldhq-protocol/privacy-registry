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
}
