#[ink::contract]
mod contract {
    use ink::prelude::vec::Vec;
    use ink::storage::Lazy;
    use ink_group::{InkGroup, InkGroupError, Member};

    use crate::{ensure, error::ContractError, helpers::validate_unique_members};

    /// Emitted when a member is added to the group
    #[ink(event)]
    pub struct MemberAddition {
        /// The member that was added.
        #[ink(topic)]
        member: AccountId,
    }

    /// Emitted when a member is removed to the group
    #[ink(event)]
    pub struct MemberRemoval {
        /// The member that was removed.
        #[ink(topic)]
        member: AccountId,
    }

    /// Emitted when a member is updated
    #[ink(event)]
    pub struct MemberUpdate {
        /// The member that was updated.
        #[ink(topic)]
        member: AccountId,
    }

    /// Emitted when the admin is updated
    #[ink(event)]
    pub struct AdminUpdate {
        /// The old admin.
        #[ink(topic)]
        old_admin: AccountId,
        /// The new admin.
        #[ink(topic)]
        new_admin: AccountId,
    }

    #[ink(storage)]
    #[derive(Default)]
    pub struct InkGroupSimple {
        /// admin of the group (can perform any action)
        admin: Lazy<AccountId>,
        total_voting_power: u64,
        members: Vec<Member>,
    }

