use anchor_lang::prelude::*;
use instructions::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod state;

declare_id!("YourProgramIDHere1111111111111111111111111111111");

#[program]
pub mod sol_flex {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::handler(ctx)
    }

    pub fn update_config(ctx: Context<UpdateConfig>, params: UpdateConfigParams) -> Result<()> {
        instructions::update_config::handler(ctx, params)
    }

    pub fn add_to_blocklist(ctx: Context<AddToBlocklist>, params: AddToBlocklistParams) -> Result<()> {
        instructions::add_to_blocklist::handler(ctx, params)
    }

    pub fn remove_from_blocklist(ctx: Context<RemoveFromBlocklist>, params: RemoveFromBlocklistParams) -> Result<()> {
        instructions::remove_from_blocklist::handler(ctx, params)
    }

    pub fn reflect(ctx: Context<Reflect>) -> Result<()> {
        instructions::reflect::handler(ctx)
    }

    pub fn jupiter_swap(ctx: Context<JupiterSwap>, params: JupiterSwapParams) -> Result<()> {
        instructions::jupiter_swap::handler(ctx, params)
    }

    pub fn set_user_preferences(ctx: Context<SetUserPreferences>, params: SetUserPreferencesParams) -> Result<()> {
        instructions::set_user_preferences::handler(ctx, params)
    }

    pub fn ban_user(ctx: Context<BanUser>, params: BanUserParams) -> Result<()> {
        instructions::ban_user::handler(ctx, params)
    }

    pub fn add_pool(ctx: Context<AddPool>, params: AddPoolParams) -> Result<()> {
        instructions::manage_pool::add_pool_handler(ctx, params)
    }

    pub fn remove_pool(ctx: Context<RemovePool>, params: RemovePoolParams) -> Result<()> {
        instructions::manage_pool::remove_pool_handler(ctx, params)
    }

    pub fn set_distribution_config(ctx: Context<SetDistributionConfig>, params: SetDistributionConfigParams) -> Result<()> {
        instructions::set_distribution_config::handler(ctx, params)
    }

    pub fn create_position(ctx: Context<CreatePosition>, params: CreatePositionParams) -> Result<()> {
        instructions::position_tracking::create_position_handler(ctx, params)
    }

    pub fn update_position(ctx: Context<UpdatePosition>, params: UpdatePositionParams) -> Result<()> {
        instructions::position_tracking::update_position_handler(ctx, params)
    }

    pub fn close_position(ctx: Context<ClosePosition>, params: ClosePositionParams) -> Result<()> {
        instructions::position_tracking::close_position_handler(ctx, params)
    }
}