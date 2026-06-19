#[derive(Debug, Deserialize, Serialize)]
pub struct FrameworkAgreement {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(default, rename = "ExpectedOperatorQuantity")]
    pub expected_operator_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "MaximumOperatorQuantity")]
    pub maximum_operator_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "Justification")]
    pub justification: Vec<cct::Text>,
    #[serde(default, rename = "Frequency")]
    pub frequency: Vec<cct::Text>,
    #[serde(default, rename = "EstimatedMaximumValueAmount")]
    pub estimated_maximum_value_amount: Option<cct::Amount>,
    #[serde(default, rename = "MaximumValueAmount")]
    pub maximum_value_amount: Option<cct::Amount>,
    #[serde(default, rename = "DurationPeriod")]
    pub duration_period: Option<Period>,
    #[serde(default, rename = "SubsequentProcessTenderRequirement")]
    pub subsequent_process_tender_requirement: Vec<TenderRequirement>,
}
