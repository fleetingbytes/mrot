//! Storage for meal records

use crate::error::Error;
use crate::Result;
use chrono::naive::NaiveDate;
use sqlite::{Connection, State, Value};
use std::{fmt, path::Path};
use tracing::{instrument, trace, Span};
use two_timer::parse;

/// Storage for meal data.
pub struct Storage {
    connection: Connection,
    path_string: String,
}

impl Storage {
    /// Query to create all of the databases tables.
    const QUERY_TO_CREATE_SQL_STORAGE_TABLES: &str = "CREATE TABLE meals (date INTEGER, meal TEXT)";
    /// Path to an in-memory storage. Useful for testing.
    const MEMORY: &str = ":memory:";

    /// Opens a storage in the given path. Given a path to a non-existing file, this will try to
    /// create a new storage in that path and then open it.
    ///
    /// Example:
    /// ```no_run
    /// use libmrot::Storage;
    ///
    /// let storage = Storage::open("./path/to/my_storage").unwrap();
    /// ```
    ///
    /// For testing purposes the special path `:memory:` gives access to an in-memory storage which will
    /// live as long as the instance of this struct.
    ///
    /// Example:
    /// ```
    /// use libmrot::Storage;
    ///
    /// let test_storage = Storage::open(":memory:").unwrap();
    /// ```
    #[instrument]
    pub fn open(path: &str) -> Result<Self> {
        trace!(%path, "Open database");
        let connection: Connection = match path {
            Self::MEMORY => Self::new(&path)?,
            _ => {
                trace!(%path, "Path is a real file");
                if Path::new(path).try_exists()? {
                    sqlite::open(path)?
                } else {
                    Self::new(path)?
                }
            }
        };
        let path_string = String::from(path);
        Ok(Self {
            connection,
            path_string,
        })
    }

    /// Creates a new storage.
    #[instrument]
    fn new(path: &str) -> Result<Connection> {
        trace!(%path, "Creating new database");
        std::fs::create_dir_all(Path::new(path).parent().ok_or(Error::NoParentDirectory)?)?;
        let connection = sqlite::open(path)?;
        connection.execute(Self::QUERY_TO_CREATE_SQL_STORAGE_TABLES)?;
        Ok(connection)
    }

    /// Adds a meal on the given dates to the storage.
    #[instrument]
    pub fn add_meal_on_dates(&self, meal: &str, dates: &Vec<String>) -> Result<()> {
        let converted_dates = Self::convert_to_timestamps(dates)?;

        self.connection.execute("BEGIN TRANSACTION")?;
        let query = "INSERT INTO meals VALUES (:date, :meal)";
        let mut statement = self.connection.prepare(query)?;

        for date in converted_dates {
            statement.reset()?;
            statement
                .bind_iter::<_, (_, Value)>([(":date", date.into()), (":meal", meal.into())])?;

            while let State::Row = statement.next()? {}
        }

        self.connection.execute("COMMIT")?;
        Ok(())
    }

    /// Convert human-readable dates to timestamps.
    #[instrument]
    fn convert_to_timestamps(dates: &Vec<String>) -> Result<Vec<i64>> {
        dates
            .iter()
            .map(|date| Self::parse_date(date).and_then(Self::convert_date_to_timestamp))
            .collect()
    }

    /// Parses a date string into NaiveDate.
    #[instrument(level = "debug", fields(result))]
    fn parse_date(date: &str) -> Result<NaiveDate> {
        let (naive_datetime, _end_date, range) = parse(date, None)?;
        if range {
            return Err(Error::TimeSpanNotSupported);
        }
        let naive_date = naive_datetime.date();
        Span::current().record("result", &naive_date.to_string());
        Ok(naive_date)
    }

    #[instrument(level = "debug", fields(result))]
    fn convert_date_to_timestamp(date: NaiveDate) -> Result<i64> {
        let timestamp = date
            .and_hms_opt(0, 0, 0)
            .ok_or(Error::TimeNotSupported)?
            .and_utc()
            .timestamp();
        Span::current().record("result", &timestamp);
        Ok(timestamp)
    }
}

impl fmt::Debug for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Storage(chg: {})", self.connection.total_change_count())
    }
}

impl fmt::Display for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Storage({})", self.path_string)
    }
}
