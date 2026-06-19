#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a trade financing instrument.
///
/// UBL Dictionary Entry Name: `Trade Financing. Details`
///
/// Generated from XSD type `TradeFinancingType`.
pub struct TradeFinancing {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this trade financing instrument.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A code signifying the type of this financing instrument.
    #[serde(default, rename = "FinancingInstrumentCode")]
    pub financing_instrument_code: Option<cct::Code>,
/// A reference to a contract document.
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: Option<DocumentReference>,
/// A reference to a document associated with this trade financing instrument.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
/// A Party that provides funds or credit to support the trade transaction described in this Trade
/// Financing instrument.
    #[serde(rename = "FinancingParty")]
    pub financing_party: Party,
/// An internal bank account used by the bank or its first agent to manage the line of credit granted to
/// the financing requester.
    #[serde(default, rename = "FinancingFinancialAccount")]
    pub financing_financial_account: Option<FinancialAccount>,
/// A clause applicable to this trade financing instrument.
    #[serde(default, rename = "Clause")]
    pub clause: Vec<Clause>,
}
