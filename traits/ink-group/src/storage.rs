use ink::primitives::AccountId;
use scale::{Decode, Encode};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash, Encode, Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
/// Member of the group
pub struct Member {
    /// Address of the member
    pub addr: AccountId,
    /// Voting power of the member (it can be 0, the member will be part of the group but can't
    /// vote)
    pub weight: u64,
}
