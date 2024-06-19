use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustSingleCycle {
    pub release_date: Option<String>,
    pub eol: DateOrBool,
    pub latest: Option<String>,
    pub latest_release_date: Option<String>,
    pub lts: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(untagged)]  // Atributo adecuado para la enumeración
pub enum DateOrBool {
    Bool(bool),
    Date(String),
}
