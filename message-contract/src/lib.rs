use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::env::log;
use near_sdk::{env, near_bindgen, PromiseOrValue};

#[near_bindgen]
#[derive(Default, BorshDeserialize, BorshSerialize)]
pub struct MessageContract {}

#[near_bindgen]
impl MessageContract {
    #[init]
    pub fn new() -> Self {
        MessageContract {}
    }

    pub fn get_wellcome_message(&mut self) -> PromiseOrValue<String> {
        let processor = env::predecessor_account_id();
        let account_id = env::signer_account_id();
        assert_ne!(processor, account_id.clone());
        log(format!(
            "processor={}, account_id={}",
            processor.to_string(),
            account_id.to_string()
        )
        .as_bytes());
        let welcome_message = format!("Hello {}!", account_id);
        PromiseOrValue::Value(welcome_message)
    }
}
