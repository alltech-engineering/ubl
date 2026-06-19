#[derive(Debug, Deserialize, Serialize)]
pub struct CorporateRegistrationScheme {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "CorporateRegistrationTypeCode")]
    pub corporate_registration_type_code: Option<cct::Code>,
    #[serde(default, rename = "JurisdictionRegionAddress")]
    pub jurisdiction_region_address: Vec<Address>,
}
