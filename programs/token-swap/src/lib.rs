pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("5j4feBFRPfb42S5Qu8pGm3vV3hof9W4NQS8GTBz1EdCx");

#[program]
pub mod token_swap {
    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>) -> Result<()> {
        instructions::make_offer::send_offered_tokens_to_vault(ctx)?;
        // instructions::make_offer::save_offer(ctx)?;
        Ok(())
    }
}
