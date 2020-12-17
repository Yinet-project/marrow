//! `mw-rt` is runtime for wasm.

#![no_std]

extern crate alloc;

pub use mw_macros::async_main;
pub use mw_macros::main;

pub mod runtime;
pub mod task;

mod utils;
