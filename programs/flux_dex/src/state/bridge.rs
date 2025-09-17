use anchor_lang::prelude::*;

/// 跨链桥配置
#[account]
pub struct CrossChainBridge {
    /// 桥接权限
    pub authority: Pubkey,

    /// 支持的链
    pub supported_chains: Vec<ChainInfo>,

    /// 资产映射
    pub asset_mappings: Vec<AssetMapping>,

    /// 桥接设置
    pub min_bridge_amount: u64,
    pub max_bridge_amount: u64,
    pub bridge_fee_bps: u16,
    pub confirmation_blocks: u16,

    /// 流动性管理
    pub total_locked_assets: u64,
    pub available_liquidity: u64,
    pub emergency_withdrawal_enabled: bool,

    /// 状态
    pub paused: bool,
    pub maintenance_mode: bool,

    /// 统计
    pub total_bridges_count: u64,
    pub total_volume_bridged: u128,

    pub bump: u8,
}

/// 链信息
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct ChainInfo {
    pub chain_id: u64,
    pub chain_name: String,
    pub bridge_contract: String,
    pub confirmation_blocks: u16,
    pub is_active: bool,
}

/// 资产映射
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Debug)]
pub struct AssetMapping {
    pub solana_mint: Pubkey,
    pub external_token_address: String,
    pub chain_id: u64,
    pub decimals: u8,
    pub is_active: bool,
}
