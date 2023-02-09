#[ink::contract]
mod contract {
    use ink::prelude::vec::Vec;
    use ink::storage::Lazy;
    use ink_group::{InkGroup, InkGroupError, Member};

    use crate::{ensure, error::ContractError, helpers::validate_unique_members};

    #[ink(storage)]
    #[derive(Default)]
    pub struct InkVotingGroup {
        /// admin of the group (can perform any action)
        admin: Lazy<AccountId>,
        total_voting_power: u64,
        members: Vec<Member>,
    }

    impl InkVotingGroup {
        #[ink(constructor)]
        /// Construct the contract with optional address (if not set caller address is set) for the
        /// admin and the initial members
        pub fn try_new(
            admin: Option<AccountId>,
            initial_members: Vec<Member>,
        ) -> Result<Self, ContractError> {
            // Check if the admin address is set and the number of new members is not zero
            let admin = admin.unwrap_or(Self::env().caller());
            if initial_members.is_empty() {
                return Err(InkGroupError::ZeroMembers {}.into());
            }
            // Check if there are not equal members addresses entered
            validate_unique_members(&initial_members)?;
            let mut instance = Self::default();
            // Set the admin
            instance.admin.set(&admin);
            // Calculate the total voting power and Save to storage each member
            let total_power: u64 = initial_members
                .into_iter()
                .map(|member| {
                    instance.members.push(member);
                    member.weight
                })
                .sum();
            // Save to storage the total voting power
            instance.total_voting_power = total_power;
            Ok(instance)
        }
    }

    impl InkGroup for InkVotingGroup {
        #[ink(message)]
        fn get_admin(&self) -> Result<AccountId, InkGroupError> {
            // Should always be some admin in case of error the logic of the contract is wrong
            let admin = self.admin.get().ok_or(InkGroupError::LogicErr {})?;
            Ok(admin)
        }

        #[ink(message)]
        fn get_members(&self) -> Result<Vec<Member>, InkGroupError> {
            // Should always be some member in case of error the logic of the contract is
            // wrong
            if self.members.is_empty() {
                return Err(InkGroupError::LogicErr {});
            }
            Ok(self.members.clone())
        }

        #[ink(message)]
        fn get_member(&self, member: AccountId) -> Result<Member, InkGroupError> {
            // Return error in case of the member is not found in the group
            let founded_member = self
                .members
                .iter()
                .cloned()
                .find(|&memb| memb.addr == member)
                .ok_or(InkGroupError::NoMember {})?;
            Ok(founded_member)
        }

        #[ink(message)]
        fn get_total_weight(&self) -> u64 {
            self.total_voting_power
        }

        #[ink(message)]
        fn update_admin(&mut self, new_admin: AccountId) -> Result<(), InkGroupError> {
            let caller = self.env().caller();
            let admin = self.get_admin()?;
            ensure!(caller == admin, InkGroupError::Unauthorized {});
            self.admin.set(&new_admin);
            Ok(())
        }

        #[ink(message)]
        /// If an already existing address is entered, the voting power is updated. Remove is applied after add, so if an address is in both, it is removed
        fn update_members(
            &mut self,
            new_members: Vec<Member>,
            remove_members: Vec<AccountId>,
        ) -> Result<(), InkGroupError> {
            let caller = self.env().caller();
            let admin = self.get_admin()?;
            ensure!(caller == admin, InkGroupError::Unauthorized {});
            validate_unique_members(&new_members)?;
            for member in new_members {
                if let Some(index) = self
                    .members
                    .iter()
                    .position(|&old_member| old_member.addr == member.addr)
                {
                    self.total_voting_power -= self.members[index].weight;
                    self.total_voting_power += member.weight;
                    self.members[index].weight = member.weight;
                } else {
                    self.members.push(member);
                    self.total_voting_power += member.weight;
                }
            }

            validate_unique_members(&self.members)?;
            for member in remove_members {
                if let Some(index) = self
                    .members
                    .iter()
                    .position(|&old_member| old_member.addr == member)
                {
                    self.total_voting_power -= self.members[index].weight;
                    self.members.remove(index);
                }
            }

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;

        const WALLET: [u8; 32] = [7; 32];

        fn default_accounts() -> test::DefaultAccounts<Environment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn set_caller(sender: AccountId) {
            ink::env::test::set_caller::<Environment>(sender);
        }

        fn build_contract() -> InkVotingGroup {
            // Set the contract's address as `WALLET`.
            let sender: AccountId = AccountId::from(WALLET);
            set_caller(sender);

            let accounts = default_accounts();

            let members = vec![
                Member {
                    addr: accounts.alice,
                    weight: 1,
                },
                Member {
                    addr: accounts.bob,
                    weight: 1,
                },
            ];

            InkVotingGroup::try_new(None, members).unwrap()
        }

        #[ink::test]
        fn construction_works() {
            let caller: AccountId = AccountId::from(WALLET);
            let accounts = default_accounts();
            let alice_member = Member {
                addr: accounts.alice,
                weight: 1,
            };
            let bob_member = Member {
                addr: accounts.bob,
                weight: 1,
            };
            let charlie_member = Member {
                addr: accounts.charlie,
                weight: 1,
            };
            let members = vec![alice_member, bob_member];
            let contract = build_contract();

            assert_eq!(contract.members.len(), 2);
            assert_eq!(contract.admin.get().unwrap(), caller);
            assert!(contract.members.iter().eq(members.iter()));
            assert!(contract.members.contains(&alice_member));
            assert!(!contract.members.contains(&charlie_member));
        }
    }
}
