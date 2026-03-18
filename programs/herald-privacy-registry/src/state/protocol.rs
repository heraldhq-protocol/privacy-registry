use anchor_lang::prelude::*;

/// On-chain registration and subscription record for a DeFi protocol.
///
/// PDA Seeds: `["protocol", protocol_pubkey.as_ref()]`
/// Allocated space: 256 bytes (8 discriminator + fields + padding)
#[account]
#[derive(InitSpace)]
pub struct ProtocolRegistryAccount {
    // ── Identity ────────────────────────────────────────────
    /// Protocol admin wallet address (the protocol's on-chain identity).
    pub owner: Pubkey, // 32

    /// SHA-256 hash of the protocol name (actual name stored off-chain).
    pub name_hash: [u8; 32], // 32

    // ── Tier / Subscription ─────────────────────────────────
    /// Tier level: 0=dev, 1=growth, 2=scale, 3=enterprise.
    pub tier: u8, // 1

    /// Unix timestamp when the current subscription period expires.
    /// 0 means not yet active (registered but not yet subscribed).
    pub subscription_expires_at: i64, // 8

    /// Unix timestamp of the last subscription renewal.
    pub last_renewed_at: i64, // 8

    /// Total number of complete billing periods successfully paid.
    pub periods_paid: u32, // 4

    // ── Usage ───────────────────────────────────────────────
    /// Number of sends consumed in the current billing period.
    pub sends_this_period: u64, // 8

    // ── State Flags ─────────────────────────────────────────
    /// Whether this protocol is allowed to send notifications.
    /// Set to false on deactivation or subscription lapse.
    pub is_active: bool, // 1

    /// Whether the protocol has been explicitly suspended by Herald (not just lapsed).
    pub is_suspended: bool, // 1

    // ── Timestamps ──────────────────────────────────────────
    /// Unix timestamp of initial registration.
    pub registered_at: i64, // 8

    /// PDA bump seed.
    pub bump: u8, // 1
}

impl ProtocolRegistryAccount {
    /// Returns `true` if the subscription is currently valid (not expired).
    pub fn subscription_is_current(&self, now: i64) -> bool {
        self.subscription_expires_at > now
    }

    /// Returns the maximum sends allowed for this tier.
    /// Uses the TIER_SEND_LIMITS constant array.
    pub fn sends_limit(&self) -> u64 {
        crate::constants::TIER_SEND_LIMITS[self.tier as usize]
    }

    /// Returns `true` if this protocol can send a notification right now.
    pub fn can_send(&self, now: i64) -> bool {
        self.is_active
            && !self.is_suspended
            && self.subscription_is_current(now)
            && self.sends_this_period < self.sends_limit()
    }
}
