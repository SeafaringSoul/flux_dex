use crate::utils::FixedPoint;
use anchor_lang::prelude::*;

/// AI 决策系统配置
#[account]
pub struct AiDecisionConfig {
    /// 系统权限
    pub authority: Pubkey,

    /// 关联池子
    pub pool: Pubkey,

    /// AI 模型参数
    pub model_version: u16,
    pub prediction_window: u16,       // 预测窗口(分钟)
    pub confidence_threshold: u16,    // 置信度阈值
    pub max_position_adjustment: u16, // 最大仓位调整比例

    /// 市场数据源
    pub price_feed: Pubkey,
    pub volume_feed: Pubkey,
    pub volatility_feed: Pubkey,

    /// 优化参数
    pub learning_rate: FixedPoint,
    pub risk_adjustment_factor: FixedPoint,
    pub profit_target_bps: u16,
    pub stop_loss_bps: u16,

    /// 状态
    pub enabled: bool,
    pub last_decision_time: i64,
    pub total_decisions: u64,
    pub successful_predictions: u64,

    /// 元数据
    pub created_at: i64,
    pub updated_at: i64,
    pub bump: u8,
}

impl AiDecisionConfig {
    pub const SIZE: usize = 8 + // discriminator
        32 + 32 + // authority, pool
        2 + 2 + 2 + 2 + // model params
        32 + 32 + 32 + // data feeds
        32 + 32 + 2 + 2 + // optimization params
        1 + 8 + 8 + 8 + // state and stats
        8 + 8 + 1 + // metadata
        32; // padding
}
