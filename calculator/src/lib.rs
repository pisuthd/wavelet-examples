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

    // Get Result - call {smart_contract_id} 0 99999 getResult
    pub fn getResult(&mut self, params: &mut Parameters) -> Result<(), String> {
        let s: String = self.result.to_string();
        log(&s);
        Ok(())
    }


    // Add 1,000 to the result - call {smart_contract_id} 0 99999 addToNumber 81000
    pub fn addToNumber(&mut self, params: &mut Parameters) -> Result<(), String> {
        let amount: u64 = params.read();
        self.result += amount;
        Ok(())
    }

    // Subtract with 500 - call {smart_contract_id} 0 99999 subtractNumber 8500
    pub fn subtractNumber(&mut self, params: &mut Parameters) -> Result<(), String> {
        let amount: u64 = params.read();
        self.result -= amount;
        Ok(())
    }

    // Multiply by 4 - call {smart_contract_id} 0 99999 multiplyWithNumber 84
    pub fn multiplyWithNumber(&mut self, params: &mut Parameters) -> Result<(), String> {
        let amount: u64 = params.read();
        self.result *= amount;
        Ok(())
    }

    // Divide by 5 - call {smart_contract_id} 0 99999 divideByNumber 85
    pub fn divideByNumber(&mut self, params: &mut Parameters) -> Result<(), String> {
        let amount: u64 = params.read();
        self.result /= amount;
        Ok(())
    }


}