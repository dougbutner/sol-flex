use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub version: u8,
    #[max_len(100)]
    pub blocklist: Vec<Pubkey>,
    pub is_initialized: bool,
    pub created_at: i64,
    pub updated_at: i64,
    pub min_reflection_pool: u64, // Minimum reflection pool amount before sending
    pub min_reflection_per_account: u64, // Minimum per-account amount to activate reflections
}

impl Config {
    pub const SEED_PREFIX: &'static [u8] = crate::constants::CONFIG_SEED;

    pub fn new(authority: Pubkey) -> Self {
        let clock = Clock::get().unwrap();
        Self {
            authority,
            version: 1,
            blocklist: Vec::new(),
            is_initialized: true,
            created_at: clock.unix_timestamp,
            updated_at: clock.unix_timestamp,
            min_reflection_pool: 100000, // 100,000 tokens minimum in reflection pool
            min_reflection_per_account: 10000, // 10,000 tokens minimum per account
        }
    }

    pub fn add_to_blocklist(&mut self, account: Pubkey) -> Result<()> {
        require!(!self.blocklist.contains(&account), crate::errors::SolFlexError::AlreadyInBlocklist);
        require!(self.blocklist.len() < crate::constants::MAX_BLOCKLIST_SIZE, crate::errors::SolFlexError::BlocklistFull);

        self.blocklist.push(account);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }

    pub fn remove_from_blocklist(&mut self, account: Pubkey) -> Result<()> {
        let position = self.blocklist.iter().position(|&x| x == account)
            .ok_or(crate::errors::SolFlexError::NotInBlocklist)?;

        self.blocklist.remove(position);
        self.updated_at = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }

    pub fn is_blocklisted(&self, account: Pubkey) -> bool {
        self.blocklist.contains(&account)
    }

    pub fn update_config(&mut self, authority: Pubkey, min_reflection_pool: u64, min_reflection_per_account: u64) -> Result<()> {
        self.authority = authority;
        self.min_reflection_pool = min_reflection_pool;
        self.min_reflection_per_account = min_reflection_per_account;
        self.updated_at = Clock::get().unwrap().unix_timestamp;
        Ok(())
    }
}