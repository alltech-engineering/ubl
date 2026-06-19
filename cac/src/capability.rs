#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a specific capability of an organization.
///
/// UBL Dictionary Entry Name: `Capability. Details`
///
/// Generated from XSD type `CapabilityType`.
pub struct Capability {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this Capability.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// This class can be used as Financial or Technical capabilities. For instance, "Turnover" or
/// "Qualified Engineers" are two possible codes.
    #[serde(default, rename = "CapabilityTypeCode")]
    pub capability_type_code: Option<cct::Code>,
/// Text describing this capability.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// A monetary amount as a measure of this capability.
    #[serde(default, rename = "ValueAmount")]
    pub value_amount: Option<cct::Amount>,
/// A quantity as a measure of this capability.
    #[serde(default, rename = "ValueQuantity")]
    pub value_quantity: Option<cct::Quantity>,
/// (Deprecated) The evidence that supports the capability claim.
    #[serde(default, rename = "EvidenceSupplied")]
    pub evidence_supplied: Vec<EvidenceSupplied>,
/// The Evidence that supports the capability claim.
    #[serde(default, rename = "SuppliedEvidence")]
    pub supplied_evidence: Vec<Evidence>,
/// The period of time for which this capability is (or has been) valid.
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<Period>,
/// A web site where the capability is detailed.
    #[serde(default, rename = "WebSite")]
    pub web_site: Option<WebSite>,
}
