
use smart_contract::log;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct Counter {
    value: u64
}

impl Counter {

    pub fn new() -> Self {
        Self { value: 0 }
    }

    pub fn current(&self) {
        let s: String = self.value.to_string();
        log(&s);
    }

    pub fn increment(&mut self) {
        self.value += 1;
    }

    pub fn decrement(&mut self) {
        if self.value > 0 {
            self.value -= 1;
        }
    }

}

