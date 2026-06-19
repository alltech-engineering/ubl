#[derive(Debug, Deserialize, Serialize)]
/// A document providing details relating to a transport service, such as transport movement,
/// identification of equipment and goods, subcontracted service providers, etc.
///
/// UBL Dictionary Entry Name: `Goods Item Itinerary. Details`
///
/// Generated from XSD type `GoodsItemItineraryType`.
pub struct GoodsItemItinerary {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
/// Identifies the earliest version of the UBL 2 schema for this document type that defines all of the
/// elements that might be encountered in the current instance.
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: Option<cct::Identifier>,
/// Identifies a user-defined customization of UBL for a specific use.
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: Option<cct::Identifier>,
/// Identifies a user-defined profile of the customization of UBL being used.
    #[serde(default, rename = "ProfileID")]
    pub profile_id: Option<cct::Identifier>,
/// Identifies an instance of executing a profile, to associate all transactions in a collaboration.
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: Option<cct::Identifier>,
/// An identifier for this document, assigned by the sender.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// (Deprecated) Indicates whether this document is a copy (true) or not (false).
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: Option<udt::Indicator>,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(rename = "IssueTime")]
    pub issue_time: udt::DateTime,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Identifies a version of a Goods Item Itinerary in order to distinguish updates.
    #[serde(rename = "VersionID")]
    pub version_id: cct::Identifier,
/// The Transport Execution Plan associated with this Goods Item Itinerary.
    #[serde(rename = "TransportExecutionPlanReferenceID")]
    pub transport_execution_plan_reference_id: cct::Identifier,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who sends this Goods Item Itinerary.
    #[serde(default, rename = "SenderParty")]
    pub sender_party: Option<cac::Party>,
/// The Party who receives this Goods Item Itinerary.
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
/// A consignment being transported in the transport service associated with this Goods Item Itinerary.
    #[serde(default, rename = "ReferencedConsignment")]
    pub referenced_consignment: Vec<cac::Consignment>,
/// Transport equipment being transported in the transport service associated with this Goods Item
/// Itinerary.
    #[serde(default, rename = "ReferencedTransportEquipment")]
    pub referenced_transport_equipment: Vec<cac::TransportEquipment>,
/// A package being transported in the transport service associated with this Goods Item Itinerary.
    #[serde(default, rename = "ReferencedPackage")]
    pub referenced_package: Vec<cac::Package>,
/// An item of goods being transported in the transport service associated with this Goods Item
/// Itinerary.
    #[serde(default, rename = "ReferencedGoodsItem")]
    pub referenced_goods_item: Vec<cac::GoodsItem>,
/// A part of a transport service that has its own Transport Execution Plan. A Transportation Segment
/// may cover services other than transport, such as terminal handling, document management, customs
/// procedures, etc.
    #[serde(default, rename = "TransportationSegment")]
    pub transportation_segment: Vec<cac::TransportationSegment>,
}
