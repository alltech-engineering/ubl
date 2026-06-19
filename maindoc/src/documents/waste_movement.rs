#[derive(Debug, Deserialize, Serialize)]
/// A document used to report the transport of waste.
///
/// UBL Dictionary Entry Name: `Waste Movement. Details`
///
/// Generated from XSD type `WasteMovementType`.
pub struct WasteMovement {
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
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTime,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// A code signifying the type of this Waste Movement.
    #[serde(default, rename = "WasteMovementTypeCode")]
    pub waste_movement_type_code: Option<cct::Code>,
/// Sequence number of this Waste Movement Document referring to the Waste Notification Document.
    #[serde(default, rename = "SequenceNumberID")]
    pub sequence_number_id: Option<cct::Identifier>,
/// Estimated total number of shipments for the Waste Notification document.
    #[serde(default, rename = "ConsignmentQuantity")]
    pub consignment_quantity: Option<cct::Quantity>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party sending this document.
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
/// The Party receiving this document.
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
/// The Party that acts as the notifier in relation to the Waste Movements covered by this document.
    #[serde(rename = "NotifierParty")]
    pub notifier_party: cac::Party,
/// The Party disposing of the waste material.
    #[serde(default, rename = "DisposalFacilityParty")]
    pub disposal_facility_party: Option<cac::Party>,
/// The Party recovering the waste material.
    #[serde(default, rename = "RecoveryFacilityParty")]
    pub recovery_facility_party: Option<cac::Party>,
/// The Party producing the waste material.
    #[serde(rename = "WasteProducerParty")]
    pub waste_producer_party: cac::Party,
/// The relevant shipment information describing the planned transport and the waste material.
    #[serde(rename = "Shipment")]
    pub shipment: cac::Shipment,
/// A reference to the Waste Notification document.
    #[serde(default, rename = "WasteNotificationDocumentReference")]
    pub waste_notification_document_reference: Option<cac::DocumentReference>,
/// A reference to a Weight Statement document.
    #[serde(default, rename = "WeightStatementDocumentReference")]
    pub weight_statement_document_reference: Vec<cac::DocumentReference>,
/// A reference to a relevant document associated with this Waste Movement.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
/// A distribution of this document to an interested Party.
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: Vec<cac::DocumentDistribution>,
}
