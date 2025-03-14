//! Meal Rotator Library
//!
//! A library for recording, planning and suggesting meals. It is the core of the meal rotator app [mrot][mrot].
//!
//! [mrot]: https://docs.rs/mrot

mod convert;
mod error;
mod meal_record;
mod storage;

pub use convert::parse_date;
pub use error::Error;
pub use meal_record::MealRecord;
pub use storage::Storage;

/// Type alias for results with libmrot's [Error].
pub type Result<T> = std::result::Result<T, Error>;
