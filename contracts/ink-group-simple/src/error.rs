use ink_group::InkGroupError;
use thiserror_no_std::Error;

#[derive(Error, Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum ContractError {
    #[error("{0}")]
    InkGroup(#[from] InkGroupError),
}
