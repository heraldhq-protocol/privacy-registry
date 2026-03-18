use anchor_lang::prelude::*;

#[error_code]
pub enum HeraldError {
    // ── Identity Errors ─────────────────────────────────────
    #[msg("Encrypted email exceeds maximum length of 200 bytes")]
    EmailTooLong,

    #[msg("Encrypted email must not be empty")]
    EmailEmpty,

    #[msg("Email hash must be exactly 32 bytes (SHA-256)")]
    InvalidEmailHash,

    #[msg("Nonce must be exactly 24 bytes")]
    InvalidNonce,

    #[msg("Update must modify at least one field")]
    EmptyUpdate,

    // ── Authorization Errors ────────────────────────────────
    #[msg("Unauthorized: signer does not match required authority")]
    Unauthorized,

    #[msg("Unauthorized: signer does not own this identity account")]
    OwnerMismatch,

    // ── Protocol Lifecycle Errors ────────────────────────────
    #[msg("Invalid tier: must be 0 (dev), 1 (growth), 2 (scale), or 3 (enterprise)")]
    InvalidTier,

    #[msg("Protocol is not active")]
    ProtocolInactive,

    #[msg("Protocol is already deactivated")]
    ProtocolAlreadyDeactivated,

    #[msg("Protocol has been suspended by Herald and cannot send notifications")]
    ProtocolSuspended,

    // ── Subscription / Billing Errors ────────────────────────
    #[msg("Protocol subscription has expired; renew to continue sending")]
    SubscriptionExpired,

    #[msg("Protocol has not yet subscribed; subscription_expires_at is zero")]
    SubscriptionNotActive,

    #[msg("Protocol has reached the maximum sends for this billing period")]
    SendsLimitExceeded,

    #[msg("Protocol sends counter would overflow")]
    SendsOverflow,

    #[msg("New subscription expiry must be in the future")]
    InvalidSubscriptionExpiry,

    #[msg("Protocol is already active; no need to reactivate")]
    ProtocolAlreadyActive,

    // ── Receipt / Notification Errors ───────────────────────
    #[msg("Invalid category: must be 0 (DeFi), 1 (Governance), 2 (Marketing), or 3 (Other)")]
    InvalidCategory,

    #[msg("Recipient hash must be exactly 32 bytes (SHA-256)")]
    InvalidRecipientHash,

    #[msg("Notification ID must be exactly 16 bytes (UUID v4)")]
    InvalidNotificationId,

    // ── Light Protocol / CPI Errors ─────────────────────────
    #[msg("Failed to initialise Light Protocol CPI accounts")]
    LightCpiAccountsError,

    #[msg("Failed to attach compressed account to Light CPI")]
    LightAccountError,

    #[msg("Light Protocol CPI invocation failed")]
    LightCpiInvocationError,

    // ── General ─────────────────────────────────────────────
    #[msg("Arithmetic overflow")]
    Overflow,

    #[msg("Clock sysvar unavailable")]
    ClockUnavailable,
}
