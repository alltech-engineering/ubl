use serde::{Deserialize, Serialize};


include!("coordinate.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a location.
///
/// UBL Dictionary Entry Name: `Location. Details`
///
/// Generated from XSD type `LocationType`.
pub struct Location {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this location, e.g., the EAN Location Number, GLN.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// Text describing this location.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Free-form text describing the physical conditions of the location.
    #[serde(default, rename = "Conditions")]
    pub conditions: Vec<cct::Text>,
/// A territorial division of a country, such as a county or state, expressed as text.
    #[serde(default, rename = "CountrySubentity")]
    pub country_subentity: Option<cct::Text>,
/// A territorial division of a country, such as a county or state, expressed as a code.
    #[serde(default, rename = "CountrySubentityCode")]
    pub country_subentity_code: Option<cct::Code>,
/// A code signifying the type of location.
    #[serde(default, rename = "LocationTypeCode")]
    pub location_type_code: Option<cct::Code>,
/// The Uniform Resource Identifier (URI) of a document providing information about this location.
    #[serde(default, rename = "InformationURI")]
    pub information_uri: Option<cct::Identifier>,
/// The name of this location.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// A period during which this location can be used (e.g., for delivery).
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Vec<crate::Period>,
/// The address of this location.
    #[serde(default, rename = "Address")]
    pub address: Option<crate::Address>,
/// The description and requirements of the storage at this location.
    #[serde(default, rename = "Storage")]
    pub storage: Option<crate::Storage>,
/// A location subsidiary to this location.
    #[serde(default, rename = "SubsidiaryLocation")]
    pub subsidiary_location: Vec<Location>,
/// The geographical coordinates of this location.
    #[serde(default, rename = "LocationCoordinate")]
    pub location_coordinate: Vec<LocationCoordinate>,
}
