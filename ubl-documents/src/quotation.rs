// UBL 2.5 Quotation document types.
//
// Reference: https://docs.oasis-open.org/ubl/cs01-UBL-2.5/UBL-2.5.html
// Generated from the authoritative XSD element declarations.

use serde::{Deserialize, Serialize};
use ubl_common::cbc::*;
use ubl_common::cac::*;
use ubl_common::cac::tendering::*;

/// RequestForQuotation — A document used to request a Quotation for goods and services from a Seller.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RequestForQuotation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    pub issue_time: IssueTime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub submission_due_date: Option<SubmissionDueDate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_code: Option<PricingCurrencyCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_count_numeric: Option<LineCountNumeric>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_validity_period: Option<RequestedValidityPeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub catalogue_document_reference: Option<CatalogueDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<AdditionalDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_customer_party: Option<OriginatorCustomerParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beneficiary_party: Vec<BeneficiaryParty>,
    pub seller_supplier_party: SellerSupplierParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_customer_party: Option<BuyerCustomerParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delivery: Vec<Delivery>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delivery_terms: Vec<DeliveryTerms>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_country: Option<DestinationCountry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract: Vec<Contract>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub request_for_quotation_line: Vec<RequestForQuotationLine>,
}

/// Quotation — A document providing a price for goods and services offered by a Seller.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quotation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_extensions: Option<UBLExtensions>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ubl_version_id: Option<UBLVersionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customization_id: Option<CustomizationID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_id: Option<ProfileID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub profile_execution_id: Option<ProfileExecutionID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copy_indicator: Option<CopyIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub uuid: Option<UUID>,
    pub issue_date: IssueDate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub issue_time: Option<IssueTime>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pricing_currency_code: Option<PricingCurrencyCode>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub line_count_numeric: Option<LineCountNumeric>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<ValidityPeriod>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub request_for_quotation_document_reference: Option<RequestForQuotationDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub additional_document_reference: Vec<AdditionalDocumentReference>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub signature: Vec<Signature>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub originator_customer_party: Option<OriginatorCustomerParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub beneficiary_party: Vec<BeneficiaryParty>,
    pub seller_supplier_party: SellerSupplierParty,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_customer_party: Option<BuyerCustomerParty>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delivery: Vec<Delivery>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub delivery_terms: Vec<DeliveryTerms>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_country: Option<DestinationCountry>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub contract: Vec<Contract>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quoted_monetary_total: Option<QuotedMonetaryTotal>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub quotation_line: Vec<QuotationLine>,
}
