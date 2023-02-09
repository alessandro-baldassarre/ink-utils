#![cfg_attr(not(feature = "std"), no_std)]
#![feature(min_specialization)]

mod error;
mod message;
mod storage;

pub use crate::error::InkGroupError;
pub use crate::message::InkGroup;
pub use crate::storage::Member;
