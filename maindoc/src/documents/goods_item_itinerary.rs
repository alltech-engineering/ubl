#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsItemItinerary {
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
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
    #[serde(rename = "IssueTime")]
    pub issue_time: udt::DateTime,
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
    #[serde(rename = "VersionID")]
    pub version_id: cct::Identifier,
    #[serde(rename = "TransportExecutionPlanReferenceID")]
    pub transport_execution_plan_reference_id: cct::Identifier,
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: Option<cac::Party>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
    #[serde(default, rename = "ReferencedConsignment")]
    pub referenced_consignment: Vec<cac::Consignment>,
    #[serde(default, rename = "ReferencedTransportEquipment")]
    pub referenced_transport_equipment: Vec<cac::TransportEquipment>,
    #[serde(default, rename = "ReferencedPackage")]
    pub referenced_package: Vec<cac::Package>,
    #[serde(default, rename = "ReferencedGoodsItem")]
    pub referenced_goods_item: Vec<cac::GoodsItem>,
    #[serde(default, rename = "TransportationSegment")]
    pub transportation_segment: Vec<cac::TransportationSegment>,
}
