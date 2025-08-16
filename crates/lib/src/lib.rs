#![no_std]

pub mod helper;
pub mod model;
pub mod module;

#[global_allocator]
static ALLOCATOR: talc::TalckWasm = unsafe { talc::TalckWasm::new_global() };
