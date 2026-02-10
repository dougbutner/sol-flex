use anchor_lang::prelude::*;
use std::collections::BTreeMap;

#[account]
#[derive(InitSpace)]
pub struct TokenAccount {
    pub owner: Pubkey,
    pub mint: Pubkey,
    pub balance: u64,
    pub is_banned: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

impl TokenAccount {
    pub const SEED_PREFIX: &'static [u8] = crate::constants::TOKEN_ACCOUNT_SEED;

    pub fn new(owner: Pubkey, mint: Pubkey) -> Self {
        let clock = Clock::get().unwrap();
        Self {
            owner,
            mint,
            balance: 0,
            is_banned: false,
            created_at: clock.unix_timestamp,
            updated_at: clock.unix_timestamp,
        }
    }
}

#[account]
#[derive(InitSpace)]
pub struct TokenPool {
    pub pool_id: u64,
    pub token_mint: Pubkey,
    pub token_program: Pubkey,
    pub pool_address: Pubkey, // Jupiter pool address
    pub is_active: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

impl TokenPool {
    pub const SEED_PREFIX: &'static [u8] = crate::constants::TOKEN_POOL_SEED;

    pub fn new(pool_id: u64, token_mint: Pubkey, token_program: Pubkey, pool_address: Pubkey) -> Self {
        let clock = Clock::get().unwrap();
        Self {
            pool_id,
            token_mint,
            token_program,
            pool_address,
            is_active: true,
            created_at: clock.unix_timestamp,
            updated_at: clock.unix_timestamp,
        }
    }
}

#[account]
#[derive(InitSpace)]
pub struct UserPreferences {
    pub owner: Pubkey,
    pub preferred_token_mint: Pubkey, // User's preferred reflection token
    pub custom_memo: String, // Custom memo for reflections
    pub tree_parent: Pubkey, // Inheritance tree parent
    pub tree_rate: u16, // Inheritance rate (0-10000 basis points)
    pub is_banned: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

impl UserPreferences {
    pub const SEED_PREFIX: &'static [u8] = crate::constants::USER_PREFERENCES_SEED;

    pub fn new(owner: Pubkey) -> Self {
        let clock = Clock::get().unwrap();
        Self {
            owner,
            preferred_token_mint: Pubkey::default(),
            custom_memo: String::new(),
            tree_parent: owner,
            tree_rate: 10000, // 100% default
            is_banned: false,
            created_at: clock.unix_timestamp,
            updated_at: clock.unix_timestamp,
        }
    }
}

// Global token pools registry (uses BTreeMap for efficient lookups)
#[account]
#[derive(InitSpace)]
pub struct GlobalTokenPools {
    pub pools: BTreeMap<u64, TokenPool>,
    pub next_pool_id: u64,
    pub authority: Pubkey,
    pub created_at: i64,
    pub updated_at: i64,
}

impl GlobalTokenPools {
    pub const SEED_PREFIX: &'static [u8] = crate::constants::GLOBAL_POOLS_SEED;

    pub fn new(authority: Pubkey) -> Self {
        let clock = Clock::get().unwrap();
        Self {
            pools: BTreeMap::new(),
            next_pool_id: 1,
            authority,
            created_at: clock.unix_timestamp,
            updated_at: clock.unix_timestamp,
        }
    }

    pub fn add_pool(&mut self, pool: TokenPool) -> Result<()> {
        require!(!self.pools.contains_key(&pool.pool_id), crate::errors::SolFlexError::PoolAlreadyExists);
        self.pools.insert(pool.pool_id, pool);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }

    pub fn remove_pool(&mut self, pool_id: u64) -> Result<()> {
        require!(self.pools.remove(&pool_id).is_some(), crate::errors::SolFlexError::PoolNotFound);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }

    pub fn get_pool(&self, pool_id: u64) -> Result<&TokenPool> {
        self.pools.get(&pool_id).ok_or(crate::errors::SolFlexError::PoolNotFound.into())
    }

    pub fn get_next_pool_id(&mut self) -> u64 {
        let id = self.next_pool_id;
        self.next_pool_id += 1;
        id
    }
}