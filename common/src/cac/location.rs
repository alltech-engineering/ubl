#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "Conditions")]
    pub conditions: Vec<super::cct::TextType>,
    #[serde(default, rename = "CountrySubentity")]
    pub country_subentity: Option<super::cct::TextType>,
    #[serde(default, rename = "CountrySubentityCode")]
    pub country_subentity_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "LocationTypeCode")]
    pub location_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "InformationURI")]
    pub information_uri: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<Period>,
    #[serde(default, rename = "Address")]
    pub address: Option<Address>,
    #[serde(default, rename = "Storage")]
    pub storage: Option<Storage>,
    #[serde(default, rename = "SubsidiaryLocation")]
    pub subsidiary_location: Vec<Location>,
    #[serde(default, rename = "LocationCoordinate")]
    pub location_coordinate: Vec<LocationCoordinate>,
}
