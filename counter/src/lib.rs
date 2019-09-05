use smart_contract_macros::smart_contract;
use serde::{Deserialize, Serialize};

use counter::Counter;

use smart_contract::log;
use smart_contract::payload::Parameters;

mod counter;


#[derive(Serialize, Deserialize)]
struct Contract {
    counter: Counter
}


#[smart_contract]
impl Contract {
    pub fn init(params: &mut Parameters) -> Self {
        Self {
            counter: Counter::new(),
        }
    }

    pub fn getCount(&mut self, params: &mut Parameters) -> Result<(), String> {
        self.counter.current();
        Ok(())
    }

    pub fn incrementCounter(&mut self, params: &mut Parameters) -> Result<(), String> {
        self.counter.increment();
        Ok(())
    }

    pub fn decrementCounter(&mut self, params: &mut Parameters) -> Result<(), String> {
        self.counter.decrement();
        Ok(())
    }

}

