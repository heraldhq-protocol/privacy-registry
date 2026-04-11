use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod events;
pub mod instructions;
pub mod state;

use instructions::*;
use instructions::remove_channel::ChannelType;
use state::AnchorCompressedProof;

declare_id!("2pxjAf8tLCakKVDuN4vY51B5TeaEQk4koPuk9NZvWqdf");

#[program]
pub mod herald_privacy_registry {
    use super::*;

    // ═══════════════════════════════════════════════════════
    //  IDENTITY INSTRUCTIONS
    // ═══════════════════════════════════════════════════════

    /// Register a new user identity with encrypted email and notification preferences.
    pub fn register_identity(
        ctx: Context<RegisterIdentity>,
        encrypted_email: Vec<u8>,
        email_hash: [u8; 32],
        nonce: [u8; 24],
        opt_in_all: bool,
        opt_in_defi: bool,
        opt_in_governance: bool,
        opt_in_marketing: bool,
        digest_mode: bool,
    ) -> Result<()> {
        instructions::register_identity::handler(
            ctx,
            encrypted_email,
            email_hash,
            nonce,
            opt_in_all,
            opt_in_defi,
            opt_in_governance,
            opt_in_marketing,
            digest_mode,
        )
    }

    /// Partially update an existing user identity.
    pub fn update_identity(
        ctx: Context<UpdateIdentity>,
        encrypted_email: Option<Vec<u8>>,
        email_hash: Option<[u8; 32]>,
        nonce: Option<[u8; 24]>,
        opt_in_all: Option<bool>,
        opt_in_defi: Option<bool>,
        opt_in_governance: Option<bool>,
        opt_in_marketing: Option<bool>,
        digest_mode: Option<bool>,
    ) -> Result<()> {
        instructions::update_identity::handler(
            ctx,
            encrypted_email,
            email_hash,
            nonce,
            opt_in_all,
            opt_in_defi,
            opt_in_governance,
            opt_in_marketing,
            digest_mode,
        )
    }

    /// Delete (close) a user identity account. Rent is refunded to the owner.
    pub fn delete_identity(ctx: Context<DeleteIdentity>) -> Result<()> {
        instructions::delete_identity::handler(ctx)
    }

    // ═══════════════════════════════════════════════════════
    //  CHANNEL MANAGEMENT INSTRUCTIONS
    // ═══════════════════════════════════════════════════════

    /// Register or update the Telegram channel for an existing identity.
    pub fn register_telegram(
        ctx: Context<RegisterTelegram>,
        encrypted_telegram_id: Vec<u8>,
        telegram_id_hash: [u8; 32],
        nonce_telegram: [u8; 24],
    ) -> Result<()> {
        instructions::register_telegram::handler(
            ctx,
            encrypted_telegram_id,
            telegram_id_hash,
            nonce_telegram,
        )
    }

    /// Register or update the SMS channel for an existing identity.
    pub fn register_sms(
        ctx: Context<RegisterSms>,
        encrypted_phone: Vec<u8>,
        phone_hash: [u8; 32],
        nonce_sms: [u8; 24],
    ) -> Result<()> {
        instructions::register_sms::handler(
            ctx,
            encrypted_phone,
            phone_hash,
            nonce_sms,
        )
    }

    /// Toggle individual channels on/off without modifying encrypted data.
    pub fn update_channel_settings(
        ctx: Context<UpdateChannelSettings>,
        channel_email: Option<bool>,
        channel_telegram: Option<bool>,
        channel_sms: Option<bool>,
    ) -> Result<()> {
        instructions::update_channels::handler(
            ctx,
            channel_email,
            channel_telegram,
            channel_sms,
        )
    }

    /// Permanently remove a channel's encrypted data (GDPR per-channel erasure).
    pub fn remove_channel(
        ctx: Context<RemoveChannel>,
        channel: ChannelType,
    ) -> Result<()> {
        instructions::remove_channel::handler(ctx, channel)
    }

    /// Lazy migration: set channel_email = true for pre-existing identities.
    pub fn migrate_identity_channels(
        ctx: Context<MigrateIdentityChannels>,
    ) -> Result<()> {
        instructions::migrate_channels::handler(ctx)
    }

    // ═══════════════════════════════════════════════════════
    //  PROTOCOL LIFECYCLE INSTRUCTIONS
    // ═══════════════════════════════════════════════════════

    /// Register a new DeFi protocol. Only callable by the Herald authority.
    pub fn register_protocol(
        ctx: Context<RegisterProtocol>,
        name_hash: [u8; 32],
        tier: u8,
    ) -> Result<()> {
        instructions::register_protocol::handler(ctx, name_hash, tier)
    }

    /// Deactivate a protocol (soft deactivation). Only callable by the Herald authority.
    pub fn deactivate_protocol(ctx: Context<DeactivateProtocol>) -> Result<()> {
        instructions::deactivate_protocol::handler(ctx)
    }

    /// Reactivate a deactivated (non-suspended) protocol. Only callable by the Herald authority.
    pub fn reactivate_protocol(ctx: Context<ReactivateProtocol>) -> Result<()> {
        instructions::reactivate_protocol::handler(ctx)
    }

    /// Hard-suspend a protocol (e.g. ToS violation). Only callable by the Herald authority.
    pub fn suspend_protocol(ctx: Context<SuspendProtocol>) -> Result<()> {
        instructions::suspend_protocol::handler(ctx)
    }

    /// Update a protocol's tier level. Only callable by the Herald authority.
    pub fn update_protocol_tier(ctx: Context<UpdateProtocolTier>, new_tier: u8) -> Result<()> {
        instructions::update_tier::handler(ctx, new_tier)
    }

    // ═══════════════════════════════════════════════════════
    //  SUBSCRIPTION / BILLING INSTRUCTIONS
    // ═══════════════════════════════════════════════════════

    /// Renew (or initially activate) a protocol's monthly subscription.
    /// Called by the Herald backend after confirming off-chain payment (Helio).
    pub fn renew_subscription(ctx: Context<RenewSubscription>) -> Result<()> {
        instructions::renew_subscription::handler(ctx)
    }

    /// Pay for subscription on-chain with USDC or USDT (Phase 2).
    /// Called directly by the protocol admin wallet.
    pub fn pay_subscription(ctx: Context<PaySubscription>, months: u8) -> Result<()> {
        instructions::pay_subscription::handler(ctx, months)
    }

    /// Reset a protocol's sends counter at the end of a billing period.
    pub fn reset_protocol_sends(ctx: Context<ResetProtocolSends>) -> Result<()> {
        instructions::reset_protocol_sends::handler(ctx)
    }

    /// Withdraw accumulated USDC/USDT from vault to Herald treasury.
    /// Only callable by the Herald authority.
    pub fn withdraw_treasury(ctx: Context<WithdrawTreasury>, amount: u64) -> Result<()> {
        instructions::withdraw_treasury::handler(ctx, amount)
    }

    // ═══════════════════════════════════════════════════════
    //  NOTIFICATION / RECEIPT INSTRUCTIONS
    // ═══════════════════════════════════════════════════════

    /// Write a ZK-compressed delivery receipt via Light Protocol CPI.
    pub fn write_receipt<'info>(
        ctx: Context<'_, '_, '_, 'info, WriteReceipt<'info>>,
        proof: AnchorCompressedProof,
        output_tree_index: u8,
        recipient_hash: [u8; 32],
        notification_id: [u8; 16],
        category: u8,
    ) -> Result<()> {
        instructions::write_receipt::handler(
            ctx,
            proof,
            output_tree_index,
            recipient_hash,
            notification_id,
            category,
        )
    }
}
