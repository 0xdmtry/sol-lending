use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct User {
    /// The wallet address that owns this user account
    pub owner: Pubkey,

    /// Amount of SOL the user has deposited into the protocol (in lamports)
    pub deposited_sol: u64,
    /// Number of shares received in exchange for deposited SOL (used for proportional ownership)
    pub deposited_sol_shares: u64,

    /// Amount of SOL the user has borrowed (in lamports)
    pub borrowed_sol: u64,
    /// Number of shares representing borrowed SOL (used for interest accrual or debt accounting)
    pub borrowed_sol_shares: u64,

    /// Amount of USDC the user has deposited into the protocol (in smallest unit, e.g., 6 decimals)
    pub deposited_usdc: u64,
    /// Shares received for deposited USDC
    pub deposited_usdc_shares: u64,

    /// Amount of USDC the user has borrowed
    pub borrowed_usdc: u64,
    /// Shares representing borrowed USDC
    pub borrowed_usdc_shares: u64,

    /// The user's USDC-associated token account address
    pub usdc_address: Pubkey,

    /// Last timestamp (Unix time) the account was updated (used for interest accrual, liquidation checks, etc.)
    pub last_updated: i64,

}

#[account]
#[derive(InitSpace)]
pub struct Bank {
    /// The authority that controls this bank account (e.g., admin or DAO)
    pub authority: Pubkey,

    /// The mint address of the token this bank manages (e.g., SOL or USDC mint)
    pub mint_address: Pubkey,

    /// Total amount of the token deposited into the bank by all users
    pub total_deposits: u64,
    /// Total number of shares issued for all deposits (used to calculate proportional ownership and yield)
    pub total_deposits_shares: u64,

    pub total_borrowed: u64,
    pub total_borrowed_shares: u64,
    
    

    /// Liquidation threshold (e.g., 85%) — if collateral value drops below this, liquidation may occur
    pub liquidation_threshold: u64,

    /// Bonus (in basis points or percent format) given to liquidators when they repay bad debt
    pub liquidation_bonus: u64,

    /// Maximum percentage of a position that can be closed in a single liquidation event (e.g., 50%)
    pub liquidation_close_factor: u64,

    /// Maximum loan-to-value ratio (e.g., 75%) allowed for borrowing against collateral
    pub max_ltv: u64,

    pub last_updated: i64,
    pub interest_rate: u64,
    
}
