#[derive(Debug, Deserialize, Serialize)]
/// A document used to request the status of a previously issued Invoice.
///
/// UBL Dictionary Entry Name: `Invoice Status Request. Details`
///
/// Generated from XSD type `InvoiceStatusRequestType`.
pub struct InvoiceStatusRequest {
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
/// The date on which the sender of the Invoice Status Request requested a status update for the
/// referenced invoice(s).
    #[serde(rename = "RequestDate")]
    pub request_date: udt::DateTime,
/// The time at which the sender of the Invoice Status Request requested a status update for the
/// referenced invoice(s).
    #[serde(default, rename = "RequestTime")]
    pub request_time: Option<udt::DateTime>,
/// Free-form text pertinent to this document, conveying information that is not contained explicitly in
/// other structures.
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// A signature applied to this document.
    #[serde(default, rename = "Signature")]
    pub signature: Vec<cac::Signature>,
/// The party sending this document.
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::Party,
/// The party receiving this document.
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::Party,
/// A reference to the Invoice for which a status update is requested.
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: Vec<cac::BillingReference>,
}
