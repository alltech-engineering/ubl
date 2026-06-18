#[derive(Debug, Deserialize, Serialize)]
pub struct ForwardingInstructions {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CarrierAssignedID")]
    pub carrier_assigned_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTimeType>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::TextType>,
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: Option<cct::CodeType>,
    #[serde(default, rename = "ShippingOrderID")]
    pub shipping_order_id: Option<cct::IdentifierType>,
    #[serde(default, rename = "ToOrderIndicator")]
    pub to_order_indicator: Option<udt::IndicatorType>,
    #[serde(default, rename = "AdValoremIndicator")]
    pub ad_valorem_indicator: Option<udt::IndicatorType>,
    #[serde(default, rename = "DeclaredCarriageValueAmount")]
    pub declared_carriage_value_amount: Option<cct::AmountType>,
    #[serde(default, rename = "OtherInstruction")]
    pub other_instruction: Vec<cct::TextType>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: Option<cac::Party>,
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: Option<cac::Party>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: Option<cac::Party>,
    #[serde(rename = "Shipment")]
    pub shipment: cac::Shipment,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Vec<cac::ExchangeRate>,
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: Vec<cac::DocumentDistribution>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
