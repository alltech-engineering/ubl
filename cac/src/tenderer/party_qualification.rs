#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the qualifications of a tenderer party.
///
/// UBL Dictionary Entry Name: `Tenderer Party Qualification. Details`
///
/// Generated from XSD type `TendererPartyQualificationType`.
pub struct TendererPartyQualification {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The procurement project lot the party is interested in.
    #[serde(default, rename = "InterestedProcurementProjectLot")]
    pub interested_procurement_project_lot: Vec<crate::ProcurementProjectLot>,
/// The qualifications of the main tenderer party.
    #[serde(rename = "MainQualifyingParty")]
    pub main_qualifying_party: crate::QualifyingParty,
/// The qualifications of a tenderer party other than the main tenderer party when bidding as a
/// consortium.
    #[serde(default, rename = "AdditionalQualifyingParty")]
    pub additional_qualifying_party: Vec<crate::QualifyingParty>,
}
