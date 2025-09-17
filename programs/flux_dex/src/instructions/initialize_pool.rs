use crate::state::seeds::*;
use crate::state::Pool;
use anchor_lang::prelude::*;
use anchor_spl::token::Mint;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;

#[derive(Accounts)]
pub struct InitializePool<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    // Pool state PDA
    #[account(
        init,
        payer = authority,
        space = Pool::SIZE,
        seeds = [POOL_SEED, authority.key().as_ref()],
        bump,
    )]
    pub pool: Account<'info, Pool>,

    // Token mints (must exist before calling initialize_pool)
    pub token_a_mint: Account<'info, Mint>,
    pub token_b_mint: Account<'info, Mint>,

    // Token vaults owned by the pool PDA
    #[account(
        init,
        payer = authority,
        token::mint = token_a_mint,
        token::authority = pool,
    )]
    pub token_a_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        token::mint = token_b_mint,
        token::authority = pool,
    )]
    pub token_b_vault: Account<'info, TokenAccount>,

    // LP token mint
    #[account(
        init,
        payer = authority,
        mint::decimals = 6,
        mint::authority = pool,
    )]
    pub lp_mint: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

pub fn initialize_pool_handler(ctx: Context<InitializePool>, base_fee_bps: u16) -> Result<()> {
    let pool = &mut ctx.accounts.pool;

    pool.authority = ctx.accounts.authority.key();
    pool.token_a_mint = ctx.accounts.token_a_mint.key();
    pool.token_b_mint = ctx.accounts.token_b_mint.key();
    pool.token_a_vault = ctx.accounts.token_a_vault.key();
    pool.token_b_vault = ctx.accounts.token_b_vault.key();
    pool.lp_mint = ctx.accounts.lp_mint.key();

    pool.token_a_reserve = 0;
    pool.token_b_reserve = 0;
    pool.lp_supply = 0;

    pool.base_fee_bps = base_fee_bps; // 默认基础费率 0.3%
    pool.dynamic_fee_enabled = false;
    pool.current_fee_bps = base_fee_bps;

    pool.alm_enabled = false;
    pool.volatility_score = 0;
    pool.last_price_update = 0;
    pool.price_oracle = Pubkey::default();

    pool.mev_protection_enabled = false;
    pool.intent_pool = Pubkey::default();
    pool.batch_size = 1;
    pool.execution_delay = 0;

    pool.total_volume_a = 0;
    pool.total_volume_b = 0;
    pool.total_fees_collected_a = 0;
    pool.total_fees_collected_b = 0;
    pool.swap_count = 0;

    pool.paused = false;
    pool.emergency_mode = false;
    pool.upgrade_authority = ctx.accounts.authority.key();

    let clock = Clock::get()?;
    pool.created_at = clock.unix_timestamp;
    pool.updated_at = clock.unix_timestamp;

    pool.bump = ctx.bumps.pool;

    Ok(())
}
