use chrono::NaiveDate;

#[derive(Debug)]
pub struct Meal {
    pub name: String,
    pub date: NaiveDate,
}
