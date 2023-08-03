#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp37 {
    use openbrush::{
        contracts::psp37::extensions::{
            metadata::*,
            mintable::*,
            burnable::*,
            enumerable::*,
            batch::*,
        },
        traits::Storage,

    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp37: psp37::Data<Balances>,
        #[storage_field]
        metadata: Data,
        next_id: u8,
    }

    impl PSP37 for Contract {}

    impl PSP37Mintable for Contract {}

    impl PSP37Burnable for Contract {}

    impl PSP37Enumerable for Contract {}

    impl PSP37Batch for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
			_instance
        }
    }
}