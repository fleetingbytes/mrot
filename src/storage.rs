//! Data Storage Abstraction

use crate::error::Error;
use crate::meal::Meal;
use chrono::NaiveDate;
use directories::ProjectDirs;
use sqlite::{Connection, State, Value};
use std::fmt;
use std::path::PathBuf;

pub struct Storage {
    connection: Connection,
}

const FILE: &str = "database.sql";

impl Storage {
    fn new(pathbuf: &PathBuf) -> Result<Connection, Error> {
        std::fs::create_dir_all(pathbuf.parent().ok_or(Error::NoParentDirectory)?)?;
        let connection = sqlite::open(pathbuf)?;
        let query = "
            CREATE TABLE meals (date INTEGER, meal TEXT);
        ";
        connection.execute(query)?;
        Ok(connection)
    }
    pub fn open() -> Result<Self, Error> {
        let dirs = ProjectDirs::from("", "", env!("CARGO_PKG_NAME"))
            .ok_or(Error::NoDirectory("ProjectDirs".to_string()))?;
        let file_path = dirs.data_dir().join(FILE);
        let connection: Connection;
        if file_path.try_exists()? {
            connection = sqlite::open(file_path)?;
        } else {
            connection = Storage::new(&file_path)?;
        }
        Ok(Self { connection })
    }
    pub fn add_meal(&self, date: NaiveDate, meal: &str) -> Result<(), Error> {
        let query = "
            INSERT INTO meals VALUES (:date, :meal);
        ";
        let mut statement = self.connection.prepare(query)?;
        statement.bind_iter::<_, (_, Value)>([
            (
                ":date",
                date.and_hms_opt(0, 0, 0)
                    .ok_or(Error::TimeNotSupported)?
                    .timestamp()
                    .into(),
            ),
            (":meal", meal.into()),
        ])?;
        while let Ok(State::Row) = statement.next() {
            return Err(Error::Storage);
        }
        Ok(())
    }
    pub fn meals_between_dates(
        &self,
        start: NaiveDate,
        end: NaiveDate,
    ) -> Result<Vec<Meal>, Error> {
        let query = "SELECT date, meal FROM meals WHERE date BETWEEN :start AND :end";
        let mut result: Vec<Meal> = Vec::new();
        for row in self
            .connection
            .prepare(query)?
            .into_iter()
            .bind_iter::<_, (_, Value)>([
                (
                    ":start",
                    start
                        .and_hms_opt(0, 0, 0)
                        .ok_or(Error::TimeNotSupported)?
                        .timestamp()
                        .into(),
                ),
                (
                    ":end",
                    end.and_hms_opt(0, 0, 0)
                        .ok_or(Error::TimeNotSupported)?
                        .timestamp()
                        .into(),
                ),
            ])?
            .map(|row| row.expect("Storage returned a malformed row"))
        {
            println!("name = {}", row.read::<&str, _>("meal"));
            println!("date = {}", row.read::<i64, _>("date"));
        }
        result.push(Meal {
            name: "fish".to_string(),
            date: NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        });
        Ok(result)
    }
}

impl fmt::Debug for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Storage(chg: {})", self.connection.total_change_count())
    }
}
