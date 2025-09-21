use anchor_lang::prelude::*;

#[event]
pub struct LiquidityAdded {
    pub user: Pubkey,
    pub pool: Pubkey,
    pub amount_a: u64,
    pub amount_b: u64,
    pub lp_tokens: u64,
    pub timestamp: i64,
}
