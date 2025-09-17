use anchor_lang::prelude::*;
pub mod error;
pub mod instructions;
pub mod state;
pub mod utils;

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
