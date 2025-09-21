use crate::state::seeds::*;
use crate::state::Pool;
use crate::FluxDexError;
use crate::LiquidityAdded;
use crate::MathUtils;
use crate::Position;
use crate::RiskProfile;
use crate::StrategyType;
use anchor_lang::prelude::*;
use anchor_spl::token;
use anchor_spl::token::Mint;
use anchor_spl::token::MintTo;
use anchor_spl::token::Token;
use anchor_spl::token::TokenAccount;
use anchor_spl::token::Transfer;

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
      constraint = user_token_a.mint == pool.token_a_mint @ FluxDexError::InvalidAccount,
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
        token::mint = lp_mint,
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

pub fn add_liquidity_handler(
    ctx: Context<AddLiquidity>,
    desired_amount_a: u64, // 用户想要添加的 Token A 数量
    desired_amount_b: u64, // 用户想要添加的 Token B 数量
    min_amount_a: u64,     // 最少接受的 Token A 数量（滑点保护）
    min_amount_b: u64,     // 最少接受的 Token B 数量（滑点保护）
    min_lp_tokens: u64,    // 最少获得的 LP Token 数量（滑点保护）
) -> Result<()> {
    let pool = &mut ctx.accounts.pool;
    let position = &mut ctx.accounts.position;

    // check：：pool is stop
    if pool.paused || pool.emergency_mode {
        return Err(FluxDexError::Unauthorized.into());
    }

    // check:: input pargam
    if desired_amount_a == 0 || desired_amount_b == 0 {
        return Err(FluxDexError::InvalidInputAmount.into());
    }

    if min_amount_a > desired_amount_a || min_amount_b > desired_amount_b {
        return Err(FluxDexError::InvalidInputAmount.into());
    }

    // check user Balance
    if ctx.accounts.user_token_a.amount < desired_amount_a {
        return Err(FluxDexError::InsufficientLiquidity.into());
    }

    if ctx.accounts.user_token_b.amount < desired_amount_b {
        return Err(FluxDexError::InsufficientLiquidity.into());
    }

    // 计算实际添加的流动性数量
    let (actual_amount_a, actual_amount_b) = if pool.lp_supply == 0 {
        // 首次添加流动性，直接使用用户指定的数量
        (desired_amount_a, desired_amount_b)
    } else {
        // 根据当前池子比例计算最优数量
        calculate_optimal_amounts(
            desired_amount_a,
            desired_amount_b,
            pool.token_a_reserve,
            pool.token_b_reserve,
        )?
    };

    // 检查滑点保护
    if actual_amount_a < min_amount_a || actual_amount_b < min_amount_b {
        return Err(FluxDexError::SlippageExceeded.into());
    }

    // 计算应该铸造的 LP 代币数量
    let lp_tokens_to_mint = MathUtils::calculate_lp_tokens(
        actual_amount_a,
        actual_amount_b,
        pool.token_a_reserve,
        pool.token_b_reserve,
        pool.lp_supply,
    )?;

    // 检查最小 LP 代币数量
    if lp_tokens_to_mint < min_lp_tokens {
        return Err(FluxDexError::SlippageExceeded.into());
    }

    // 执行代币转账：用户 -> 池子
    transfer_tokens_to_pool(
        &ctx.accounts.user,
        &ctx.accounts.user_token_a,
        &ctx.accounts.pool_token_a_vault,
        &ctx.accounts.token_program,
        actual_amount_a,
    )?;

    transfer_tokens_to_pool(
        &ctx.accounts.user,
        &ctx.accounts.user_token_b,
        &ctx.accounts.pool_token_b_vault,
        &ctx.accounts.token_program,
        actual_amount_b,
    )?;

    // 铸造 LP 代币给用户
    mint_lp_tokens(
        &pool,
        &ctx.accounts.lp_mint,
        &ctx.accounts.user_lp_token,
        &ctx.accounts.token_program,
        lp_tokens_to_mint,
    )?;

    // 更新池子状态
    pool.token_a_reserve = pool
        .token_a_reserve
        .checked_add(actual_amount_a)
        .ok_or(FluxDexError::Overflow)?;

    pool.token_b_reserve = pool
        .token_b_reserve
        .checked_add(actual_amount_b)
        .ok_or(FluxDexError::Overflow)?;

    pool.lp_supply = pool
        .lp_supply
        .checked_add(lp_tokens_to_mint)
        .ok_or(FluxDexError::Overflow)?;

    let clock = Clock::get()?;
    pool.updated_at = clock.unix_timestamp;

    // 初始化或更新用户 Position
    if position.lp_tokens == 0 {
        // 首次添加流动性，初始化 Position
        position.owner = ctx.accounts.user.key();
        position.pool = ctx.accounts.pool.key();
        position.strategy_type = StrategyType::Passive; // 默认被动策略
        position.risk_profile = RiskProfile::Balanced; // 默认平衡风险
        position.auto_rebalance = false;
        position.rebalance_threshold_bps = 500; // 5% 阈值

        position.initial_deposit_a = actual_amount_a;
        position.initial_deposit_b = actual_amount_b;
        position.realized_fees_a = 0;
        position.realized_fees_b = 0;
        position.unrealized_pnl_a = 0;
        position.unrealized_pnl_b = 0;

        position.mev_rewards_earned = 0;
        position.mev_rewards_claimed = 0;

        position.created_at = clock.unix_timestamp;
        position.last_rebalanced = clock.unix_timestamp;
        position.bump = ctx.bumps.position;
    } else {
        // 增加现有 Position
        position.initial_deposit_a = position
            .initial_deposit_a
            .checked_add(actual_amount_a)
            .ok_or(FluxDexError::Overflow)?;

        position.initial_deposit_b = position
            .initial_deposit_b
            .checked_add(actual_amount_b)
            .ok_or(FluxDexError::Overflow)?;
    }

    position.lp_tokens = position
        .lp_tokens
        .checked_add(lp_tokens_to_mint)
        .ok_or(FluxDexError::Overflow)?;

    // 发出事件
    emit!(LiquidityAdded {
        user: ctx.accounts.user.key(),
        pool: ctx.accounts.pool.key(),
        amount_a: actual_amount_a,
        amount_b: actual_amount_b,
        lp_tokens: lp_tokens_to_mint,
        timestamp: clock.unix_timestamp,
    });

    msg!(
        "✅ Liquidity added: {} token A, {} token B, {} LP tokens minted",
        actual_amount_a,
        actual_amount_b,
        lp_tokens_to_mint
    );
    Ok(())
}

