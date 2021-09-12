#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod command;
pub mod core;
mod encoder;
pub mod error;
mod parser;
pub mod reply;
