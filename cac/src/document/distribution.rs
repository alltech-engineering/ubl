#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentDistribution {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "DocumentTypeCode")]
    pub document_type_code: Option<cct::Code>,
    #[serde(default, rename = "DistributionTypeCode")]
    pub distribution_type_code: Option<cct::Code>,
    #[serde(default, rename = "DistributionType")]
    pub distribution_type: Vec<cct::Text>,
    #[serde(default, rename = "PrintQualifier")]
    pub print_qualifier: Option<cct::Text>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "MaximumCopiesNumeric")]
    pub maximum_copies_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "MaximumOriginalsNumeric")]
    pub maximum_originals_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "Communication")]
    pub communication: Option<crate::Communication>,
    #[serde(rename = "Party")]
    pub party: crate::Party,
}
