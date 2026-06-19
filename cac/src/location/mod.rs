use serde::{Deserialize, Serialize};


include!("coordinate.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "Conditions")]
    pub conditions: Vec<cct::Text>,
    #[serde(default, rename = "CountrySubentity")]
    pub country_subentity: Option<cct::Text>,
    #[serde(default, rename = "CountrySubentityCode")]
    pub country_subentity_code: Option<cct::Code>,
    #[serde(default, rename = "LocationTypeCode")]
    pub location_type_code: Option<cct::Code>,
    #[serde(default, rename = "InformationURI")]
    pub information_uri: Option<cct::Identifier>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<crate::Period>,
    #[serde(default, rename = "Address")]
    pub address: Option<crate::Address>,
    #[serde(default, rename = "Storage")]
    pub storage: Option<crate::Storage>,
    #[serde(default, rename = "SubsidiaryLocation")]
    pub subsidiary_location: Vec<Location>,
    #[serde(default, rename = "LocationCoordinate")]
    pub location_coordinate: Vec<LocationCoordinate>,
}
