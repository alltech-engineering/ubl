#[derive(Debug, Deserialize, Serialize)]
pub struct IntegerListType {
    #[serde(default, rename = "int")]
    pub int: Vec<i32>,
}
