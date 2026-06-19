#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a scheme for corporate registration.
///
/// UBL Dictionary Entry Name: `Corporate Registration Scheme. Details`
///
/// Generated from XSD type `CorporateRegistrationSchemeType`.
pub struct CorporateRegistrationScheme {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this registration scheme.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The name of this registration scheme.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// A code signifying the type of this registration scheme.
    #[serde(default, rename = "CorporateRegistrationTypeCode")]
    pub corporate_registration_type_code: Option<cct::Code>,
/// A geographic area in which this registration scheme applies.
    #[serde(default, rename = "JurisdictionRegionAddress")]
    pub jurisdiction_region_address: Vec<Address>,
}
