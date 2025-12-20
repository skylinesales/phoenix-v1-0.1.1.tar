#[cfg(test)]
mod cpi_tests {
    use super::*;

    #[test]
    fn test_cpi_module_exists() {
        // This test verifies that the CPI module is available when the feature is enabled
        #[cfg(feature = "cpi")]
        {
            // Test that we can reference the CPI module functions
            // This is a compile-time test - if it compiles, the module structure is correct
            use crate::program::cpi::{
                get_vault_address_helper,
                get_seat_address_helper,
            };
            
            // Dummy test to verify module structure
            assert!(true);
        }
        
        #[cfg(not(feature = "cpi"))]
        {
            // When CPI feature is not enabled, this is expected
            assert!(true);
        }
    }

    #[test]
    fn test_helper_functions_available() {
        #[cfg(feature = "cpi")]
        {
            use crate::program::cpi::{
                get_vault_address_helper,
                get_seat_address_helper,
            };
            use solana_program::pubkey::Pubkey;
            
            // Test that helper functions work
            let market = Pubkey::new_unique();
            let mint = Pubkey::new_unique();
            let trader = Pubkey::new_unique();
            
            let (vault, _bump) = get_vault_address_helper(&market, &mint);
            let (seat, _bump) = get_seat_address_helper(&market, &trader);
            
            // Verify PDAs are valid pubkeys
            assert_ne!(vault, Pubkey::default());
            assert_ne!(seat, Pubkey::default());
        }
    }
}
