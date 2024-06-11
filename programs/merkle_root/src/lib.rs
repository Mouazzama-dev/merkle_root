use anchor_lang::prelude::*;
use sha2::{Sha256, Digest};

declare_id!("7rXPhTNNtikfkRP6edT3JEM6uCiJS9kCwz5HsHrLnQeq");

#[program]
mod merkle_rewards {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, root: [u8; 32]) -> Result<()> {
        let merkle_account = &mut ctx.accounts.merkle_account;
        merkle_account.merkle_root = root;
        Ok(())
    }

    pub fn claim(ctx: Context<Claim>, proof: Vec<[u8; 32]>, leaf: [u8; 32]) -> Result<()> {
        let merkle_account = &ctx.accounts.merkle_account;

        // Verify the proof
        let mut computed_hash = leaf;
        for p in proof.iter() {
            if computed_hash <= *p {
                computed_hash = hash_concat(&computed_hash, p);
            } else {
                computed_hash = hash_concat(p, &computed_hash);
            }
        }

        require!(computed_hash == merkle_account.merkle_root, ErrorCode::InvalidProof);

        // Process the reward claim
        let user = &mut ctx.accounts.user;
        user.has_claimed = true;
        // Transfer reward logic here

        Ok(())
    }
}

fn hash_concat(a: &[u8; 32], b: &[u8; 32]) -> [u8; 32] {
    let mut hasher = Sha256::new();
    hasher.update(a);
    hasher.update(b);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

#[account]
pub struct MerkleAccount {
    pub merkle_root: [u8; 32],
}

#[account]
pub struct User {
    pub has_claimed: bool,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 32)]
    pub merkle_account: Account<'info, MerkleAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)]
    pub user: Account<'info, User>,
    pub merkle_account: Account<'info, MerkleAccount>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid proof provided.")]
    InvalidProof,
}
