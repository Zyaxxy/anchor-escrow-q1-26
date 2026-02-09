use anchor_lang::prelude::*;

use crate::Escrow;

#[derive(Accounts)]
pub struct make<'info>{
    #[account(mut)]
    pub maker: Signer<'info>,
    mint_a: InterfaceAccount<'info, Mint>,
    mint_b:InterfaceAccount<'info, Mint>,

}