use anchor_lang::prelude::*;

/// Herald's USDC/USDT treasury vault.
///
/// PDA Seeds: `["vault"]`
/// Holds accumulated subscription payments.
/// Only withdrawable by HERALD_AUTHORITY to HERALD_TREASURY.
#[account]
#[derive(InitSpace)]
pub struct SubscriptionVaultAccount {
    /// Herald treasury authority (Squads 2-of-3 multisig).
    pub authority: Pubkey, // 32

    /// Total USDC accumulated (6-decimal base units).
    pub total_usdc_collected: u64, // 8

    /// Total USDT accumulated (6-decimal base units).
    pub total_usdt_collected: u64, // 8

    /// Last withdrawal timestamp.
    pub last_withdrawal_at: i64, // 8

    /// Total withdrawal count (for audit trail).
    pub withdrawal_count: u32, // 4

    /// PDA bump.
    pub bump: u8, // 1
}
