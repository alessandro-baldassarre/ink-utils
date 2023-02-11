#![cfg_attr(not(feature = "std"), no_std)]

mod error;
mod message;
mod storage;

pub use crate::error::InkGroupError;
pub use crate::message::InkGroup;
pub use crate::storage::Member;
