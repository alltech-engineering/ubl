#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a secondary hazard associated with a hazardous item.
///
/// UBL Dictionary Entry Name: `Secondary Hazard. Details`
///
/// Generated from XSD type `SecondaryHazardType`.
pub struct SecondaryHazard {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this secondary hazard.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// Text of the placard notation corresponding to the hazard class of this secondary hazard. Can also be
/// the hazard identification number of the orange placard (upper part) required on the means of
/// transport.
    #[serde(default, rename = "PlacardNotation")]
    pub placard_notation: Option<cct::Text>,
/// Text of the placard endorsement for this secondary hazard that is to be shown on the shipping papers
/// for a hazardous item. Can also be used for the number of the orange placard (lower part) required on
/// the means of transport.
    #[serde(default, rename = "PlacardEndorsement")]
    pub placard_endorsement: Option<cct::Text>,
/// A code signifying the emergency procedures for this secondary hazard.
    #[serde(default, rename = "EmergencyProceduresCode")]
    pub emergency_procedures_code: Option<cct::Code>,
/// Additional information about the hazardous substance, which can be used (for example) to specify the
/// type of regulatory requirements that apply to this secondary hazard.
    #[serde(default, rename = "Extension")]
    pub extension: Vec<cct::Text>,
}
