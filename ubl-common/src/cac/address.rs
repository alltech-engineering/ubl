// UBL Address aggregates — physical, postal, and address lines.

use serde::{Deserialize, Serialize};
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Address {
    pub id: Option<ID>,
    pub address_format_code: Option<AddressFormatCode>,
    pub address_type_code: Option<AddressTypeCode>,
    pub postbox: Option<Postbox>,
    pub floor: Option<Floor>,
    pub room: Option<Room>,
    pub street_name: Option<StreetName>,
    pub additional_street_name: Option<AdditionalStreetName>,
    pub building_name: Option<BuildingName>,
    pub building_number: Option<BuildingNumber>,
    pub department: Option<Department>,
    pub city_name: Option<CityName>,
    pub postal_zone: Option<PostalZone>,
    pub country_subentity: Option<CountrySubentity>,
    pub country_subentity_code: Option<CountrySubentityCode>,
    pub country: Option<Country>,
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
    pub postbox: Option<Postbox>,
    pub floor: Option<Floor>,
    pub room: Option<Room>,
    pub street_name: Option<StreetName>,
    pub additional_street_name: Option<AdditionalStreetName>,
    pub building_name: Option<BuildingName>,
    pub building_number: Option<BuildingNumber>,
    pub department: Option<Department>,
    pub city_name: Option<CityName>,
    pub postal_zone: Option<PostalZone>,
    pub country_subentity: Option<CountrySubentity>,
    pub country_subentity_code: Option<CountrySubentityCode>,
    pub country: Option<Country>,
    pub address_line: Vec<AddressLine>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Country {
    pub identification_code: Option<CountryCode>,
    pub name: Option<CountryName>,
}
