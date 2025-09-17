use crate::error::FluxDexError;
use anchor_lang::prelude::*;

/// 固定点小数类型，用于高精度计算
#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, Debug, PartialEq)]
pub struct FixedPoint {
    pub value: u128,
}

impl FixedPoint {
    pub const DECIMALS: u32 = 18;
    pub const SCALE: u128 = 1_000_000_000_000_000_000; // 10^18

    pub fn new(value: u128) -> Self {
        Self { value }
    }

    pub fn from_u64(value: u64) -> Self {
        Self {
            value: value as u128 * Self::SCALE,
        }
    }

    pub fn to_u64(&self) -> Result<u64> {
        Ok((self.value / Self::SCALE) as u64)
    }

    pub fn mul(&self, other: &Self) -> Result<Self> {
        let result = self
            .value
            .checked_mul(other.value)
            .ok_or(FluxDexError::Overflow)?
            .checked_div(Self::SCALE)
            .ok_or(FluxDexError::DivisionByZero)?;
        Ok(Self::new(result))
    }

    pub fn div(&self, other: &Self) -> Result<Self> {
        require!(other.value > 0, FluxDexError::DivisionByZero);
        let result = self
            .value
            .checked_mul(Self::SCALE)
            .ok_or(FluxDexError::Overflow)?
            .checked_div(other.value)
            .ok_or(FluxDexError::DivisionByZero)?;
        Ok(Self::new(result))
    }
}

/// 数学工具库
pub struct MathUtils;

impl MathUtils {
    /// 计算恒定乘积AMM的兑换输出量 (x * y = k)
    pub fn calculate_amm_output(
        input_amount: u64,
        input_reserve: u64,
        output_reserve: u64,
        fee_bps: u16,
    ) -> Result<u64> {
        require!(input_amount > 0, FluxDexError::InvalidInputAmount);
        require!(
            input_reserve > 0 && output_reserve > 0,
            FluxDexError::InsufficientLiquidity
        );

        let fee_factor = 10000u128
            .checked_sub(fee_bps as u128)
            .ok_or(FluxDexError::InvalidCalculation)?;

        let input_with_fee = (input_amount as u128)
            .checked_mul(fee_factor)
            .ok_or(FluxDexError::Overflow)?;

        let numerator = input_with_fee
            .checked_mul(output_reserve as u128)
            .ok_or(FluxDexError::Overflow)?;

        let denominator = (input_reserve as u128)
            .checked_mul(10000)
            .ok_or(FluxDexError::Overflow)?
            .checked_add(input_with_fee)
            .ok_or(FluxDexError::Overflow)?;

        let output = numerator
            .checked_div(denominator)
            .ok_or(FluxDexError::DivisionByZero)?;

        Ok(output as u64)
    }

    /// 计算动态费率（基于波动性和流动性）
    pub fn calculate_dynamic_fee(
        base_fee: u16,
        volatility_score: u16, // 0-10000 (0-100%)
        liquidity_score: u16,  // 0-10000 (0-100%)
    ) -> Result<u16> {
        require!(base_fee <= 10000, FluxDexError::InvalidFeeTier);
        require!(volatility_score <= 10000, FluxDexError::InvalidCalculation);
        require!(liquidity_score <= 10000, FluxDexError::InvalidCalculation);

        // 波动性越高，费率越高
        let volatility_adjustment = volatility_score / 100; // 最高增加1%

        // 流动性越低，费率越高
        let liquidity_adjustment = if liquidity_score < 1000 {
            (1000 - liquidity_score) / 100
        } else {
            0
        };

        let adjusted_fee = (base_fee as u32)
            .saturating_add(volatility_adjustment as u32)
            .saturating_add(liquidity_adjustment as u32)
            .min(10000); // 最高10%

        Ok(adjusted_fee as u16)
    }

    /// 计算价格影响
    pub fn calculate_price_impact(
        input_amount: u64,
        input_reserve: u64,
        output_reserve: u64,
    ) -> Result<u16> {
        require!(input_amount > 0, FluxDexError::InvalidInputAmount);
        require!(
            input_reserve > 0 && output_reserve > 0,
            FluxDexError::InsufficientLiquidity
        );

        let k = (input_reserve as u128)
            .checked_mul(output_reserve as u128)
            .ok_or(FluxDexError::Overflow)?;

        let new_input_reserve = (input_reserve as u128)
            .checked_add(input_amount as u128)
            .ok_or(FluxDexError::Overflow)?;

        let new_output_reserve = k
            .checked_div(new_input_reserve)
            .ok_or(FluxDexError::DivisionByZero)?;

        let output_change = (output_reserve as u128)
            .checked_sub(new_output_reserve)
            .ok_or(FluxDexError::Underflow)?;

        let price_impact = output_change
            .checked_mul(10000)
            .ok_or(FluxDexError::Overflow)?
            .checked_div(output_reserve as u128)
            .ok_or(FluxDexError::DivisionByZero)?;

        Ok(price_impact as u16)
    }

