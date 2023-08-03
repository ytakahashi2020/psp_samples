#![cfg_attr(not(feature = "std"), no_std, no_main)]
#![feature(min_specialization)]

#[openbrush::contract]
pub mod my_psp34 {
    use openbrush::{
        contracts::psp34::extensions::{
            metadata::*,
            mintable::*,
            burnable::*,
        },
        traits::{
            Storage,
            String,
        },
    };

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        metadata: Data,
        next_id: u8,
    }

    impl PSP34 for Contract {}

    impl PSP34Mintable for Contract {}

    impl PSP34Burnable for Contract {}

    impl Contract {
        #[ink(constructor)]
        pub fn new() -> Self {
            let mut _instance = Self::default();
			psp34::Internal::_mint_to(&mut _instance, Self::env().caller(), Id::U8(1)).expect("Can mint");
			let collection_id = _instance.collection_id();
			metadata::Internal::_set_attribute(&mut _instance, collection_id.clone(), String::from("name"), String::from("MyPSP34"));
			metadata::Internal::_set_attribute(&mut _instance, collection_id, String::from("symbol"), String::from("MPSP"));
			_instance
        }

        // #[ink(message)]
        // pub fn mint_token(&mut self) -> Result<(), PSP34Error> {
        //     self._mint_to(Self::env().caller(), Id::U8(self.next_id))?;
        //     self.next_id += 1;
        //     Ok(())
        // }

        // #[ink(message)]
        // pub fn mint(&mut self, id: Id) -> Result<(), PSP34Error> {
        //     self._mint_to(Self::env().caller(), id)
        // }
    }
}