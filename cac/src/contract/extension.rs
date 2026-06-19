#[derive(Debug, Deserialize, Serialize)]
/// A class to describe possible extensions to a contract.
///
/// UBL Dictionary Entry Name: `Contract Extension. Details`
///
/// Generated from XSD type `ContractExtensionType`.
pub struct ContractExtension {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// A description for the possible options that can be carried out during the execution of the contract.
    #[serde(default, rename = "OptionsDescription")]
    pub options_description: Vec<cct::Text>,
/// The fixed minimum number of contract extensions or renewals.
    #[serde(default, rename = "MinimumNumberNumeric")]
    pub minimum_number_numeric: Option<cct::Numeric>,
/// The maximum allowed number of contract extensions.
    #[serde(default, rename = "MaximumNumberNumeric")]
    pub maximum_number_numeric: Option<cct::Numeric>,
/// Indicates that the contract can be extended using renewals.
    #[serde(default, rename = "RenewalsIndicator")]
    pub renewals_indicator: Option<udt::Indicator>,
/// The period during which the option for extending the contract is available.
    #[serde(default, rename = "OptionValidityPeriod")]
    pub option_validity_period: Option<crate::Period>,
/// The period allowed for each contract extension.
    #[serde(default, rename = "Renewal")]
    pub renewal: Vec<crate::Renewal>,
}
