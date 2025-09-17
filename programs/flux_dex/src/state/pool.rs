use crate::utils::FixedPoint;
use anchor_lang::prelude::*;

/// 流动性池主结构
#[account]
pub struct Pool {
    /// 池子权限控制
    pub authority: Pubkey,

    /// 代币信息
    pub token_a_mint: Pubkey,
    pub token_b_mint: Pubkey,
    pub token_a_vault: Pubkey,
    pub token_b_vault: Pubkey,

    /// 储备量
    pub token_a_reserve: u64,
    pub token_b_reserve: u64,

    /// LP 代币信息
    pub lp_mint: Pubkey,
    pub lp_supply: u64,

    /// 费率设置
    pub base_fee_bps: u16, // 基础费率
    pub dynamic_fee_enabled: bool, // 是否启用动态费率
    pub current_fee_bps: u16,      // 当前有效费率

    /// 智能流动性管理
    pub alm_enabled: bool, // 是否启用智能流动性管理
    pub volatility_score: u16,  // 波动率分数 (0-10000)
    pub last_price_update: i64, // 上次价格更新时间
    pub price_oracle: Pubkey,   // 价格预言机

    /// MEV 保护
    pub mev_protection_enabled: bool,
    pub intent_pool: Pubkey,  // 交易意图池地址
    pub batch_size: u8,       // 批量执行大小
    pub execution_delay: u16, // 执行延迟(秒)

    /// 统计数据
    pub total_volume_a: u128, // 累计交易量A
    pub total_volume_b: u128,        // 累计交易量B
    pub total_fees_collected_a: u64, // 累计手续费A
    pub total_fees_collected_b: u64, // 累计手续费B
    pub swap_count: u64,             // 交易次数

    /// 治理和控制
    pub paused: bool, // 是否暂停
    pub emergency_mode: bool,      // 紧急模式
    pub upgrade_authority: Pubkey, // 升级权限

    /// 元数据
    pub created_at: i64,
    pub updated_at: i64,
    pub bump: u8,
}

impl Pool {
    pub const SIZE: usize = 8 + // discriminator
        32 + // authority
        32 + 32 + 32 + 32 + // token mints and vaults
        8 + 8 + // reserves
        32 + 8 + // lp mint and supply
        2 + 1 + 2 + // fee settings
        1 + 2 + 8 + 32 + // ALM settings
        1 + 32 + 1 + 2 + // MEV protection
        16 + 16 + 8 + 8 + 8 + // statistics
        1 + 1 + 32 + // governance
        8 + 8 + 1 + // metadata
        64; // padding for future upgrades

    /// 获取当前价格
    pub fn get_current_price(&self) -> FixedPoint {
        if self.token_a_reserve == 0 {
            return FixedPoint::from_u64(0);
        }
        let price =
            (self.token_b_reserve as u128 * FixedPoint::SCALE) / self.token_a_reserve as u128;
        FixedPoint::new(price)
    }

    /// 检查是否需要重新平衡
    pub fn needs_rebalance(&self) -> bool {
        // 实现重新平衡逻辑
        false // placeholder
    }
}
