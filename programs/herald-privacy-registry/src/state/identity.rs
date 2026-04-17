use anchor_lang::prelude::*;

/// User identity account storing encrypted email, channel data, and notification preferences.
///
/// PDA Seeds: `["identity", owner.key().as_ref()]`
///
/// BACKWARD COMPATIBILITY:
/// The existing fields are unchanged. New channel fields are appended.
/// Programs using InitSpace will auto-include new fields in space calculation.
///
/// PRIVACY INVARIANT:
/// ALL channel identifiers are stored as NaCl box ciphertext.
/// The pattern is identical for every channel:
///   [ephemeral_pubkey (32 bytes) || nacl_box_ciphertext]
/// Herald's Nitro Enclave decrypts using the wallet owner's X25519 key.
#[account]
#[derive(InitSpace)]
pub struct IdentityAccount {
    // ── Original fields (unchanged) ──────────────────────────────
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

    // ── Channel flags ─────────────────────────────────────────────
    /// Email channel enabled (default true — all existing accounts).
    pub channel_email: bool, // 1

    /// Telegram channel enabled.
    pub channel_telegram: bool, // 1

    /// SMS channel enabled.
    pub channel_sms: bool, // 1

    // ── Telegram channel ──────────────────────────────────────────
    /// NaCl box encrypted Telegram chat_id (int64 as string).
    /// Format: [ephemeral_pubkey(32) || box(chat_id_str, nonce_tg)].
    /// Empty Vec = not registered.
    /// Max 80 bytes: 32 ephemeral + ~16 overhead + max 10 char chat_id.
    #[max_len(80)]
    pub encrypted_telegram_id: Vec<u8>, // 4 + 80

    /// SHA-256 of the Telegram chat_id string.
    /// Allows Herald to detect chat_id changes without decrypting.
    pub telegram_id_hash: [u8; 32], // 32

    /// NaCl nonce for Telegram encryption (separate from email nonce).
    pub nonce_telegram: [u8; 24], // 24

    // ── SMS channel ───────────────────────────────────────────────
    /// NaCl box encrypted E.164 phone number (e.g. "+14155552671").
    /// Format: [ephemeral_pubkey(32) || box(phone_e164, nonce_sms)].
    /// Empty Vec = not registered.
    /// Max 65 bytes: 32 ephemeral + ~16 overhead + max 15 char E.164.
    #[max_len(65)]
    pub encrypted_phone: Vec<u8>, // 4 + 65

    /// SHA-256 of the E.164 phone number string.
    pub phone_hash: [u8; 32], // 32

    /// NaCl nonce for phone encryption.
    pub nonce_sms: [u8; 24], // 24
}

impl IdentityAccount {
    /// Total space with all channel extensions.
    /// Used for PDA allocation in register_identity.
    pub const SPACE: usize = 8      // discriminator
        + 32                        // owner
        + (4 + 200)                 // encrypted_email
        + 32                        // email_hash
        + 24                        // nonce
        + 8                         // registered_at
        + 5                         // opt_in_* + digest_mode
        + 1                         // bump
        + 3                         // channel_email/telegram/sms
        + (4 + 80)                  // encrypted_telegram_id
        + 32                        // telegram_id_hash
        + 24                        // nonce_telegram
        + (4 + 65)                  // encrypted_phone
        + 32                        // phone_hash
        + 24                        // nonce_sms
        + 64; // future-proof padding

    /// Returns true if this identity has at least one delivery channel configured.
    pub fn has_any_channel(&self) -> bool {
        (self.channel_email && !self.encrypted_email.is_empty())
            || (self.channel_telegram && !self.encrypted_telegram_id.is_empty())
            || (self.channel_sms && !self.encrypted_phone.is_empty())
    }

    /// Returns active channel count for analytics.
    pub fn active_channel_count(&self) -> u8 {
        let mut count = 0u8;
        if self.channel_email && !self.encrypted_email.is_empty() {
            count += 1;
        }
        if self.channel_telegram && !self.encrypted_telegram_id.is_empty() {
            count += 1;
        }
        if self.channel_sms && !self.encrypted_phone.is_empty() {
            count += 1;
        }
        count
    }

    /// Returns true if this identity should receive a notification for the given category.
    /// Extended to check channel availability in addition to opt-in.
    /// Category 3 (system) always bypasses opt-in.
    pub fn should_receive(&self, category: u8) -> bool {
        if !self.opt_in_all {
            return false;
        }
        let category_ok = match category {
            0 => self.opt_in_defi,
            1 => self.opt_in_governance,
            2 => self.opt_in_marketing,
            3 => true, // system notifications bypass opt-in
            _ => false,
        };
        category_ok && self.has_any_channel()
    }
}
