#[derive(Debug, Deserialize, Serialize)]
pub struct UnsignedDataObjectProperties {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "UnsignedDataObjectProperty")]
    pub unsigned_data_object_property: Vec<Any>,
}
