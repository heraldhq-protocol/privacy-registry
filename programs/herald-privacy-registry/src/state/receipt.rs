use anchor_lang::prelude::*;
use light_sdk::LightDiscriminator;

/// ZK-compressed delivery receipt stored as a Light Protocol compressed account.
///
/// This is NOT a standard Anchor account – it lives as a hashed leaf in a
/// Light Protocol State Merkle tree. The struct is serialised, hashed, and
/// appended via CPI to the Light System Program.
#[derive(
    Clone, Debug, Default, LightDiscriminator, AnchorSerialize, AnchorDeserialize, PartialEq,
)]
pub struct DeliveryReceipt {
    /// The protocol that triggered the notification.
    pub protocol_pubkey: Pubkey, // 32

    /// SHA-256 hash of the recipient's wallet pubkey.
    pub recipient_hash: [u8; 32], // 32

    /// UUID v4 notification identifier (16 bytes).
    pub notification_id: [u8; 16], // 16

    /// Unix timestamp of delivery.
    pub timestamp: i64, // 8

    /// Always `true` when written (proof of delivery).
    pub delivered: bool, // 1

    /// Notification category: 0 = DeFi, 1 = Governance, 2 = Marketing, 3 = Other.
    pub category: u8, // 1
}
