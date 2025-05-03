mod instructions;
mod state;

use anchor_lang::prelude::*;
use instructions::*;

declare_id!("EDBnBUPG61SCSTcB6g7MFVqF51pzzS9ptFK5WkSnTPqA");

#[program]
pub mod lending_v2 {
    use super::*;

    pub fn init_bank(
        ctx: Context<InitBank>,
        liquidation_threshold: u64,
        max_ltv: u64,
    ) -> Result<()> {
        instructions::admin::process_init_bank(ctx, liquidation_threshold, max_ltv)
    }

    pub fn init_user(ctx: Context<InitUser>, usdc_address: Pubkey) -> Result<()> {
        instructions::admin::process_init_user(ctx, usdc_address)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        process_deposit(ctx, amount)
    }
}
