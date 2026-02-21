use anchor_lang::prelude::*;
use crate::state::{Config, UserPreferences};

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SetUserPreferencesParams {
    pub preferred_pool_id: u64,
    pub custom_memo: String,
}

#[derive(Accounts)]
#[instruction(params: SetUserPreferencesParams)]
pub struct SetUserPreferences<'info> {
    #[account(
        seeds = [Config::SEED_PREFIX],
        bump = config.bump
    )]
    pub config: Account<'info, Config>,

    #[account(
        init_if_needed,
        payer = authority,
        space = 8 + UserPreferences::INIT_SPACE,
        seeds = [UserPreferences::SEED_PREFIX, user.key().as_ref()],
        bump
    )]
    pub user_preferences: Account<'info, UserPreferences>,

    /// CHECK: User whose preference PDA is derived and updated.
    pub user: UncheckedAccount<'info>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SetUserPreferences>, params: SetUserPreferencesParams) -> Result<()> {
    let config = &ctx.accounts.config;
    let user_preferences = &mut ctx.accounts.user_preferences;
    let user = &ctx.accounts.user;
    let authority = &ctx.accounts.authority;

    // Either the user or program authority can set/update preferences.
    require!(
        authority.key() == user.key() || authority.key() == config.authority,
        crate::errors::SolFlexError::Unauthorized
    );

    // Validate parameters
    require!(params.custom_memo.len() <= 200, crate::errors::SolFlexError::InvalidMemoLength);

    // Initialize the user preferences
    if user_preferences.owner == Pubkey::default() {
        **user_preferences = UserPreferences::new(user.key());
    }

    // Update preferences
    user_preferences.preferred_pool_id = params.preferred_pool_id;
    user_preferences.custom_memo = params.custom_memo;
    user_preferences.updated_at = Clock::get().unwrap().unix_timestamp;

    msg!("User preferences updated for {}", user.key());

    Ok(())
}