#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the awarding of a tender in a tendering process.
///
/// UBL Dictionary Entry Name: `Tender Result. Details`
///
/// Generated from XSD type `TenderResultType`.
pub struct TenderResult {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this tender result.
    #[serde(default, rename = "AwardID")]
    pub award_id: Option<cct::Identifier>,
/// A code signifying the result of the tendering process.
    #[serde(default, rename = "TenderResultCode")]
    pub tender_result_code: Option<cct::Code>,
/// Text describing the result of the tendering process.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The monetary value of the advertisement for this tendering process.
    #[serde(default, rename = "AdvertisementAmount")]
    pub advertisement_amount: Option<cct::Amount>,
/// The date on which this result was formalized.
    #[serde(rename = "AwardDate")]
    pub award_date: udt::DateTime,
/// The time at which this result was formalized.
    #[serde(default, rename = "AwardTime")]
    pub award_time: Option<udt::DateTime>,
/// The total number of tenders received in this tendering process.
    #[serde(default, rename = "ReceivedTenderQuantity")]
    pub received_tender_quantity: Option<cct::Quantity>,
/// The least expensive tender received in the tendering process.
    #[serde(default, rename = "LowerTenderAmount")]
    pub lower_tender_amount: Option<cct::Amount>,
/// The most expensive tender received in this tendering process.
    #[serde(default, rename = "HigherTenderAmount")]
    pub higher_tender_amount: Option<cct::Amount>,
/// The date on which the awarded contract begins.
    #[serde(default, rename = "StartDate")]
    pub start_date: Option<udt::DateTime>,
/// The number of electronic tenders received.
    #[serde(default, rename = "ReceivedElectronicTenderQuantity")]
    pub received_electronic_tender_quantity: Option<cct::Quantity>,
/// The number of foreing tenders received.
    #[serde(default, rename = "ReceivedForeignTenderQuantity")]
    pub received_foreign_tender_quantity: Option<cct::Quantity>,
/// A contract governing this tender result.
    #[serde(default, rename = "Contract")]
    pub contract: Option<crate::Contract>,
/// The awarded tendered project associated with this tender result.
    #[serde(default, rename = "AwardedTenderedProject")]
    pub awarded_tendered_project: Option<crate::TenderedProject>,
/// The period during which a contract associated with the awarded project is to be formalized.
    #[serde(default, rename = "ContractFormalizationPeriod")]
    pub contract_formalization_period: Option<crate::Period>,
/// Subcontract terms for this tender result.
    #[serde(default, rename = "SubcontractTerms")]
    pub subcontract_terms: Vec<crate::SubcontractTerms>,
/// A party that is identified as the awarded by a tender result.
    #[serde(default, rename = "WinningParty")]
    pub winning_party: Vec<crate::WinningParty>,
}
