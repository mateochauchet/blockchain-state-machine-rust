use std::collections::BTreeMap;

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new(),
        }
    }

    /// Set the balance of an account `who` to some `amount`.
    pub fn set_balance(&mut self, who: &String, amount: u128) {
        self.balances.insert(who.clone(), amount);
    }

    /// Get the balance of an account `who`.
    /// If the account has no stored balance, we return zero.
    pub fn balance(&self, who: &String) -> u128 {
        *self.balances.get(&who.clone()).unwrap_or(&0)
    }

    /// Transfer `amount` from one account to another.
    /// This function verifies that `from` has at least `amount` balance to transfer,
    /// and that no mathematical overflows occur.
    pub fn transfer(
        &mut self,
        caller: String,
        to: String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        if caller_balance < amount {
            return Err("Not enough balance");
        }

        let new_caller_balance = caller_balance.checked_sub(amount).ok_or("error")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("error")?;

        self.set_balance(&caller, new_caller_balance);
        self.set_balance(&to, new_to_balance);

        Ok(())
    }
}

mod tests {
    use std::f32::consts::E;

    use super::Pallet;

    #[test]
    fn init_balances() {
        let mut balances = Pallet::new();

        let alice = "alice".to_string();
        let bob = "bob".to_string();

        assert_eq!(balances.balance(&alice), 0);
        balances.set_balance(&alice, 100);

        assert_eq!(balances.balance(&alice), 100);
        assert_eq!(balances.balance(&bob), 0);
    }

    #[test]
    fn transfer_balance() {
        let mut balances = Pallet::new();

        let alice = "alice".to_string();
        let bob = "bob".to_string();

        balances.set_balance(&alice, 100);

        assert_eq!(balances.balance(&alice), 100);
        assert_eq!(balances.balance(&bob), 0);

        assert_eq!(
            balances.transfer(alice.clone(), bob.clone(), 150),
            Err("Not enough balance")
        );

        let _ = balances.transfer(alice.clone(), bob.clone(), 50);

        assert_eq!(balances.balance(&alice), 50);
        assert_eq!(balances.balance(&bob), 50);
    }
}
