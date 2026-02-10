use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

pub use instructions::*;
pub use state::*;

declare_id!("H5CNM8BPgH63ut9RK7aoJBohjk8YPtErvrpjzadpFH33");

#[program]
pub mod anchor_escrow_q1_26 {
    use super::*;
    pub fn make(ctx: Context<Make>, seed: u64, deposit: u64, receive: u64) -> Result<()> {
        ctx.accounts.init_escrow(seed, receive, &ctx.bumps)?;
        ctx.accounts.deposit(deposit)?;
        Ok(())
    }
}

