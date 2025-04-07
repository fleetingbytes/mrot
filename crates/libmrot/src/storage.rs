//! Storage for meal records

use crate::{convert::convert_to_timestamps, error::Error, MealRecord, Period, Result};
use chrono::naive::NaiveDate;
use rand::seq::IteratorRandom;
use sqlite::{Connection, State, Value};
use std::{cmp::min, fmt, path::Path};
use tracing::{instrument, trace};

/// Storage for meal records.
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
    /// live as long as the returned struct.
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
    ///
    /// Example:
    /// ```
    /// use libmrot::Storage;
    ///
    /// // open in-memory storage
    /// let storage = Storage::open(":memory:").unwrap();
    ///
    /// // prepare dates where each meal was consumed
    /// let bolognese_dates = vec![String::from("from yesterday through today")];
    /// let rib_eye_steak_dates = vec![
    ///     "this Sunday".to_string(),
    ///     "2025-03-13 through 2025-03-14".to_string(),
    ///     ];
    ///
    /// // store the data in the storage
    /// storage.add_meal_on_dates("bolognese", &bolognese_dates).unwrap();
    /// storage.add_meal_on_dates("rib eye steak", &rib_eye_steak_dates).unwrap();
    /// ```
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

    /// Suggest meals to cook.
    /// Each suggested meal comes as a [`MealRecord`] with the date of date of its
    /// latest consumption.
    ///
    /// Function takes two kinds of filters:
    /// 1. `option_ignore_period`: optional [`Period`] to ignore the kinds of meals within it
    /// 2. `ignore`: to ignore specific kinds of meals in general
    ///
    /// Example:
    /// ```
    /// use libmrot::{MealRecord, Period, Storage};
    ///
    /// // open in-memory storage
    /// let storage = Storage::open(":memory:").unwrap();
    ///
    /// // fill storage with data
    /// storage.add_meal_on_dates(
    ///     "spaghetti",
    ///     &vec![String::from("from March 1 through March 2")]
    ///     ).unwrap();
    /// storage.add_meal_on_dates(
    ///     "meat balls",
    ///     &vec![
    ///         String::from("from March 3 through March 4"),
    ///         String::from("March 11"),
    ///     ]).unwrap();
    /// storage.add_meal_on_dates("pizza", &vec![String::from("March 5")]).unwrap();
    /// storage.add_meal_on_dates("steak", &vec![String::from("March 6")]).unwrap();
    /// storage.add_meal_on_dates(
    ///     "lentils and wieners",
    ///     &vec![String::from("March 8 through March 9")]
    ///     ).unwrap();
    ///
    /// // we are going to ignore the kinds of meals
    /// // which were or will be consumed on these dates:
    /// let option_period: Option<Period> = Some(Period::new(
    ///     "from March 10 through March 22"
    ///     ).unwrap());
    /// // we are also going to ignore spaghetti in general
    /// let ignore = vec![String::from("spaghetti")];
    ///
    /// // get meal suggestions
    /// let suggestions: Vec<MealRecord> = storage.what(3, option_period, ignore).unwrap();
    ///
    /// // we expect the suggestions to contain the records of pizza, steak, lentils and wieners.
    /// // Meat balls were ignored because one of their dates is inside the ignore period
    /// // Spaghetti were ignored by our `ignore` vector,
    /// let expected_suggestions: Vec<MealRecord> = vec![
    ///     MealRecord::new("pizza", "March 5").unwrap(),
    ///     MealRecord::new("steak", "March 6").unwrap(),
    ///     MealRecord::new("lentils and wieners", "March 9").unwrap(),
    /// ];
    ///
    /// assert_eq!(suggestions, expected_suggestions);
    /// ```
    #[instrument]
    pub fn what(
        &self,
        number: u64,
        option_ignore_period: Option<Period>,
        ignore_list: Vec<String>,
    ) -> Result<Vec<MealRecord>> {
        let mut candidates = self.get_meal_candidates(option_ignore_period, ignore_list)?;
        let suggestions = Self::pick_n_meal_records(number as usize, &mut candidates);
        Ok(suggestions)
    }

    #[instrument(level = "debug")]
    fn get_meal_candidates(
        &self,
        option_period: Option<Period>,
        ignore_list: Vec<String>,
    ) -> Result<Vec<MealRecord>> {
        let mut last_cooked_unique_meals = self.get_last_cooked_unique()?;
        let planned_meal_records = match option_period {
            None => Vec::new(),
            Some(period) => self.get_meal_records_in_period(period)?,
        };
        let mut ignored_meals: Vec<_> = ignore_list
            .into_iter()
            .chain(planned_meal_records.into_iter().map(|record| record.meal()))
            .collect();
        ignored_meals.sort();
        ignored_meals.dedup();
        Self::filter_meal_records(&mut last_cooked_unique_meals, &ignored_meals);
        Ok(last_cooked_unique_meals)
    }

    #[instrument(level = "trace")]
    fn get_meal_records_in_period(&self, period: Period) -> Result<Vec<MealRecord>> {
        let condition = "WHERE date >= :start AND date <= :end";
        let condition_params: Vec<(&str, Value)> = vec![
            (":start", period.first_day_timestamp().into()),
            (":end", period.last_day_timestamp().into()),
        ];
        self.select_records(condition, &condition_params)
    }

    /// Outputs meal records with unique meals and their respective last dates. The result vector is sorted
    /// by date.
    ///
    /// Example:
    /// ```
    /// use libmrot::{MealRecord, Storage};
    ///
    /// // open in-memory storage
    /// let storage = Storage::open(":memory:").unwrap();
    ///
    /// // fill storage with some data
    /// storage.add_meal_on_dates(
    ///     "spaghetti",
    ///     &vec![String::from("from March 1 through March 2, 2025")],
    ///     ).unwrap();
    /// storage.add_meal_on_dates(
    ///      "curry",
    ///      &vec![String::from("from March 3 through March 4, 2025")],
    ///      ).unwrap();
    ///
    /// // get unique meals
    /// let unique_meals = storage.get_last_cooked_unique().unwrap();
    /// let expected_meal_records = vec![
    ///     MealRecord::new("spaghetti", "March 2, 2025").unwrap(),
    ///     MealRecord::new("curry", "March 4, 2025").unwrap(),
    /// ];
    /// assert_eq!(unique_meals, expected_meal_records);
    /// ```
    #[instrument]
    pub fn get_last_cooked_unique(&self) -> Result<Vec<MealRecord>> {
        let query = "SELECT meal, MAX(date) AS date FROM meals GROUP BY meal ORDER BY date ASC";
        let mut statement = self.connection.prepare(query)?;
        let mut records = Vec::new();
        while let Ok(State::Row) = statement.next() {
            let timestamp = statement.read::<i64, _>("date")?;
            let meal = statement.read::<String, _>("meal")?;
            records.push(MealRecord::from_meal_and_timestamp(&meal, timestamp)?);
        }
        Ok(records)
    }

    #[instrument(level = "trace")]
    fn filter_meal_records(records: &mut Vec<MealRecord>, ignore: &Vec<String>) -> () {
        records.retain(|r| !ignore.contains(&r.meal()));
    }

    #[instrument(level = "debug")]
    fn pick_n_meal_records(number: usize, candidates: &mut Vec<MealRecord>) -> Vec<MealRecord> {
        _ = candidates.split_off(min(number, candidates.len()));
        candidates.drain(..).collect()
    }

    /// Samples one random meal record from all unique recorded meals.
    ///
    /// Example:
    /// ```
    /// use libmrot::Storage;
    ///
    /// // prepare storage with some data
    /// let storage = Storage::open(":memory:").unwrap();
    /// let yesterday = vec![String::from("yesterday")];
    /// let today = vec![String::from("today")];
    /// storage.add_meal_on_dates("pork liver", &yesterday).unwrap();
    /// storage.add_meal_on_dates("champaign and caviar", &today).unwrap();
    ///
    /// // pick a random meal
    /// let random_pick = storage.random().unwrap().unwrap();
    /// println!("Let's have {} again, yay!", random_pick.meal());
    /// ```
    ///
    /// None:
    ///
    /// Returns none if there are no meal records in the storage.
    /// ```
    /// use libmrot::Storage;
    ///
    /// // open an empty in-memory storage
    /// let storage = Storage::open(":memory:").unwrap();
    ///
    /// assert_eq!(storage.random().unwrap(), None);
    /// ```
    #[instrument]
    pub fn random(&self) -> Result<Option<MealRecord>> {
        let unique = self.get_last_cooked_unique()?;
        Ok(unique.into_iter().choose(&mut rand::rng()))
    }

    /// Show what meals were consumed in the given date range.
    ///
    /// Example:
    /// ```
    /// use libmrot::{MealRecord, Storage};
    ///
    /// // open in-memory storage
    /// let storage = Storage::open(":memory:").unwrap();
    ///
    /// // fill storage with some data
    /// storage.add_meal_on_dates(
    ///     "spaghetti",
    ///     &vec![String::from("from March 1 through March 2, 2025")],
    ///     ).unwrap();
    /// storage.add_meal_on_dates(
    ///     "curry",
    ///     &vec![String::from("from March 3 through March 4, 2025")],
    ///     ).unwrap();
    ///
    /// // get recorded data
    /// let actual_meal_records = storage.show("March 2025").unwrap();
    /// let expected_meal_records = vec![
    ///     MealRecord::new("spaghetti", "March 1, 2025").unwrap(),
    ///     MealRecord::new("spaghetti", "March 2, 2025").unwrap(),
    ///     MealRecord::new("curry", "March 3, 2025").unwrap(),
    ///     MealRecord::new("curry", "March 4, 2025").unwrap(),
    /// ];
    /// assert_eq!(actual_meal_records, expected_meal_records);
    ///
    /// actual_meal_records.into_iter().for_each(|record| println!("{}", record));
    /// ```
    /// will print:
    /// ```text
    /// spaghetti (2025-03-01)
    /// spaghetti (2025-03-02)
    /// curry (2025-03-03)
    /// curry (2025-03-04)
    /// ```
    #[instrument]
    pub fn show(&self, date_range: &str) -> Result<Vec<MealRecord>> {
        //let timestamps = convert_to_timestamps(&vec![String::from(date_range)])?;
        // timestamps are guaranteed to be a vector of at least one element, so we can unwrap
        //let start = timestamps.iter().next().unwrap();
        //let end = timestamps.iter().last().unwrap();
        let period = Period::new(date_range)?;
        self.get_meal_records_in_period(period)
    }

    /// Show on what dates a meal was recorded.
    ///
    /// Example:
    /// ```
    /// use libmrot::Storage;
    /// use chrono::NaiveDate;
    ///
    /// // open in-memory storage
    /// let storage = Storage::open(":memory:").unwrap();
    ///
    /// // fill storage with some data
    /// storage.add_meal_on_dates(
    ///     "spaghetti",
    ///     &vec![String::from("from March 1 through March 2, 2025")],
    ///     ).unwrap();
    /// storage.add_meal_on_dates(
    ///     "curry",
    ///     &vec![String::from("from March 3 through March 4, 2025")],
    ///     ).unwrap();
    ///
    /// // get dates on which spaghetti where recorded
    /// let actual_dates = storage.when("spaghetti").unwrap();
    ///
    /// // expected dates
    /// let expected_dates = vec![
    ///     NaiveDate::from_ymd_opt(2025, 3, 1).unwrap(),
    ///     NaiveDate::from_ymd_opt(2025, 3, 2).unwrap(),
    /// ];
    /// actual_dates.into_iter().for_each(|date| println!("{}", date));
    /// ```
    /// will print:
    /// ```text
    /// 2025-03-01
    /// 2025-03-02
    /// ```
    #[instrument]
    pub fn when(&self, meal: &str) -> Result<Vec<NaiveDate>> {
        let condition = "WHERE meal = :meal";
        let condition_params: Vec<(&str, Value)> = vec![(":meal", meal.into())];
        let meal_records = self.select_records(condition, &condition_params)?;
        let naive_dates: Vec<NaiveDate> =
            meal_records.into_iter().map(|r| r.naive_date()).collect();
        Ok(naive_dates)
    }

    /// Remove all meal records in the given period. Optionally, remove only records of one specific meal in
    /// that period. Returns the deleted records.
    ///
    /// Example:
    /// ```
    /// use libmrot::{MealRecord, Period, Storage};
    ///
    /// // open in-memory storage
    /// let storage = Storage::open(":memory:").unwrap();
    ///
    /// // fill storage with some data
    /// storage.add_meal_on_dates(
    ///     "spaghetti",
    ///     &vec![String::from("from March 1 through March 2, 2025")],
    ///     ).unwrap();
    /// storage.add_meal_on_dates(
    ///     "curry",
    ///     &vec![String::from("from March 3 through March 4, 2025")],
    ///     ).unwrap();
    ///
    /// // remove spaghetti in March
    /// let period = Period::new("March").unwrap();
    /// let deleted_records = storage.remove(period, Some("spaghetti".to_string())).unwrap();
    ///
    /// let expected_deleted_records = vec![
    ///     MealRecord::new("spaghetti", "March 1").unwrap(),
    ///     MealRecord::new("spaghetti", "March 2").unwrap(),
    /// ];
    ///
    /// assert_eq!(deleted_records, expected_deleted_records);
    ///
    /// // remove all meals on March 3rd
    /// let period = Period::new("March 3").unwrap();
    /// let deleted_records = storage.remove(period, None).unwrap();
    ///
    /// let expected_deleted_records = vec![
    ///     MealRecord::new("curry", "March 3").unwrap(),
    /// ];
    ///
    /// assert_eq!(deleted_records, expected_deleted_records);
    /// ```
    #[instrument]
    pub fn remove(&self, period: Period, option_meal: Option<String>) -> Result<Vec<MealRecord>> {
        let action_clause = "DELETE FROM meals";
        let action_params = Vec::new();

        let condition = match option_meal {
            None => "WHERE date >= :start AND DATE <= :end",
            Some(_) => "WHERE meal = :meal AND date >= :start AND DATE <= :end",
        };
        let mut condition_params: Vec<(&str, Value)> = vec![
            (":start", period.first_day_timestamp().into()),
            (":end", period.last_day_timestamp().into()),
        ];
        if let Some(meal) = option_meal {
            condition_params.push((":meal", meal.into()));
        }

        let delete_func = || {
            let records = self.select_records(condition, &condition_params)?;
            self.manipulate_records(action_clause, &action_params, condition, &condition_params)?;
            Ok(records)
        };

        let records = self.sql_transaction(delete_func)?;
        Ok(records)
    }

    /// Rename a meal from *old_name* to *new_name*, optionally rename only in the given period.
    ///
    /// Example:
    /// ```
    /// use libmrot::{MealRecord, Period, Storage};
    ///
    /// // open in-memory storage
    /// let storage = Storage::open(":memory:").unwrap();
    ///
    /// // fill storage with some data
    /// storage.add_meal_on_dates(
    ///     "spaghetti",
    ///     &vec![String::from("from March 1 through March 2")],
    ///     ).unwrap();
    /// storage.add_meal_on_dates(
    ///     "curry",
    ///     &vec![String::from("from March 3 through March 4")],
    ///     ).unwrap();
    ///
    /// // rename spaghetti to penne on March 1st
    /// let old_records = storage.rename("spaghetti", "penne", Some(Period::new("March 1").unwrap())).unwrap();
    ///
    /// let expected_old_records = vec![
    ///     MealRecord::new("spaghetti", "March 1").unwrap(),
    /// ];
    ///
    /// assert_eq!(old_records, expected_old_records);
    ///
    /// // check current records
    /// let current_records = storage.show("from March 1 through March 2").unwrap();
    /// let expected_current_records = vec![
    ///     MealRecord::new("penne", "March 1").unwrap(),
    ///     MealRecord::new("spaghetti", "March 2").unwrap(),
    /// ];
    ///
    /// assert_eq!(current_records, expected_current_records);
    /// ```
    #[instrument]
    pub fn rename(
        &self,
        old_name: &str,
        new_name: &str,
        option_period: Option<Period>,
    ) -> Result<Vec<MealRecord>> {
        let action_clause = "UPDATE meals SET meal = :new_name";
        let action_params: Vec<(&str, Value)> = vec![(":new_name", new_name.into())];

        let condition = match option_period {
            None => "WHERE meal = :meal",
            Some(_) => "WHERE meal = :meal AND date >= :start AND date <= :end",
        };
        let mut condition_params: Vec<(&str, Value)> = vec![(":meal", old_name.into())];
        if let Some(period) = option_period {
            condition_params.push((":start", period.first_day_timestamp().into()));
            condition_params.push((":end", period.last_day_timestamp().into()));
        }

        let update_func = || {
            let records = self.select_records(condition, &condition_params)?;
            self.manipulate_records(action_clause, &action_params, condition, &condition_params)?;
            Ok(records)
        };

        let records = self.sql_transaction(update_func)?;
        Ok(records)
    }

    fn select_records(
        &self,
        condition: &str,
        selection_params: &[(&str, Value)],
    ) -> Result<Vec<MealRecord>> {
        let select_query = format!(
            "SELECT date, meal FROM meals {} ORDER BY date ASC",
            condition
        );
        let mut select_statement = self.connection.prepare(select_query)?;
        for &(name, ref value) in selection_params {
            select_statement.bind((name, value))?;
        }

        let mut records = Vec::new();
        while let Ok(State::Row) = select_statement.next() {
            let timestamp = select_statement.read::<i64, _>("date")?;
            let meal = select_statement.read::<String, _>("meal")?;
            records.push(MealRecord::from_meal_and_timestamp(&meal, timestamp)?);
        }

        Ok(records)
    }

    fn manipulate_records(
        &self,
        action_clause: &str,
        action_params: &[(&str, Value)],
        condition: &str,
        condition_params: &[(&str, Value)],
    ) -> Result<()> {
        let query = format!("{} {}", action_clause, condition);
        let mut statement = self.connection.prepare(query)?;
        for &(name, ref value) in action_params.into_iter().chain(condition_params) {
            statement.bind((name, value))?;
        }
        statement.next()?;
        Ok(())
    }

    fn sql_transaction<F>(&self, func: F) -> Result<Vec<MealRecord>>
    where
        F: Fn() -> Result<Vec<MealRecord>>,
    {
        self.connection.execute("BEGIN TRANSACTION")?;

        let records = func()?;

        self.connection.execute("COMMIT").map_err(|e| {
            let _ = self.connection.execute("ROLLBACK");
            e
        })?;
        Ok(records)
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
