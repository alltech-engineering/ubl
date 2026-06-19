#[derive(Debug, Deserialize, Serialize)]
pub struct TradeFinancing {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "FinancingInstrumentCode")]
    pub financing_instrument_code: Option<cct::Code>,
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
