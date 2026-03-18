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

/// Emitted when a protocol's subscription is renewed (by Herald authority).
#[event]
pub struct SubscriptionRenewed {
    pub protocol: Pubkey,
    pub tier: u8,
    pub new_expiry: i64,
    pub periods_paid: u32,
    pub timestamp: i64,
}

/// Emitted when a subscription renewal is marked as overdue by Herald.
#[event]
pub struct SubscriptionExpiredEvent {
    pub protocol: Pubkey,
    pub expired_at: i64,
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
