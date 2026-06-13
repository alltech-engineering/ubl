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
    #[serde(default)]
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
    #[serde(default)]
    pub address_line: Vec<AddressLine>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Country {
    pub identification_code: Option<CountryCode>,
    pub name: Option<CountryName>,
}


#[cfg(test)]
mod tests {
    use super::*;

    fn empty_postal() -> PostalAddress {
        PostalAddress {
            street_name: None, city_name: None, postal_zone: None, country: None,
            id: None, address_format_code: None, address_type_code: None,
            block_name: None, building_name: None, building_number: None,
            city_subdivision_name: None, country_subentity: None, country_subentity_code: None,
            department: None, description: None, district: None,
            floor: None, inhouse_mail: None, mark_attention: None, mark_care: None,
            plot_identification: None, postbox: None, region: None, room: None,
            additional_street_name: None, timezone_offset: None, address_line: vec![],
        }
    }

    #[test]
    fn test_postal_address_roundtrip() {
        let mut addr = empty_postal();
        addr.street_name = Some(StreetName::new("123 Main St"));
        addr.city_name = Some(CityName::new("Cape Town"));
        addr.postal_zone = Some(PostalZone::new("8001"));
        addr.country = Some(Country {
            identification_code: Some(CountryCode::new("ZA")), name: None,
        });
        let json = serde_json::to_string(&addr).unwrap();
        let addr2: PostalAddress = serde_json::from_str(&json).unwrap();
        assert_eq!(addr.street_name.unwrap().0, addr2.street_name.unwrap().0);
    }

    #[test]
    fn test_country_roundtrip() {
        let c = Country { identification_code: Some(CountryCode::new("ZA")), name: None };
        let json = serde_json::to_string(&c).unwrap();
        let c2: Country = serde_json::from_str(&json).unwrap();
        assert_eq!(c.identification_code.unwrap().value(), c2.identification_code.unwrap().value());
    }
}
