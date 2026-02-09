use anchor_lang::prelude::*;

use anchor_spl::token_interface::{Mint , TokenInterface,TokenAccount};

use crate::Escrow;

#[derive(Accounts)]
pub struct Make<'info>{
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(mint::token_program = token_program)]
    pub mint_a: InterfaceAccount<'info, Mint>,
    #[account(mint::token_program = token_program)]
    pub mint_b: InterfaceAccount<'info, Mint>,
    pub maker_ata_a: InterfaceAccount<'info, TokenAccount>,
    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info , TokenInterface>
}