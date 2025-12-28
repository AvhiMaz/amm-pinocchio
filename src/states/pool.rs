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
    pub _padding: [u8; 4],
}

impl Pool {
    pub const LEN: usize = core::mem::size_of::<Self>();

    pub fn set_inner_full(&mut self, args: Pool) {
        self.authority = args.authority;
        self.token_a = args.token_a;
        self.token_b = args.token_b;
        self.lp_mint = args.lp_mint;
        self.vault_a = args.vault_a;
        self.vault_b = args.vault_b;
        self.reserve_a = args.reserve_a;
        self.reserve_b = args.reserve_b;
        self.fee_rate = args.fee_rate;
        self.bump = args.bump;
        self.lp_mint_bump = args.lp_mint_bump;
    }

    pub fn load_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError> {
        bytemuck::try_from_bytes_mut(data).map_err(|_| ProgramError::InvalidAccountData)
    }

    pub fn load(data: &[u8]) -> Result<&Self, ProgramError> {
        bytemuck::try_from_bytes(data).map_err(|_| ProgramError::InvalidAccountData)
    }
}
