use anchor_lang::prelude::*;
use crate::state::Config;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct RemoveFromBlocklistParams {
    pub account_to_unblock: Pubkey,
}

#[derive(Accounts)]
pub struct RemoveFromBlocklist<'info> {
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

pub fn handler(ctx: Context<RemoveFromBlocklist>, params: RemoveFromBlocklistParams) -> Result<()> {
    let config = &mut ctx.accounts.config;

    config.remove_from_blocklist(params.account_to_unblock)?;

    msg!("Account {} removed from blocklist by authority: {}", params.account_to_unblock, ctx.accounts.authority.key());
    Ok(())
}