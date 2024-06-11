use anchor_lang::prelude::*;

declare_id!("7rXPhTNNtikfkRP6edT3JEM6uCiJS9kCwz5HsHrLnQeq");

#[program]
pub mod merkle_root {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
