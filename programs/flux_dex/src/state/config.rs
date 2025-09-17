use anchor_lang::prelude::*;

/// 全局配置账户
#[account]
pub struct GlobalConfig {
    /// 超级管理员
    pub super_admin: Pubkey,

    /// 协议参数
    pub protocol_fee_bps: u16,
    pub treasury: Pubkey,

    /// 功能开关
    pub alm_enabled_globally: bool,
    pub mev_protection_enabled_globally: bool,
    pub cross_chain_enabled: bool,
    pub emergency_mode: bool,

    /// 版本控制
    pub version: u16,
    pub upgrade_authority: Pubkey,

    /// 统计数据
    pub total_pools: u64,
    pub total_volume: u128,
    pub total_fees_collected: u64,

    /// 创建时间
    pub created_at: i64,
    pub bump: u8,
}

impl GlobalConfig {
    pub const SIZE: usize = 8 + // discriminator
        32 + // super_admin
        2 + 32 + // protocol fee and treasury
        1 + 1 + 1 + 1 + // feature flags
        2 + 32 + // version control
        8 + 16 + 8 + // statistics
        8 + 1 + // metadata
        32; // padding
}

/// 用户配置
#[account]
pub struct UserConfig {
    /// 用户地址
    pub owner: Pubkey,

    /// 默认设置
    pub default_slippage_bps: u16,
    pub default_deadline: u16,
    pub auto_compound_enabled: bool,

    /// MEV 保护偏好
    pub mev_protection_preference: bool,
    pub max_mev_wait_time: u16,

    /// 通知设置
    pub notifications_enabled: bool,
    pub rebalance_notifications: bool,
    pub yield_notifications: bool,

    /// 统计
    pub total_swaps: u64,
    pub total_liquidity_provided: u128,
    pub total_fees_earned: u64,
    pub total_mev_rewards: u64,

    /// 最后活动时间
    pub last_activity: i64,
    pub created_at: i64,
    pub bump: u8,
}

impl UserConfig {
    pub const SIZE: usize = 8 + // discriminator
        32 + // owner
        2 + 2 + 1 + // default settings
        1 + 2 + // mev preferences
        1 + 1 + 1 + // notification settings
        8 + 16 + 8 + 8 + // statistics
        8 + 8 + 1 + // metadata
        16; // padding
}
