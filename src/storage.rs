//! Data Storage Abstraction

use crate::error::Error;
use chrono::NaiveDateTime;
use directories::ProjectDirs;
use sqlite::{Connection, State, Value};
use std::fmt;
use std::path::PathBuf;

pub struct Storage {
    connection: Connection,
}

const FILE: &str = "database.sql";
const DATE_FORMAT: &str = "%Y-%m-%d";

impl Storage {
    fn new(pathbuf: &PathBuf) -> Result<Connection, Error> {
        std::fs::create_dir_all(pathbuf.parent().ok_or(Error::NoParentDirectory)?)?;
        let connection = sqlite::open(pathbuf)?;
        let query = "
            CREATE TABLE meals (date TEXT, meal TEXT, plan BOOLEAN);
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
    pub fn add_meal(&self, date: NaiveDateTime, meal: &str, plan: bool) -> Result<(), Error> {
        let query = "
            INSERT INTO meals VALUES (:date, :meal, :plan);
        ";
        let mut statement = self.connection.prepare(query)?;
        statement.bind_iter::<_, (_, Value)>([
            (":date", date.format(DATE_FORMAT).to_string().into()),
            (":meal", meal.into()),
            (":plan", (plan as i64).into()),
        ])?;
        while let Ok(State::Row) = statement.next() {
            return Err(Error::Storage);
        }
        Ok(())
    }
}

impl fmt::Debug for Storage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "Storage(chg: {})", self.connection.total_change_count())
    }
}
