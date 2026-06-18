#[derive(Debug, Deserialize, Serialize)]
pub struct TendererPartyQualification {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "InterestedProcurementProjectLot")]
    pub interested_procurement_project_lot: Vec<ProcurementProjectLot>,
    #[serde(rename = "MainQualifyingParty")]
    pub main_qualifying_party: QualifyingParty,
    #[serde(default, rename = "AdditionalQualifyingParty")]
    pub additional_qualifying_party: Vec<QualifyingParty>,
}
