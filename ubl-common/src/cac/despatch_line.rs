// DespatchLine — UBL CAC aggregate
// A line in a Despatch Advice document.
use crate::cbc::*;

#[derive(Debug, Clone, Partialserde::Serialize, serde::Deserialize)]
pub struct DespatchLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<Uuid>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub note: Vec<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_status_code: Option<LineStatusCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivered_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<Item>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub document_reference: Vec<DocumentReference>,
}
use super::document_reference::DocumentReference;
use super::item::Item;
