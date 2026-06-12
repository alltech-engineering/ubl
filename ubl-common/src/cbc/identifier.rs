// UBL Identifier types — identifiers with optional scheme metadata.

use serde::{Deserialize, Serialize};

/// The base Identifier type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Identifier {
    pub value: String,
    pub scheme_id: Option<String>,
    pub scheme_agency_id: Option<String>,
    pub scheme_version_id: Option<String>,
}

impl Identifier {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into(), scheme_id: None, scheme_agency_id: None, scheme_version_id: None }
    }
    pub fn with_scheme(mut self, scheme_id: impl Into<String>) -> Self {
        self.scheme_id = Some(scheme_id.into());
        self
    }
}

macro_rules! define_identifier {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(pub Identifier);
        impl $name {
            pub fn new(value: impl Into<String>) -> Self { Self(Identifier::new(value)) }
            pub fn value(&self) -> &str { &self.0.value }
        }
    };
}

// Core identifiers
define_identifier!(ID, "A generic identifier. The primary identifier for many UBL elements.");
define_identifier!(UUID, "A universally unique identifier (RFC 4122).");
define_identifier!(GLN, "Global Location Number (GS1).");
define_identifier!(GTIN, "Global Trade Item Number (GS1).");
define_identifier!(SSCC, "Serial Shipping Container Code (GS1).");

// Party identifiers
define_identifier!(AccountID, "An account identifier.");
define_identifier!(AdditionalAccountID, "An additional account identifier.");
define_identifier!(AgencyID, "An agency identifier.");
define_identifier!(BuyerReference, "A reference assigned by the buyer.");
define_identifier!(BuyerEventID, "An event identifier assigned by the buyer.");
define_identifier!(SellerEventID, "An event identifier assigned by the seller.");
define_identifier!(GoodsItemPassportID, "A goods item passport identifier.");
define_identifier!(GoodsItemPassportCounterfoilID, "A goods item passport counterfoil identifier.");
define_identifier!(CompanyID, "A company registration identifier.");
define_identifier!(CustomerAssignedAccountID, "An account ID assigned by the customer.");
define_identifier!(SupplierAssignedAccountID, "An account ID assigned by the supplier.");
define_identifier!(BrokerAssignedID, "An identifier assigned by a broker.");
define_identifier!(CarrierAssignedID, "An identifier assigned by a carrier.");
define_identifier!(ConsigneeAssignedID, "An identifier assigned by the consignee.");
define_identifier!(ConsignorAssignedID, "An identifier assigned by the consignor.");
define_identifier!(ContractedCarrierAssignedID, "An identifier assigned by a contracted carrier.");

// Document identifiers
define_identifier!(CustomizationID, "An identifier for a UBL customization.");
define_identifier!(ProfileID, "An identifier for a business process profile.");
define_identifier!(ProfileExecutionID, "An identifier for a profile execution instance.");
define_identifier!(DocumentID, "An identifier for a document.");
define_identifier!(LineID, "An identifier for a line within a document.");
define_identifier!(ParentDocumentID, "An identifier for a parent document.");
define_identifier!(PreviousDocumentID, "An identifier for a previous document version.");
define_identifier!(OriginalDocumentID, "An identifier for the original document.");
define_identifier!(SequenceID, "A sequence identifier.");
define_identifier!(VersionID, "A version identifier.");
define_identifier!(RevisionID, "A revision identifier.");
define_identifier!(PreviousVersionID, "The identifier of a previous version.");
define_identifier!(InstructionID, "An identifier for an instruction.");
define_identifier!(SalesOrderID, "A sales order identifier.");
define_identifier!(PurchaseOrderID, "A purchase order identifier.");
define_identifier!(ReceiptID, "A receipt identifier.");
define_identifier!(RequestID, "A request identifier.");
define_identifier!(ResponseID, "A response identifier.");
define_identifier!(ContractFolderID, "A contract folder identifier.");
define_identifier!(TenderEnvelopeID, "A tender envelope identifier.");
define_identifier!(AwardID, "An award identifier.");

// Item identifiers
define_identifier!(ItemIdentificationID, "An item identifier.");
define_identifier!(BarcodeSymbologyID, "A barcode symbology identifier.");
define_identifier!(BatchID, "A batch/lot identifier.");
define_identifier!(LotNumberID, "A lot number identifier.");
define_identifier!(SerialID, "A serial number identifier.");
define_identifier!(TrackingID, "A tracking identifier.");
define_identifier!(TraceID, "A trace identifier.");

// Transport identifiers
define_identifier!(AircraftID, "An aircraft identifier.");
define_identifier!(RailCarID, "A rail car identifier.");
define_identifier!(VesselID, "A vessel identifier.");
define_identifier!(VehicleID, "A vehicle identifier.");
define_identifier!(ContainerID, "A container identifier.");
define_identifier!(TransportEquipmentID, "A transport equipment identifier.");
define_identifier!(LicensePlateID, "A license plate identifier.");
define_identifier!(SealID, "A seal identifier.");
define_identifier!(TripID, "A trip identifier.");
define_identifier!(VoyageID, "A voyage identifier.");
define_identifier!(WaybillID, "A waybill identifier.");
define_identifier!(BillOfLadingID, "A bill of lading identifier.");

