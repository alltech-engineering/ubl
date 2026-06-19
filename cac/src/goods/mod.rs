use serde::{Deserialize, Serialize};


include!("processing.rs");
include!("item_container.rs");
include!("item.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class describing a Goods Item Passport or ATA Carnet Counterfoil
///
/// UBL Dictionary Entry Name: `Goods Item Passport Counterfoil. Details`
///
/// Generated from XSD type `GoodsItemPassportCounterfoilType`.
pub struct GoodsItemPassportCounterfoil {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// This identifier for this Goods Item Passport Counterfoil
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// The identifier of the Goods Item Passport or ATA Carnet of this counterfoil, usually the number on
/// the upper part of the orange hazard placard required on the means of transport
    #[serde(default, rename = "GoodsItemPassportID")]
    pub goods_item_passport_id: Option<cct::Identifier>,
/// Final date of re-exportation, if less than the overall validity period of te Goods Item Passport or
/// ATA Carnet
    #[serde(default, rename = "FinalReexportationDate")]
    pub final_reexportation_date: Option<udt::DateTime>,
/// Free-form text conveying information that is not contained explicitly in other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// The location of the customs office to where the counterfoil has been presented
    #[serde(default, rename = "CustomsOfficeLocation")]
    pub customs_office_location: Option<crate::Location>,
/// A goods item associated with this counterfoil
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: Option<GoodsItem>,
/// A reference to a document used for the export of the goods related to this counterfoil
    #[serde(default, rename = "ExportationDocumentReference")]
    pub exportation_document_reference: Vec<crate::DocumentReference>,
/// A reference to a document used for the import of the goods related to this counterfoil
    #[serde(default, rename = "ImportationDocumentReference")]
    pub importation_document_reference: Vec<crate::DocumentReference>,
/// A reference to a document used for the re-exportation of the goods related to this counterfoil
    #[serde(default, rename = "ReexportationDocumentReference")]
    pub reexportation_document_reference: Vec<crate::DocumentReference>,
/// A reference to a document used for re-importation of the goods related to this counterfoil
    #[serde(default, rename = "ReimportationDocumentReference")]
    pub reimportation_document_reference: Vec<crate::DocumentReference>,
/// A reference to a voucher related to this counterfoil
    #[serde(default, rename = "VoucherDocumentReference")]
    pub voucher_document_reference: Vec<crate::DocumentReference>,
}
