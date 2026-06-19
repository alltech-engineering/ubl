#[derive(Debug, Deserialize, Serialize)]
/// A document issued by the party who acts as an agent for a transportation carrier or other agents to
/// the party who gives instructions for the transportation services (shipper, consignor, etc.) stating
/// the details of the transportation, charges, and terms and conditions under which the transportation
/// service is provided. The party issuing this document does not necessarily provide the physical
/// transportation service. The information in the Bill of Lading corresponds to the information on the
/// Forwarding Instructions. It is used for any mode of transport. A Bill of Lading can serve as a
/// contractual document between the parties for the transportation service. The document evidences a
/// contract of carriage by sea and the acceptance of responsibility for the goods by the carrier, by
/// which the carrier undertakes to deliver the goods against surrender of the document. A provision in
/// the document that the goods are to be delivered to the order of a named person, or to order, or to
/// bearer, constitutes such an undertaking.
///
/// UBL Dictionary Entry Name: `Bill Of Lading. Details`
///
/// Generated from XSD type `BillOfLadingType`.
pub struct BillOfLading {
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
/// Reference number (such as a booking reference number) assigned by a carrier or its agent to identify
/// a specific shipment when cargo space is reserved prior to loading.
    #[serde(default, rename = "CarrierAssignedID")]
    pub carrier_assigned_id: Option<cct::Identifier>,
/// A universally unique identifier for an instance of this document.
    #[serde(default, rename = "UUID")]
    pub uuid: Option<cct::Identifier>,
/// The date, assigned by the sender, on which this document was issued.
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
/// The time, assigned by the sender, at which this document was issued.
    #[serde(default, rename = "IssueTime")]
    pub issue_time: Option<udt::DateTime>,
/// Text, assigned by the sender, that identifies this document to business users.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// Textual description of the document instance.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A code signifying the status of the Bill Of Lading (revision, replacement, etc.).
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: Option<cct::Code>,
/// Reference number to identify a Shipping Order or Forwarding Instruction.
    #[serde(default, rename = "ShippingOrderID")]
    pub shipping_order_id: Option<cct::Identifier>,
/// Indicates whether the transport document is consigned to order.
    #[serde(default, rename = "ToOrderIndicator")]
    pub to_order_indicator: Option<udt::Indicator>,
/// A term used in commerce in reference to certain duties, called ad valorem duties, which are levied
/// on commodities at certain rates per centum on their value.
    #[serde(default, rename = "AdValoremIndicator")]
    pub ad_valorem_indicator: Option<udt::Indicator>,
/// Value declared by the shipper or his agent solely for the purpose of varying the carrier's level of
/// liability from that provided in the contract of carriage in case of loss or damage to goods or
/// delayed delivery.
    #[serde(default, rename = "DeclaredCarriageValueAmount")]
    pub declared_carriage_value_amount: Option<cct::Amount>,
/// Other free-text instructions to the forwarders or carriers related to the shipment. This element
/// ought to be used only where such information cannot be represented in other structured information
/// entities within the document.
    #[serde(default, rename = "OtherInstruction")]
    pub other_instruction: Vec<cct::Text>,
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
/// An identifiable collection of one or more goods items to be transported between the seller party and
/// the buyer party.
    #[serde(default, rename = "Shipment")]
    pub shipment: Option<cac::Shipment>,
/// A reference to another document associated with this document.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<cac::DocumentReference>,
/// Information that directly relates to the rate of exchange (conversion) between two currencies.
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Vec<cac::ExchangeRate>,
/// A list of interested parties to whom this document is distributed.
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: Vec<cac::DocumentDistribution>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
}
