use serde::{Deserialize, Serialize};

include!("line.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to define common information related to an address.
///
/// UBL Dictionary Entry Name: `Address. Details`
///
/// Generated from XSD type `AddressType`.
pub struct Address {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this address within an agreed scheme of address identifiers.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A mutually agreed code signifying the type of this address.
    #[serde(default, rename = "AddressTypeCode")]
    pub address_type_code: Option<cct::Code>,
/// A mutually agreed code signifying the format of this address.
    #[serde(default, rename = "AddressFormatCode")]
    pub address_format_code: Option<cct::Code>,
/// A post office box number registered for postal delivery by a postal service provider.
    #[serde(default, rename = "Postbox")]
    pub postbox: Option<cct::Text>,
/// An identifiable floor of a building.
    #[serde(default, rename = "Floor")]
    pub floor: Option<cct::Text>,
/// An identifiable room, suite, or apartment of a building.
    #[serde(default, rename = "Room")]
    pub room: Option<cct::Text>,
/// The name of the street, road, avenue, way, etc. to which the number of the building is attached (may
/// be repeated only to provide the same content in multiple natural languages).
    #[serde(default, rename = "StreetName")]
    pub street_name: Vec<cct::Text>,
/// An additional street name used to further clarify the address (may be repeated only to provide the
/// same content in multiple natural languages).
    #[serde(default, rename = "AdditionalStreetName")]
    pub additional_street_name: Vec<cct::Text>,
/// The name of the block (an area surrounded by streets and usually containing several buildings) in
/// which this address is located.
    #[serde(default, rename = "BlockName")]
    pub block_name: Option<cct::Text>,
/// The name of a building.
    #[serde(default, rename = "BuildingName")]
    pub building_name: Option<cct::Text>,
/// The number of a building within the street.
    #[serde(default, rename = "BuildingNumber")]
    pub building_number: Option<cct::Text>,
/// Text describing this address for clarification or specificity
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The specific identifable location within a building where mail is delivered.
    #[serde(default, rename = "InhouseMail")]
    pub inhouse_mail: Option<cct::Text>,
/// The department of the addressee.
    #[serde(default, rename = "Department")]
    pub department: Option<cct::Text>,
/// The name, expressed as text, of a person or department in an organization to whose attention
/// incoming mail is directed; corresponds to the printed forms "for the attention of", "FAO", and
/// ATTN:".
    #[serde(default, rename = "MarkAttention")]
    pub mark_attention: Option<cct::Text>,
/// The name, expressed as text, of a person or organization at this address into whose care incoming
/// mail is entrusted; corresponds to the printed forms "care of" and "c/o".
    #[serde(default, rename = "MarkCare")]
    pub mark_care: Option<cct::Text>,
/// An identifier (e.g., a parcel number) for the piece of land associated with this address.
    #[serde(default, rename = "PlotIdentification")]
    pub plot_identification: Option<cct::Text>,
/// The name of the subdivision of a city, town, or village in which this address is located, such as
/// the name of its district or borough.
    #[serde(default, rename = "CitySubdivisionName")]
    pub city_subdivision_name: Option<cct::Text>,
/// The name of a city, town, or village (may be repeated only to provide the same content in multiple
/// natural languages).
    #[serde(default, rename = "CityName")]
    pub city_name: Vec<cct::Text>,
/// The postal identifier for this address according to the relevant national postal service, such as a
/// ZIP code or Post Code (may be repeated only to provide the same content in multiple natural
/// languages).
    #[serde(default, rename = "PostalZone")]
    pub postal_zone: Vec<cct::Text>,
/// The political or administrative division of a country in which this address is located, such as the
/// name of its county, province, or state, expressed as text (may be repeated only to provide the same
/// content in multiple natural languages).
    #[serde(default, rename = "CountrySubentity")]
    pub country_subentity: Vec<cct::Text>,
/// The political or administrative division of a country in which this address is located, such as a
/// county, province, or state, expressed as a code (typically nationally agreed).
    #[serde(default, rename = "CountrySubentityCode")]
    pub country_subentity_code: Option<cct::Code>,
/// The recognized geographic or economic region or group of countries in which this address is located.
    #[serde(default, rename = "Region")]
    pub region: Option<cct::Text>,
/// The district or geographical division of a country or region in which this address is located.
    #[serde(default, rename = "District")]
    pub district: Option<cct::Text>,
/// The time zone in which this address is located (as an offset from Universal Coordinated Time (UTC))
/// at the time of exchange.
    #[serde(default, rename = "TimezoneOffset")]
    pub timezone_offset: Option<cct::Text>,
/// A single address line expressed as unstructured text.
    #[serde(default, rename = "AddressLine")]
    pub address_line: Vec<AddressLine>,
/// The country in which this address is situated.
    #[serde(default, rename = "Country")]
    pub country: Option<crate::Country>,
/// The geographical coordinates of this address.
    #[serde(default, rename = "LocationCoordinate")]
    pub location_coordinate: Vec<crate::LocationCoordinate>,
}
