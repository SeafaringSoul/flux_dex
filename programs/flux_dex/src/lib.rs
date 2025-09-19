use anchor_lang::prelude::*;

// 先声明模块
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

// 导入所需的类型 - 注意这里要指定具体路径
pub use error::*;
pub use instructions::*;
pub use state::*;
pub use utils::*;

declare_id!("EKbi7QjTXoTk5hSpRh5fpscNDrGut2yUaohtfemH1Peg");

#[program]
pub mod flux_dex {
    use super::*;

    pub fn initialize_pool(ctx: Context<InitializePool>, base_fee_bps: u16) -> Result<()> {
        instructions::initialize_pool_handler(ctx, base_fee_bps)
    }
}
