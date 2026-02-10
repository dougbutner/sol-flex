use anchor_lang::prelude::*;
use crate::state::{Config, UserPreferences};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct BanUserParams {
    pub user_to_ban: Pubkey,
    pub ban_status: bool,
}

#[derive(Accounts)]
#[instruction(params: BanUserParams)]
pub struct BanUser<'info> {
    #[account(
        seeds = [Config::SEED_PREFIX],
        bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        mut,
        seeds = [UserPreferences::SEED_PREFIX, params.user_to_ban.as_ref()],
        bump
    )]
    pub user_preferences: Account<'info, UserPreferences>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

pub fn handler(ctx: Context<BanUser>, params: BanUserParams) -> Result<()> {
    let config = &ctx.accounts.config;
    let user_preferences = &mut ctx.accounts.user_preferences;
    let authority = &ctx.accounts.authority;

    // Check authorization - only the config authority or the user themselves (for banning only)
    let is_authorized = if params.ban_status {
        // Anyone can ban themselves
        authority.key() == params.user_to_ban || authority.key() == config.authority
    } else {
        // Only authority can unban
        authority.key() == config.authority
    };

    require!(is_authorized, crate::errors::SolFlexError::Unauthorized);

    // Update ban status
    user_preferences.is_banned = params.ban_status;
    user_preferences.updated_at = Clock::get().unwrap().unix_timestamp;

    msg!("User {} ban status set to {}", params.user_to_ban, params.ban_status);

    Ok(())
}