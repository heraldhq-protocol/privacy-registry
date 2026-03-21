use anchor_lang::prelude::*;

/// User identity account storing encrypted email and notification preferences.
///
/// PDA Seeds: `["identity", owner.key().as_ref()]`
/// Space: 8 (discriminator) + fields ≈ 342 bytes; allocated via InitSpace.
#[account]
#[derive(InitSpace)]
pub struct IdentityAccount {
    /// The wallet that owns this identity.
    pub owner: Pubkey, // 32

    /// NaCl-encrypted email address (max 200 bytes).
    #[max_len(200)]
    pub encrypted_email: Vec<u8>, // 4 + 200

    /// SHA-256 hash of the plaintext email (for change detection without decryption).
    pub email_hash: [u8; 32], // 32

    /// NaCl encryption nonce.
    pub nonce: [u8; 24], // 24

    /// Unix timestamp of initial registration.
    pub registered_at: i64, // 8

    // ── Opt-in preferences ──────────────────────────────────
    /// Global opt-in for all notification categories.
    pub opt_in_all: bool, // 1

    /// Opt-in for DeFi notifications.
    pub opt_in_defi: bool, // 1

    /// Opt-in for governance notifications.
    pub opt_in_governance: bool, // 1

    /// Opt-in for marketing notifications.
    pub opt_in_marketing: bool, // 1

    /// When true, deliver notifications in a daily digest instead of real-time.
    pub digest_mode: bool, // 1

    /// PDA bump seed.
    pub bump: u8, // 1
}

impl IdentityAccount {
    /// Returns true if this identity should receive a notification for the given category.
    /// Category 3 (system) always bypasses opt-in.
    pub fn should_receive(&self, category: u8) -> bool {
        if !self.opt_in_all {
            return false;
        }
        match category {
            0 => self.opt_in_defi,
            1 => self.opt_in_governance,
            2 => self.opt_in_marketing,
            3 => true, // system notifications bypass opt-in
            _ => false,
        }
    }
}
