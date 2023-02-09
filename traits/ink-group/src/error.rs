use ink::primitives::AccountId;
use thiserror_no_std::Error;

#[derive(Error, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum InkGroupError {
    #[error("Logic contract error")]
    LogicErr {},
    #[error("Unauthorized")]
    Unauthorized {},
    #[error("entered duplicate member")]
    DuplicateMember { member: AccountId },
    #[error("no members entered")]
    ZeroMembers {},
    #[error("member not found")]
    NoMember {},
}
