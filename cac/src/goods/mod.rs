use serde::{Deserialize, Serialize};


include!("processing.rs");
include!("item_container.rs");
include!("item.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsItemPassportCounterfoil {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "GoodsItemPassportID")]
    pub goods_item_passport_id: Option<cct::Identifier>,
    #[serde(default, rename = "FinalReexportationDate")]
    pub final_reexportation_date: Option<udt::DateTime>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "CustomsOfficeLocation")]
    pub customs_office_location: Option<crate::Location>,
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: Option<GoodsItem>,
    #[serde(default, rename = "ExportationDocumentReference")]
    pub exportation_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "ImportationDocumentReference")]
    pub importation_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "ReexportationDocumentReference")]
    pub reexportation_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "ReimportationDocumentReference")]
    pub reimportation_document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "VoucherDocumentReference")]
    pub voucher_document_reference: Vec<crate::DocumentReference>,
}
