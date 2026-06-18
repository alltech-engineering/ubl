#[derive(Debug, Deserialize, Serialize)]
pub struct Address {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AddressTypeCode")]
    pub address_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "AddressFormatCode")]
    pub address_format_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Postbox")]
    pub postbox: Option<super::cct::TextType>,
    #[serde(default, rename = "Floor")]
    pub floor: Option<super::cct::TextType>,
    #[serde(default, rename = "Room")]
    pub room: Option<super::cct::TextType>,
    #[serde(default, rename = "StreetName")]
    pub street_name: Vec<super::cct::TextType>,
    #[serde(default, rename = "AdditionalStreetName")]
    pub additional_street_name: Vec<super::cct::TextType>,
    #[serde(default, rename = "BlockName")]
    pub block_name: Option<super::cct::TextType>,
    #[serde(default, rename = "BuildingName")]
    pub building_name: Option<super::cct::TextType>,
    #[serde(default, rename = "BuildingNumber")]
    pub building_number: Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "InhouseMail")]
    pub inhouse_mail: Option<super::cct::TextType>,
    #[serde(default, rename = "Department")]
    pub department: Option<super::cct::TextType>,
    #[serde(default, rename = "MarkAttention")]
    pub mark_attention: Option<super::cct::TextType>,
    #[serde(default, rename = "MarkCare")]
    pub mark_care: Option<super::cct::TextType>,
    #[serde(default, rename = "PlotIdentification")]
    pub plot_identification: Option<super::cct::TextType>,
    #[serde(default, rename = "CitySubdivisionName")]
    pub city_subdivision_name: Option<super::cct::TextType>,
    #[serde(default, rename = "CityName")]
    pub city_name: Vec<super::cct::TextType>,
    #[serde(default, rename = "PostalZone")]
    pub postal_zone: Vec<super::cct::TextType>,
    #[serde(default, rename = "CountrySubentity")]
    pub country_subentity: Vec<super::cct::TextType>,
    #[serde(default, rename = "CountrySubentityCode")]
    pub country_subentity_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Region")]
    pub region: Option<super::cct::TextType>,
    #[serde(default, rename = "District")]
    pub district: Option<super::cct::TextType>,
    #[serde(default, rename = "TimezoneOffset")]
    pub timezone_offset: Option<super::cct::TextType>,
    #[serde(default, rename = "AddressLine")]
    pub address_line: Vec<AddressLine>,
    #[serde(default, rename = "Country")]
    pub country: Option<Country>,
    #[serde(default, rename = "LocationCoordinate")]
    pub location_coordinate: Vec<LocationCoordinate>,
}
