#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsItemPassportCounterfoil {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "GoodsItemPassportID")]
    pub goods_item_passport_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "FinalReexportationDate")]
    pub final_reexportation_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<super::cct::TextType>,
    #[serde(default, rename = "CustomsOfficeLocation")]
    pub customs_office_location: Option<Location>,
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: Option<GoodsItem>,
    #[serde(default, rename = "ExportationDocumentReference")]
    pub exportation_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "ImportationDocumentReference")]
    pub importation_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "ReexportationDocumentReference")]
    pub reexportation_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "ReimportationDocumentReference")]
    pub reimportation_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "VoucherDocumentReference")]
    pub voucher_document_reference: Vec<DocumentReference>,
}
