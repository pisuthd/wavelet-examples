use smart_contract_macros::smart_contract;
use smart_contract::log;
use smart_contract::payload::Parameters;

pub struct HelloWorld;

#[smart_contract]
impl HelloWorld {

    pub fn init(params: &mut Parameters) -> Self {
        Self {}
    }

    // call {smart_contract_id} 0 999999 print Shello_world
    pub fn print(&mut self, params: &mut Parameters) -> Result<(), String> {
        let message : String = params.read();

        log(&message);

        Ok(())
    }

}