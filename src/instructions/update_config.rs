use anchor_lang::prelude::*;
use crate::state::Config;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateConfigParams {
    pub authority: Pubkey,
    pub min_reflection_pool: u64,
    pub min_reflection_per_account: u64,
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
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

pub fn handler(ctx: Context<UpdateConfig>, params: UpdateConfigParams) -> Result<()> {
    let config = &mut ctx.accounts.config;

    config.authority = params.authority;
    config.min_reflection_pool = params.min_reflection_pool;
    config.min_reflection_per_account = params.min_reflection_per_account;
    config.updated_at = Clock::get().unwrap().unix_timestamp;

    msg!("Config updated by authority: {}", ctx.accounts.authority.key());
    Ok(())
}