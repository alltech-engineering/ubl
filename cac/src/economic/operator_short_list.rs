#[derive(Debug, Deserialize, Serialize)]
/// A class to provide information about the preselection of a short list of economic operators for
/// consideration as possible candidates in a tendering process.
///
/// UBL Dictionary Entry Name: `Economic Operator Short List. Details`
///
/// Generated from XSD type `EconomicOperatorShortListType`.
pub struct EconomicOperatorShortList {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// Text describing the criteria used to restrict the number of candidates.
    #[serde(default, rename = "LimitationDescription")]
    pub limitation_description: Vec<cct::Text>,
/// The number of economic operators expected to be on the short list.
    #[serde(default, rename = "ExpectedQuantity")]
    pub expected_quantity: Option<cct::Quantity>,
/// The maximum number of economic operators on the short list.
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: Option<cct::Quantity>,
/// The minimum number of economic operators on the short list.
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: Option<cct::Quantity>,
/// The Party who is preselected to submit a Tender in a negotiated procedure. Negotiated procedure is a
/// type of procedure where the Buyer can set the Parties to be invited in the procurement project.
    #[serde(default, rename = "PreSelectedParty")]
    pub pre_selected_party: Vec<crate::Party>,
}
