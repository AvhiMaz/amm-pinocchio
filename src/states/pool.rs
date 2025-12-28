use bytemuck::{Pod, Zeroable};
use pinocchio::{program_error::ProgramError, pubkey::Pubkey};

#[repr(C)]
#[derive(Copy, Clone, Pod, Zeroable)]
pub struct Pool {
    pub authority: Pubkey,
    pub token_a: Pubkey,
    pub token_b: Pubkey,
    pub lp_mint: Pubkey,
    pub vault_a: Pubkey,
    pub vault_b: Pubkey,
    pub reserve_a: u64,
    pub reserve_b: u64,
    pub fee_rate: u16,
    pub bump: u8,
    pub lp_mint_bump: u8,
    pub padding: [u8; 4],
}

impl Pool {
    pub const LEN: usize = 32 + 32 + 32 + 32 + 32 + 32 + 8 + 8 + 2 + 1 + 1 + 4;

    pub fn set_inner_full(
        &mut self,
        authority: Pubkey,
        token_a: Pubkey,
        token_b: Pubkey,
        lp_mint: Pubkey,
        vault_a: Pubkey,
        vault_b: Pubkey,
        reserve_a: u64,
        reserve_b: u64,
        fee_rate: u16,
        bump: u8,
        lp_mint_bump: u8,
        padding: [u8; 4],
    ) {
        self.authority = authority;
        self.token_a = token_a;
        self.token_b = token_b;
        self.lp_mint = lp_mint;
        self.vault_a = vault_a;
        self.vault_b = vault_b;
        self.reserve_a = reserve_a;
        self.reserve_b = reserve_b;
        self.fee_rate = fee_rate;
        self.bump = bump;
        self.lp_mint_bump = lp_mint_bump;
        self.padding = padding;
    }

    pub fn load_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        bytemuck::try_from_bytes_mut(data).map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn load(data: &[u8]) -> Result<&Self, ProgramError> {
        bytemuck::try_from_bytes(data).map_err(|_| ProgramError::InvalidAccountData)
    }
}
