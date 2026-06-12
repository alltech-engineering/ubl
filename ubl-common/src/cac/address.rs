// UBL Address aggregates — physical, postal, and address lines.

use serde::{Deserialize, Serialize};
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub id: Option<ID>,
    pub address_format_code: Option<AddressFormatCode>,
    pub address_type_code: Option<AddressTypeCode>,
    pub block_name: Option<BlockName>,
    pub building_name: Option<BuildingName>,
    pub building_number: Option<BuildingNumber>,
    pub city_name: Option<CityName>,
    pub city_subdivision_name: Option<CitySubdivisionName>,
    pub country_subentity: Option<CountrySubentity>,
    pub country_subentity_code: Option<CountrySubentityCode>,
    pub department: Option<Department>,
    pub description: Option<Description>,
    pub district: Option<District>,
    pub floor: Option<Floor>,
    pub inhouse_mail: Option<InhouseMail>,
    pub mark_attention: Option<MarkAttention>,
    pub mark_care: Option<MarkCare>,
    pub plot_identification: Option<PlotIdentification>,
    pub postal_zone: Option<PostalZone>,
    pub postbox: Option<Postbox>,
    pub region: Option<Region>,
    pub room: Option<Room>,
    pub street_name: Option<StreetName>,
    pub additional_street_name: Option<AdditionalStreetName>,
    pub country: Option<Country>,
    pub timezone_offset: Option<TimezoneOffset>,
    pub address_line: Vec<AddressLine>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AddressLine {
    pub line: Text,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostalAddress {
    pub id: Option<ID>,
    pub address_format_code: Option<AddressFormatCode>,
    pub address_type_code: Option<AddressTypeCode>,
    pub block_name: Option<BlockName>,
    pub building_name: Option<BuildingName>,
    pub building_number: Option<BuildingNumber>,
    pub city_name: Option<CityName>,
    pub city_subdivision_name: Option<CitySubdivisionName>,
    pub country_subentity: Option<CountrySubentity>,
    pub country_subentity_code: Option<CountrySubentityCode>,
    pub department: Option<Department>,
    pub description: Option<Description>,
    pub district: Option<District>,
    pub floor: Option<Floor>,
    pub inhouse_mail: Option<InhouseMail>,
    pub mark_attention: Option<MarkAttention>,
    pub mark_care: Option<MarkCare>,
    pub plot_identification: Option<PlotIdentification>,
    pub postal_zone: Option<PostalZone>,
    pub postbox: Option<Postbox>,
    pub region: Option<Region>,
    pub room: Option<Room>,
    pub street_name: Option<StreetName>,
    pub additional_street_name: Option<AdditionalStreetName>,
    pub country: Option<Country>,
    pub timezone_offset: Option<TimezoneOffset>,
    pub address_line: Vec<AddressLine>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Country {
    pub identification_code: Option<CountryCode>,
    pub name: Option<CountryName>,
}
