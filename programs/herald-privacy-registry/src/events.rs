use anchor_lang::prelude::*;

// ═══════════════════════════════════════════════════════════
//  IDENTITY EVENTS
// ═══════════════════════════════════════════════════════════

/// Emitted when a new user identity is created.
#[event]
pub struct IdentityRegistered {
    pub wallet: Pubkey,
    pub email_hash: [u8; 32],
    pub opt_in_all: bool,
    pub opt_in_defi: bool,
    pub opt_in_governance: bool,
    pub opt_in_marketing: bool,
    pub digest_mode: bool,
    pub timestamp: i64,
}

/// Emitted when a user updates their identity.
#[event]
pub struct IdentityUpdated {
    pub wallet: Pubkey,
    pub email_changed: bool,
    pub preferences_changed: bool,
    pub timestamp: i64,
}

/// Emitted when notification preferences specifically change.
#[event]
pub struct PreferencesUpdated {
    pub wallet: Pubkey,
    pub opt_in_all: bool,
    pub opt_in_defi: bool,
    pub opt_in_governance: bool,
    pub opt_in_marketing: bool,
    pub digest_mode: bool,
    pub timestamp: i64,
}

/// Emitted when a user deletes their identity account.
#[event]
pub struct IdentityDeleted {
    pub wallet: Pubkey,
    pub timestamp: i64,
}

// ═══════════════════════════════════════════════════════════
//  PROTOCOL LIFECYCLE EVENTS
// ═══════════════════════════════════════════════════════════

/// Emitted when a new DeFi protocol is registered.
#[event]
pub struct ProtocolRegistered {
    pub protocol: Pubkey,
    pub name_hash: [u8; 32],
    pub tier: u8,
    pub timestamp: i64,
}

/// Emitted when a protocol's tier is changed.
#[event]
pub struct ProtocolTierUpdated {
    pub protocol: Pubkey,
    pub old_tier: u8,
    pub new_tier: u8,
    pub timestamp: i64,
}

/// Emitted when a protocol is deactivated.
#[event]
pub struct ProtocolDeactivated {
    pub protocol: Pubkey,
    pub timestamp: i64,
}

/// Emitted when a previously deactivated protocol is reactivated.
#[event]
pub struct ProtocolReactivated {
    pub protocol: Pubkey,
    pub timestamp: i64,
}

/// Emitted when a protocol is suspended by Herald.
#[event]
pub struct ProtocolSuspended {
    pub protocol: Pubkey,
    pub timestamp: i64,
}

// ═══════════════════════════════════════════════════════════
//  SUBSCRIPTION / BILLING EVENTS
// ═══════════════════════════════════════════════════════════

/// Emitted by `renew_subscription` (Helio path).
#[event]
pub struct SubscriptionRenewed {
    pub protocol: Pubkey,
    pub tier: u8,
    pub new_expiry: i64,
    pub periods_paid: u32,
    /// Zero when called via authority (Helio); actual USDC amount for on-chain payments.
    pub usdc_paid: u64,
    /// "helio_webhook" | "on_chain_usdc" | "on_chain_usdt" (padded to 20 bytes).
    pub payment_source: [u8; 20],
    pub timestamp: i64,
}

/// Emitted at the end of a billing period when the sends counter is reset.
#[event]
pub struct PeriodReset {
    pub protocol: Pubkey,
    pub sends_last_period: u64,
    pub tier: u8,
    pub timestamp: i64,
}

// ═══════════════════════════════════════════════════════════
//  NOTIFICATION / RECEIPT EVENTS
// ═══════════════════════════════════════════════════════════

/// Emitted when a ZK-compressed delivery receipt is written.
#[event]
pub struct NotificationDelivered {
    pub protocol: Pubkey,
    pub recipient_hash: [u8; 32],
    pub notification_id: [u8; 16],
    pub category: u8,
    pub sends_this_period: u64,
    pub timestamp: i64,
}

/// Emitted on every send so off-chain indexers can track usage.
#[event]
pub struct ProtocolSendRecorded {
    pub protocol: Pubkey,
    pub sends_this_period: u64,
    pub sends_limit: u64,
    pub timestamp: i64,
}

// ═══════════════════════════════════════════════════════════
//  CHANNEL EVENTS
// ═══════════════════════════════════════════════════════════

/// Emitted when a Telegram channel is registered or updated for an identity.
#[event]
pub struct TelegramRegistered {
    pub wallet: Pubkey,
    pub telegram_id_hash: [u8; 32],
    pub timestamp: i64,
}

/// Emitted when an SMS channel is registered or updated for an identity.
#[event]
pub struct SmsRegistered {
    pub wallet: Pubkey,
    pub phone_hash: [u8; 32],
    pub timestamp: i64,
}

/// Emitted when channel enable/disable flags are toggled.
#[event]
pub struct ChannelSettingsUpdated {
    pub wallet: Pubkey,
    pub channel_email: bool,
    pub channel_telegram: bool,
    pub channel_sms: bool,
    pub timestamp: i64,
}

/// Emitted when a channel's encrypted data is permanently removed (GDPR erasure).
#[event]
pub struct ChannelRemoved {
    pub wallet: Pubkey,
    /// 0 = Telegram, 1 = SMS
    pub channel: u8,
    pub timestamp: i64,
}

// ═══════════════════════════════════════════════════════════
//  NOTIFICATION KEY EVENTS
// ═══════════════════════════════════════════════════════════

/// Emitted when a user registers their sealed X25519 notification key.
#[event]
pub struct NotificationKeyRegistered {
    pub wallet: Pubkey,
    pub version: u8,
    pub registered_at: i64,
}

/// Emitted when a user rotates their sealed notification key.
#[event]
pub struct NotificationKeyRotated {
    pub wallet: Pubkey,
    pub version: u8,
    pub rotation_count: u32,
    pub rotated_at: i64,
}

/// Emitted when a user revokes (zeroes out) their notification key.
#[event]
pub struct NotificationKeyRevoked {
    pub wallet: Pubkey,
    pub revoked_at: i64,
}
