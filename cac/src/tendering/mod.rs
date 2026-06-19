use serde::{Deserialize, Serialize};


include!("terms.rs");
include!("criterion_response.rs");
include!("process.rs");
include!("criterion_property_group.rs");
include!("criterion.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct TenderingCriterionProperty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<cct::Code>,
    #[serde(default, rename = "ValueDataTypeCode")]
    pub value_data_type_code: Option<cct::Code>,
    #[serde(default, rename = "ValueUnitCode")]
    pub value_unit_code: Option<cct::Code>,
    #[serde(default, rename = "ValueCurrencyCode")]
    pub value_currency_code: Option<cct::Code>,
    #[serde(default, rename = "ExpectedAmount")]
    pub expected_amount: Option<cct::Amount>,
    #[serde(default, rename = "ExpectedID")]
    pub expected_id: Option<cct::Identifier>,
    #[serde(default, rename = "ExpectedIndicator")]
    pub expected_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "ExpectedCode")]
    pub expected_code: Option<cct::Code>,
    #[serde(default, rename = "ExpectedValueNumeric")]
    pub expected_value_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "ExpectedDescription")]
    pub expected_description: Option<cct::Text>,
    #[serde(default, rename = "ExpectedURI")]
    pub expected_uri: Option<cct::Identifier>,
    #[serde(default, rename = "MaximumAmount")]
    pub maximum_amount: Option<cct::Amount>,
    #[serde(default, rename = "MinimumAmount")]
    pub minimum_amount: Option<cct::Amount>,
    #[serde(default, rename = "MaximumValueNumeric")]
    pub maximum_value_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "MinimumValueNumeric")]
    pub minimum_value_numeric: Option<cct::Numeric>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "TranslationTypeCode")]
    pub translation_type_code: Option<cct::Code>,
    #[serde(default, rename = "CertificationLevelDescription")]
    pub certification_level_description: Vec<cct::Text>,
    #[serde(default, rename = "CopyQualityTypeCode")]
    pub copy_quality_type_code: Option<cct::Code>,
    #[serde(default, rename = "ApplicablePeriod")]
    pub applicable_period: Vec<crate::Period>,
    #[serde(default, rename = "TemplateEvidence")]
    pub template_evidence: Vec<crate::Evidence>,
}