    /// 计算LP代币数量
    pub fn calculate_lp_tokens(
        deposit_a: u64,
        deposit_b: u64,
        reserve_a: u64,
        reserve_b: u64,
        total_supply: u64,
    ) -> Result<u64> {
        if total_supply == 0 {
            // 首次添加流动性，使用几何平均
            let geometric_mean = Self::sqrt((deposit_a as u128) * (deposit_b as u128))?;
            return Ok(geometric_mean as u64);
        }

        // 计算两个比例，取较小值防止套利
        let lp_from_a = (deposit_a as u128)
            .checked_mul(total_supply as u128)
            .ok_or(FluxDexError::Overflow)?
            .checked_div(reserve_a as u128)
            .ok_or(FluxDexError::DivisionByZero)?;

        let lp_from_b = (deposit_b as u128)
            .checked_mul(total_supply as u128)
            .ok_or(FluxDexError::Overflow)?
            .checked_div(reserve_b as u128)
            .ok_or(FluxDexError::DivisionByZero)?;

        Ok(lp_from_a.min(lp_from_b) as u64)
    }

    /// 计算移除流动性时的资产数量
    pub fn calculate_remove_amounts(
        lp_tokens: u64,
        total_supply: u64,
        reserve_a: u64,
        reserve_b: u64,
    ) -> Result<(u64, u64)> {
        require!(lp_tokens > 0, FluxDexError::InvalidInputAmount);
        require!(total_supply > 0, FluxDexError::InsufficientLiquidity);
        require!(
            lp_tokens <= total_supply,
            FluxDexError::InsufficientLiquidity
        );

        let amount_a = (lp_tokens as u128)
            .checked_mul(reserve_a as u128)
            .ok_or(FluxDexError::Overflow)?
            .checked_div(total_supply as u128)
            .ok_or(FluxDexError::DivisionByZero)?;

        let amount_b = (lp_tokens as u128)
            .checked_mul(reserve_b as u128)
            .ok_or(FluxDexError::Overflow)?
            .checked_div(total_supply as u128)
            .ok_or(FluxDexError::DivisionByZero)?;

        Ok((amount_a as u64, amount_b as u64))
    }

    /// 计算智能流动性管理的最优价格区间
    pub fn calculate_optimal_range(
        current_price: FixedPoint,
        volatility: u16,  // 历史波动率
        risk_profile: u8, // 风险档次: 1=保守, 2=平衡, 3=激进
    ) -> Result<(FixedPoint, FixedPoint)> {
        require!(
            risk_profile >= 1 && risk_profile <= 3,
            FluxDexError::InvalidRiskProfile
        );

        // 根据风险档次调整价格区间宽度
        let base_range = match risk_profile {
            1 => 2000, // 保守: ±20%
            2 => 3000, // 平衡: ±30%
            3 => 5000, // 激进: ±50%
            _ => return Err(FluxDexError::InvalidRiskProfile.into()),
        };

        // 根据波动率调整区间
        let volatility_factor = 10000u128 + (volatility as u128 * 50); // 波动率每1%扩大0.5%区间
        let adjusted_range = (base_range as u128 * volatility_factor) / 10000;

        let range_multiplier =
            FixedPoint::new(FixedPoint::SCALE + (adjusted_range * FixedPoint::SCALE) / 10000);

        let lower_price = current_price.div(&range_multiplier)?;
        let upper_price = current_price.mul(&range_multiplier)?;

        Ok((lower_price, upper_price))
    }

    /// 计算再平衡信号
    pub fn should_rebalance(
        current_price: FixedPoint,
        lower_bound: FixedPoint,
        upper_bound: FixedPoint,
        threshold: u16, // 阈值，如5% = 500
    ) -> Result<bool> {
        let range_size = upper_bound.value - lower_bound.value;
        let threshold_amount = (range_size * threshold as u128) / 10000;

        let needs_rebalance = current_price.value <= lower_bound.value + threshold_amount
            || current_price.value >= upper_bound.value - threshold_amount;

        Ok(needs_rebalance)
    }

    /// 计算平方根（用于几何平均等计算）
    pub fn sqrt(x: u128) -> Result<u128> {
        if x == 0 {
            return Ok(0);
        }

        let mut result = x;
        let mut temp = (x / 2) + 1;

        while temp < result {
            result = temp;
            temp = (x / temp + temp) / 2;
        }

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_point_operations() {
        let a = FixedPoint::from_u64(100);
        let b = FixedPoint::from_u64(50);

        let result = a.mul(&b).unwrap();
        assert_eq!(result.to_u64().unwrap(), 5000);

        let result = a.div(&b).unwrap();
        assert_eq!(result.to_u64().unwrap(), 2);
    }

    #[test]
    fn test_amm_calculation() {
        let output = MathUtils::calculate_amm_output(
            1000,  // input
            10000, // input reserve
            20000, // output reserve
            300,   // 3% fee
        )
        .unwrap();

        // 验证输出量合理
        assert!(output > 0);
        assert!(output < 2000); // 应该小于无滑点情况下的输出
    }

    #[test]
    fn test_dynamic_fee() {
        let fee = MathUtils::calculate_dynamic_fee(
            300,  // 3% base fee
            1000, // 10% volatility
            5000, // 50% liquidity score
        )
        .unwrap();

        assert!(fee > 300); // 应该高于基础费率
        assert!(fee <= 10000); // 不应超过最大费率
    }
}
