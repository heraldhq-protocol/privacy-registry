use anchor_lang::prelude::*;
use light_sdk::{derive_light_cpi_signer, CpiSigner};

/// Herald backend authority pubkey.
/// This key is controlled by the Herald backend (stored in AWS KMS).
/// ⚠️ Replace with the actual authority pubkey before mainnet deployment.
pub const HERALD_AUTHORITY: Pubkey =
    Pubkey::from_str_const("AyMjTKQcZh2WF2DG63kwR8yNvSLv8EfYzZgA1geZnXVL");

/// USDC mint on Solana mainnet.
pub const USDC_MINT: Pubkey =
    Pubkey::from_str_const("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

/// USDT mint on Solana mainnet.
pub const USDT_MINT: Pubkey =
    Pubkey::from_str_const("Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB");

/// Herald treasury multisig (Squads 2-of-3).
/// All subscription payments accumulate here.
/// ⚠️ Replace with the actual Squads multisig pubkey before mainnet deployment.
/// TODO(#prod): Replace with real Squads multisig PDA. Current value is a placeholder
/// that matches HERALD_AUTHORITY for testing — this defeats the multisig separation.
pub const HERALD_TREASURY: Pubkey =
    Pubkey::from_str_const("AyMjTKQcZh2WF2DG63kwR8yNvSLv8EfYzZgA1geZnXVL");

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

/// USDC price per tier per billing period (6-decimal base units).
///   Tier 0 (dev):        FREE
///   Tier 1 (growth):     $99.00 USDC
///   Tier 2 (scale):     $299.00 USDC
///   Tier 3 (enterprise): $999.00 USDC
pub const TIER_PRICES_USDC: [u64; 4] = [
    0,
    99_000_000,  // $99.00
    299_000_000, // $299.00
    999_000_000, // $999.00
];

/// Maximum batch size for write_receipt (Light Protocol limit per tx).
pub const MAX_RECEIPT_BATCH: usize = 10;

// ── PDA seeds ───────────────────────────────────────────────────────────────

pub const IDENTITY_SEED: &[u8] = b"identity";
pub const PROTOCOL_SEED: &[u8] = b"protocol";
pub const VAULT_SEED: &[u8] = b"vault";
