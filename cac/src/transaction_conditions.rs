#[derive(Debug, Deserialize, Serialize)]
/// A class to describe purchasing, sales, or payment conditions.
///
/// UBL Dictionary Entry Name: `Transaction Conditions. Details`
///
/// Generated from XSD type `TransactionConditionsType`.
pub struct TransactionConditions {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for conditions of the transaction, typically purchase/sales conditions.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A code signifying a type of action relating to sales or payment conditions.
    #[serde(default, rename = "ActionCode")]
    pub action_code: Option<cct::Code>,
/// Text describing the transaction conditions.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// A document associated with these transaction conditions.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
}