// 辅助函数：计算最优添加数量
fn calculate_optimal_amounts(
    desired_a: u64,
    desired_b: u64,
    reserve_a: u64,
    reserve_b: u64,
) -> Result<(u64, u64)> {
    // 根据当前池子比例计算最优数量
    let ratio_a = (desired_a as u128 * reserve_b as u128) / reserve_a as u128;
    let ratio_b = desired_b as u128;

    if ratio_a <= ratio_b {
        // 以 token A 为准
        let optimal_b = (desired_a as u128 * reserve_b as u128) / reserve_a as u128;
        Ok((desired_a, optimal_b as u64))
    } else {
        // 以 token B 为准
        let optimal_a = (desired_b as u128 * reserve_a as u128) / reserve_b as u128;
        Ok((optimal_a as u64, desired_b))
    }
}

// 辅助函数：转账代币到池子
fn transfer_tokens_to_pool<'info>(
    user: &Signer<'info>,
    user_token_account: &Account<'info, TokenAccount>,
    pool_vault: &Account<'info, TokenAccount>,
    token_program: &Program<'info, Token>,
    amount: u64,
) -> Result<()> {
    let cpi_accounts = Transfer {
        from: user_token_account.to_account_info(),
        to: pool_vault.to_account_info(),
        authority: user.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

    token::transfer(cpi_ctx, amount)?;
    Ok(())
}

// 辅助函数：铸造 LP 代币
fn mint_lp_tokens<'info>(
    pool: &Account<'info, Pool>,
    lp_mint: &Account<'info, Mint>,
    user_lp_account: &Account<'info, TokenAccount>,
    token_program: &Program<'info, Token>,
    amount: u64,
) -> Result<()> {
    let authority_seed = &[POOL_SEED, pool.authority.as_ref(), &[pool.bump]];
    let signer = &[&authority_seed[..]];

    let cpi_accounts = MintTo {
        mint: lp_mint.to_account_info(),
        to: user_lp_account.to_account_info(),
        authority: pool.to_account_info(),
    };
    let cpi_program = token_program.to_account_info();
    let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer);

    token::mint_to(cpi_ctx, amount)?;
    Ok(())
}
