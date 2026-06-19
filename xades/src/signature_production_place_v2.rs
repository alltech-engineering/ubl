#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureProductionPlaceV2 {
    #[serde(default, rename = "City")]
    pub city: Option<String>,
    #[serde(default, rename = "StreetAddress")]
    pub street_address: Option<String>,
    #[serde(default, rename = "StateOrProvince")]
    pub state_or_province: Option<String>,
    #[serde(default, rename = "PostalCode")]
    pub postal_code: Option<String>,
    #[serde(default, rename = "CountryName")]
    pub country_name: Option<String>,
}
