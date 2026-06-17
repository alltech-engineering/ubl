// UBL Line Reference aggregates — references to lines in related documents.
// Used across despatch, receipt, work report, catalogue, and quotation modules.
//
// UBL elements: cac:DespatchLineReference, cac:ReceiptLineReference,
//   cac:WorkReportLineReference, cac:CatalogueLineReference, cac:QuotationLineReference

use crate::cac::document_reference::DocumentReference;
use crate::cbc::*;
use serde::{Deserialize, Serialize};

/// A reference to a Despatch Line.
/// UBL element: cac:DespatchLineReference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DespatchLineReference {
    pub line_id: LineID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_status_code: Option<LineStatusCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<DocumentReference>,
}

/// A reference to a Receipt Line.
/// UBL element: cac:ReceiptLineReference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReceiptLineReference {
    pub line_id: LineID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_status_code: Option<LineStatusCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<DocumentReference>,
}

/// A reference to a Work Report Line (from a Statement of Work report).
/// UBL element: cac:WorkReportLineReference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WorkReportLineReference {
    pub line_id: LineID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_status_code: Option<LineStatusCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<DocumentReference>,
}

/// A reference to a line in a Catalogue document.
/// UBL element: cac:CatalogueLineReference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CatalogueLineReference {
    pub line_id: LineID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_status_code: Option<LineStatusCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<DocumentReference>,
}

/// A reference to a line in a Quotation document.
/// UBL element: cac:QuotationLineReference
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuotationLineReference {
    pub line_id: LineID,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_status_code: Option<LineStatusCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub document_reference: Option<DocumentReference>,
}
