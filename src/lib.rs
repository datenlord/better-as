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
    clippy::cast_lossless
)]
#![cfg_attr(not(feature = "std"), no_std)]

pub mod character;
pub mod checked_cast;
pub mod extending_cast;
pub mod pointer;
pub mod truncating_cast;
pub mod wrapping_cast;

pub use self::checked_cast::{CheckedCast, NumCastError};
pub use self::extending_cast::ExtendingCast;
pub use self::truncating_cast::TruncatingCast;
pub use self::wrapping_cast::WrappingCast;
