//! Explicit type casting

#![deny(
    // missing_docs,
    missing_debug_implementations,
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(
    clippy::as_conversions,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation,
    clippy::cast_lossless,
    clippy::missing_errors_doc  // TODO: add doc
)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod character;
pub mod number;
pub mod pointer;