// Financial identifiers
define_identifier!(PaymentID, "A payment identifier.");
define_identifier!(PaymentOrderReference, "A payment order reference.");
define_identifier!(PaymentAlternativeID, "An alternative payment identifier.");
define_identifier!(SettlementID, "A settlement identifier.");
define_identifier!(TaxID, "A tax identifier (e.g., VAT number).");
define_identifier!(TaxSchemeID, "A tax scheme identifier.");
define_identifier!(MandateID, "A mandate identifier (e.g., SEPA mandate).");
define_identifier!(LanguageID, "A language identifier (e.g., en, fr, de).");
define_identifier!(ReferenceID, "A generic reference identifier.");
define_identifier!(TransactionID, "A transaction identifier.");
define_identifier!(BudgetAccountID, "A budget account identifier.");

// Other
define_identifier!(AttributeID, "An attribute identifier.");
define_identifier!(CertificateReferenceID, "A certificate reference identifier.");
define_identifier!(ConsumptionReportID, "A consumption report identifier.");
define_identifier!(ConsumptionID, "A consumption identifier.");
define_identifier!(ForecastID, "A forecast identifier.");
define_identifier!(GUID, "A globally unique identifier.");
define_identifier!(HazardClassID, "A hazard class identifier.");
define_identifier!(ImmobilizationCertificateID, "An immobilization certificate identifier.");
define_identifier!(InventoryID, "An inventory identifier.");
define_identifier!(IssuerID, "An issuer identifier.");
define_identifier!(LocationID, "A location identifier.");
define_identifier!(NationalityID, "A nationality identifier.");
define_identifier!(NetworkID, "A network identifier.");
define_identifier!(PricingReferenceID, "A pricing reference identifier.");
define_identifier!(ProductTraceID, "A product trace identifier.");
define_identifier!(ProviderID, "A provider identifier.");
define_identifier!(RegistrationID, "A registration identifier.");
define_identifier!(RequiredCustomsID, "A required customs identifier.");
define_identifier!(SignatureID, "A signature identifier.");
define_identifier!(SpecificationID, "A specification identifier.");
define_identifier!(ValidatorID, "A validator identifier.");
define_identifier!(VariantID, "A variant identifier.");
define_identifier!(WebsiteURI, "A website URI identifier.");
define_identifier!(SchemeURI, "A URI identifying a classification scheme.");
define_identifier!(URI, "A generic Uniform Resource Identifier.");


// --- Missing ---
define_identifier!(ChipApplicationID, "A chip application identifier.");
define_identifier!(ReleaseID, "A release identifier.");
define_identifier!(PaymentMeansID, "A payment means identifier.");
define_identifier!(SalesOrderLineID, "A sales order line identifier.");
define_identifier!(RegistrationNationalityID, "A registration nationality identifier.");
define_identifier!(SequenceNumberID, "A sequence number identifier (e.g., for order change sequencing).");
define_identifier!(UBLVersionID, "Identifies the earliest version of the UBL 2 schema for this document type.");

