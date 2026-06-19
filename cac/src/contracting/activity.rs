#[derive(Debug, Deserialize, Serialize)]
/// The nature of the type of business of the organization.
///
/// UBL Dictionary Entry Name: `Contracting Activity. Details`
///
/// Generated from XSD type `ContractingActivityType`.
pub struct ContractingActivity {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// A code specifying the nature of the type of business of the organization.
    #[serde(default, rename = "ActivityTypeCode")]
    pub activity_type_code: Option<cct::Code>,
/// The nature of the type of business of the organization, expressed as text.
    #[serde(default, rename = "ActivityType")]
    pub activity_type: Vec<cct::Text>,
}
