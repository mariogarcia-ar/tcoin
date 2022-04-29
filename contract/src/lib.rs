// https://learn.figment.io/tutorials/write-and-deploy-a-smart-contract-on-near
// near call $ID transfer '{"new_owner_id":"amiyarust.testnet", "amount":50}' --accountId amiyatulu.testnet --amount 50


// To conserve gas, efficient serialization is achieved through Borsh (http://borsh.io/)
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, setup_alloc};
use near_sdk::collections::LookupMap;
use near_sdk::{ Promise};
// use near_sdk::{json_types::U128, AccountId, Promise};

setup_alloc!();

// Contract
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct TCOin {
    records: LookupMap<String, String>,
}

impl Default for TCOin {
  fn default() -> Self {
    Self {
      records: LookupMap::new(b"a".to_vec()),
    }
  }
}

// Methods
// https://www.near-sdk.io/contract-structure/near-bindgen
#[near_bindgen]
impl TCOin {
    pub fn set_message(&mut self, message: String) {
        let account_id = env::signer_account_id();

        // Use env::log to record logs permanently to the blockchain!
        env::log(format!("Saving message '{}' for account '{}'", message, account_id,).as_bytes());

        self.records.insert(&account_id, &message);
    }

    pub fn get_message(&self, account_id: String) -> String {
        match self.records.get(&account_id) {
            Some(message) => message,
            None => "Hello".to_string(), // Return "Hello" by default
        }
    }

    #[payable]
    pub fn clain_reward(&mut self) -> Promise {
        let amount: u128 = 5_500_000_000_000_000_000_000_000; // 1 $NEAR as yoctoNEAR
        Promise::new(env::predecessor_account_id()).transfer(amount)
    }
}

// Test
// cargo test -- --nocapture
#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::MockedBlockchain;
    use near_sdk::{testing_env, VMContext}; 
    // use near_sdk::test_utils::VMContextBuilder;
    // use near_sdk::{ AccountId};    

    // mock the context for testing, notice "signer_account_id" that was accessed above from env::
    fn get_context(input: Vec<u8>, is_view: bool) -> VMContext {
        VMContext {
            current_account_id: "alice_near".to_string(),
            signer_account_id: "bob_near".to_string(),
            signer_account_pk: vec![0, 1, 2],
            predecessor_account_id: "carol_near".to_string(),
            input,
            block_index: 0,
            block_timestamp: 0,
            account_balance: 0,
            account_locked_balance: 0,
            storage_usage: 0,
            attached_deposit: 0,
            prepaid_gas: 10u64.pow(18),
            random_seed: vec![0, 1, 2],
            is_view,
            output_data_receivers: vec![],
            epoch_height: 19,
        }
    }

    // fn get_context_predecessor(predecessor: AccountId) -> VMContextBuilder {
    //     let mut builder = VMContextBuilder::new();
    //     builder.predecessor_account_id(predecessor);
    //     builder
    // }

    #[test]
    fn set_then_get_message() {
        let context = get_context(vec![], false);
        testing_env!(context);
        let mut contract = TCOin::default();
        contract.set_message("howdy".to_string());
        assert_eq!(
            "howdy".to_string(),
            contract.get_message("bob_near".to_string())
        );
    }

    #[test]
    fn get_default_message() {
        let context = get_context(vec![], true);
        testing_env!(context);
        let contract = TCOin::default();
        // this test did not call set_message so should return the default "Hello" message
        assert_eq!(
            "Hello".to_string(),
            contract.get_message("francis.near".to_string())
        );
    }

    // https://crates.io/crates/near-sdk-sim
    // #[test]
    // fn set_clain_reward() {
    //     let context = get_context(vec![], true);
    //     testing_env!(context);
    //     let contract = TCOin::default();
    //     // this test did not call set_message so should return the default "Hello" message
    //     assert_eq!(
    //         "Hello".to_string(),
    //         contract.get_message("francis.near".to_string())
    //     );
    // }
}
