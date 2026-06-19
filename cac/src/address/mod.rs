use serde::{Deserialize, Serialize};

include!("line.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "AddressTypeCode")]
    pub address_type_code: Option<cct::Code>,
    #[serde(default, rename = "AddressFormatCode")]
    pub address_format_code: Option<cct::Code>,
    #[serde(default, rename = "Postbox")]
    pub postbox: Option<cct::Text>,
    #[serde(default, rename = "Floor")]
    pub floor: Option<cct::Text>,
    #[serde(default, rename = "Room")]
    pub room: Option<cct::Text>,
    #[serde(default, rename = "StreetName")]
    pub street_name: Vec<cct::Text>,
    #[serde(default, rename = "AdditionalStreetName")]
    pub additional_street_name: Vec<cct::Text>,
    #[serde(default, rename = "BlockName")]
    pub block_name: Option<cct::Text>,
    #[serde(default, rename = "BuildingName")]
    pub building_name: Option<cct::Text>,
    #[serde(default, rename = "BuildingNumber")]
    pub building_number: Option<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "InhouseMail")]
    pub inhouse_mail: Option<cct::Text>,
    #[serde(default, rename = "Department")]
    pub department: Option<cct::Text>,
    #[serde(default, rename = "MarkAttention")]
    pub mark_attention: Option<cct::Text>,
    #[serde(default, rename = "MarkCare")]
    pub mark_care: Option<cct::Text>,
    #[serde(default, rename = "PlotIdentification")]
    pub plot_identification: Option<cct::Text>,
    #[serde(default, rename = "CitySubdivisionName")]
    pub city_subdivision_name: Option<cct::Text>,
    #[serde(default, rename = "CityName")]
    pub city_name: Vec<cct::Text>,
    #[serde(default, rename = "PostalZone")]
    pub postal_zone: Vec<cct::Text>,
    #[serde(default, rename = "CountrySubentity")]
    pub country_subentity: Vec<cct::Text>,
    #[serde(default, rename = "CountrySubentityCode")]
    pub country_subentity_code: Option<cct::Code>,
    #[serde(default, rename = "Region")]
    pub region: Option<cct::Text>,
    #[serde(default, rename = "District")]
    pub district: Option<cct::Text>,
    #[serde(default, rename = "TimezoneOffset")]
    pub timezone_offset: Option<cct::Text>,
    #[serde(default, rename = "AddressLine")]
    pub address_line: Vec<AddressLine>,
    #[serde(default, rename = "Country")]
    pub country: Option<crate::Country>,
    #[serde(default, rename = "LocationCoordinate")]
    pub location_coordinate: Vec<crate::LocationCoordinate>,
}
