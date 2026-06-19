#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a tendering framework agreement.
///
/// UBL Dictionary Entry Name: `Framework Agreement. Details`
///
/// Generated from XSD type `FrameworkAgreementType`.
pub struct FrameworkAgreement {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The number of economic operators expected to participate in this framework agreement.
    #[serde(default, rename = "ExpectedOperatorQuantity")]
    pub expected_operator_quantity: Option<cct::Quantity>,
/// The maximum number of economic operators allowed to participate in this framework agreement.
    #[serde(default, rename = "MaximumOperatorQuantity")]
    pub maximum_operator_quantity: Option<cct::Quantity>,
/// Text describing the justification for this framework agreement.
    #[serde(default, rename = "Justification")]
    pub justification: Vec<cct::Text>,
/// Text describing the frequency with which subsequent contracts will be awarded.
    #[serde(default, rename = "Frequency")]
    pub frequency: Vec<cct::Text>,
/// The estimated value which will be spent within a framework agreement over its whole duration,
/// including options and renewals.
    #[serde(default, rename = "EstimatedMaximumValueAmount")]
    pub estimated_maximum_value_amount: Option<cct::Amount>,
/// The maximum Value which can be spent within a framework agreement over its whole duration, including
/// options and renewals.
    #[serde(default, rename = "MaximumValueAmount")]
    pub maximum_value_amount: Option<cct::Amount>,
/// The period during which this framework agreement applies.
    #[serde(default, rename = "DurationPeriod")]
    pub duration_period: Option<Period>,
/// A tender requirement intended for consumption by downstream tendering processes derived from the
/// establishment of this framework agreement.
    #[serde(default, rename = "SubsequentProcessTenderRequirement")]
    pub subsequent_process_tender_requirement: Vec<TenderRequirement>,
}
