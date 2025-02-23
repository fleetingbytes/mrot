//! Meal Rotator Library
//!
//! A library for recording, planning and suggesting meals. It is the core of the meal rotator app [mrot][mrot].
//!
//! [mrot]: https://docs.rs/mrot

mod error;
#[allow(dead_code)]
mod meal_record;
mod storage;

pub use error::Error;
pub use storage::Storage;

/// Type alias for results with libmrot's [Error].
pub type Result<T> = std::result::Result<T, Error>;
