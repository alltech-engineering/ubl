#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a response to a criterion property.
///
/// UBL Dictionary Entry Name: `Tendering Criterion Response. Details`
///
/// Generated from XSD type `TenderingCriterionResponseType`.
pub struct TenderingCriterionResponse {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this criterion property response.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The name of the criterion property response
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// A description of the criterion response
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// An identifier for this item of criterion support.
    #[serde(default, rename = "ValidatedCriterionPropertyID")]
    pub validated_criterion_property_id: Option<cct::Identifier>,
/// A code specifying the confidentiality level of the response to this criterion.
    #[serde(default, rename = "ConfidentialityLevelCode")]
    pub confidentiality_level_code: Option<cct::Code>,
/// The criterion requirement property values.
    #[serde(default, rename = "ResponseValue")]
    pub response_value: Vec<crate::ResponseValue>,
/// The period to which this criterion property response applies.
    #[serde(default, rename = "ApplicablePeriod")]
    pub applicable_period: Vec<crate::Period>,
/// (Deprecated) A reference to the evidence supporting this criterion property response.
    #[serde(default, rename = "EvidenceSupplied")]
    pub evidence_supplied: Vec<crate::EvidenceSupplied>,
/// A reference to the Evidence supporting this criterion property response.
    #[serde(default, rename = "SuppliedEvidence")]
    pub supplied_evidence: Vec<crate::Evidence>,
/// One or more lots to which the criterion response applies
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: Vec<crate::ProcurementProjectLotReference>,
/// One or more classification to which this criterion response applies
    #[serde(default, rename = "CommodityClassification")]
    pub commodity_classification: Vec<crate::CommodityClassification>,
}
