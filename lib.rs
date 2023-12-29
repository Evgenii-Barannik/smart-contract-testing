#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[openbrush::implementation(PSP34, PSP34Burnable, PSP34Mintable, AccessControl)]
#[openbrush::contract]
pub mod my_access_control {
    use openbrush::contracts::psp34::PSP34Error;
    use openbrush::modifiers;
    use openbrush::traits::Storage;
    use openbrush::contracts::access_control::RoleType;

    #[ink(storage)]
    #[derive(Default, Storage)]
    pub struct Contract {
        #[storage_field]
        psp34: psp34::Data,
        #[storage_field]
        access: access_control::Data,
    }

    const MINTER: RoleType = ink::selector_id!("MINTER");

    impl Contract {
        #[ink(constructor)]
        pub fn new(_arg: bool) -> Self {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            access_control::Internal::_init_with_admin(&mut instance, Some(caller));

            instance
        }

        #[ink(message)]
        pub fn grant_minter_role(&mut self) -> () {
            let mut instance = Self::default();
            let caller = instance.env().caller();
            access_control::Internal::_init_with_admin(&mut instance, Some(caller));
            AccessControl::grant_role(&mut instance, MINTER, Some(caller)).expect("Should grant MINTER role");
        }
        
        #[ink(message)]
        #[modifiers(only_role(MINTER))]
        pub fn mint_token(&mut self, id: u8) -> Result<(), PSP34Error> {
            psp34::Internal::_mint_to(self, Self::env().caller(), Id::U8(id))
        }

        // #[ink(message)]
        // pub fn total_supply(&self) -> u128 {
        //     self.psp34.total_supply.get().unwrap_or_default()
        // }
    }
    
}