// --- Generated from UBL 2.5 XSD ---
define_identifier!(AccessToolsURI, "UBL CBC type: AccessToolsURI.");
define_identifier!(AuctionURI, "UBL CBC type: AuctionURI.");
define_identifier!(AuthorizationID, "UBL CBC type: AuthorizationID.");
define_identifier!(AwardingCriterionID, "UBL CBC type: AwardingCriterionID.");
define_identifier!(BlockchainID, "UBL CBC type: BlockchainID.");
define_identifier!(BusinessClassificationEvidenceID, "UBL CBC type: BusinessClassificationEvidenceID.");
define_identifier!(BusinessIdentityEvidenceID, "UBL CBC type: BusinessIdentityEvidenceID.");
define_identifier!(BuyerProfileURI, "UBL CBC type: BuyerProfileURI.");
define_identifier!(CV2ID, "UBL CBC type: CV2ID.");
define_identifier!(CertificateURI, "UBL CBC type: CertificateURI.");
define_identifier!(EconomicOperatorRegistryURI, "UBL CBC type: EconomicOperatorRegistryURI.");
define_identifier!(ElectronicAddressID, "UBL CBC type: ElectronicAddressID.");
define_identifier!(EndpointID, "UBL CBC type: EndpointID.");
define_identifier!(EndpointURI, "UBL CBC type: EndpointURI.");
define_identifier!(ExchangeMarketID, "UBL CBC type: ExchangeMarketID.");
define_identifier!(ExchangeNetworkID, "UBL CBC type: ExchangeNetworkID.");
define_identifier!(ExpectedID, "UBL CBC type: ExpectedID.");
define_identifier!(ExpectedURI, "UBL CBC type: ExpectedURI.");
define_identifier!(ExtendedID, "UBL CBC type: ExtendedID.");
define_identifier!(FormatID, "UBL CBC type: FormatID.");
define_identifier!(FreightForwarderAssignedID, "UBL CBC type: FreightForwarderAssignedID.");
define_identifier!(GateID, "UBL CBC type: GateID.");
define_identifier!(IdentificationID, "UBL CBC type: IdentificationID.");
define_identifier!(InformationURI, "UBL CBC type: InformationURI.");
define_identifier!(IssueNumberID, "UBL CBC type: IssueNumberID.");
define_identifier!(IssuerScopeID, "UBL CBC type: IssuerScopeID.");
define_identifier!(JourneyID, "UBL CBC type: JourneyID.");
define_identifier!(LoadingSequenceID, "UBL CBC type: LoadingSequenceID.");
define_identifier!(LogoReferenceID, "UBL CBC type: LogoReferenceID.");
define_identifier!(LotsGroupID, "UBL CBC type: LotsGroupID.");
define_identifier!(LowerOrangeHazardPlacardID, "UBL CBC type: LowerOrangeHazardPlacardID.");
define_identifier!(MMSIRegistrationID, "UBL CBC type: MMSIRegistrationID.");
define_identifier!(MarkingID, "UBL CBC type: MarkingID.");
define_identifier!(MerchantID, "UBL CBC type: MerchantID.");
define_identifier!(NationalID, "UBL CBC type: NationalID.");
define_identifier!(OID, "UBL CBC type: OID.");
define_identifier!(OntologyURI, "UBL CBC type: OntologyURI.");
define_identifier!(OpenTenderID, "UBL CBC type: OpenTenderID.");
define_identifier!(OriginalContractingSystemID, "UBL CBC type: OriginalContractingSystemID.");
define_identifier!(OriginalJobID, "UBL CBC type: OriginalJobID.");
define_identifier!(ParentDocumentLineReferenceID, "UBL CBC type: ParentDocumentLineReferenceID.");
define_identifier!(ParentDocumentVersionID, "UBL CBC type: ParentDocumentVersionID.");
define_identifier!(ParticipantID, "UBL CBC type: ParticipantID.");
define_identifier!(PaymentPlatformID, "UBL CBC type: PaymentPlatformID.");
define_identifier!(PaymentRailID, "UBL CBC type: PaymentRailID.");
define_identifier!(PaymentTerminalID, "UBL CBC type: PaymentTerminalID.");
define_identifier!(PaymentTermsDetailsURI, "UBL CBC type: PaymentTermsDetailsURI.");
define_identifier!(PerformingCarrierAssignedID, "UBL CBC type: PerformingCarrierAssignedID.");
define_identifier!(PositionInPortID, "UBL CBC type: PositionInPortID.");
define_identifier!(PrepaidPaymentReferenceID, "UBL CBC type: PrepaidPaymentReferenceID.");
define_identifier!(PreviousJobID, "UBL CBC type: PreviousJobID.");
define_identifier!(PrimaryAccountNumberID, "UBL CBC type: PrimaryAccountNumberID.");
define_identifier!(ProtocolID, "UBL CBC type: ProtocolID.");
define_identifier!(RadioCallSignID, "UBL CBC type: RadioCallSignID.");
define_identifier!(ReferencedConsignmentID, "UBL CBC type: ReferencedConsignmentID.");
define_identifier!(RequestForQuotationLineID, "UBL CBC type: RequestForQuotationLineID.");
define_identifier!(ResponseURI, "UBL CBC type: ResponseURI.");
define_identifier!(RevisedForecastLineID, "UBL CBC type: RevisedForecastLineID.");
define_identifier!(SchemaURI, "UBL CBC type: SchemaURI.");
define_identifier!(SecurityID, "UBL CBC type: SecurityID.");
define_identifier!(SerialNumberID, "UBL CBC type: SerialNumberID.");
define_identifier!(ShippingOrderID, "UBL CBC type: ShippingOrderID.");
define_identifier!(StowagePositionID, "UBL CBC type: StowagePositionID.");
define_identifier!(SubscriberID, "UBL CBC type: SubscriberID.");
define_identifier!(SuccessiveSequenceID, "UBL CBC type: SuccessiveSequenceID.");
define_identifier!(TankID, "UBL CBC type: TankID.");
define_identifier!(TrailerLicensePlateID, "UBL CBC type: TrailerLicensePlateID.");
define_identifier!(TrainID, "UBL CBC type: TrainID.");
define_identifier!(TransportExecutionPlanReferenceID, "UBL CBC type: TransportExecutionPlanReferenceID.");
define_identifier!(TransportationServiceDetailsURI, "UBL CBC type: TransportationServiceDetailsURI.");
define_identifier!(TypeID, "UBL CBC type: TypeID.");
define_identifier!(UpperOrangeHazardPlacardID, "UBL CBC type: UpperOrangeHazardPlacardID.");
define_identifier!(ValidatedCriterionPropertyID, "UBL CBC type: ValidatedCriterionPropertyID.");
define_identifier!(WeighingDeviceID, "UBL CBC type: WeighingDeviceID.");
define_identifier!(WorkItemID, "UBL CBC type: WorkItemID.");
