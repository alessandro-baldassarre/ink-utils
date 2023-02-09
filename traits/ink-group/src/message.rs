use ink::prelude::vec::Vec;
use ink::primitives::AccountId;

use crate::{error::InkGroupError, storage::Member};

#[ink::trait_definition]
pub trait InkGroup {
    // Getters
    #[ink(message)]
    /// Return the actual admin
    fn get_admin(&self) -> Result<AccountId, InkGroupError>;

    #[ink(message)]
    /// Return all members info
    fn get_members(&self) -> Result<Vec<Member>, InkGroupError>;

    #[ink(message)]
    /// Return a specific member info request by contract address
    fn get_member(&self, member: AccountId) -> Result<Member, InkGroupError>;

    #[ink(message)]
    /// Return the total voting power weight of the grop
    fn get_total_weight(&self) -> u64;

    // Setters
    #[ink(message)]
    /// Update the admin
    fn update_admin(&mut self, admin: AccountId) -> Result<(), InkGroupError>;

    #[ink(message)]
    /// Update the members in the group
    fn update_members(
        &mut self,
        new_members: Vec<Member>,
        remove_members: Vec<AccountId>,
    ) -> Result<(), InkGroupError>;
}
