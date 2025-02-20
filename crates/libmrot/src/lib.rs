//! Meal Rotator Library
//!
//! A library for recording, planning and suggesting meals. It is the core of the meal rotator app [mrot][mrot].
//!
//! [mrot]: https://docs.rs/mrot

pub mod error;
mod meal_record;
pub mod storage;

pub use error::Error;

/// Type alias for results with libmrot's [Error].
pub type Result<T> = core::result::Result<T, Error>;
