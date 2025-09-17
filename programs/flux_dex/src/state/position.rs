use crate::utils::FixedPoint;
use anchor_lang::prelude::*;

/// 用户流动性头寸
#[account]
pub struct Position {
    /// 所属用户
    pub owner: Pubkey,

    /// 关联池子
    pub pool: Pubkey,

    /// 流动性代币数量
    pub lp_tokens: u64,

    /// 智能流动性管理策略
    pub strategy_type: StrategyType,
    pub risk_profile: RiskProfile,
    pub price_range_lower: FixedPoint,
    pub price_range_upper: FixedPoint,
    pub auto_rebalance: bool,
    pub rebalance_threshold_bps: u16,

    /// 收益追踪
    pub initial_deposit_a: u64,
    pub initial_deposit_b: u64,
    pub realized_fees_a: u64,
    pub realized_fees_b: u64,
    pub unrealized_pnl_a: i64, // 可为负
    pub unrealized_pnl_b: i64,

    /// MEV 收益分享
    pub mev_rewards_earned: u64,
    pub mev_rewards_claimed: u64,

    /// 元数据
    pub created_at: i64,
    pub last_rebalanced: i64,
    pub bump: u8,
}

impl Position {
    pub const SIZE: usize = 8 + // discriminator
        32 + 32 + // owner, pool
        8 + // lp_tokens
        1 + 1 + 32 + 32 + 1 + 2 + // strategy settings
        8 + 8 + 8 + 8 + 8 + 8 + // pnl tracking
        8 + 8 + // mev rewards
        8 + 8 + 1 + // metadata
        32; // padding
}

/// 智能流动性管理策略
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum StrategyType {
    /// 被动流动性提供（传统AMM）
    Passive,
    /// 主动管理（AI驱动）
    Active,
    /// 范围做市
    RangeMarketMaking,
    /// 动态对冲
    DynamicHedging,
}

/// 风险档次
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum RiskProfile {
    /// 保守：更宽价格区间，更少重新平衡
    Conservative,
    /// 平衡：中等价格区间和重新平衡频率  
    Balanced,
    /// 激进：更窄价格区间，更频繁重新平衡
    Aggressive,
}
