use smart_contract_macros::smart_contract;
use smart_contract::log;
use smart_contract::payload::Parameters;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/*
    Inspired by Truffle's Pet Shop - https://www.trufflesuite.com/tutorials/pet-shop
*/



#[derive(Serialize, Deserialize)]
struct Adoption {
    adopters: HashMap<u8, [u8; 32]>
}

#[smart_contract]
impl Adoption {

    pub fn init(params: &mut Parameters) -> Self {

        Self {
            adopters: HashMap::new()
        }
    }

    // Adopting a pet
    pub fn adopt(&mut self, params: &mut Parameters) -> Result<(), String> {
        let petId : u8 = params.read();

        if petId < 0 || petId > 15 {
            return Err("PetId must be in between 0-15".into());
        }

        self.adopters.insert(petId, params.sender);

        Ok(())
    }

    // Retrieving the adopters
    pub fn getAdopters(&mut self, params: &mut Parameters) -> Result<(), String> {
        log(&serde_json::to_string(&self.adopters).unwrap());
        Ok(())
    }

}