use anchor_lang::prelude::*;
use crate::state::{Config, GlobalTokenPools};

#[derive(Accounts)]
pub struct InitializeGlobalPools<'info> {
    #[account(
        seeds = [Config::SEED_PREFIX],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + GlobalTokenPools::INIT_SPACE,
        seeds = [GlobalTokenPools::SEED_PREFIX],
        bump
    )]
    pub global_pools: Account<'info, GlobalTokenPools>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<InitializeGlobalPools>) -> Result<()> {
    let config = &ctx.accounts.config;
    let global_pools = &mut ctx.accounts.global_pools;

    require!(
        ctx.accounts.authority.key() == config.authority,
        crate::errors::SolFlexError::Unauthorized
    );

    if global_pools.created_at == 0 {
        let bump = ctx.bumps.global_pools;
        **global_pools = GlobalTokenPools::new(config.authority, bump);
    } else {
        global_pools.authority = config.authority;
        global_pools.updated_at = Clock::get()?.unix_timestamp;
    }

    msg!("Global pools registry initialized/updated");
    Ok(())
}
