//! Cross-Program Invocation (CPI) helpers for integrating Phoenix into other Solana programs.
//!
//! This module provides helper functions to invoke Phoenix instructions from other Solana programs.
//! When using Phoenix with the `cpi` feature enabled, these helpers allow seamless integration.

use crate::phoenix_log_authority;
use crate::program::{
    loaders::get_vault_address,
    validation::loaders::get_seat_address,
    PhoenixInstruction,
};
use crate::state::OrderPacket;
use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::invoke,
    program::invoke_signed,
    pubkey::Pubkey,
};

/// Invoke a swap instruction via CPI.
///
/// # Arguments
/// * `program_id` - The Phoenix program ID
/// * `market` - Market account
/// * `trader` - Trader account (must be signer)
/// * `base_account` - Trader's base token account
/// * `quote_account` - Trader's quote token account
/// * `base_vault` - Market's base vault account
/// * `quote_vault` - Market's quote vault account
/// * `token_program` - SPL Token program
/// * `order_packet` - The order details
///
/// # Example
/// ```ignore
/// use phoenix::program::cpi::swap;
/// use phoenix::state::OrderPacket;
///
/// let order_packet = OrderPacket {
///     // ... order details
/// };
///
/// swap(
///     phoenix_program.key,
///     market_info,
///     trader_info,
///     base_account_info,
///     quote_account_info,
///     base_vault_info,
///     quote_vault_info,
///     token_program_info,
///     &order_packet,
/// )?;
/// ```
pub fn swap<'a>(
    program_id: &Pubkey,
    market: &AccountInfo<'a>,
    trader: &AccountInfo<'a>,
    base_account: &AccountInfo<'a>,
    quote_account: &AccountInfo<'a>,
    base_vault: &AccountInfo<'a>,
    quote_vault: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    order_packet: &OrderPacket,
) -> ProgramResult {
    let instruction = Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*program_id, false),
            AccountMeta::new_readonly(phoenix_log_authority::id(), false),
            AccountMeta::new(*market.key, false),
            AccountMeta::new_readonly(*trader.key, true),
            AccountMeta::new(*base_account.key, false),
            AccountMeta::new(*quote_account.key, false),
            AccountMeta::new(*base_vault.key, false),
            AccountMeta::new(*quote_vault.key, false),
            AccountMeta::new_readonly(*token_program.key, false),
        ],
        data: [
            PhoenixInstruction::Swap.to_vec(),
            order_packet.try_to_vec().unwrap(),
        ]
        .concat(),
    };

    invoke(
        &instruction,
        &[
            market.clone(),
            trader.clone(),
            base_account.clone(),
            quote_account.clone(),
            base_vault.clone(),
            quote_vault.clone(),
            token_program.clone(),
        ],
    )
}

/// Invoke a swap instruction via CPI with program derived address signing.
///
/// # Arguments
/// * `program_id` - The Phoenix program ID
/// * `market` - Market account
/// * `trader` - Trader account (PDA that will sign)
/// * `base_account` - Trader's base token account
/// * `quote_account` - Trader's quote token account
/// * `base_vault` - Market's base vault account
/// * `quote_vault` - Market's quote vault account
/// * `token_program` - SPL Token program
/// * `order_packet` - The order details
/// * `signer_seeds` - Seeds for PDA signing
pub fn swap_signed<'a>(
    program_id: &Pubkey,
    market: &AccountInfo<'a>,
    trader: &AccountInfo<'a>,
    base_account: &AccountInfo<'a>,
    quote_account: &AccountInfo<'a>,
    base_vault: &AccountInfo<'a>,
    quote_vault: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    order_packet: &OrderPacket,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let instruction = Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*program_id, false),
            AccountMeta::new_readonly(phoenix_log_authority::id(), false),
            AccountMeta::new(*market.key, false),
            AccountMeta::new_readonly(*trader.key, true),
            AccountMeta::new(*base_account.key, false),
            AccountMeta::new(*quote_account.key, false),
            AccountMeta::new(*base_vault.key, false),
            AccountMeta::new(*quote_vault.key, false),
            AccountMeta::new_readonly(*token_program.key, false),
        ],
        data: [
            PhoenixInstruction::Swap.to_vec(),
            order_packet.try_to_vec().unwrap(),
        ]
        .concat(),
    };

    invoke_signed(
        &instruction,
        &[
            market.clone(),
            trader.clone(),
            base_account.clone(),
            quote_account.clone(),
            base_vault.clone(),
            quote_vault.clone(),
            token_program.clone(),
        ],
        signer_seeds,
    )
}

