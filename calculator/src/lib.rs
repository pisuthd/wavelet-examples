use smart_contract_macros::smart_contract;

use smart_contract::log;
use smart_contract::payload::Parameters;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Contract {
    result: u64
}

#[smart_contract]
impl Contract {
    pub fn init(params: &mut Parameters) -> Self {

        let initial : u64 = 0;

        Self {
            result: initial
        }
    }

    pub fn getResult(&mut self, params: &mut Parameters) -> Result<(), String> {
        let s: String = self.result.to_string();
        log(&s);
        Ok(())
    }

    pub fn addToNumber(&mut self, params: &mut Parameters) -> Result<(), String> {
        let amount: u64 = params.read();
        self.result += amount;
        Ok(())
    }

    pub fn subtractNumber(&mut self, params: &mut Parameters) -> Result<(), String> {
        let amount: u64 = params.read();
        self.result -= amount;
        Ok(())
    }

    pub fn multiplyWithNumber(&mut self, params: &mut Parameters) -> Result<(), String> {
        let amount: u64 = params.read();
        self.result *= amount;
        Ok(())
    }

    pub fn divideByNumber(&mut self, params: &mut Parameters) -> Result<(), String> {
        let amount: u64 = params.read();
        self.result /= amount;
        Ok(())
    }


}