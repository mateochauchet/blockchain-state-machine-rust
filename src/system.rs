/* TODO: You might need to update your imports. */

use std::collections::BTreeMap;

/// This is the System Pallet.
/// It handles low level state needed for your blockchain.
pub struct Pallet {
    /// The current block number.
    block_number: u32,

    /// A map from an account to their nonce.
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    pub fn new() -> Self {
        Pallet {
            block_number: 0,
            nonce: BTreeMap::new(),
        }
    }

    pub fn block_number(&self) -> u32 {
        self.block_number
    }

    pub fn inc_block_number(&mut self) {
        self.block_number += 1;
    }

    // Increment the nonce of an account. This helps us keep track of how many transactions each
    // account has made.
    pub fn inc_nonce(&mut self, who: &String) {
        let nonce: u32 = *self.nonce.get(who).unwrap_or(&0);
        let new_nonce = nonce + 1;
        self.nonce.insert(who.clone(), new_nonce);
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn init_system() {
        let mut system = super::Pallet::new();

        let alice = "alice".to_string();

        assert_eq!(system.block_number(), 0);
        assert_eq!(system.nonce.len(), 0);

        system.inc_block_number();

        assert_eq!(system.block_number(), 1);

        system.inc_nonce(&alice);

        assert_eq!(system.nonce.get("alice"), Some(&1));
        assert_eq!(system.nonce.get("bob"), None);
    }
}
