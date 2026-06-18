#[derive(Debug, Deserialize, Serialize)]
pub struct TenderResult {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "AwardID")]
    pub award_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TenderResultCode")]
    pub tender_result_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "AdvertisementAmount")]
    pub advertisement_amount: Option<super::cct::AmountType>,
    #[serde(rename = "AwardDate")]
    pub award_date: super::udt::DateTimeType,
    #[serde(default, rename = "AwardTime")]
    pub award_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ReceivedTenderQuantity")]
    pub received_tender_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "LowerTenderAmount")]
    pub lower_tender_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "HigherTenderAmount")]
    pub higher_tender_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "StartDate")]
    pub start_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ReceivedElectronicTenderQuantity")]
    pub received_electronic_tender_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "ReceivedForeignTenderQuantity")]
    pub received_foreign_tender_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "Contract")]
    pub contract: Option<Contract>,
    #[serde(default, rename = "AwardedTenderedProject")]
    pub awarded_tendered_project: Option<TenderedProject>,
    #[serde(default, rename = "ContractFormalizationPeriod")]
    pub contract_formalization_period: Option<Period>,
    #[serde(default, rename = "SubcontractTerms")]
    pub subcontract_terms: Vec<SubcontractTerms>,
    #[serde(default, rename = "WinningParty")]
    pub winning_party: Vec<WinningParty>,
}
