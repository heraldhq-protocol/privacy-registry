use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

use crate::constants::{HERALD_AUTHORITY, HERALD_TREASURY, VAULT_SEED};
use crate::errors::HeraldError;
use crate::state::SubscriptionVaultAccount;

/// Accounts for `withdraw_treasury`.
///
/// Withdraw accumulated USDC/USDT from the vault PDA to Herald treasury.
/// Only callable by HERALD_AUTHORITY. Destination must be the treasury.
#[derive(Accounts)]
pub struct WithdrawTreasury<'info> {
    /// Herald backend authority — must match `HERALD_AUTHORITY`.
    #[account(
        constraint = authority.key() == HERALD_AUTHORITY @ HeraldError::Unauthorized,
    )]
    pub authority: Signer<'info>,

    /// Herald treasury vault PDA (used as signer via seeds).
    #[account(
        mut,
        seeds = [VAULT_SEED],
        bump = vault_account.bump,
    )]
    pub vault_account: Account<'info, SubscriptionVaultAccount>,

    /// Vault's token account (source of withdrawal).
    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    /// Treasury's token account (destination).
    /// Constrained to the known HERALD_TREASURY address.
    #[account(
        mut,
        constraint = treasury_token_account.owner == HERALD_TREASURY @ HeraldError::Unauthorized,
    )]
    pub treasury_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
}

/// Withdraw accumulated USDC/USDT from vault to Herald treasury (Squads multisig).
pub fn handler(ctx: Context<WithdrawTreasury>, amount: u64) -> Result<()> {
    // Use vault PDA as signer via seeds
    let bump = ctx.accounts.vault_account.bump;
    let seeds: &[&[u8]] = &[VAULT_SEED, &[bump]];
    let signer_seeds = &[seeds];

    let transfer_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        Transfer {
            from: ctx.accounts.vault_token_account.to_account_info(),
            to: ctx.accounts.treasury_token_account.to_account_info(),
            authority: ctx.accounts.vault_account.to_account_info(),
        },
        signer_seeds,
    );
    token::transfer(transfer_ctx, amount)?;

    let vault = &mut ctx.accounts.vault_account;
    vault.last_withdrawal_at = Clock::get()
        .map_err(|_| error!(HeraldError::ClockUnavailable))?
        .unix_timestamp;
    vault.withdrawal_count = vault
        .withdrawal_count
        .checked_add(1)
        .ok_or_else(|| error!(HeraldError::Overflow))?;

    Ok(())
}
