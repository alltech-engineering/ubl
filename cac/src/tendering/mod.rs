use serde::{Deserialize, Serialize};


include!("terms.rs");
include!("criterion_response.rs");
include!("process.rs");
include!("criterion_property_group.rs");
include!("criterion.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the criterion properties.
///
/// UBL Dictionary Entry Name: `Tendering Criterion Property. Details`
///
/// Generated from XSD type `TenderingCriterionPropertyType`.
pub struct TenderingCriterionProperty {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier to refer to the criterion property.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The name of the criterion property.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// A description of the criterion property.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// A mutually agreed code signifying the type of the property.
    #[serde(default, rename = "TypeCode")]
    pub type_code: Option<cct::Code>,
/// The data type of the numeric value and any constraints on the data type metadata.
    #[serde(default, rename = "ValueDataTypeCode")]
    pub value_data_type_code: Option<cct::Code>,
/// The unit of measure of the numeric value as a quantity or measure.
    #[serde(default, rename = "ValueUnitCode")]
    pub value_unit_code: Option<cct::Code>,
/// The currency of the numeric value as an amount.
    #[serde(default, rename = "ValueCurrencyCode")]
    pub value_currency_code: Option<cct::Code>,
/// The expected amount that the responder has to provide in the criterion response.
    #[serde(default, rename = "ExpectedAmount")]
    pub expected_amount: Option<cct::Amount>,
/// The expected identifier that the responder has to provide in the criterion response.
    #[serde(default, rename = "ExpectedID")]
    pub expected_id: Option<cct::Identifier>,
/// The expected indicator (true or false) that the responder has to provide in the criterion response.
    #[serde(default, rename = "ExpectedIndicator")]
    pub expected_indicator: Option<udt::Indicator>,
/// The expected code that the responder has to provide in the criterion response.
    #[serde(default, rename = "ExpectedCode")]
    pub expected_code: Option<cct::Code>,
/// The expected value that the responder has to provide in the criterion response.
    #[serde(default, rename = "ExpectedValueNumeric")]
    pub expected_value_numeric: Option<cct::Numeric>,
/// The description of the of the expected
    #[serde(default, rename = "ExpectedDescription")]
    pub expected_description: Option<cct::Text>,
/// The expected URL that the responder has to provide in the criterion response.
    #[serde(default, rename = "ExpectedURI")]
    pub expected_uri: Option<cct::Identifier>,
/// The maximum amount the response must have.
    #[serde(default, rename = "MaximumAmount")]
    pub maximum_amount: Option<cct::Amount>,
/// The minimum amount the response must have.
    #[serde(default, rename = "MinimumAmount")]
    pub minimum_amount: Option<cct::Amount>,
/// The maximum value the response must have.
    #[serde(default, rename = "MaximumValueNumeric")]
    pub maximum_value_numeric: Option<cct::Numeric>,
/// The minimum value the response must have.
    #[serde(default, rename = "MinimumValueNumeric")]
    pub minimum_value_numeric: Option<cct::Numeric>,
/// The maximum quantity value the response must have.
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<cct::Quantity>,
/// The minimum quantity value the response must have.
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<cct::Quantity>,
/// The type of Transation that the requirement will be translated for example certified translation
    #[serde(default, rename = "TranslationTypeCode")]
    pub translation_type_code: Option<cct::Code>,
/// The description of the level of the expected certification
    #[serde(default, rename = "CertificationLevelDescription")]
    pub certification_level_description: Vec<cct::Text>,
/// The type of Copy quality, expressed as a code.
    #[serde(default, rename = "CopyQualityTypeCode")]
    pub copy_quality_type_code: Option<cct::Code>,
/// The period to which this criterion property will apply.
    #[serde(default, rename = "ApplicablePeriod")]
    pub applicable_period: Vec<crate::Period>,
/// An evidence that can be used to meet this criterion property.
    #[serde(default, rename = "TemplateEvidence")]
    pub template_evidence: Vec<crate::Evidence>,
}
