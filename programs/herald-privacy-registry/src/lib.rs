use anchor_lang::prelude::*;

declare_id!("2pxjAf8tLCakKVDuN4vY51B5TeaEQk4koPuk9NZvWqdf");

#[program]
pub mod herald_privacy_registry {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
