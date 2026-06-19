#[derive(Debug, Deserialize, Serialize)]
pub struct BillOfLading {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "CarrierAssignedID")]
    pub carrier_assigned_id: Option<cct::Identifier>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: Option<cct::Code>,
    #[serde(default, rename = "ShippingOrderID")]
    pub shipping_order_id: Option<cct::Identifier>,
    #[serde(default, rename = "ToOrderIndicator")]
    pub to_order_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "AdValoremIndicator")]
    pub ad_valorem_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "DeclaredCarriageValueAmount")]
    pub declared_carriage_value_amount: Option<cct::Amount>,
    #[serde(default, rename = "OtherInstruction")]
    pub other_instruction: Vec<cct::Text>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: Option<cac::Party>,
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: Option<cac::Party>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: Option<cac::Party>,
    #[serde(default, rename = "Shipment")]
    pub shipment: Option<cac::Shipment>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Vec<cac::ExchangeRate>,
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: Vec<cac::DocumentDistribution>,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
