use crate::state::seeds::*;
use crate::state::Pool;
use crate::FluxDexError;
use crate::Position;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
    mut,
    seeds = [POOL_SEED,pool.authority.as_ref()],
    bump = pool.bump
  )]
    pub pool: Account<'info, Pool>,

    #[account(
      mut,
      constraint = user_token_a.mint = pool.token_a_mint @ FluxDexError::InvalidAccount,
      constraint = user_token_a.owner == user.key() @ FluxDexError::Unauthorized,
    )]
    pub user_token_a: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_token_b.mint == pool.token_b_mint @ FluxDexError::InvalidAccount,
        constraint = user_token_b.owner == user.key() @ FluxDexError::Unauthorized,
    )]
    pub user_token_b: Account<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        token::mint = pool.lp_mint,
        token::authority = user,
    )]
    pub user_lp_token: Account<'info, TokenAccount>,

    // Pool 的  token A 金库
    #[account(
        mut,
        constraint = pool_token_a_vault.key() == pool.token_a_vault @ FluxDexError::InvalidAccount,
    )]
    pub pool_token_a_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = pool_token_b_vault.key() == pool.token_b_vault @ FluxDexError::InvalidAccount,
    )]
    pub pool_token_b_vault: Account<'info, TokenAccount>,

    /// LP Token Mint (Pool 拥有铸造权限)
    #[account(
        mut,
        constraint = lp_mint.key() == pool.lp_mint @ FluxDexError::InvalidAccount,
    )]
    pub lp_mint: Account<'info, Mint>,

    /// 用户的 Position 账户 (如果不存在会创建)
    #[account(
        init_if_needed,
        payer = user,
        space = Position::SIZE,
        seeds = [b"position", user.key().as_ref(), pool.key().as_ref()],
        bump,
    )]
    pub position: Account<'info, Position>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}
