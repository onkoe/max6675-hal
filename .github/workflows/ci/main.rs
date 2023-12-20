//! This file checks to see if the `error_in_core` feature is stable.
//!
//! It'll send a warning to the compiler once it is, as it'll be an unnecessary
//! feature gate.

#![cfg_attr(feature = "nightly", feature(error_in_core))]
fn main() {}
