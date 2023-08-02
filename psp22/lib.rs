#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp22 {
    use openbrush::{
        contracts::psp22::extensions::{
            mintable::*,
            burnable::*,
            metadata::*,
        },
        traits::{
            Storage,
            String,
        },

    };


    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Psp22Contract {
        #[storage_field]
        psp22: psp22::Data,
        #[storage_field]
        metadata: metadata::Data,
    }

    impl PSP22 for Psp22Contract {}

    impl PSP22Mintable for Psp22Contract {}

    impl PSP22Burnable for Psp22Contract {}

    impl PSP22Metadata for Psp22Contract {}

    impl Psp22Contract {
        #[ink(constructor)]
        pub fn new(total_supply: Balance, name: Option<String>, symbol: Option<String>, decimal: u8) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();

            instance.metadata.name = name;
            instance.metadata.symbol = symbol;
            instance.metadata.decimals = decimal;

            psp22::Internal::_mint_to(&mut instance, caller, total_supply).expect("Should mint total_supply");

            instance
        }

    }
}