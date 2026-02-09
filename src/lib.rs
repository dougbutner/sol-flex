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

    pub fn reflect(ctx: Context<Reflect>, params: ReflectParams) -> Result<()> {
        instructions::reflect::handler(ctx, params)
    }
}