    impl InkGroupSimple {
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
                    // Emit the event that the member was added
                    Self::env().emit_event(MemberAddition {
                        member: member.addr,
                    });
                    member.weight
                })
                .sum();
            // Save to storage the total voting power
            instance.total_voting_power = total_power;
            Ok(instance)
        }
    }

    impl InkGroup for InkGroupSimple {
        #[ink(message)]
        /// Return current admin.
        fn get_admin(&self) -> Result<AccountId, InkGroupError> {
            // Should always be some admin in case of error the logic of the contract is wrong
            let admin = self.admin.get().ok_or(InkGroupError::LogicErr {})?;
            Ok(admin)
        }

        #[ink(message)]
        /// Return all members info.
        fn get_members(&self) -> Result<Vec<Member>, InkGroupError> {
            // Should always be some member in case of error the logic of the contract is
            // wrong
            if self.members.is_empty() {
                return Err(InkGroupError::LogicErr {});
            }
            Ok(self.members.clone())
        }

        #[ink(message)]
        /// Return member info searched by address.
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
        /// Return the total voting power.
        fn get_total_weight(&self) -> u64 {
            self.total_voting_power
        }

        #[ink(message)]
        /// Change the admin (only current admin can).
        fn update_admin(&mut self, new_admin: AccountId) -> Result<(), InkGroupError> {
            let caller = self.env().caller();
            let admin = self.get_admin()?;
            ensure!(caller == admin, InkGroupError::Unauthorized {});
            self.admin.set(&new_admin);
            // Emit event that the admin was updated
            self.env().emit_event(AdminUpdate {
                old_admin: admin,
                new_admin,
            });
            Ok(())
        }

        #[ink(message)]
        /// If an already existing address is entered, the voting power is updated. Remove is applied after add, so if an address is in both, it is removed.
        fn update_members(
            &mut self,
            new_members: Vec<Member>,
            remove_members: Vec<AccountId>,
        ) -> Result<(), InkGroupError> {
            let caller = self.env().caller();
            let admin = self.get_admin()?;
            ensure!(caller == admin, InkGroupError::Unauthorized {});
            validate_unique_members(&new_members)?;
            // for every new member check if already exist in the group, in that case update the voting power
            // otherwise add the member to the group
            for member in new_members {
                if let Some(index) = self
                    .members
                    .iter()
                    .position(|&old_member| old_member.addr == member.addr)
                {
                    // first subtract the old vote weight from the total
                    self.total_voting_power -= self.members[index].weight;
                    // then add the new vote weight to the total
                    self.total_voting_power += member.weight;
                    // last change the old vote weight of the member to the new
                    self.members[index].weight = member.weight;
                    // Emit event that the member was updated
                    self.env().emit_event(MemberUpdate {
                        member: self.members[index].addr,
                    })
                } else {
                    // add the new member and then add the vote weight to the total
                    self.members.push(member);
                    // Emit the event that the member was added
                    self.env().emit_event(MemberAddition {
                        member: member.addr,
                    });
                    self.total_voting_power += member.weight;
                }
            }
            // for each member to be removed check that it actually already exists within the group
            // and in this case first subtract the weight of the vote from the total and then
            // delete the member otherwise do nothing
            for member in remove_members {
                if let Some(index) = self
                    .members
                    .iter()
                    .position(|&old_member| old_member.addr == member)
                {
                    self.total_voting_power -= self.members[index].weight;
                    let removed_member_addr = self.members[index].addr;
                    self.members.remove(index);
                    // Emit the event that the member was removed
                    self.env().emit_event(MemberRemoval {
                        member: removed_member_addr,
                    });
                }
            }

            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use ink::env::test;

        // Integration test setup

        fn default_accounts() -> test::DefaultAccounts<Environment> {
            ink::env::test::default_accounts::<Environment>()
        }

        fn set_caller(sender: AccountId) {
            ink::env::test::set_caller::<Environment>(sender);
        }

        fn build_contract() -> InkGroupSimple {
            let accounts = default_accounts();

            let alice_member = Member {
                addr: accounts.alice,
                weight: 1,
            };
            let bob_member = Member {
                addr: accounts.bob,
                weight: 1,
            };

            let members = vec![alice_member, bob_member];

            set_caller(alice_member.addr);

            InkGroupSimple::try_new(None, members).unwrap()
        }

        #[ink::test]
        /// The default constructor does its job.
        fn construction_works() {
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
            assert_eq!(contract.admin.get().unwrap(), accounts.alice);
            assert!(contract.members.iter().eq(members.iter()));
            assert!(contract.members.contains(&alice_member));
            assert!(!contract.members.contains(&charlie_member));
        }

        #[ink::test]
        /// Get the current admin of the group
        fn get_admin_works() {
            let accounts = default_accounts();
            let contract = build_contract();
            let response = InkGroupSimple::get_admin(&contract).unwrap();
            assert_eq!(response, accounts.alice);
        }

        #[ink::test]
        /// Get the members of the group
        fn get_members_works() {
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
            let response = InkGroupSimple::get_members(&contract).unwrap();
            assert_eq!(response, members);
            assert!(!response.contains(&charlie_member));
        }

        #[ink::test]
        /// Get member info searched by address
        fn get_member_works() {
            let accounts = default_accounts();
            let alice_member = Member {
                addr: accounts.alice,
                weight: 1,
            };
            let contract = build_contract();
            let response = InkGroupSimple::get_member(&contract, accounts.alice).unwrap();
            assert_eq!(response, alice_member);
            let err_response = InkGroupSimple::get_member(&contract, accounts.eve).unwrap_err();
            assert_eq!(err_response, InkGroupError::NoMember {});
        }

        #[ink::test]
        /// Get total voting power
        fn get_total_weight_works() {
            let contract = build_contract();
            let response = InkGroupSimple::get_total_weight(&contract);
            assert_eq!(response, 2);
        }

        #[ink::test]
        /// Update admin
        fn update_admin_works() {
            let accounts = default_accounts();
            let mut contract = build_contract();
            set_caller(accounts.bob);
            let err_response =
                InkGroupSimple::update_admin(&mut contract, accounts.bob).unwrap_err();
            assert_eq!(err_response, InkGroupError::Unauthorized {});
            set_caller(accounts.alice);
            InkGroupSimple::update_admin(&mut contract, accounts.bob).unwrap();
            assert_eq!(contract.admin.get().unwrap(), accounts.bob);
        }

        #[ink::test]
        /// Update members
        fn update_members_works() {
            let accounts = default_accounts();
            let mut contract = build_contract();
            set_caller(accounts.bob);
            let err_response =
                InkGroupSimple::update_admin(&mut contract, accounts.bob).unwrap_err();
            assert_eq!(err_response, InkGroupError::Unauthorized {});
            set_caller(accounts.alice);
            let update_alice = Member {
                addr: accounts.alice,
                weight: 2,
            };
            let bob_member = Member {
                addr: accounts.bob,
                weight: 1,
            };
            let charlie_member = Member {
                addr: accounts.charlie,
                weight: 1,
            };
            InkGroupSimple::update_members(&mut contract, vec![update_alice], vec![]).unwrap();
            let result = InkGroupSimple::get_member(&contract, accounts.alice).unwrap();
            let total_voting_power = InkGroupSimple::get_total_weight(&contract);
            assert_eq!(result.weight, 2);
            assert_eq!(total_voting_power, 3);
            InkGroupSimple::update_members(&mut contract, vec![charlie_member], vec![]).unwrap();
            let result = InkGroupSimple::get_members(&contract).unwrap();
            let total_voting_power = InkGroupSimple::get_total_weight(&contract);
            assert_eq!(result.len(), 3);
            assert_eq!(total_voting_power, 4);
            InkGroupSimple::update_members(&mut contract, vec![], vec![accounts.alice]).unwrap();
            let result = InkGroupSimple::get_members(&contract).unwrap();
            let total_voting_power = InkGroupSimple::get_total_weight(&contract);
            assert_eq!(result.len(), 2);
            assert_eq!(total_voting_power, 2);
            let err_response =
                InkGroupSimple::update_members(&mut contract, vec![bob_member, bob_member], vec![])
                    .unwrap_err();
            assert_eq!(
                err_response,
                InkGroupError::DuplicateMember {
                    member: accounts.bob
                }
            );
        }
    }

    #[cfg(all(test, feature = "e2e-tests"))]
    mod e2e_tests {
        use super::InkGroupSimpleRef;
        use ink_e2e::build_message;
        type E2EResult<T> = std::result::Result<T, Box<dyn std::error::Error>>;

        #[ink_e2e::test]
        async fn e2e_can_add_members(mut client: ink_e2e::Client<C, E>) -> E2EResult<()> {
            let alice_member = Member {
                addr: ink_e2e::alice(),
                weight: 1,
            };
            let bob_member = Member {
                addr: ink_e2e::bob(),
                weight: 1,
            };

            let members = vec![alice_member, bob_member];
            let constructor = InkGroupSimpleRef::try_new(None, members);
            let contract_addr = client
                .instantiate("ink_voting_group", &ink_e2e::alice(), constructor, 0, None)
                .await
                .expect("Instantiate failed")
                .account_id;
            Ok(())
        }
    }
}
