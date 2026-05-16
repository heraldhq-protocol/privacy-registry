use anchor_lang::prelude::*;
use light_sdk::{derive_light_cpi_signer, CpiSigner};

// ╔═══════════════════════════════════════════════════════════════════════════╗
// ║  TODO(#prod): PRE-MAINNET CREDENTIAL CHECKLIST                          ║
// ║                                                                          ║
// ║  Before deploying to mainnet, replace ALL placeholder values below:      ║
// ║                                                                          ║
// ║  1. HERALD_AUTHORITY  — Set to real KMS-backed authority pubkey          ║
// ║                         Must match herald-sdk-ts/src/constants.ts        ║
// ║                                                                          ║
// ║  Also update in herald-sdk-ts/src/constants.ts:                         ║
// ║  2. HERALD_ENCLAVE_WRAPPING_PUBKEY — Real X25519 enclave pubkey         ║
// ║  3. HERALD_AUTHORITY               — Must match this file               ║
// ║  4. HERALD_RECEIPT_MERKLE_TREE     — After Light Protocol tree init     ║
// ╚═══════════════════════════════════════════════════════════════════════════╝

/// Herald backend authority pubkey.
/// This key is controlled by the Herald backend (stored in AWS KMS).
/// Must match HERALD_AUTHORITY in herald-sdk-ts/src/constants.ts.
pub const HERALD_AUTHORITY: Pubkey =
    Pubkey::from_str_const("4K2GVdXoetgHoho5NJzHo5J5YazPNR1t6GS7jPpxUREA");

/// Light Protocol CPI signer, derived at compile-time from the program ID.
pub const LIGHT_CPI_SIGNER: CpiSigner =
    derive_light_cpi_signer!("2pxjAf8tLCakKVDuN4vY51B5TeaEQk4koPuk9NZvWqdf");

/// Maximum allowed length for the encrypted email field in bytes.
pub const MAX_ENCRYPTED_EMAIL_LEN: usize = 200;

/// Maximum allowed length for the encrypted Telegram chat_id field in bytes.
/// 32 (ephemeral pubkey) + ~16 (NaCl overhead) + max 10 char chat_id ≈ 80 bytes.
pub const MAX_ENCRYPTED_TELEGRAM_ID_LEN: usize = 80;

/// Maximum allowed length for the encrypted E.164 phone number field in bytes.
/// 32 (ephemeral pubkey) + ~16 (NaCl overhead) + max 15 char E.164 ≈ 65 bytes.
pub const MAX_ENCRYPTED_PHONE_LEN: usize = 65;

// ── Subscription billing constants ──────────────────────────────────────────

/// Monthly subscription duration in seconds (30 days).
pub const SUBSCRIPTION_PERIOD_SECS: i64 = 30 * 24 * 60 * 60;

/// Maximum sends per billing period by tier.
///   Tier 0 (dev):        1,000 sends / month
///   Tier 1 (growth):    50,000 sends / month
///   Tier 2 (scale):    250,000 sends / month
///   Tier 3 (enterprise): 1,000,000 sends / month
pub const TIER_SEND_LIMITS: [u64; 4] = [1_000, 50_000, 250_000, 1_000_000];

/// Maximum batch size for write_receipt (Light Protocol limit per tx).
pub const MAX_RECEIPT_BATCH: usize = 10;

// ── PDA seeds ───────────────────────────────────────────────────────────────

pub const IDENTITY_SEED: &[u8] = b"identity";
pub const PROTOCOL_SEED: &[u8] = b"protocol";

// ── Notification Key constants ───────────────────────────────────

/// Maximum notification key rotations before the user must revoke and re-register.
/// Prevents unbounded state growth and rotation spam.
pub const MAX_NOTIFICATION_KEY_ROTATIONS: u32 = 1_000;

/// Current supported notification key version.
/// Bump this when the sealing format changes.
pub const NOTIFICATION_KEY_VERSION: u8 = 1;
