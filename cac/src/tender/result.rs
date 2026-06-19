#[derive(Debug, Deserialize, Serialize)]
pub struct TenderResult {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "AwardID")]
    pub award_id: Option<cct::Identifier>,
    #[serde(default, rename = "TenderResultCode")]
    pub tender_result_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "AdvertisementAmount")]
    pub advertisement_amount: Option<cct::Amount>,
    #[serde(rename = "AwardDate")]
    pub award_date: udt::DateTime,
    #[serde(default, rename = "AwardTime")]
    pub award_time: Option<udt::DateTime>,
    #[serde(default, rename = "ReceivedTenderQuantity")]
    pub received_tender_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "LowerTenderAmount")]
    pub lower_tender_amount: Option<cct::Amount>,
    #[serde(default, rename = "HigherTenderAmount")]
    pub higher_tender_amount: Option<cct::Amount>,
    #[serde(default, rename = "StartDate")]
    pub start_date: Option<udt::DateTime>,
    #[serde(default, rename = "ReceivedElectronicTenderQuantity")]
    pub received_electronic_tender_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "ReceivedForeignTenderQuantity")]
    pub received_foreign_tender_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "Contract")]
    pub contract: Option<crate::Contract>,
    #[serde(default, rename = "AwardedTenderedProject")]
    pub awarded_tendered_project: Option<crate::TenderedProject>,
    #[serde(default, rename = "ContractFormalizationPeriod")]
    pub contract_formalization_period: Option<crate::Period>,
    #[serde(default, rename = "SubcontractTerms")]
    pub subcontract_terms: Vec<crate::SubcontractTerms>,
    #[serde(default, rename = "WinningParty")]
    pub winning_party: Vec<crate::WinningParty>,
}
