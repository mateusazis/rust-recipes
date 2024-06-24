#![allow(dead_code)]

pub mod alignment;
pub mod alloc;
pub mod associated_types;
pub mod asyncer;
pub mod cell;
pub mod closures;
pub mod conditional_compilation;
pub mod dynamic_dispatch;
pub mod errors;
pub mod ffi;
pub mod generics;
pub mod generics_vs_associated_types;
pub mod glibc;
pub mod hashes;
pub mod inline_assembly;
pub mod interfaces;
pub mod manual_async;
pub mod math;
pub mod pinning;
pub mod references;
pub mod simd;
pub mod slices;
pub mod template_functions;
pub mod threading;
pub mod threading_shared_state;
pub mod traits;
pub mod unions;

mod manual_async_futures;
