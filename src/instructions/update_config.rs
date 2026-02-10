use anchor_lang::prelude::*;
use crate::state::Config;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct UpdateConfigParams {
    pub new_authority: Option<Pubkey>,
    pub min_reflection_pool: Option<u64>,
    pub min_reflection_per_account: Option<u64>,
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

    if let Some(new_authority) = params.new_authority {
        config.authority = new_authority;
    }

    if let Some(min_reflection_pool) = params.min_reflection_pool {
        config.min_reflection_pool = min_reflection_pool;
    }

    if let Some(min_reflection_per_account) = params.min_reflection_per_account {
        config.min_reflection_per_account = min_reflection_per_account;
    }

    config.updated_at = Clock::get().unwrap().unix_timestamp;

    msg!("Config updated by authority: {}", ctx.accounts.authority.key());
    Ok(())
}