use anchor_lang::prelude::*;
use crate::state::UserPreferences;

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct SetUserPreferencesParams {
    pub preferred_pool_id: u64,
    pub custom_memo: String,
}

#[derive(Accounts)]
#[instruction(params: SetUserPreferencesParams)]
pub struct SetUserPreferences<'info> {
    #[account(
        init,
        payer = user,
        space = 8 + UserPreferences::INIT_SPACE,
        seeds = [UserPreferences::SEED_PREFIX, user.key().as_ref()],
        bump
    )]
    pub user_preferences: Account<'info, UserPreferences>,

    #[account(mut)]
    pub user: Signer<'info>,

    pub system_program: Program<'info, System>,
}

pub fn handler(ctx: Context<SetUserPreferences>, params: SetUserPreferencesParams) -> Result<()> {
    let user_preferences = &mut ctx.accounts.user_preferences;
    let user = &ctx.accounts.user;

    // Validate parameters
    require!(params.custom_memo.len() <= 200, crate::errors::SolFlexError::InvalidMemoLength);

    // Initialize the user preferences
    **user_preferences = UserPreferences::new(user.key());

    // Update preferences
    user_preferences.preferred_pool_id = params.preferred_pool_id;
    user_preferences.custom_memo = params.custom_memo;
    user_preferences.updated_at = Clock::get().unwrap().unix_timestamp;

    msg!("User preferences updated for {}", user.key());

    Ok(())
}