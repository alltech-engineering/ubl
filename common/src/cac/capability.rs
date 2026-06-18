#[derive(Debug, Deserialize, Serialize)]
pub struct Capability {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CapabilityTypeCode")]
    pub capability_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "ValueAmount")]
    pub value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "ValueQuantity")]
    pub value_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "EvidenceSupplied")]
    pub evidence_supplied: Vec<EvidenceSupplied>,
    #[serde(default, rename = "SuppliedEvidence")]
    pub supplied_evidence: Vec<Evidence>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<Period>,
    #[serde(default, rename = "WebSite")]
    pub web_site: Option<WebSite>,
}
