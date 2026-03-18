use anchor_lang::prelude::*;
use light_sdk::{derive_light_cpi_signer, CpiSigner};

/// Herald backend authority pubkey.
/// This key is controlled by the Herald backend (stored in AWS KMS).
/// ⚠️ Replace with the actual authority pubkey before mainnet deployment.
pub const HERALD_AUTHORITY: Pubkey =
    solana_program::pubkey!("HERALDxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxAAAA");

/// Maximum allowed length for the encrypted email field in bytes.
pub const MAX_ENCRYPTED_EMAIL_LEN: usize = 200;

/// Light Protocol CPI signer, derived at compile-time from the program ID.
pub const LIGHT_CPI_SIGNER: CpiSigner =
    derive_light_cpi_signer!("2pxjAf8tLCakKVDuN4vY51B5TeaEQk4koPuk9NZvWqdf");

// ── Subscription billing constants ──────────────────────────────────────────

/// Monthly subscription duration in seconds (30 days).
pub const SUBSCRIPTION_PERIOD_SECS: i64 = 30 * 24 * 60 * 60;

/// Maximum sends per billing period by tier.
///   Tier 0 (dev):        1,000 sends / month
///   Tier 1 (growth):    50,000 sends / month
///   Tier 2 (scale):    250,000 sends / month
///   Tier 3 (enterprise): 1,000,000 sends / month
pub const TIER_SEND_LIMITS: [u64; 4] = [1_000, 50_000, 250_000, 1_000_000];
