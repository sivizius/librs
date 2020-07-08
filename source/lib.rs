//! …

#![warn(clippy::all)]
#![allow(clippy::suspicious_else_formatting)]

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![warn(missing_docs)]
#![warn(future_incompatible)]

#![no_std]
#![feature(asm)]
#![feature(never_type)]

/// Syscalls.
pub mod linux;