/// Invoke a place limit order instruction via CPI.
///
/// # Arguments
/// * `program_id` - The Phoenix program ID
/// * `market` - Market account
/// * `trader` - Trader account (must be signer)
/// * `seat` - Trader's seat account
/// * `base_account` - Trader's base token account
/// * `quote_account` - Trader's quote token account
/// * `base_vault` - Market's base vault account
/// * `quote_vault` - Market's quote vault account
/// * `token_program` - SPL Token program
/// * `order_packet` - The order details
pub fn place_limit_order<'a>(
    program_id: &Pubkey,
    market: &AccountInfo<'a>,
    trader: &AccountInfo<'a>,
    seat: &AccountInfo<'a>,
    base_account: &AccountInfo<'a>,
    quote_account: &AccountInfo<'a>,
    base_vault: &AccountInfo<'a>,
    quote_vault: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    order_packet: &OrderPacket,
) -> ProgramResult {
    let instruction = Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*program_id, false),
            AccountMeta::new_readonly(phoenix_log_authority::id(), false),
            AccountMeta::new(*market.key, false),
            AccountMeta::new_readonly(*trader.key, true),
            AccountMeta::new_readonly(*seat.key, false),
            AccountMeta::new(*base_account.key, false),
            AccountMeta::new(*quote_account.key, false),
            AccountMeta::new(*base_vault.key, false),
            AccountMeta::new(*quote_vault.key, false),
            AccountMeta::new_readonly(*token_program.key, false),
        ],
        data: [
            PhoenixInstruction::PlaceLimitOrder.to_vec(),
            order_packet.try_to_vec().unwrap(),
        ]
        .concat(),
    };

    invoke(
        &instruction,
        &[
            market.clone(),
            trader.clone(),
            seat.clone(),
            base_account.clone(),
            quote_account.clone(),
            base_vault.clone(),
            quote_vault.clone(),
            token_program.clone(),
        ],
    )
}

/// Invoke a place limit order instruction via CPI with PDA signing.
pub fn place_limit_order_signed<'a>(
    program_id: &Pubkey,
    market: &AccountInfo<'a>,
    trader: &AccountInfo<'a>,
    seat: &AccountInfo<'a>,
    base_account: &AccountInfo<'a>,
    quote_account: &AccountInfo<'a>,
    base_vault: &AccountInfo<'a>,
    quote_vault: &AccountInfo<'a>,
    token_program: &AccountInfo<'a>,
    order_packet: &OrderPacket,
    signer_seeds: &[&[&[u8]]],
) -> ProgramResult {
    let instruction = Instruction {
        program_id: *program_id,
        accounts: vec![
            AccountMeta::new_readonly(*program_id, false),
            AccountMeta::new_readonly(phoenix_log_authority::id(), false),
            AccountMeta::new(*market.key, false),
            AccountMeta::new_readonly(*trader.key, true),
            AccountMeta::new_readonly(*seat.key, false),
            AccountMeta::new(*base_account.key, false),
            AccountMeta::new(*quote_account.key, false),
            AccountMeta::new(*base_vault.key, false),
            AccountMeta::new(*quote_vault.key, false),
            AccountMeta::new_readonly(*token_program.key, false),
        ],
        data: [
            PhoenixInstruction::PlaceLimitOrder.to_vec(),
            order_packet.try_to_vec().unwrap(),
        ]
        .concat(),
    };

    invoke_signed(
        &instruction,
        &[
            market.clone(),
            trader.clone(),
            seat.clone(),
            base_account.clone(),
            quote_account.clone(),
            base_vault.clone(),
            quote_vault.clone(),
            token_program.clone(),
        ],
        signer_seeds,
    )
}

/// Helper function to get the vault address for a market and mint.
///
/// Returns the vault PDA and bump seed.
pub fn get_vault_address_helper(market: &Pubkey, mint: &Pubkey) -> (Pubkey, u8) {
    get_vault_address(market, mint)
}

/// Helper function to get the seat address for a market and trader.
///
/// Returns the seat PDA and bump seed.
pub fn get_seat_address_helper(market: &Pubkey, trader: &Pubkey) -> (Pubkey, u8) {
    get_seat_address(market, trader)
}
