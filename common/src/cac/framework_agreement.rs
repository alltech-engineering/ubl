#[derive(Debug, Deserialize, Serialize)]
pub struct FrameworkAgreement {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ExpectedOperatorQuantity")]
    pub expected_operator_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumOperatorQuantity")]
    pub maximum_operator_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "Justification")]
    pub justification: Vec<super::cct::TextType>,
    #[serde(default, rename = "Frequency")]
    pub frequency: Vec<super::cct::TextType>,
    #[serde(default, rename = "EstimatedMaximumValueAmount")]
    pub estimated_maximum_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "MaximumValueAmount")]
    pub maximum_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "DurationPeriod")]
    pub duration_period: Option<Period>,
    #[serde(default, rename = "SubsequentProcessTenderRequirement")]
    pub subsequent_process_tender_requirement: Vec<TenderRequirement>,
}
