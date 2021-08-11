#![no_std]

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod command;
mod core;
mod encoder;
pub mod error;
mod parser;
pub mod reply;
