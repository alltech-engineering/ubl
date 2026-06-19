#[derive(Debug, Deserialize, Serialize)]
/// A transport document describing a shipment It is issued by the party who undertakes to provide
/// transportation services, or undertakes to arrange for their provision, to the party who gives
/// instructions for the transportation services (shipper, consignor, etc.). It states the instructions
/// for the beneficiary and may contain the details of the transportation, charges, and terms and
/// conditions under which the transportation service is provided.
///
/// UBL Dictionary Entry Name: `Waybill. Details`
///
/// Generated from XSD type `WaybillType`.
pub struct Waybill {
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
/// An identifier (in the form of a reference number) assigned by a carrier or its agent to identify a
/// specific shipment.
    #[serde(default, rename = "CarrierAssignedID")]
    pub carrier_assigned_id: Option<cct::Identifier>,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The version of this waybill.
    #[serde(default, rename = "VersionID")]
    pub version_id: Option<cct::Identifier>,
/// The status of this waybill (draft, signed, approved, etc.), expressed as a code.
    #[serde(default, rename = "StatusCode")]
    pub status_code: Option<cct::Code>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// Text, assigned by the sender, that identifies this document to business users.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// Text describing the contents of the Waybill.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// An identifier (in the form of a reference number) of the Shipping Order or Forwarding Instruction
/// associated with this shipment.
    #[serde(default, rename = "ShippingOrderID")]
    pub shipping_order_id: Option<cct::Identifier>,
/// The type of waybill (Bill of Laden, Airwaybill, CMR, House waybill, etc.) expressed as a code.
    #[serde(default, rename = "WaybillTypeCode")]
    pub waybill_type_code: Option<cct::Code>,
/// An indicator of whether this waybill is consolidated from other waybills (true) or not (false).
    #[serde(default, rename = "ConsolidatedIndicator")]
    pub consolidated_indicator: Option<udt::Indicator>,
/// A term used in commerce in reference to certain duties, called ad valorem duties, which are levied
/// on commodities at certain rates per centum on their value.
    #[serde(default, rename = "AdValoremIndicator")]
    pub ad_valorem_indicator: Option<udt::Indicator>,
/// Value declared by the shipper or his agent solely for the purpose of varying the carrier's level of
/// liability from that provided in the contract of carriage in case of loss or damage to goods or
/// delayed delivery.
    #[serde(default, rename = "DeclaredCarriageValueAmount")]
    pub declared_carriage_value_amount: Option<cct::Amount>,
/// Other free-text instructions related to the shipment to the forwarders or carriers. This ought to be
/// used only where such information cannot be represented in other structured information entities
/// within the document.
    #[serde(default, rename = "OtherInstruction")]
    pub other_instruction: Vec<cct::Text>,
/// The location where this waybill was issued.
    #[serde(default, rename = "IssueLocation")]
    pub issue_location: Option<cac::Location>,
/// The Party who sends this Waybill.
    #[serde(default, rename = "SenderParty")]
    pub sender_party: Option<cac::Party>,
/// The Party who receives this Waybill.
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: Option<cac::Party>,
/// The Party who is reponsible for sending the goods.
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: Option<cac::Party>,
/// The Party who provides the transport of goods between named points.
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: Option<cac::Party>,
/// The Party who combines individual smaller consignments into a single larger shipment (a so-called
/// consolidated consignment or shipment) which is sent to a counterpart who mirrors the consolidator's
/// activity by dividing the consolidated consignment into its original components.
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: Option<cac::Party>,
/// A description of the shipment.
    #[serde(rename = "Shipment")]
    pub shipment: cac::Shipment,
/// A reference to another document associated with this document.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
/// Information about the rate of exchange (conversion) between two currencies.
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Vec<cac::ExchangeRate>,
/// A list of interested parties to whom this document is distributed.
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: Vec<cac::DocumentDistribution>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
