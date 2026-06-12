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
define_identifier!(AgencyName, "An agency name identifier.");
define_identifier!(BuyerReference, "A reference assigned by the buyer.");
define_identifier!(BuyerEventID, "An event identifier assigned by the buyer.");
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
define_identifier!(ModelName, "A model name identifier.");

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


// --- Missing ---
define_identifier!(ChipApplicationID, "A chip application identifier.");
