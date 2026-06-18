#[derive(Debug, Deserialize, Serialize)]
pub struct TradeFinancing {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "FinancingInstrumentCode")]
    pub financing_instrument_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: Option<DocumentReference>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(rename = "FinancingParty")]
    pub financing_party: Party,
    #[serde(default, rename = "FinancingFinancialAccount")]
    pub financing_financial_account: Option<FinancialAccount>,
    #[serde(default, rename = "Clause")]
    pub clause: Vec<Clause>,
}
