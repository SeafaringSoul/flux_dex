use anchor_lang::prelude::*;

#[error_code]
pub enum FluxDexError {
    // 通用错误 (0-99)
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Invalid input amount")]
    InvalidInputAmount,
    #[msg("Insufficient liquidity")]
    InsufficientLiquidity,
    #[msg("Overflow occurred")]
    Overflow,
    #[msg("Underflow occurred")]
    Underflow,
    #[msg("Invalid account")]
    InvalidAccount,
    #[msg("Account already initialized")]
    AccountAlreadyInitialized,

    // 流动性管理错误 (100-199)
    #[msg("Invalid price range")]
    InvalidPriceRange,
    #[msg("Position not found")]
    PositionNotFound,
    #[msg("Position not empty")]
    PositionNotEmpty,
    #[msg("Invalid tick spacing")]
    InvalidTickSpacing,
    #[msg("Invalid strategy type")]
    InvalidStrategyType,
    #[msg("Strategy not active")]
    StrategyNotActive,
    #[msg("Rebalance threshold not met")]
    RebalanceThresholdNotMet,
    #[msg("Invalid risk profile")]
    InvalidRiskProfile,

    // 交易错误 (200-299)
    #[msg("Slippage tolerance exceeded")]
    SlippageExceeded,
    #[msg("Deadline exceeded")]
    DeadlineExceeded,
    #[msg("Invalid swap path")]
    InvalidSwapPath,
    #[msg("Invalid fee tier")]
    InvalidFeeTier,
    #[msg("Price impact too high")]
    PriceImpactTooHigh,
    #[msg("Minimum output not met")]
    MinimumOutputNotMet,

    // MEV保护错误 (300-399)
    #[msg("Intent expired")]
    IntentExpired,
    #[msg("Intent already executed")]
    IntentAlreadyExecuted,
    #[msg("Invalid intent signature")]
    InvalidIntentSignature,
    #[msg("Batch execution failed")]
    BatchExecutionFailed,
    #[msg("Intent pool full")]
    IntentPoolFull,
    #[msg("Invalid intent priority")]
    InvalidIntentPriority,
    #[msg("MEV protection not enabled")]
    MevProtectionNotEnabled,

    // 治理错误 (400-499)
    #[msg("Proposal not active")]
    ProposalNotActive,
    #[msg("Voting period ended")]
    VotingPeriodEnded,
    #[msg("Insufficient voting power")]
    InsufficientVotingPower,
    #[msg("Proposal already executed")]
    ProposalAlreadyExecuted,
    #[msg("Invalid proposal")]
    InvalidProposal,
    #[msg("Quorum not reached")]
    QuorumNotReached,

    // 数学计算错误 (500-599)
    #[msg("Division by zero")]
    DivisionByZero,
    #[msg("Invalid sqrt price")]
    InvalidSqrtPrice,
    #[msg("Tick out of range")]
    TickOutOfRange,
    #[msg("Invalid calculation")]
    InvalidCalculation,
    #[msg("Price out of bounds")]
    PriceOutOfBounds,
    #[msg("Liquidity calculation failed")]
    LiquidityCalculationFailed,

    // 跨链错误 (600-699)
    #[msg("Cross-chain transfer failed")]
    CrossChainTransferFailed,
    #[msg("Invalid bridge signature")]
    InvalidBridgeSignature,
    #[msg("Chain not supported")]
    ChainNotSupported,
    #[msg("Asset not supported")]
    AssetNotSupported,
    #[msg("Bridge paused")]
    BridgePaused,
    #[msg("Insufficient bridge liquidity")]
    InsufficientBridgeLiquidity,
    #[msg("Cross-chain timeout")]
    CrossChainTimeout,

    // AI 决策系统错误 (700-799)
    #[msg("AI model not initialized")]
    AiModelNotInitialized,
    #[msg("Invalid prediction data")]
    InvalidPredictionData,
    #[msg("Optimization failed")]
    OptimizationFailed,
    #[msg("Strategy recommendation failed")]
    StrategyRecommendationFailed,
    #[msg("Market data unavailable")]
    MarketDataUnavailable,

    // 费用和收益错误 (800-899)
    #[msg("Fee calculation failed")]
    FeeCalculationFailed,
    #[msg("Reward distribution failed")]
    RewardDistributionFailed,
    #[msg("Insufficient fee balance")]
    InsufficientFeeBalance,
    #[msg("Dynamic fee adjustment failed")]
    DynamicFeeAdjustmentFailed,
}

// 错误工具函数
pub fn map_math_error(error: &str) -> FluxDexError {
    match error {
        "Division by zero" => FluxDexError::DivisionByZero,
        "Overflow" => FluxDexError::Overflow,
        "Underflow" => FluxDexError::Underflow,
        "Price out of bounds" => FluxDexError::PriceOutOfBounds,
        _ => FluxDexError::InvalidCalculation,
    }
}

// 错误检查宏
#[macro_export]
macro_rules! require {
    ($condition:expr, $error:expr) => {
        if !$condition {
            return Err($error.into());
        }
    };
}

#[macro_export]
macro_rules! require_eq {
    ($left:expr, $right:expr, $error:expr) => {
        if $left != $right {
            return Err($error.into());
        }
    };
}

#[macro_export]
macro_rules! require_neq {
    ($left:expr, $right:expr, $error:expr) => {
        if $left == $right {
            return Err($error.into());
        }
    };
}

#[macro_export]
macro_rules! require_gt {
    ($left:expr, $right:expr, $error:expr) => {
        if $left <= $right {
            return Err($error.into());
        }
    };
}

#[macro_export]
macro_rules! require_gte {
    ($left:expr, $right:expr, $error:expr) => {
        if $left < $right {
            return Err($error.into());
        }
    };
}
