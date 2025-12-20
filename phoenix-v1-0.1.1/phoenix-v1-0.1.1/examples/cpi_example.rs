//! Example: Using Phoenix DEX via Cross-Program Invocation (CPI)
//!
//! This example demonstrates how to integrate Phoenix into your Solana program
//! to execute trades via CPI.

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

// Import Phoenix types when using the cpi feature
#[cfg(feature = "cpi")]
use phoenix::{
    program::cpi::{swap_signed, get_vault_address_helper},
    state::{OrderPacket, Side},
    quantities::WrapperU64,
};

// Program instruction
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub enum ProgramInstruction {
    /// Execute a swap on Phoenix DEX
    ///
    /// Accounts expected:
    /// 0. `[signer]` Authority (PDA of this program)
    /// 1. `[writable]` Phoenix market
    /// 2. `[writable]` Base token account
    /// 3. `[writable]` Quote token account
    /// 4. `[writable]` Base vault
    /// 5. `[writable]` Quote vault
    /// 6. `[]` Token program
    /// 7. `[]` Phoenix program
    ExecuteSwap {
        /// The side of the order (Bid/Ask)
        side: u8,
        /// Price in ticks
        price_in_ticks: u64,
        /// Number of base lots
        num_base_lots: u64,
    },
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = ProgramInstruction::try_from_slice(instruction_data)
        .map_err(|_| ProgramError::InvalidInstructionData)?;

    match instruction {
        ProgramInstruction::ExecuteSwap {
            side,
            price_in_ticks,
            num_base_lots,
        } => {
            msg!("Instruction: ExecuteSwap");
            process_execute_swap(
                program_id,
                accounts,
                side,
                price_in_ticks,
                num_base_lots,
            )
        }
    }
}

#[cfg(feature = "cpi")]
fn process_execute_swap(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    side: u8,
    price_in_ticks: u64,
    num_base_lots: u64,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    
    let authority = next_account_info(accounts_iter)?;
    let market = next_account_info(accounts_iter)?;
    let base_account = next_account_info(accounts_iter)?;
    let quote_account = next_account_info(accounts_iter)?;
    let base_vault = next_account_info(accounts_iter)?;
    let quote_vault = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let phoenix_program = next_account_info(accounts_iter)?;

    // Verify authority is a PDA of this program
    let (expected_authority, bump) = Pubkey::find_program_address(
        &[b"authority"],
        program_id,
    );
    
    if authority.key != &expected_authority {
        return Err(ProgramError::InvalidAccountData);
    }

    // Create the order packet
    let order_packet = OrderPacket::new_ioc_by_lots(
        if side == 0 { Side::Bid } else { Side::Ask },
        WrapperU64::new(price_in_ticks),
        num_base_lots,
        WrapperU64::new(u64::MAX), // Self trade behavior
        None, // Match limit
        false, // Use only deposited funds
    );

    msg!(
        "Executing swap: side={:?}, price={}, lots={}",
        if side == 0 { "Bid" } else { "Ask" },
        price_in_ticks,
        num_base_lots
    );

    // Execute the swap via CPI with PDA signing
    let authority_seeds = &[b"authority".as_ref(), &[bump]];
    swap_signed(
        phoenix_program.key,
        market,
        authority,
        base_account,
        quote_account,
        base_vault,
        quote_vault,
        token_program,
        &order_packet,
        &[authority_seeds],
    )?;

    msg!("Swap executed successfully");
    Ok(())
}

#[cfg(not(feature = "cpi"))]
fn process_execute_swap(
    _program_id: &Pubkey,
    _accounts: &[AccountInfo],
    _side: u8,
    _price_in_ticks: u64,
    _num_base_lots: u64,
) -> ProgramResult {
    msg!("Error: This example requires the 'cpi' feature to be enabled");
    Err(ProgramError::InvalidInstructionData)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_instruction_serialization() {
        let instruction = ProgramInstruction::ExecuteSwap {
            side: 0,
            price_in_ticks: 1000,
            num_base_lots: 10,
        };
        
        let serialized = instruction.try_to_vec().unwrap();
        let deserialized = ProgramInstruction::try_from_slice(&serialized).unwrap();
        
        match deserialized {
            ProgramInstruction::ExecuteSwap {
                side,
                price_in_ticks,
                num_base_lots,
            } => {
                assert_eq!(side, 0);
                assert_eq!(price_in_ticks, 1000);
                assert_eq!(num_base_lots, 10);
            }
        }
    }
}
