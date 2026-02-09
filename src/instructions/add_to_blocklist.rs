use anchor_lang::prelude::*;
use crate::state::Config;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct AddToBlocklistParams {
    pub account_to_block: Pubkey,
}

#[derive(Accounts)]
pub struct AddToBlocklist<'info> {
    #[account(
        mut,
        seeds = [Config::SEED_PREFIX],
        bump,
        has_one = authority
    )]
    pub config: Account<'info, Config>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<AddToBlocklist>, params: AddToBlocklistParams) -> Result<()> {
    let config = &mut ctx.accounts.config;

    config.add_to_blocklist(params.account_to_block)?;

    msg!("Account {} added to blocklist by authority: {}", params.account_to_block, ctx.accounts.authority.key());
    Ok(())
}