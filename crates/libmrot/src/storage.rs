//! Storage for meal records

use crate::{
    convert::{convert_to_naive_date, convert_to_timestamps},
    error::Error,
    LookAhead, MealRecord, Result,
};
use chrono::naive::NaiveDate;
use sqlite::{Connection, State, Value};
use std::{fmt, path::Path};
use tracing::{instrument, trace};

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
        let converted_dates = convert_to_timestamps(dates)?;

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

    /// Show on what dates a meal was recorded.
    #[instrument]
    pub fn when(&self, meal: &str) -> Result<Vec<NaiveDate>> {
        let query = "SELECT date FROM meals WHERE meal = :meal ORDER BY date ASC";
        let mut statement = self.connection.prepare(query)?;
        statement.bind((":meal", meal))?;
        let mut naive_dates: Vec<NaiveDate> = vec![];
        while let Ok(State::Row) = statement.next() {
            let timestamp = statement.read::<i64, _>("date")?;
            let naive_date = convert_to_naive_date(timestamp)?;
            naive_dates.push(naive_date);
        }
        Ok(naive_dates)
    }

    /// Show what meals were recorded in the given date range.
    #[instrument]
    pub fn show(&self, date_range: &str) -> Result<Vec<MealRecord>> {
        let timestamps = convert_to_timestamps(&vec![String::from(date_range)])?;
        // timestamps are guaranteed to be a vector of at least one element, so we can unwrap
        let start = timestamps.iter().next().unwrap();
        let end = timestamps.iter().last().unwrap();
        let query =
            "SELECT date, meal FROM meals WHERE date >= :start AND date <= :end ORDER BY date ASC";
        let mut statement = self.connection.prepare(query)?;
        statement
            .bind_iter::<_, (_, Value)>([(":start", (*start).into()), (":end", (*end).into())])?;
        let mut records = Vec::new();
        while let Ok(State::Row) = statement.next() {
            let timestamp = statement.read::<i64, _>("date")?;
            let meal = statement.read::<String, _>("meal")?;
            records.push(MealRecord { meal, timestamp });
        }
        Ok(records)
    }

    /// Suggest meals to cook. Returns [MealRecord]s of the suggested meals and the last dates when
    /// they were cooked. Ignores the meals in the *ignore* vector and meals recorded on the dates
    /// in the look_ahead vector.
    pub fn what(
        &self,
        number: u64,
        look_ahead: Option<LookAhead>,
        ignore: Vec<String>,
    ) -> Result<Vec<MealRecord>> {
        let candidates = self.get_meal_candidates(look_ahead, ignore)?;
        let suggestions = Self::pick_n_meal_records(number, candidates);
        Ok(suggestions)
    }

    fn get_meal_candidates(
        &self,
        look_ahead: Option<LookAhead>,
        ignore: Vec<String>,
    ) -> Result<Vec<MealRecord>> {
        let last_cooked_unique_meals = self.get_last_cooked_unique()?;
        let planned_meal_records = match look_ahead {
            None => Vec::new(),
            Some(period) => self.get_meal_records_between_dates(
                period.first_day_timestamp(),
                period.last_day_timestamp(),
            )?,
        };
        let mut ignored_meals: Vec<_> = ignore
            .into_iter()
            .chain(planned_meal_records.into_iter().map(|record| record.meal))
            .collect();
        ignored_meals.sort();
        ignored_meals.dedup();
        let candidates = Self::filter_meal_records(last_cooked_unique_meals, ignored_meals);
        Ok(candidates)
    }

    fn get_meal_records_between_dates(&self, _start: i64, _end: i64) -> Result<Vec<MealRecord>> {
        todo!();
    }

    fn get_last_cooked_unique(&self) -> Result<Vec<MealRecord>> {
        todo!();
    }

    fn filter_meal_records(_records: Vec<MealRecord>, _ignore: Vec<String>) -> Vec<MealRecord> {
        todo!();
    }

    fn pick_n_meal_records(_number: u64, _candidates: Vec<MealRecord>) -> Vec<MealRecord> {
        todo!();
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
