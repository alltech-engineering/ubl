#[derive(Debug, Deserialize, Serialize)]
pub struct TenderingCriterionProperty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ValueDataTypeCode")]
    pub value_data_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ValueUnitCode")]
    pub value_unit_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ValueCurrencyCode")]
    pub value_currency_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ExpectedAmount")]
    pub expected_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "ExpectedID")]
    pub expected_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ExpectedIndicator")]
    pub expected_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ExpectedCode")]
    pub expected_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ExpectedValueNumeric")]
    pub expected_value_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "ExpectedDescription")]
    pub expected_description: Option<super::cct::TextType>,
    #[serde(default, rename = "ExpectedURI")]
    pub expected_uri: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "MaximumAmount")]
    pub maximum_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "MinimumAmount")]
    pub minimum_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "MaximumValueNumeric")]
    pub maximum_value_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "MinimumValueNumeric")]
    pub minimum_value_numeric: Option<super::cct::NumericType>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "TranslationTypeCode")]
    pub translation_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CertificationLevelDescription")]
    pub certification_level_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "CopyQualityTypeCode")]
    pub copy_quality_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ApplicablePeriod")]
    pub applicable_period: Vec<Period>,
    #[serde(default, rename = "TemplateEvidence")]
    pub template_evidence: Vec<Evidence>,
}
