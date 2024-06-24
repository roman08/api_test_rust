use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub birth_date: NaiveDate,
    pub custom_data: CustomData,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(name: String, birth_date_ymd: (i32, u32, u32)) -> Self {
        let (year, month, day) = birth_date_ymd;
        let id = uuid::Uuid::parse_str("f764e106-3337-4731-bc19-72fa748add5d").unwrap();
        Self {
            id,
            name,
            birth_date: NaiveDate::from_ymd_opt(year, month, day).expect("Invalid date"), // Usar expect
            custom_data: CustomData { random: 1 },
            created_at: Some(Utc::now()),
            updated_at: None,
        }
    }
}
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CustomData {
    pub random: u32,
}
