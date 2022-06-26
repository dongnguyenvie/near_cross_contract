use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, ext_contract, near_bindgen, AccountId, Gas, Promise};

// define the methods we'll use on the other contract
#[ext_contract(ext_message)]
pub trait Receiver {
    fn get_wellcome_message(&mut self) -> PromiseOrValue<String>;
}

#[ext_contract(ext_self)]
pub trait MyContract {
    fn my_callback(&self) -> String;
}

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct ServiceContract {}

#[near_bindgen]
impl ServiceContract {
    #[private]
    pub fn my_callback(&mut self) -> String {
        match env::promise_result(0) {
            near_sdk::PromiseResult::NotReady => "oops!".to_string(),
            near_sdk::PromiseResult::Successful(result) => {
                let message = near_sdk::serde_json::from_slice::<String>(&result).unwrap();
                message
            }
            near_sdk::PromiseResult::Failed => "oops!".to_string(),
        }
    }
}

#[near_bindgen]
impl ServiceContract {
    #[init]
    pub fn new() -> Self {
        let this = Self {};
        this
    }

    pub fn say_hello(&mut self) -> Promise {
        let message_contact: AccountId = String::from("dev-1656233421007-65579532520509");
        ext_message::get_wellcome_message(&message_contact, 0, (5_000_000_000_000)).then(
            ext_self::my_callback(&env::current_account_id(), 0, 5_000_000_000_000),
        )
    }
}
