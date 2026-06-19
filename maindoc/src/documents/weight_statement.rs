#[derive(Debug, Deserialize, Serialize)]
/// A document used to report weight or verified mass measurements in the transport chain.
///
/// UBL Dictionary Entry Name: `Weight Statement. Details`
///
/// Generated from XSD type `WeightStatementType`.
pub struct WeightStatement {
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
/// A code signifying the type of Weight Statement.
    #[serde(default, rename = "WeightStatementTypeCode")]
    pub weight_statement_type_code: Option<cct::Code>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The Party who sends this Weight Statement (e.g. Weighing Station, Shipper, Freight Forwarder,
/// Carrier, ...).
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
/// The Party who receives this Weight Statement (e.g. carrier, terminal operator, ...).
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
/// The Party who executes the weight measure (e.g. weighing station).
    #[serde(default, rename = "WeighingParty")]
    pub weighing_party: Option<cac::Party>,
/// The Party who plays the role of the Shipper (BCO, FF or NVOCC) who is responsible for the VGM (e.g.
/// according the SOLAS Convention).
    #[serde(default, rename = "ShipperParty")]
    pub shipper_party: Option<cac::Party>,
/// The Party who signs the Verified Gross Mass (VGM) on behalf of the Shipper.
    #[serde(default, rename = "ResponsibleParty")]
    pub responsible_party: Option<cac::Party>,
/// The relevant shipment information with details on the transport equipment weight or mass
/// measurements, including verified gross mass (VGM) information.
    #[serde(rename = "Shipment")]
    pub shipment: cac::Shipment,
}
