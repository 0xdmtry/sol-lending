use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Cannot borrow: exceeds maximum loan-to-value (LTV) ratio.")]
    OverLTV,
    #[msg("Loan would become undercollateralized")]
    UnderCollateralized,
    #[msg("Withdrawal failed: insufficient available funds.")]
    InsufficientFunds,
    #[msg("Cannot repay more than the borrowed amount.")]
    OverRepay,
    #[msg("Borrow amount exceeds the allowable limit.")]
    OverBorrowableAmount,
    #[msg("User is currently not undercollateralized.")]
    NotUndercollateralized,
}
