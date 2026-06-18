#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentDistribution {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "DocumentTypeCode")]
    pub document_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "DistributionTypeCode")]
    pub distribution_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "DistributionType")]
    pub distribution_type: Vec<super::cct::TextType>,
    #[serde(default, rename = "PrintQualifier")]
    pub print_qualifier: Option<super::cct::TextType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "MaximumCopiesNumeric")]
    pub maximum_copies_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "MaximumOriginalsNumeric")]
    pub maximum_originals_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "Communication")]
    pub communication: Option<Communication>,
    #[serde(rename = "Party")]
    pub party: Party,
}
