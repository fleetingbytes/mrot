//! Implementation of [Storage][storage-trait] as a Sqlite Database.
//!
//! [storage-trait]: storage::Storage

use storage::Storage;
use two_timer::parse;
use chrono::naive::NaiveDate;

const QUERY_TO_CREATE_SQL_STORAGE_TABLES: &str = "CREATE TABLE meals (date INTEGER, meal TEXT);";

pub struct SqliteStorage {
    connection: Connection,
}

impl Storage for SqliteStorage {
    /// Opens a storage.
    #[instrument]
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let connection: Connection = match path {
            ":memory:" => sqlite::open(path)?,
            _ => {
                if path.try_exists()? {
                    sqlite::open(path)?
                } else {
                    Self::new(path)?
                }
            }
        };
        Ok(Self { connection })
    }

    /// Creates a new storage.
    #[instrument]
    fn new<P: AsRef<Path>>(path: P) -> Result<Connection> {
        std::fs::create_dir_all(path.as_ref().parent().ok_or(Error::NoParentDirectory)?)?;
        let connection = sqlite::open(path)?;
        connection.execute(QUERY_TO_CREATE_SQL_STRORAGE_TABLES)?;
        Ok(connection)
    }

    /// Adds a meal on the given dates to the storage.
    #[instrument]
    pub fn add_meal_on_dates(&self, meal: &str, dates: &Vec<String>) -> Result<()> {
        let converted_dates = Self::convert_to_timestamps(dates)?;

        let converted_date = Self::convert_date_to_timestamp(date)?;
        let query = "
            INSERT INTO meals VALUES (:date, :meal);
        ";
        let mut statement = self.connection.prepare(query)?;
        statement.bind_iter::<_, (_, Value)>([
            (":date", converted_date.into()),
            (":meal", meal.into()),
        ])?;
        while let Ok(State::Row) = statement.next() {
            return Err(Error::Storage);
        }
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

impl fmt::Debug for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Storage(chg: {})", self.connection.total_change_count())
    }
}
