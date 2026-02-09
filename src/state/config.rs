use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Config {
    pub authority: Pubkey,
    pub version: u8,
    pub blocklist: Vec<Pubkey>,
    pub is_initialized: bool,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Config {
    pub const SEED_PREFIX: &'static [u8] = b"config";

    pub fn new(authority: Pubkey) -> Self {
        let clock = Clock::get().unwrap();
        Self {
            authority,
            version: 1,
            blocklist: Vec::new(),
            is_initialized: true,
            created_at: clock.unix_timestamp,
            updated_at: clock.unix_timestamp,
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
}