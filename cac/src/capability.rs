#[derive(Debug, Deserialize, Serialize)]
pub struct Capability {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "CapabilityTypeCode")]
    pub capability_type_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "ValueAmount")]
    pub value_amount: Option<cct::Amount>,
    #[serde(default, rename = "ValueQuantity")]
    pub value_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "EvidenceSupplied")]
    pub evidence_supplied: Vec<EvidenceSupplied>,
    #[serde(default, rename = "SuppliedEvidence")]
    pub supplied_evidence: Vec<Evidence>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<Period>,
    #[serde(default, rename = "WebSite")]
    pub web_site: Option<WebSite>,
}
