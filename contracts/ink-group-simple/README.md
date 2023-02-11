
# Ink Group Simple

This is a simple implementation of [ink-group](https://github.com/alessandro-baldassarre/ink-utils/tree/main/traits/ink-group) specification.
Members are stored by an address and voting power weight.
Admin is the only allowed to update and modify the storage.

## Constructor

```rust
pub fn try_new(
            admin: Option<AccountId>,
            initial_members: Vec<Member>,
        ) -> Result<Self, ContractError>
```

```rust
pub struct Member {
    pub addr: AccountId,
    pub weight: u64,
}
```

To construct the contract you must provide a list of members. You can also provide an optional admin address, in case you not provide it the sender address is set to admin.

## Messages

The contract implements all the methods describe in the [ink-group](https://github.com/alessandro-baldassarre/ink-utils/tree/main/traits/ink-group) specification.

## Events

Events emit during contract execution ([ink! - Events](https://use.ink/basics/events))

```rust
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
```

