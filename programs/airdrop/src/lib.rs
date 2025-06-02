use anchor_lang::prelude::*;
use solana_program::{program_error::ProgramError, sysvar::instructions};
use solana_program::{ed25519_program::ID as ED25519_ID, instruction::Instruction};
use solana_program::pubkey;

declare_id!("GjPcPEmbJ7gXvWMitrHmFG7uYEAah3NTrbNmHdsCsCDo");

#[program]
pub mod airdrop {
    use super::*;

    pub fn verify_airdrop_signature(
        ctx: Context<VerifyAirdrop>,
        recipient: Pubkey,
        amount: u64,
        signature: [u8; 64],
    ) -> Result<()> {
        const EXPECTED_DISTRIBUTOR: Pubkey = pubkey!("3hfsuSzwmg95ndJH3iJBddSgcoJ647ZVpVk2PfzPiE9V"); // Replace with the actual distributor's public key before deployment.
        let message = build_airdrop_message(&recipient, amount);
        let ed25519_instruction_result = instructions::load_instruction_at_checked(0, &ctx.accounts.ed25519_instruction_sysvar);
        if ed25519_instruction_result.is_err() {
            return Err(ed25519_instruction_result.unwrap_err().into());
        }
        let ed25519_instruction = ed25519_instruction_result.unwrap();

        verify_ed25519_ix(
            &ed25519_instruction,
            &EXPECTED_DISTRIBUTOR.to_bytes(),
            &message,
            &signature,
        )?;

        Ok(())
    }
}

pub fn build_airdrop_message(recipient: &Pubkey, amount: u64) -> Vec<u8> {
    let mut message = Vec::new();
    message.extend_from_slice(recipient.as_ref());
    message.extend_from_slice(&amount.to_le_bytes());
    message
}

pub fn verify_ed25519_ix(ed25519_instruction: &Instruction, pubkey: &[u8; 32], message: &[u8], signature: &[u8; 64]) -> Result<()> {
    if ed25519_instruction.program_id != ED25519_ID || ed25519_instruction.accounts.len() != 0 || ed25519_instruction.data.len() != 16 + 32 + 64 + message.len() {
        return Err(ProgramError::InvalidInstructionData.into());
    }
    check_ed25519_data(&ed25519_instruction.data, pubkey, message, signature)?;
    Ok(())
}

pub fn check_ed25519_data(data: &[u8], pubkey: &[u8], message: &[u8], signature: &[u8]) -> Result<()> {
    let data_pubkey = &data[16..48];
    let data_signature = &data[48..112];
    let data_message = &data[112..];

    if data_pubkey != pubkey || data_signature != signature || data_message != message {
        return Err(ErrorCode::InvalidDistributorKey.into());
    }

    Ok(())
}

#[derive(Accounts)]
pub struct VerifyAirdrop<'info> {
    /// CHECK: This is safe because the account is constrained to the Instructions sysvar address,
    /// and load_instruction_at_checked validates the instruction data.
    #[account(address = instructions::ID)]
    pub ed25519_instruction_sysvar: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Invalid distributor public key in Ed25519 instruction")]
    InvalidDistributorKey,
}