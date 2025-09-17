use crate::utils::FixedPoint;
use anchor_lang::prelude::*;

/// MEV 保护 - 交易意图池
#[account]
pub struct IntentPool {
    /// 池子权限
    pub authority: Pubkey,

    /// 关联的流动性池
    pub liquidity_pool: Pubkey,

    /// 意图队列设置
    pub max_intents: u16,
    pub current_intent_count: u16,
    pub batch_execution_delay: u16, // 批量执行延迟(秒)
    pub min_batch_size: u8,
    pub max_batch_size: u8,

    /// MEV 收益分配
    pub mev_fee_bps: u16, // MEV 收益费率
    pub protocol_fee_bps: u16,  // 协议费率
    pub lp_reward_pool: Pubkey, // LP奖励池地址

    /// 状态管理
    pub paused: bool,
    pub emergency_mode: bool,

    /// 统计
    pub total_intents_processed: u64,
    pub total_mev_captured: u64,
    pub total_rewards_distributed: u64,

    /// 元数据
    pub created_at: i64,
    pub bump: u8,
}

impl IntentPool {
    pub const SIZE: usize = 8 + // discriminator
        32 + 32 + // authority, liquidity_pool
        2 + 2 + 2 + 1 + 1 + // intent queue settings
        2 + 2 + 32 + // fee distribution
        1 + 1 + // state
        8 + 8 + 8 + // statistics
        8 + 1 + // metadata
        16; // padding
}

/// 单个交易意图
#[account]
pub struct TradingIntent {
    /// 交易发起者
    pub trader: Pubkey,

    /// 关联意图池
    pub intent_pool: Pubkey,

    /// 交易参数
    pub input_mint: Pubkey,
    pub output_mint: Pubkey,
    pub input_amount: u64,
    pub min_output_amount: u64,
    pub max_slippage_bps: u16,

    /// 优先级设置
    pub priority_fee: u64,
    pub max_wait_time: u16, // 最大等待时间(秒)

    /// 执行状态
    pub status: IntentStatus,
    pub executed_output_amount: u64,
    pub execution_price: FixedPoint,
    pub gas_used: u64,

    /// 时间戳
    pub created_at: i64,
    pub expires_at: i64,
    pub executed_at: i64,

    /// 批次信息
    pub batch_id: Option<u64>,

    pub bump: u8,
}

/// 交易意图状态
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub enum IntentStatus {
    /// 待处理
    Pending,
    /// 已加入批次
    Batched,
    /// 执行成功
    Executed,
    /// 执行失败
    Failed,
    /// 已过期
    Expired,
    /// 已取消
    Cancelled,
}

impl TradingIntent {
    pub const SIZE: usize = 8 + // discriminator
        32 + 32 + // trader, intent_pool
        32 + 32 + 8 + 8 + 2 + // trading params
        8 + 2 + // priority
        1 + 8 + 32 + 8 + // execution status
        8 + 8 + 8 + // timestamps
        9 + // Option<u64> for batch_id
        1 + // bump
        16; // padding
}
