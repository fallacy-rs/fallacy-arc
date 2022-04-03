//! A library for fallible Arc.

#![feature(allocator_api)]

mod arc;
mod weak;

pub use arc::Arc;
pub use weak::Weak;

pub use fallacy_alloc::AllocError;
