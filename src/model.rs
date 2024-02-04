use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ToDoModel {
    pub id: Uuid,
    pub nume_sarcina: Option<String>,
    pub notita_sarcina: Option<String>,
    pub ora_sarcina: Option<String>,
    pub data_sarcina: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    #[serde(rename = "updatedAt")]
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}
