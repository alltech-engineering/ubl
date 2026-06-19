#[derive(Debug, Deserialize, Serialize)]
/// A document issued to a forwarder, giving instructions regarding the action to be taken for the
/// forwarding of goods described therein. Forwarding Instructions is used by any party who gives
/// instructions for the transportation services required for a consignment of goods to any party who is
/// contracted to provide the transportation services. The parties who issue this document are commonly
/// referred to as the shipper or consignor, while the parties who receive this document are forwarders,
/// carriers, shipping agents, etc. This document may also be issued by a forwarder or shipping agent in
/// its capacity as a shipper. This document can be used to arrange for the transportation (1) of
/// different types of goods or cargoes; (2) whether containerized or non-containerized; (3) through
/// different modes of transport including multi-modal; and (4) from any origin to any destination.
///
/// UBL Dictionary Entry Name: `Forwarding Instructions. Details`
///
/// Generated from XSD type `ForwardingInstructionsType`.
pub struct ForwardingInstructions {
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
/// Reference number assigned by a carrier or its agent to identify a specific shipment, such as a
/// booking reference number when cargo space is reserved prior to loading.
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
/// A code signifying the status of the Forwarding Instructions with respect to its original state. This
/// code may be used if the document precedes the event and is subsequently found to be incorrect and in
/// need of cancellation or revision.
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: Option<cct::Code>,
/// Reference number to identify a Shipping Order.
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
/// Contains other free-text instructions to the forwarders or carriers related to the shipment. This
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
