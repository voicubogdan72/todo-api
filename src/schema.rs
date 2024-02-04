use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptins {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateToDoSchema {
    pub nume_sarcina: String,
    pub notita_sarcina: String,
    pub ora_sarcina: String,
    pub data_sarcina: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateToDoSchmea {
    pub nume_sarcina: Option<String>,
    pub notita_sarcina: Option<String>,
    pub ora_sarcina: Option<String>,
    pub data_sarcina: Option<String>,
}
