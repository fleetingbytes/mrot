use chrono::NaiveDate;

/// Container for a meal and a date on which it was recorded.
#[derive(Debug, Default)]
pub struct MealRecord {
    /// The meal.
    pub meal: String,
    /// The date on which this meal was recorded.
    pub date: NaiveDate,
}
