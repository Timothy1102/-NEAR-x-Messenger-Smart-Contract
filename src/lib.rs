use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::private::de::TagOrContentField;
use near_sdk::{env, log, near_bindgen, AccountId, Balance};

near_sdk::setup_alloc!();

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    account_list: UnorderedMap<AccountId, Balance>,
    pool: u32,
    top_accounts: UnorderedMap<AccountId, Balance>,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            account_list: UnorderedMap::new(b"a".to_vec()),
            pool: 100,
            top_accounts: UnorderedMap::new(b"t".to_vec()),
        }
    }
}

#[near_bindgen]
impl Contract {

    // return top 3 account with highest tx volume. 
    pub fn get_top(&self) -> (String, String, String) {
        let mut iter = self.top_accounts.iter();
        let len = self.top_accounts.len();
        // "default" means that position doesn't contain an account (total user < 3).
        let acc1 = String::from("default");
        let acc2 = String::from("default");
        let acc3 = String::from("default");
        let mut tup: (String, String, String) = (acc1, acc2, acc3);
        for i in 0..len {
            let next = iter.next().unwrap();
            if i == 0 {
                tup.0 = next.0;
            } else if i == 1 {
                tup.1 = next.0;
            } else if i == 2 {
                tup.2 = next.0;
            }
        }
        tup
    }

    // (helper function) return the account with lowest transaction volume in the top 3 accounts with highest transaction volume.
    pub fn minacc(&self) -> (AccountId, Balance) {
        let len = self.top_accounts.len();
        let mut iter = self.top_accounts.iter();
        let mut min: u128 = u128::MAX;
        let mut min_acc = String::from("default");
        for i in 0..len {
            let acc = iter.next().unwrap();
            if acc.1 < min {
                min_acc = acc.0;
                min = acc.1;
            }
        }
        log!("min acc: {}", min_acc);
        log!("min: {}", min);
        (min_acc, min)
    }

    // update account_list and top_accounts (top 3 accounts with highest transaction volume)
    pub fn update_list(&mut self, acc: String, amount: u128) {
        if self.account_list.get(&acc).is_some() {
            let old_vol = self.account_list.remove(&acc).unwrap();
            let current_vol = amount.checked_add(old_vol).unwrap();
            self.account_list.insert(&acc, &current_vol);
            // update top_accounts
            if self.top_accounts.len() < 3 {
                let old_vol = self.top_accounts.remove(&acc).unwrap();
                let current_vol = amount.checked_add(old_vol).unwrap();
                self.top_accounts.insert(&acc, &current_vol);
            } else {
                if self.top_accounts.get(&acc).is_some() {
                    let old_vol = self.top_accounts.remove(&acc).unwrap();
                    let current_vol = amount.checked_add(old_vol).unwrap();
                    self.top_accounts.insert(&acc, &current_vol);
                } else {
                    let minacc = Contract::minacc(&self);
                    let minaccount = minacc.0;
                    let min_bal = minacc.1;
                    if current_vol > min_bal {
                        self.top_accounts.remove(&minaccount);
                        self.top_accounts.insert(&acc, &current_vol);
                    }
                }
            }
        } else {
            self.account_list.insert(&acc, &amount);
            if self.top_accounts.len() < 3 {
                self.top_accounts.insert(&acc, &amount);
            } else {
                let minacc = Contract::minacc(&self);
                let minaccount = minacc.0;
                let bal = minacc.1;
                if amount > bal {
                    self.top_accounts.remove(&minaccount);
                    self.top_accounts.insert(&acc, &amount);
                }
            }
        }
        let minacc = Contract::minacc(&self);
        log!("minacc : {:?}", minacc);
    }

    // get the total tx volume of an account
    pub fn get_vol(&self, acc: String) -> Option<u128> {
        env::log(b"read");
        return self.account_list.get(&acc);
    }

    // delete an account from the account_list
    pub fn delete(&mut self, acc: String) {
        env::log(b"delete");
        self.account_list.remove(&acc);
    }

    // set the prize pool to any value (in NEAR) - default is 100 NEAR.
    pub fn set_pool(&mut self, pool: u32) {
        self.pool = pool;
    }

    // set the prize pool to 100 NEAR - default.
    pub fn set_pool_to_default(&mut self) {
        self.pool = 100;
    }

    pub fn get_pool(&self) -> u32 {
        return self.pool;
    }

    pub fn calculate_reward(&self) -> ((AccountId, Balance), (AccountId, Balance), (AccountId, Balance)) {
        let mut total: u128= 0;
        let pool = u128::from(self.pool);
        let top_acc = Contract::get_top(&self);
        let acc1 = top_acc.0;
        let acc2 = top_acc.1;
        let acc3 = top_acc.2;

        let mut reward1: u128 = 0;
        let mut reward2: u128 = 0;
        let mut reward3: u128 = 0;

        if self.top_accounts.get(&acc1).is_some() {
            let bal = self.top_accounts.get(&acc1).unwrap();
            total = total.checked_add(bal).unwrap();
        }
        if self.top_accounts.get(&acc2).is_some() {
            let bal = self.top_accounts.get(&acc2).unwrap();
            total = total.checked_add(bal).unwrap();
        }
        if self.top_accounts.get(&acc3).is_some() {
            let bal = self.top_accounts.get(&acc3).unwrap();
            total = total.checked_add(bal).unwrap();
        }

        if self.top_accounts.get(&acc1).is_some() {
            let bal = self.top_accounts.get(&acc1).unwrap();
            reward1 = (bal.checked_mul(pool)).unwrap().checked_div(total).unwrap();
        }
        if self.top_accounts.get(&acc2).is_some() {
            let bal = self.top_accounts.get(&acc2).unwrap();
            reward2 = (bal.checked_mul(pool)).unwrap().checked_div(total).unwrap();
        }
        if self.top_accounts.get(&acc3).is_some() {
            let bal = self.top_accounts.get(&acc3).unwrap();
            reward3 = (bal.checked_mul(pool)).unwrap().checked_div(total).unwrap();
        }

        // log!("{:?}", ((&acc1, reward1), (&acc2, reward2), (&acc3, reward3)));

        log!("{}", &acc1);
        log!("{}", reward1);
        log!("{}", &acc2);
        log!("{}", reward2);
        log!("{}", &acc3);
        log!("{}", reward3);

        return ((acc1, reward1), (acc2, reward2), (acc3, reward3));

    }

    // clear all account_list and top_accounts
    pub fn clear(&mut self) {
        self.account_list.clear();
        self.top_accounts.clear();
    }

}

