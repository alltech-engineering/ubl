use serde::{Deserialize, Serialize};
use crate::{cac, cbc, cct, ds, dsig_11, ext, qdt, sac, sbc, udt, xades, xs};
use crate::{
    UblDocumentSignaturesType, ValidationDataType, SignaturePolicyStoreType,
    SignaturePolicyStoreTypeContent, CompleteCertificateRefsTypeV2Type,
    RecomputedDigestValueType, RenewedDigestsType,
};

pub type ExceptionNotification = ExceptionNotificationType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExceptionNotificationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(rename = "ExceptionObservationPeriod")]
    pub exception_observation_period: cac::PeriodType,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "ExceptionNotificationLine")]
    pub exception_notification_line: ::std::vec::Vec<cac::ExceptionNotificationLineType>,
}

pub type RemittanceAdvice = RemittanceAdviceType;

#[derive(Debug, Deserialize, Serialize)]
pub struct RemittanceAdviceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "TotalDebitAmount")]
    pub total_debit_amount: ::core::option::Option<cct::AmountType>,
    #[serde(default, rename = "TotalCreditAmount")]
    pub total_credit_amount: ::core::option::Option<cct::AmountType>,
    #[serde(default, rename = "TotalPaymentAmount")]
    pub total_payment_amount: ::core::option::Option<cct::AmountType>,
    #[serde(default, rename = "PaymentOrderReference")]
    pub payment_order_reference: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "PayerReference")]
    pub payer_reference: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "InvoicingPartyReference")]
    pub invoicing_party_reference: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: ::core::option::Option<cac::BillingReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "AccountingCustomerParty")]
    pub accounting_customer_party: cac::CustomerPartyType,
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierPartyType,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: ::core::option::Option<cac::PaymentMeansType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(default, rename = "RemittanceAdviceLine")]
    pub remittance_advice_line: ::std::vec::Vec<cac::RemittanceAdviceLineType>,
}

pub type TransportationStatus = TransportationStatusType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TransportationStatusType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CarrierAssignedID")]
    pub carrier_assigned_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ShippingOrderID")]
    pub shipping_order_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "OtherInstruction")]
    pub other_instruction: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "TransportationStatusTypeCode")]
    pub transportation_status_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "TransportExecutionStatusCode")]
    pub transport_execution_status_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Consignment")]
    pub consignment: ::std::vec::Vec<cac::ConsignmentType>,
    #[serde(default, rename = "TransportEvent")]
    pub transport_event: ::std::vec::Vec<cac::TransportEventType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "TransportationStatusRequestDocumentReference")]
    pub transportation_status_request_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "TransportExecutionPlanDocumentReference")]
    pub transport_execution_plan_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "UpdatedPickupTransportEvent")]
    pub updated_pickup_transport_event: ::core::option::Option<cac::TransportEventType>,
    #[serde(default, rename = "UpdatedDeliveryTransportEvent")]
    pub updated_delivery_transport_event: ::core::option::Option<cac::TransportEventType>,
    #[serde(default, rename = "StatusLocation")]
    pub status_location: ::std::vec::Vec<cac::LocationType>,
    #[serde(default, rename = "StatusPeriod")]
    pub status_period: ::std::vec::Vec<cac::PeriodType>,
}

pub type WorkReport = WorkReportType;

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkReportType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "WorkQuantityTotal")]
    pub work_quantity_total: ::std::vec::Vec<cac::WorkQuantityTotalType>,
    #[serde(default, rename = "ReportedPeriod")]
    pub reported_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: ::core::option::Option<cac::OrderReferenceType>,
    #[serde(default, rename = "ProjectReference")]
    pub project_reference: ::core::option::Option<cac::ProjectReferenceType>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: ::std::vec::Vec<cac::BillingReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierPartyType,
    #[serde(rename = "BuyerCustomerParty")]
    pub buyer_customer_party: cac::CustomerPartyType,
    #[serde(default, rename = "ApproverParty")]
    pub approver_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<cac::AllowanceChargeType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(default, rename = "StatementMonetaryTotal")]
    pub statement_monetary_total: ::core::option::Option<cac::MonetaryTotalType>,
    #[serde(default, rename = "WorkReportLine")]
    pub work_report_line: ::std::vec::Vec<cac::WorkReportLineType>,
}

pub type SelfBilledCreditNote = SelfBilledCreditNoteType;

#[derive(Debug, Deserialize, Serialize)]
pub struct SelfBilledCreditNoteType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "DueDate")]
    pub due_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "CreditNoteTypeCode")]
    pub credit_note_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "TaxCurrencyCode")]
    pub tax_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PaymentCurrencyCode")]
    pub payment_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PaymentAlternativeCurrencyCode")]
    pub payment_alternative_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "BuyerReference")]
    pub buyer_reference: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "DefaultLanguageCode")]
    pub default_language_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(default, rename = "DiscrepancyResponse")]
    pub discrepancy_response: ::std::vec::Vec<cac::ResponseType>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: ::core::option::Option<cac::OrderReferenceType>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: ::std::vec::Vec<cac::BillingReferenceType>,
    #[serde(default, rename = "DespatchDocumentReference")]
    pub despatch_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "DeliveryNoteDocumentReference")]
    pub delivery_note_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "WorkReportDocumentReference")]
    pub work_report_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ReceiptDocumentReference")]
    pub receipt_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "StatementDocumentReference")]
    pub statement_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "OriginatorDocumentReference")]
    pub originator_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ProjectReference")]
    pub project_reference: ::std::vec::Vec<cac::ProjectReferenceType>,
    #[serde(default, rename = "BuyerAssignedReference")]
    pub buyer_assigned_reference: ::std::vec::Vec<cac::BuyerAssignedReferenceType>,
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: ::std::vec::Vec<cac::PurchaseReferenceType>,
    #[serde(default, rename = "Annotation")]
    pub annotation: ::std::vec::Vec<cac::AnnotationType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "AccountingCustomerParty")]
    pub accounting_customer_party: cac::CustomerPartyType,
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierPartyType,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "TaxRepresentativeParty")]
    pub tax_representative_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::std::vec::Vec<cac::DeliveryType>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: ::std::vec::Vec<cac::DeliveryTermsType>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: ::std::vec::Vec<cac::PaymentMeansType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<cac::PaymentTermsType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<cac::AllowanceChargeType>,
    #[serde(default, rename = "TaxExchangeRate")]
    pub tax_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PaymentExchangeRate")]
    pub payment_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PaymentAlternativeExchangeRate")]
    pub payment_alternative_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(default, rename = "WithholdingTaxTotal")]
    pub withholding_tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: cac::MonetaryTotalType,
    #[serde(default, rename = "CollectionCreditNoteLine")]
    pub collection_credit_note_line: ::std::vec::Vec<cac::CreditNoteLineType>,
    #[serde(default, rename = "CreditNoteLine")]
    pub credit_note_line: ::std::vec::Vec<cac::CreditNoteLineType>,
}

pub type Quotation = QuotationType;

#[derive(Debug, Deserialize, Serialize)]
pub struct QuotationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "RequestForQuotationDocumentReference")]
    pub request_for_quotation_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Contract")]
    pub contract: ::std::vec::Vec<cac::ContractType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierPartyType,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::std::vec::Vec<cac::DeliveryType>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: ::core::option::Option<cac::DeliveryTermsType>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: ::core::option::Option<cac::PaymentMeansType>,
    #[serde(default, rename = "TransactionConditions")]
    pub transaction_conditions: ::core::option::Option<cac::TransactionConditionsType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<cac::AllowanceChargeType>,
    #[serde(default, rename = "DestinationCountry")]
    pub destination_country: ::core::option::Option<cac::CountryType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(rename = "QuotedMonetaryTotal")]
    pub quoted_monetary_total: cac::MonetaryTotalType,
    #[serde(default, rename = "QuotationLine")]
    pub quotation_line: ::std::vec::Vec<cac::QuotationLineType>,
}

pub type ProcurementStatusRequest = ProcurementStatusRequestType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcurementStatusRequestType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ContractFolderID")]
    pub contract_folder_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "ContractName")]
    pub contract_name: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: ::std::vec::Vec<cac::ContractingPartyType>,
    #[serde(rename = "EconomicOperatorParty")]
    pub economic_operator_party: cac::EconomicOperatorPartyType,
    #[serde(default, rename = "TenderingProcess")]
    pub tendering_process: ::core::option::Option<cac::TenderingProcessType>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: ::core::option::Option<cac::ProcurementProjectType>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: ::std::vec::Vec<cac::ProcurementProjectLotType>,
}

pub type FulfilmentCancellation = FulfilmentCancellationType;

#[derive(Debug, Deserialize, Serialize)]
pub struct FulfilmentCancellationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "CancellationNote")]
    pub cancellation_note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "DespatchDocumentReference")]
    pub despatch_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ReceiptDocumentReference")]
    pub receipt_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: ::std::vec::Vec<cac::OrderReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Contract")]
    pub contract: ::std::vec::Vec<cac::ContractType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "DeliveryCustomerParty")]
    pub delivery_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "DespatchSupplierParty")]
    pub despatch_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
}

pub type ImportCustomsDeclaration = ImportCustomsDeclarationType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ImportCustomsDeclarationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "SubTypeCode")]
    pub sub_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "NatureOfTransactionCode")]
    pub nature_of_transaction_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "CustomsExitOfficeLocation")]
    pub customs_exit_office_location: ::core::option::Option<cac::LocationType>,
    #[serde(default, rename = "JurisdictionRegionAddress")]
    pub jurisdiction_region_address: ::core::option::Option<cac::AddressType>,
    #[serde(default, rename = "ImporterParty")]
    pub importer_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ConsigneeParty")]
    pub consignee_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "CustomsParty")]
    pub customs_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "NotifierParty")]
    pub notifier_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "Shipment")]
    pub shipment: cac::ShipmentType,
    #[serde(default, rename = "PreviousCustomsDeclaration")]
    pub previous_customs_declaration: ::core::option::Option<cac::CustomsDeclarationType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type GuaranteeCertificate = GuaranteeCertificateType;

#[derive(Debug, Deserialize, Serialize)]
pub struct GuaranteeCertificateType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::IdentifierType,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "GuaranteeTypeCode")]
    pub guarantee_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Purpose")]
    pub purpose: ::std::vec::Vec<cct::TextType>,
    #[serde(rename = "LiabilityAmount")]
    pub liability_amount: cct::AmountType,
    #[serde(default, rename = "ConstitutionCode")]
    pub constitution_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ApplicablePeriod")]
    pub applicable_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "ApplicableRegulation")]
    pub applicable_regulation: ::std::vec::Vec<cac::RegulationType>,
    #[serde(default, rename = "GuaranteeDocumentReference")]
    pub guarantee_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ImmobilizedSecurity")]
    pub immobilized_security: ::std::vec::Vec<cac::ImmobilizedSecurityType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "GuarantorParty")]
    pub guarantor_party: cac::PartyType,
    #[serde(rename = "InterestedParty")]
    pub interested_party: cac::PartyType,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::core::option::Option<cac::PartyType>,
}

pub type ForwardingInstructions = ForwardingInstructionsType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ForwardingInstructionsType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CarrierAssignedID")]
    pub carrier_assigned_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "ShippingOrderID")]
    pub shipping_order_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ToOrderIndicator")]
    pub to_order_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "AdValoremIndicator")]
    pub ad_valorem_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "DeclaredCarriageValueAmount")]
    pub declared_carriage_value_amount: ::core::option::Option<cct::AmountType>,
    #[serde(default, rename = "OtherInstruction")]
    pub other_instruction: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "Shipment")]
    pub shipment: cac::ShipmentType,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: ::std::vec::Vec<cac::ExchangeRateType>,
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: ::std::vec::Vec<cac::DocumentDistributionType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type UnsubscribeFromProcedureResponse = UnsubscribeFromProcedureResponseType;

#[derive(Debug, Deserialize, Serialize)]
pub struct UnsubscribeFromProcedureResponseType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ContractFolderID")]
    pub contract_folder_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "UnsubscribeToProcedureDocumentReference")]
    pub unsubscribe_to_procedure_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "EconomicOperatorParty")]
    pub economic_operator_party: cac::EconomicOperatorPartyType,
    #[serde(rename = "ContractingParty")]
    pub contracting_party: cac::ContractingPartyType,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: ::core::option::Option<cac::ProcurementProjectType>,
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: ::std::vec::Vec<cac::ProcurementProjectLotReferenceType>,
}

pub type RequestForQuotation = RequestForQuotationType;

#[derive(Debug, Deserialize, Serialize)]
pub struct RequestForQuotationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(rename = "IssueTime")]
    pub issue_time: udt::DateTimeType,
    #[serde(default, rename = "SubmissionDueDate")]
    pub submission_due_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "RequestedValidityPeriod")]
    pub requested_validity_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "CatalogueDocumentReference")]
    pub catalogue_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierPartyType,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::std::vec::Vec<cac::DeliveryType>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: ::std::vec::Vec<cac::DeliveryTermsType>,
    #[serde(default, rename = "DestinationCountry")]
    pub destination_country: ::core::option::Option<cac::CountryType>,
    #[serde(default, rename = "Contract")]
    pub contract: ::std::vec::Vec<cac::ContractType>,
    #[serde(default, rename = "RequestForQuotationLine")]
    pub request_for_quotation_line: ::std::vec::Vec<cac::RequestForQuotationLineType>,
}

pub type GoodsCertificate = GoodsCertificateType;

#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsCertificateType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "ApplicableTerritoryAddress")]
    pub applicable_territory_address: ::core::option::Option<cac::AddressType>,
    #[serde(default, rename = "ExporterParty")]
    pub exporter_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ImporterParty")]
    pub importer_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "WarehouseParty")]
    pub warehouse_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ConsigneeParty")]
    pub consignee_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "IssuerParty")]
    pub issuer_party: cac::PartyType,
    #[serde(default, rename = "LegalAuthorityParty")]
    pub legal_authority_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ApplicantParty")]
    pub applicant_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "Shipment")]
    pub shipment: cac::ShipmentType,
    #[serde(default, rename = "Attestation")]
    pub attestation: ::std::vec::Vec<cac::AttestationType>,
    #[serde(default, rename = "GoodsProcessing")]
    pub goods_processing: ::std::vec::Vec<cac::GoodsProcessingType>,
    #[serde(default, rename = "OriginalDocumentReference")]
    pub original_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "PreviousDocumentReference")]
    pub previous_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type TendererQualificationResponse = TendererQualificationResponseType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TendererQualificationResponseType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::IdentifierType,
    #[serde(default, rename = "ContractName")]
    pub contract_name: ::std::vec::Vec<cct::TextType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "ResolutionDocumentReference")]
    pub resolution_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "QualificationResolution")]
    pub qualification_resolution: ::std::vec::Vec<cac::QualificationResolutionType>,
    #[serde(default, rename = "AppealTerms")]
    pub appeal_terms: ::core::option::Option<cac::AppealTermsType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type CertificateOfOrigin = CertificateOfOriginType;

#[derive(Debug, Deserialize, Serialize)]
pub struct CertificateOfOriginType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "ExporterParty")]
    pub exporter_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ImporterParty")]
    pub importer_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "EndorserParty")]
    pub endorser_party: ::std::vec::Vec<cac::EndorserPartyType>,
    #[serde(rename = "CertificateOfOriginApplication")]
    pub certificate_of_origin_application: cac::CertificateOfOriginApplicationType,
    #[serde(rename = "IssuerEndorsement")]
    pub issuer_endorsement: cac::EndorsementType,
    #[serde(default, rename = "EmbassyEndorsement")]
    pub embassy_endorsement: ::core::option::Option<cac::EndorsementType>,
    #[serde(default, rename = "InsuranceEndorsement")]
    pub insurance_endorsement: ::core::option::Option<cac::EndorsementType>,
}

pub type TransportExecutionPlan = TransportExecutionPlanType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TransportExecutionPlanType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "DocumentStatusReasonCode")]
    pub document_status_reason_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "DocumentStatusReasonDescription")]
    pub document_status_reason_description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "TransportUserRemarks")]
    pub transport_user_remarks: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "TransportServiceProviderRemarks")]
    pub transport_service_provider_remarks: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "TransportUserParty")]
    pub transport_user_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "TransportServiceProviderParty")]
    pub transport_service_provider_party: cac::PartyType,
    #[serde(default, rename = "BillToParty")]
    pub bill_to_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "TransportExecutionPlanRequestDocumentReference")]
    pub transport_execution_plan_request_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "TransportExecutionPlanDocumentReference")]
    pub transport_execution_plan_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "TransportServiceDescriptionDocumentReference")]
    pub transport_service_description_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "TransportContract")]
    pub transport_contract: ::core::option::Option<cac::ContractType>,
    #[serde(default, rename = "TransportServiceProviderResponseRequiredPeriod")]
    pub transport_service_provider_response_required_period:
        ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "TransportUserResponseRequiredPeriod")]
    pub transport_user_response_required_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(default, rename = "MainTransportationService")]
    pub main_transportation_service: ::core::option::Option<cac::TransportationServiceType>,
    #[serde(default, rename = "AdditionalTransportationService")]
    pub additional_transportation_service: ::std::vec::Vec<cac::TransportationServiceType>,
    #[serde(default, rename = "ServiceStartTimePeriod")]
    pub service_start_time_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "ServiceEndTimePeriod")]
    pub service_end_time_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "FromLocation")]
    pub from_location: ::core::option::Option<cac::LocationType>,
    #[serde(default, rename = "ToLocation")]
    pub to_location: ::core::option::Option<cac::LocationType>,
    #[serde(default, rename = "AtLocation")]
    pub at_location: ::core::option::Option<cac::LocationType>,
    #[serde(default, rename = "TransportExecutionTerms")]
    pub transport_execution_terms: ::core::option::Option<cac::TransportExecutionTermsType>,
    #[serde(default, rename = "Consignment")]
    pub consignment: ::std::vec::Vec<cac::ConsignmentType>,
}

pub type ForecastRevision = ForecastRevisionType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastRevisionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(rename = "SequenceNumberID")]
    pub sequence_number_id: cct::IdentifierType,
    #[serde(default, rename = "RevisionStatusCode")]
    pub revision_status_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PurposeCode")]
    pub purpose_code: ::core::option::Option<cct::CodeType>,
    #[serde(rename = "ForecastPeriod")]
    pub forecast_period: cac::PeriodType,
    #[serde(default, rename = "OriginalDocumentReference")]
    pub original_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "ForecastRevisionLine")]
    pub forecast_revision_line: ::std::vec::Vec<cac::ForecastRevisionLineType>,
}

pub type Catalogue = CatalogueType;

#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogueType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ActionCode")]
    pub action_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<cct::TextType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "RevisionDate")]
    pub revision_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "RevisionTime")]
    pub revision_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(default, rename = "ReferencedContract")]
    pub referenced_contract: ::std::vec::Vec<cac::ContractType>,
    #[serde(default, rename = "SourceCatalogueReference")]
    pub source_catalogue_reference: ::core::option::Option<cac::CatalogueReferenceType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ApplicableTerritoryAddress")]
    pub applicable_territory_address: ::std::vec::Vec<cac::AddressType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "ProviderParty")]
    pub provider_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "TradingTerms")]
    pub trading_terms: ::std::vec::Vec<cac::TradingTermsType>,
    #[serde(default, rename = "CatalogueLine")]
    pub catalogue_line: ::std::vec::Vec<cac::CatalogueLineType>,
}

pub type TradeItemLocationProfile = TradeItemLocationProfileType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TradeItemLocationProfileType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ProfileStatusCode")]
    pub profile_status_code: ::core::option::Option<cct::CodeType>,
    #[serde(rename = "Period")]
    pub period: cac::PeriodType,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "ItemManagementProfile")]
    pub item_management_profile: ::std::vec::Vec<cac::ItemManagementProfileType>,
}

pub type WeightStatement = WeightStatementType;

#[derive(Debug, Deserialize, Serialize)]
pub struct WeightStatementType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "WeightStatementTypeCode")]
    pub weight_statement_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "WeighingParty")]
    pub weighing_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ShipperParty")]
    pub shipper_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ResponsibleParty")]
    pub responsible_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "Shipment")]
    pub shipment: cac::ShipmentType,
}

pub type TenderReceipt = TenderReceiptType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TenderReceiptType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::IdentifierType,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "ContractName")]
    pub contract_name: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(rename = "RegisteredDate")]
    pub registered_date: udt::DateTimeType,
    #[serde(rename = "RegisteredTime")]
    pub registered_time: udt::DateTimeType,
    #[serde(default, rename = "TenderDocumentReference")]
    pub tender_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
}

pub type ReceiptAdvice = ReceiptAdviceType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ReceiptAdviceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "ReceiptAdviceTypeCode")]
    pub receipt_advice_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "DeliveryAcceptanceCode")]
    pub delivery_acceptance_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "RejectReasonCode")]
    pub reject_reason_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "RejectReason")]
    pub reject_reason: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "RejectActionCode")]
    pub reject_action_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: ::std::vec::Vec<cac::OrderReferenceType>,
    #[serde(default, rename = "DespatchDocumentReference")]
    pub despatch_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "DeliveryCustomerParty")]
    pub delivery_customer_party: cac::CustomerPartyType,
    #[serde(rename = "DespatchSupplierParty")]
    pub despatch_supplier_party: cac::SupplierPartyType,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "Shipment")]
    pub shipment: ::core::option::Option<cac::ShipmentType>,
    #[serde(default, rename = "ReceiptLine")]
    pub receipt_line: ::std::vec::Vec<cac::ReceiptLineType>,
}

pub type GoodsItemPassport = GoodsItemPassportType;

#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsItemPassportType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "StatusCode")]
    pub status_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Status")]
    pub status: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ExportReasonCode")]
    pub export_reason_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "ExportReason")]
    pub export_reason: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "HolderParty")]
    pub holder_party: cac::PartyType,
    #[serde(default, rename = "RepresentativeParty")]
    pub representative_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ExportingGuarantorParty")]
    pub exporting_guarantor_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ImportingGuarantorParty")]
    pub importing_guarantor_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ExportingCustomsParty")]
    pub exporting_customs_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ImportingCustomsParty")]
    pub importing_customs_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "Shipment")]
    pub shipment: cac::ShipmentType,
    #[serde(default, rename = "GoodsItemPassportCounterfoil")]
    pub goods_item_passport_counterfoil: ::std::vec::Vec<cac::GoodsItemPassportCounterfoilType>,
    #[serde(default, rename = "IssuerEndorsement")]
    pub issuer_endorsement: ::core::option::Option<cac::EndorsementType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: ::std::vec::Vec<cac::DocumentDistributionType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type ApplicationResponse = ApplicationResponseType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ApplicationResponseType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "ResponseDate")]
    pub response_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "ResponseTime")]
    pub response_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "DocumentResponse")]
    pub document_response: ::std::vec::Vec<cac::DocumentResponseType>,
}

pub type ContractAwardNotice = ContractAwardNoticeType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractAwardNoticeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::IdentifierType,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "RequestedPublicationDate")]
    pub requested_publication_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "RegulatoryDomain")]
    pub regulatory_domain: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "NoticeTypeCode")]
    pub notice_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PublishAwardIndicator")]
    pub publish_award_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "NoticeLanguageCode")]
    pub notice_language_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AdditionalNoticeLanguage")]
    pub additional_notice_language: ::std::vec::Vec<cac::LanguageType>,
    #[serde(default, rename = "PreviousDocumentReference")]
    pub previous_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "MinutesDocumentReference")]
    pub minutes_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: ::std::vec::Vec<cac::ContractingPartyType>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "TenderingTerms")]
    pub tendering_terms: ::core::option::Option<cac::TenderingTermsType>,
    #[serde(default, rename = "TenderingProcess")]
    pub tendering_process: ::core::option::Option<cac::TenderingProcessType>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: ::core::option::Option<cac::ProcurementProjectType>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: ::std::vec::Vec<cac::ProcurementProjectLotType>,
    #[serde(default, rename = "TenderResult")]
    pub tender_result: ::std::vec::Vec<cac::TenderResultType>,
}

pub type CommonTransportationReport = CommonTransportationReportType;

#[derive(Debug, Deserialize, Serialize)]
pub struct CommonTransportationReportType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "ReportTypeCode")]
    pub report_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "ReportType")]
    pub report_type: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ReporterParty")]
    pub reporter_party: cac::PartyType,
    #[serde(default, rename = "AuthorityParty")]
    pub authority_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "ReportingLocation")]
    pub reporting_location: ::core::option::Option<cac::LocationType>,
    #[serde(default, rename = "Shipment")]
    pub shipment: ::core::option::Option<cac::ShipmentType>,
    #[serde(default, rename = "TransportMeans")]
    pub transport_means: ::std::vec::Vec<cac::TransportMeansType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type ProcurementStatus = ProcurementStatusType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProcurementStatusType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ContractFolderID")]
    pub contract_folder_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "ContractName")]
    pub contract_name: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ProcedureCode")]
    pub procedure_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "TenderSubmissionDeadlinePeriod")]
    pub tender_submission_deadline_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "InvitationSubmissionPeriod")]
    pub invitation_submission_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "ParticipationRequestReceptionPeriod")]
    pub participation_request_reception_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "ProcurementLegislationDocumentReference")]
    pub procurement_legislation_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "FiscalLegislationDocumentReference")]
    pub fiscal_legislation_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "EnvironmentalLegislationDocumentReference")]
    pub environmental_legislation_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "EmploymentLegislationDocumentReference")]
    pub employment_legislation_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "ProcedureStatusRequestDocumentReference")]
    pub procedure_status_request_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "ContractingParty")]
    pub contracting_party: cac::ContractingPartyType,
    #[serde(rename = "EconomicOperatorParty")]
    pub economic_operator_party: cac::EconomicOperatorPartyType,
    #[serde(default, rename = "DocumentProviderParty")]
    pub document_provider_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "TenderRecipientParty")]
    pub tender_recipient_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: ::core::option::Option<cac::ProcurementProjectType>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: ::std::vec::Vec<cac::ProcurementProjectLotType>,
}

pub type InventoryReport = InventoryReportType;

#[derive(Debug, Deserialize, Serialize)]
pub struct InventoryReportType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "InventoryPeriod")]
    pub inventory_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "RetailerCustomerParty")]
    pub retailer_customer_party: cac::CustomerPartyType,
    #[serde(rename = "InventoryReportingParty")]
    pub inventory_reporting_party: cac::PartyType,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "InventoryReportLine")]
    pub inventory_report_line: ::std::vec::Vec<cac::InventoryReportLineType>,
}

pub type BusinessInformation = BusinessInformationType;

#[derive(Debug, Deserialize, Serialize)]
pub struct BusinessInformationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "BriefDescription")]
    pub brief_description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "RequestedPublicationDate")]
    pub requested_publication_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "RegulatoryDomain")]
    pub regulatory_domain: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "NoticeTypeCode")]
    pub notice_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "NoticeLanguageCode")]
    pub notice_language_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AdditionalNoticeLanguage")]
    pub additional_notice_language: ::std::vec::Vec<cac::LanguageType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "BusinessParty")]
    pub business_party: cac::PartyType,
    #[serde(default, rename = "BrochureDocumentReference")]
    pub brochure_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "BusinessCapability")]
    pub business_capability: ::std::vec::Vec<cac::CapabilityType>,
    #[serde(default, rename = "BusinessPartyGroup")]
    pub business_party_group: ::std::vec::Vec<cac::PartyGroupType>,
    #[serde(default, rename = "OperationType")]
    pub operation_type: ::std::vec::Vec<cac::OperationTypeType>,
    #[serde(default, rename = "NoticeSubType")]
    pub notice_sub_type: ::core::option::Option<cac::NoticeSubTypeType>,
}

pub type AttachedDocument = AttachedDocumentType;

#[derive(Debug, Deserialize, Serialize)]
pub struct AttachedDocumentType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "DocumentTypeCode")]
    pub document_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "DocumentType")]
    pub document_type: ::core::option::Option<cct::TextType>,
    #[serde(rename = "ParentDocumentID")]
    pub parent_document_id: cct::IdentifierType,
    #[serde(default, rename = "ParentDocumentTypeCode")]
    pub parent_document_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "ParentDocumentVersionID")]
    pub parent_document_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(rename = "Attachment")]
    pub attachment: cac::AttachmentType,
    #[serde(default, rename = "ParentDocumentLineReference")]
    pub parent_document_line_reference: ::std::vec::Vec<cac::LineReferenceType>,
}

pub type DebitNote = DebitNoteType;

#[derive(Debug, Deserialize, Serialize)]
pub struct DebitNoteType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "DueDate")]
    pub due_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "DebitNoteTypeCode")]
    pub debit_note_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "TaxCurrencyCode")]
    pub tax_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PaymentCurrencyCode")]
    pub payment_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PaymentAlternativeCurrencyCode")]
    pub payment_alternative_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "DefaultLanguageCode")]
    pub default_language_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(default, rename = "DiscrepancyResponse")]
    pub discrepancy_response: ::std::vec::Vec<cac::ResponseType>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: ::core::option::Option<cac::OrderReferenceType>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: ::std::vec::Vec<cac::BillingReferenceType>,
    #[serde(default, rename = "DespatchDocumentReference")]
    pub despatch_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "DeliveryNoteDocumentReference")]
    pub delivery_note_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "WorkReportDocumentReference")]
    pub work_report_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ReceiptDocumentReference")]
    pub receipt_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "StatementDocumentReference")]
    pub statement_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "OriginatorDocumentReference")]
    pub originator_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ProjectReference")]
    pub project_reference: ::std::vec::Vec<cac::ProjectReferenceType>,
    #[serde(default, rename = "BuyerAssignedReference")]
    pub buyer_assigned_reference: ::std::vec::Vec<cac::BuyerAssignedReferenceType>,
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: ::std::vec::Vec<cac::PurchaseReferenceType>,
    #[serde(default, rename = "Annotation")]
    pub annotation: ::std::vec::Vec<cac::AnnotationType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierPartyType,
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "TaxRepresentativeParty")]
    pub tax_representative_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "PrepaidPayment")]
    pub prepaid_payment: ::std::vec::Vec<cac::PaymentType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<cac::AllowanceChargeType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::std::vec::Vec<cac::DeliveryType>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: ::std::vec::Vec<cac::DeliveryTermsType>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: ::std::vec::Vec<cac::PaymentMeansType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<cac::PaymentTermsType>,
    #[serde(default, rename = "TaxExchangeRate")]
    pub tax_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PaymentExchangeRate")]
    pub payment_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PaymentAlternativeExchangeRate")]
    pub payment_alternative_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(default, rename = "WithholdingTaxTotal")]
    pub withholding_tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(default, rename = "RequestedMonetaryTotal")]
    pub requested_monetary_total: ::core::option::Option<cac::MonetaryTotalType>,
    #[serde(default, rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: ::core::option::Option<cac::MonetaryTotalType>,
    #[serde(default, rename = "CollectionDebitNoteLine")]
    pub collection_debit_note_line: ::std::vec::Vec<cac::DebitNoteLineType>,
    #[serde(default, rename = "DebitNoteLine")]
    pub debit_note_line: ::std::vec::Vec<cac::DebitNoteLineType>,
}

pub type SelfBilledInvoice = SelfBilledInvoiceType;

#[derive(Debug, Deserialize, Serialize)]
pub struct SelfBilledInvoiceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "DueDate")]
    pub due_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "InvoiceTypeCode")]
    pub invoice_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "TaxCurrencyCode")]
    pub tax_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PaymentCurrencyCode")]
    pub payment_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PaymentAlternativeCurrencyCode")]
    pub payment_alternative_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "BuyerReference")]
    pub buyer_reference: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "DefaultLanguageCode")]
    pub default_language_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: ::core::option::Option<cac::OrderReferenceType>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: ::std::vec::Vec<cac::BillingReferenceType>,
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "DespatchDocumentReference")]
    pub despatch_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "WorkReportDocumentReference")]
    pub work_report_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "DeliveryNoteDocumentReference")]
    pub delivery_note_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ReceiptDocumentReference")]
    pub receipt_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "StatementDocumentReference")]
    pub statement_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "OriginatorDocumentReference")]
    pub originator_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ProjectReference")]
    pub project_reference: ::std::vec::Vec<cac::ProjectReferenceType>,
    #[serde(default, rename = "BuyerAssignedReference")]
    pub buyer_assigned_reference: ::std::vec::Vec<cac::BuyerAssignedReferenceType>,
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: ::std::vec::Vec<cac::PurchaseReferenceType>,
    #[serde(default, rename = "Annotation")]
    pub annotation: ::std::vec::Vec<cac::AnnotationType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "AccountingCustomerParty")]
    pub accounting_customer_party: cac::CustomerPartyType,
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierPartyType,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "TaxRepresentativeParty")]
    pub tax_representative_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::std::vec::Vec<cac::DeliveryType>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: ::std::vec::Vec<cac::DeliveryTermsType>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: ::core::option::Option<cac::PaymentMeansType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<cac::PaymentTermsType>,
    #[serde(default, rename = "PrepaidPayment")]
    pub prepaid_payment: ::std::vec::Vec<cac::PaymentType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<cac::AllowanceChargeType>,
    #[serde(default, rename = "TaxExchangeRate")]
    pub tax_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PaymentExchangeRate")]
    pub payment_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PaymentAlternativeExchangeRate")]
    pub payment_alternative_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(default, rename = "WithholdingTaxTotal")]
    pub withholding_tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: cac::MonetaryTotalType,
    #[serde(default, rename = "CollectionInvoiceLine")]
    pub collection_invoice_line: ::std::vec::Vec<cac::InvoiceLineType>,
    #[serde(default, rename = "InvoiceLine")]
    pub invoice_line: ::std::vec::Vec<cac::InvoiceLineType>,
}

pub type TenderWithdrawal = TenderWithdrawalType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TenderWithdrawalType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ContractFolderID")]
    pub contract_folder_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "ContractName")]
    pub contract_name: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "WithdrawOfferIndicator")]
    pub withdraw_offer_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "TenderDocumentReference")]
    pub tender_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "TenderNotificationDocumentReference")]
    pub tender_notification_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: ::std::vec::Vec<cac::ContractingPartyType>,
    #[serde(rename = "TendererParty")]
    pub tenderer_party: cac::PartyType,
}

pub type PackingList = PackingListType;

#[derive(Debug, Deserialize, Serialize)]
pub struct PackingListType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "OtherInstruction")]
    pub other_instruction: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "Shipment")]
    pub shipment: cac::ShipmentType,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: ::std::vec::Vec<cac::DocumentDistributionType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type ExportCustomsDeclaration = ExportCustomsDeclarationType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExportCustomsDeclarationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "ExportTypeCode")]
    pub export_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "ExportReasonCode")]
    pub export_reason_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ExporterParty")]
    pub exporter_party: cac::PartyType,
    #[serde(rename = "CustomsDeclaration")]
    pub customs_declaration: cac::CustomsDeclarationType,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type Invoice = InvoiceType;

#[derive(Debug, Deserialize, Serialize)]
pub struct InvoiceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "DueDate")]
    pub due_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "InvoiceTypeCode")]
    pub invoice_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "TaxCurrencyCode")]
    pub tax_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PaymentCurrencyCode")]
    pub payment_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PaymentAlternativeCurrencyCode")]
    pub payment_alternative_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "BuyerReference")]
    pub buyer_reference: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "DefaultLanguageCode")]
    pub default_language_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: ::core::option::Option<cac::OrderReferenceType>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: ::std::vec::Vec<cac::BillingReferenceType>,
    #[serde(default, rename = "DespatchDocumentReference")]
    pub despatch_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "DeliveryNoteDocumentReference")]
    pub delivery_note_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "WorkReportDocumentReference")]
    pub work_report_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ReceiptDocumentReference")]
    pub receipt_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "StatementDocumentReference")]
    pub statement_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "OriginatorDocumentReference")]
    pub originator_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ProjectReference")]
    pub project_reference: ::std::vec::Vec<cac::ProjectReferenceType>,
    #[serde(default, rename = "BuyerAssignedReference")]
    pub buyer_assigned_reference: ::std::vec::Vec<cac::BuyerAssignedReferenceType>,
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: ::std::vec::Vec<cac::PurchaseReferenceType>,
    #[serde(default, rename = "Annotation")]
    pub annotation: ::std::vec::Vec<cac::AnnotationType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierPartyType,
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "TaxRepresentativeParty")]
    pub tax_representative_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::std::vec::Vec<cac::DeliveryType>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: ::core::option::Option<cac::DeliveryTermsType>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: ::std::vec::Vec<cac::PaymentMeansType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<cac::PaymentTermsType>,
    #[serde(default, rename = "PrepaidPayment")]
    pub prepaid_payment: ::std::vec::Vec<cac::PaymentType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<cac::AllowanceChargeType>,
    #[serde(default, rename = "TaxExchangeRate")]
    pub tax_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PaymentExchangeRate")]
    pub payment_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PaymentAlternativeExchangeRate")]
    pub payment_alternative_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(default, rename = "WithholdingTaxTotal")]
    pub withholding_tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: cac::MonetaryTotalType,
    #[serde(default, rename = "CollectionInvoiceLine")]
    pub collection_invoice_line: ::std::vec::Vec<cac::InvoiceLineType>,
    #[serde(default, rename = "InvoiceLine")]
    pub invoice_line: ::std::vec::Vec<cac::InvoiceLineType>,
}

pub type OrderChange = OrderChangeType;

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderChangeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "SalesOrderID")]
    pub sales_order_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(rename = "SequenceNumberID")]
    pub sequence_number_id: cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "RequestedInvoiceCurrencyCode")]
    pub requested_invoice_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "TaxCurrencyCode")]
    pub tax_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "CustomerReference")]
    pub customer_reference: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(rename = "OrderReference")]
    pub order_reference: cac::OrderReferenceType,
    #[serde(default, rename = "QuotationDocumentReference")]
    pub quotation_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "OriginatorDocumentReference")]
    pub originator_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Contract")]
    pub contract: ::std::vec::Vec<cac::ContractType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "BuyerCustomerParty")]
    pub buyer_customer_party: cac::CustomerPartyType,
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierPartyType,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::std::vec::Vec<cac::DeliveryType>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: ::core::option::Option<cac::DeliveryTermsType>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: ::std::vec::Vec<cac::PaymentMeansType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<cac::PaymentTermsType>,
    #[serde(default, rename = "TransactionConditions")]
    pub transaction_conditions: ::core::option::Option<cac::TransactionConditionsType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<cac::AllowanceChargeType>,
    #[serde(default, rename = "TaxExchangeRate")]
    pub tax_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PaymentExchangeRate")]
    pub payment_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "DestinationCountry")]
    pub destination_country: ::core::option::Option<cac::CountryType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(default, rename = "AnticipatedMonetaryTotal")]
    pub anticipated_monetary_total: ::core::option::Option<cac::MonetaryTotalType>,
    #[serde(default, rename = "OrderLine")]
    pub order_line: ::std::vec::Vec<cac::OrderLineType>,
}

pub type OrderResponse = OrderResponseType;

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderResponseType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "SalesOrderID")]
    pub sales_order_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "OrderResponseCode")]
    pub order_response_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "TaxCurrencyCode")]
    pub tax_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "TotalPackagesQuantity")]
    pub total_packages_quantity: ::core::option::Option<cct::QuantityType>,
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: ::core::option::Option<cct::MeasureType>,
    #[serde(default, rename = "NetWeightMeasure")]
    pub net_weight_measure: ::core::option::Option<cct::MeasureType>,
    #[serde(default, rename = "NetNetWeightMeasure")]
    pub net_net_weight_measure: ::core::option::Option<cct::MeasureType>,
    #[serde(default, rename = "GrossVolumeMeasure")]
    pub gross_volume_measure: ::core::option::Option<cct::MeasureType>,
    #[serde(default, rename = "NetVolumeMeasure")]
    pub net_volume_measure: ::core::option::Option<cct::MeasureType>,
    #[serde(default, rename = "CustomerReference")]
    pub customer_reference: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: ::std::vec::Vec<cac::OrderReferenceType>,
    #[serde(default, rename = "OrderDocumentReference")]
    pub order_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "OrderChangeDocumentReference")]
    pub order_change_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "OriginatorDocumentReference")]
    pub originator_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Contract")]
    pub contract: ::std::vec::Vec<cac::ContractType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierPartyType,
    #[serde(rename = "BuyerCustomerParty")]
    pub buyer_customer_party: cac::CustomerPartyType,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::std::vec::Vec<cac::DeliveryType>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: ::core::option::Option<cac::DeliveryTermsType>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: ::std::vec::Vec<cac::PaymentMeansType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<cac::PaymentTermsType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<cac::AllowanceChargeType>,
    #[serde(default, rename = "TransactionConditions")]
    pub transaction_conditions: ::core::option::Option<cac::TransactionConditionsType>,
    #[serde(default, rename = "TaxExchangeRate")]
    pub tax_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PaymentExchangeRate")]
    pub payment_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "DestinationCountry")]
    pub destination_country: ::core::option::Option<cac::CountryType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(default, rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: ::core::option::Option<cac::MonetaryTotalType>,
    #[serde(default, rename = "OrderLine")]
    pub order_line: ::std::vec::Vec<cac::OrderLineType>,
}

pub type QualificationApplicationRequest = QualificationApplicationRequestType;

#[derive(Debug, Deserialize, Serialize)]
pub struct QualificationApplicationRequestType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::IdentifierType,
    #[serde(default, rename = "ContractName")]
    pub contract_name: ::std::vec::Vec<cct::TextType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProcedureCode")]
    pub procedure_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "QualificationApplicationTypeCode")]
    pub qualification_application_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "WeightScoringMethodologyNote")]
    pub weight_scoring_methodology_note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "WeightingTypeCode")]
    pub weighting_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: ::std::vec::Vec<cac::ContractingPartyType>,
    #[serde(default, rename = "EconomicOperatorParty")]
    pub economic_operator_party: ::std::vec::Vec<cac::EconomicOperatorPartyType>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: ::core::option::Option<cac::ProcurementProjectType>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: ::std::vec::Vec<cac::ProcurementProjectLotType>,
    #[serde(default, rename = "TenderingCriterion")]
    pub tendering_criterion: ::std::vec::Vec<cac::TenderingCriterionType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type OrderResponseSimple = OrderResponseSimpleType;

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderResponseSimpleType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(rename = "AcceptedIndicator")]
    pub accepted_indicator: udt::IndicatorType,
    #[serde(default, rename = "RejectionNote")]
    pub rejection_note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "CustomerReference")]
    pub customer_reference: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<cct::TextType>,
    #[serde(rename = "OrderReference")]
    pub order_reference: cac::OrderReferenceType,
    #[serde(default, rename = "OrderChangeDocumentReference")]
    pub order_change_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierPartyType,
    #[serde(rename = "BuyerCustomerParty")]
    pub buyer_customer_party: cac::CustomerPartyType,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: ::core::option::Option<cac::CustomerPartyType>,
}

pub type InstructionForReturns = InstructionForReturnsType;

#[derive(Debug, Deserialize, Serialize)]
pub struct InstructionForReturnsType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierPartyType,
    #[serde(rename = "RetailerCustomerParty")]
    pub retailer_customer_party: cac::CustomerPartyType,
    #[serde(default, rename = "ManufacturerParty")]
    pub manufacturer_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "Shipment")]
    pub shipment: ::core::option::Option<cac::ShipmentType>,
    #[serde(default, rename = "InstructionForReturnsLine")]
    pub instruction_for_returns_line: ::std::vec::Vec<cac::InstructionForReturnsLineType>,
}

pub type ProofOfReexportation = ProofOfReexportationType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofOfReexportationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ExportingCustomsParty")]
    pub exporting_customs_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ImportingGuarantorParty")]
    pub importing_guarantor_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ExportingGuarantorParty")]
    pub exporting_guarantor_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "GoodsItemPassportCounterfoil")]
    pub goods_item_passport_counterfoil: ::std::vec::Vec<cac::GoodsItemPassportCounterfoilType>,
    #[serde(default, rename = "ReexportationEvidence")]
    pub reexportation_evidence: ::std::vec::Vec<cac::EvidenceType>,
    #[serde(default, rename = "GoodsItemPassportAttachment")]
    pub goods_item_passport_attachment: ::core::option::Option<cac::AttachmentType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type ItemInformationRequest = ItemInformationRequestType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ItemInformationRequestType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(rename = "Period")]
    pub period: cac::PeriodType,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "ItemInformationRequestLine")]
    pub item_information_request_line: ::std::vec::Vec<cac::ItemInformationRequestLineType>,
}

pub type ContractNotice = ContractNoticeType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ContractNoticeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::IdentifierType,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "RequestedPublicationDate")]
    pub requested_publication_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "RegulatoryDomain")]
    pub regulatory_domain: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "NoticeTypeCode")]
    pub notice_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "NoticeLanguageCode")]
    pub notice_language_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AdditionalNoticeLanguage")]
    pub additional_notice_language: ::std::vec::Vec<cac::LanguageType>,
    #[serde(default, rename = "FrequencyPeriod")]
    pub frequency_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: ::std::vec::Vec<cac::ContractingPartyType>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::std::vec::Vec<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "TenderingTerms")]
    pub tendering_terms: ::core::option::Option<cac::TenderingTermsType>,
    #[serde(default, rename = "TenderingProcess")]
    pub tendering_process: ::core::option::Option<cac::TenderingProcessType>,
    #[serde(rename = "ProcurementProject")]
    pub procurement_project: cac::ProcurementProjectType,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: ::std::vec::Vec<cac::ProcurementProjectLotType>,
}

pub type TransportExecutionPlanRequest = TransportExecutionPlanRequestType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TransportExecutionPlanRequestType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "DocumentStatusReasonCode")]
    pub document_status_reason_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "DocumentStatusReasonDescription")]
    pub document_status_reason_description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "TransportUserRemarks")]
    pub transport_user_remarks: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "TransportUserParty")]
    pub transport_user_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "TransportServiceProviderParty")]
    pub transport_service_provider_party: cac::PartyType,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "BillToParty")]
    pub bill_to_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "TransportExecutionPlanDocumentReference")]
    pub transport_execution_plan_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "TransportServiceDescriptionDocumentReference")]
    pub transport_service_description_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "TransportContract")]
    pub transport_contract: ::core::option::Option<cac::ContractType>,
    #[serde(default, rename = "TransportServiceProviderResponseDeadlinePeriod")]
    pub transport_service_provider_response_deadline_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(default, rename = "MainTransportationService")]
    pub main_transportation_service: ::core::option::Option<cac::TransportationServiceType>,
    #[serde(default, rename = "AdditionalTransportationService")]
    pub additional_transportation_service: ::std::vec::Vec<cac::TransportationServiceType>,
    #[serde(default, rename = "ServiceStartTimePeriod")]
    pub service_start_time_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "ServiceEndTimePeriod")]
    pub service_end_time_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "FromLocation")]
    pub from_location: ::core::option::Option<cac::LocationType>,
    #[serde(default, rename = "ToLocation")]
    pub to_location: ::core::option::Option<cac::LocationType>,
    #[serde(default, rename = "AtLocation")]
    pub at_location: ::core::option::Option<cac::LocationType>,
    #[serde(default, rename = "TransportExecutionTerms")]
    pub transport_execution_terms: ::core::option::Option<cac::TransportExecutionTermsType>,
    #[serde(default, rename = "Consignment")]
    pub consignment: ::std::vec::Vec<cac::ConsignmentType>,
}

pub type CatalogueItemSpecificationUpdate = CatalogueItemSpecificationUpdateType;

#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogueItemSpecificationUpdateType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<cct::TextType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "RevisionDate")]
    pub revision_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "RevisionTime")]
    pub revision_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(rename = "RelatedCatalogueReference")]
    pub related_catalogue_reference: cac::CatalogueReferenceType,
    #[serde(default, rename = "ReferencedContract")]
    pub referenced_contract: ::std::vec::Vec<cac::ContractType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "ProviderParty")]
    pub provider_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "TradingTerms")]
    pub trading_terms: ::std::vec::Vec<cac::TradingTermsType>,
    #[serde(default, rename = "DefaultLanguage")]
    pub default_language: ::core::option::Option<cac::LanguageType>,
    #[serde(default, rename = "CatalogueItemSpecificationUpdateLine")]
    pub catalogue_item_specification_update_line:
        ::std::vec::Vec<cac::CatalogueItemSpecificationUpdateLineType>,
}

pub type DeliveryNote = DeliveryNoteType;

#[derive(Debug, Deserialize, Serialize)]
pub struct DeliveryNoteType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: ::std::vec::Vec<cac::OrderReferenceType>,
    #[serde(default, rename = "ProjectReference")]
    pub project_reference: ::std::vec::Vec<cac::ProjectReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "DespatchSupplierParty")]
    pub despatch_supplier_party: cac::SupplierPartyType,
    #[serde(rename = "DeliveryCustomerParty")]
    pub delivery_customer_party: cac::CustomerPartyType,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "Shipment")]
    pub shipment: ::core::option::Option<cac::ShipmentType>,
    #[serde(default, rename = "DespatchLine")]
    pub despatch_line: ::std::vec::Vec<cac::DespatchLineType>,
}

pub type Enquiry = EnquiryType;

#[derive(Debug, Deserialize, Serialize)]
pub struct EnquiryType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "LatestReplyDate")]
    pub latest_reply_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "LatestReplyTime")]
    pub latest_reply_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "RequestorParty")]
    pub requestor_party: cac::PartyType,
    #[serde(rename = "ResponderParty")]
    pub responder_party: cac::PartyType,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Attachment")]
    pub attachment: ::std::vec::Vec<cac::AttachmentType>,
}

pub type FreightInvoice = FreightInvoiceType;

#[derive(Debug, Deserialize, Serialize)]
pub struct FreightInvoiceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "DueDate")]
    pub due_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "InvoiceTypeCode")]
    pub invoice_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "TaxCurrencyCode")]
    pub tax_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PaymentCurrencyCode")]
    pub payment_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PaymentAlternativeCurrencyCode")]
    pub payment_alternative_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(default, rename = "Shipment")]
    pub shipment: ::std::vec::Vec<cac::ShipmentType>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: ::core::option::Option<cac::OrderReferenceType>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: ::std::vec::Vec<cac::BillingReferenceType>,
    #[serde(default, rename = "DespatchDocumentReference")]
    pub despatch_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ReceiptDocumentReference")]
    pub receipt_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "OriginatorDocumentReference")]
    pub originator_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ProjectReference")]
    pub project_reference: ::std::vec::Vec<cac::ProjectReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierPartyType,
    #[serde(rename = "AccountingCustomerParty")]
    pub accounting_customer_party: cac::CustomerPartyType,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "TaxRepresentativeParty")]
    pub tax_representative_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: ::std::vec::Vec<cac::PaymentMeansType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<cac::PaymentTermsType>,
    #[serde(default, rename = "PrepaidPayment")]
    pub prepaid_payment: ::std::vec::Vec<cac::PaymentType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<cac::AllowanceChargeType>,
    #[serde(default, rename = "TaxExchangeRate")]
    pub tax_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PaymentExchangeRate")]
    pub payment_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PaymentAlternativeExchangeRate")]
    pub payment_alternative_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(default, rename = "WithholdingTaxTotal")]
    pub withholding_tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: cac::MonetaryTotalType,
    #[serde(default, rename = "InvoiceLine")]
    pub invoice_line: ::std::vec::Vec<cac::InvoiceLineType>,
}

pub type TransportServiceDescriptionRequest = TransportServiceDescriptionRequestType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TransportServiceDescriptionRequestType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(rename = "IssueTime")]
    pub issue_time: udt::DateTimeType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ServiceInformationPreferenceCode")]
    pub service_information_preference_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "TransportServiceProviderParty")]
    pub transport_service_provider_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "TransportationService")]
    pub transportation_service: ::std::vec::Vec<cac::TransportationServiceType>,
}

pub type ExpressionOfInterestRequest = ExpressionOfInterestRequestType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExpressionOfInterestRequestType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ContractFolderID")]
    pub contract_folder_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "ContractName")]
    pub contract_name: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "PreferredLanguageLocaleCode")]
    pub preferred_language_locale_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "EconomicOperatorParty")]
    pub economic_operator_party: cac::EconomicOperatorPartyType,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: ::std::vec::Vec<cac::ContractingPartyType>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: ::core::option::Option<cac::ProcurementProjectType>,
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: ::std::vec::Vec<cac::ProcurementProjectLotReferenceType>,
}

pub type TenderStatusRequest = TenderStatusRequestType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TenderStatusRequestType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ContractFolderID")]
    pub contract_folder_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "ContractName")]
    pub contract_name: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: ::std::vec::Vec<cac::ContractingPartyType>,
    #[serde(rename = "EconomicOperatorParty")]
    pub economic_operator_party: cac::EconomicOperatorPartyType,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: ::core::option::Option<cac::ProcurementProjectType>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: ::std::vec::Vec<cac::ProcurementProjectLotType>,
}

pub type ExceptionCriteria = ExceptionCriteriaType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExceptionCriteriaType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ValidityPeriod")]
    pub validity_period: cac::PeriodType,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "ExceptionCriteriaLine")]
    pub exception_criteria_line: ::std::vec::Vec<cac::ExceptionCriteriaLineType>,
}

pub type DocumentStatus = DocumentStatusType;

#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentStatusType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "DocumentResponse")]
    pub document_response: ::core::option::Option<cac::DocumentResponseType>,
    #[serde(default, rename = "AdditionalDocumentResponse")]
    pub additional_document_response: ::std::vec::Vec<cac::DocumentResponseType>,
}

pub type Manifest = ManifestType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ManifestType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "ManifestTypeCode")]
    pub manifest_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "ManifestType")]
    pub manifest_type: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "AdValoremIndicator")]
    pub ad_valorem_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "DeclaredCarriageValueAmount")]
    pub declared_carriage_value_amount: ::core::option::Option<cct::AmountType>,
    #[serde(rename = "SendingLogisticsOperatorParty")]
    pub sending_logistics_operator_party: cac::PartyType,
    #[serde(default, rename = "AuthorityParty")]
    pub authority_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ConsigneeParty")]
    pub consignee_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "CrewPerson")]
    pub crew_person: ::std::vec::Vec<cac::PersonType>,
    #[serde(default, rename = "PassengerPerson")]
    pub passenger_person: ::std::vec::Vec<cac::PersonType>,
    #[serde(default, rename = "Shipment")]
    pub shipment: ::core::option::Option<cac::ShipmentType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: ::std::vec::Vec<cac::DocumentDistributionType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type DigitalAgreement = DigitalAgreementType;

#[derive(Debug, Deserialize, Serialize)]
pub struct DigitalAgreementType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "AgreementTypeCode")]
    pub agreement_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(rename = "VersionID")]
    pub version_id: cct::IdentifierType,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "RequiredResponseMessageLevelCode")]
    pub required_response_message_level_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "GovernorParty")]
    pub governor_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ParticipantParty")]
    pub participant_party: ::std::vec::Vec<cac::ParticipantPartyType>,
    #[serde(default, rename = "AgreementCountry")]
    pub agreement_country: ::std::vec::Vec<cac::CountryType>,
    #[serde(default, rename = "RequiredCertificationDocumentReference")]
    pub required_certification_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "DigitalAgreementTerms")]
    pub digital_agreement_terms: ::core::option::Option<cac::DigitalAgreementTermsType>,
    #[serde(default, rename = "DigitalProcess")]
    pub digital_process: ::std::vec::Vec<cac::DigitalProcessType>,
}

pub type WasteMovement = WasteMovementType;

#[derive(Debug, Deserialize, Serialize)]
pub struct WasteMovementType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "WasteMovementTypeCode")]
    pub waste_movement_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "SequenceNumberID")]
    pub sequence_number_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ConsignmentQuantity")]
    pub consignment_quantity: ::core::option::Option<cct::QuantityType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(rename = "NotifierParty")]
    pub notifier_party: cac::PartyType,
    #[serde(default, rename = "DisposalFacilityParty")]
    pub disposal_facility_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "RecoveryFacilityParty")]
    pub recovery_facility_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "WasteProducerParty")]
    pub waste_producer_party: cac::PartyType,
    #[serde(rename = "Shipment")]
    pub shipment: cac::ShipmentType,
    #[serde(default, rename = "WasteNotificationDocumentReference")]
    pub waste_notification_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "WeightStatementDocumentReference")]
    pub weight_statement_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: ::std::vec::Vec<cac::DocumentDistributionType>,
}

pub type ProofOfReexportationReminder = ProofOfReexportationReminderType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofOfReexportationReminderType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(rename = "ProcedureCode")]
    pub procedure_code: cct::CodeType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "GoodsItemPassportID")]
    pub goods_item_passport_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ProofOfReexportationRequestDocumentReference")]
    pub proof_of_reexportation_request_document_reference: cac::DocumentReferenceType,
    #[serde(rename = "ImportingGuarantorParty")]
    pub importing_guarantor_party: cac::PartyType,
    #[serde(rename = "ExportingGuarantorParty")]
    pub exporting_guarantor_party: cac::PartyType,
    #[serde(default, rename = "ImportingCustomsParty")]
    pub importing_customs_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "IssuerEndorsement")]
    pub issuer_endorsement: ::core::option::Option<cac::EndorsementType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<cac::PaymentTermsType>,
    #[serde(default, rename = "GoodsItemPassportCounterfoil")]
    pub goods_item_passport_counterfoil: ::std::vec::Vec<cac::GoodsItemPassportCounterfoilType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type Order = OrderType;

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "SalesOrderID")]
    pub sales_order_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "OrderTypeCode")]
    pub order_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "RequestedInvoiceCurrencyCode")]
    pub requested_invoice_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "TaxCurrencyCode")]
    pub tax_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "CustomerReference")]
    pub customer_reference: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(default, rename = "QuotationDocumentReference")]
    pub quotation_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "OrderDocumentReference")]
    pub order_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "OriginatorDocumentReference")]
    pub originator_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "CatalogueReference")]
    pub catalogue_reference: ::core::option::Option<cac::CatalogueReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Contract")]
    pub contract: ::std::vec::Vec<cac::ContractType>,
    #[serde(default, rename = "ProjectReference")]
    pub project_reference: ::std::vec::Vec<cac::ProjectReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "BuyerCustomerParty")]
    pub buyer_customer_party: cac::CustomerPartyType,
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierPartyType,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::std::vec::Vec<cac::DeliveryType>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: ::std::vec::Vec<cac::DeliveryTermsType>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: ::std::vec::Vec<cac::PaymentMeansType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<cac::PaymentTermsType>,
    #[serde(default, rename = "TransactionConditions")]
    pub transaction_conditions: ::core::option::Option<cac::TransactionConditionsType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<cac::AllowanceChargeType>,
    #[serde(default, rename = "TaxExchangeRate")]
    pub tax_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PaymentExchangeRate")]
    pub payment_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "DestinationCountry")]
    pub destination_country: ::core::option::Option<cac::CountryType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(default, rename = "AnticipatedMonetaryTotal")]
    pub anticipated_monetary_total: ::core::option::Option<cac::MonetaryTotalType>,
    #[serde(default, rename = "OrderLine")]
    pub order_line: ::std::vec::Vec<cac::OrderLineType>,
}

pub type DespatchAdvice = DespatchAdviceType;

#[derive(Debug, Deserialize, Serialize)]
pub struct DespatchAdviceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "DespatchAdviceTypeCode")]
    pub despatch_advice_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: ::std::vec::Vec<cac::OrderReferenceType>,
    #[serde(default, rename = "ProjectReference")]
    pub project_reference: ::std::vec::Vec<cac::ProjectReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "DespatchSupplierParty")]
    pub despatch_supplier_party: cac::SupplierPartyType,
    #[serde(rename = "DeliveryCustomerParty")]
    pub delivery_customer_party: cac::CustomerPartyType,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "Shipment")]
    pub shipment: ::core::option::Option<cac::ShipmentType>,
    #[serde(default, rename = "DespatchLine")]
    pub despatch_line: ::std::vec::Vec<cac::DespatchLineType>,
}

pub type CreditNote = CreditNoteType;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreditNoteType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "DueDate")]
    pub due_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "CreditNoteTypeCode")]
    pub credit_note_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "TaxCurrencyCode")]
    pub tax_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PaymentCurrencyCode")]
    pub payment_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PaymentAlternativeCurrencyCode")]
    pub payment_alternative_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "BuyerReference")]
    pub buyer_reference: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "DefaultLanguageCode")]
    pub default_language_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(default, rename = "DiscrepancyResponse")]
    pub discrepancy_response: ::std::vec::Vec<cac::ResponseType>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: ::core::option::Option<cac::OrderReferenceType>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: ::std::vec::Vec<cac::BillingReferenceType>,
    #[serde(default, rename = "DespatchDocumentReference")]
    pub despatch_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "DeliveryNoteDocumentReference")]
    pub delivery_note_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "WorkReportDocumentReference")]
    pub work_report_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ReceiptDocumentReference")]
    pub receipt_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "StatementDocumentReference")]
    pub statement_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "OriginatorDocumentReference")]
    pub originator_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ProjectReference")]
    pub project_reference: ::std::vec::Vec<cac::ProjectReferenceType>,
    #[serde(default, rename = "BuyerAssignedReference")]
    pub buyer_assigned_reference: ::std::vec::Vec<cac::BuyerAssignedReferenceType>,
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: ::std::vec::Vec<cac::PurchaseReferenceType>,
    #[serde(default, rename = "Annotation")]
    pub annotation: ::std::vec::Vec<cac::AnnotationType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierPartyType,
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "TaxRepresentativeParty")]
    pub tax_representative_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::std::vec::Vec<cac::DeliveryType>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: ::std::vec::Vec<cac::DeliveryTermsType>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: ::std::vec::Vec<cac::PaymentMeansType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<cac::PaymentTermsType>,
    #[serde(default, rename = "TaxExchangeRate")]
    pub tax_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PaymentExchangeRate")]
    pub payment_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PaymentAlternativeExchangeRate")]
    pub payment_alternative_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<cac::AllowanceChargeType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(default, rename = "WithholdingTaxTotal")]
    pub withholding_tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: cac::MonetaryTotalType,
    #[serde(default, rename = "CollectionCreditNoteLine")]
    pub collection_credit_note_line: ::std::vec::Vec<cac::CreditNoteLineType>,
    #[serde(default, rename = "CreditNoteLine")]
    pub credit_note_line: ::std::vec::Vec<cac::CreditNoteLineType>,
}

pub type QualificationApplicationResponse = QualificationApplicationResponseType;

#[derive(Debug, Deserialize, Serialize)]
pub struct QualificationApplicationResponseType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::IdentifierType,
    #[serde(default, rename = "ContractName")]
    pub contract_name: ::std::vec::Vec<cct::TextType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "EconomicOperatorGroupName")]
    pub economic_operator_group_name: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProcedureCode")]
    pub procedure_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "QualificationApplicationTypeCode")]
    pub qualification_application_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "WeightScoringMethodologyNote")]
    pub weight_scoring_methodology_note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "WeightingTypeCode")]
    pub weighting_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: ::std::vec::Vec<cac::ContractingPartyType>,
    #[serde(default, rename = "EconomicOperatorParty")]
    pub economic_operator_party: ::std::vec::Vec<cac::EconomicOperatorPartyType>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: ::core::option::Option<cac::ProcurementProjectType>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: ::std::vec::Vec<cac::ProcurementProjectLotType>,
    #[serde(default, rename = "TenderingCriterion")]
    pub tendering_criterion: ::std::vec::Vec<cac::TenderingCriterionType>,
    #[serde(default, rename = "TenderingCriterionResponse")]
    pub tendering_criterion_response: ::std::vec::Vec<cac::TenderingCriterionResponseType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Evidence")]
    pub evidence: ::std::vec::Vec<cac::EvidenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type Statement = StatementType;

#[derive(Debug, Deserialize, Serialize)]
pub struct StatementType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(rename = "DocumentCurrencyCode")]
    pub document_currency_code: cct::CodeType,
    #[serde(default, rename = "TotalDebitAmount")]
    pub total_debit_amount: ::core::option::Option<cct::AmountType>,
    #[serde(default, rename = "TotalCreditAmount")]
    pub total_credit_amount: ::core::option::Option<cct::AmountType>,
    #[serde(default, rename = "TotalBalanceAmount")]
    pub total_balance_amount: ::core::option::Option<cct::AmountType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "StatementTypeCode")]
    pub statement_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "StatementPeriod")]
    pub statement_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierPartyType,
    #[serde(rename = "AccountingCustomerParty")]
    pub accounting_customer_party: cac::CustomerPartyType,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: ::std::vec::Vec<cac::PaymentMeansType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<cac::PaymentTermsType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<cac::AllowanceChargeType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(default, rename = "StatementLine")]
    pub statement_line: ::std::vec::Vec<cac::StatementLineType>,
}

pub type Tender = TenderType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TenderType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "TenderTypeCode")]
    pub tender_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::IdentifierType,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "ContractName")]
    pub contract_name: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "CallForTenderDocumentReference")]
    pub call_for_tender_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "TendererParty")]
    pub tenderer_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "TendererQualificationDocumentReference")]
    pub tenderer_qualification_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "SubcontractorParty")]
    pub subcontractor_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: ::std::vec::Vec<cac::ContractingPartyType>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "TenderedProject")]
    pub tendered_project: ::std::vec::Vec<cac::TenderedProjectType>,
}

pub type UtilityStatement = UtilityStatementType;

#[derive(Debug, Deserialize, Serialize)]
pub struct UtilityStatementType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(rename = "UtilityStatementTypeCode")]
    pub utility_statement_type_code: cct::CodeType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(rename = "DocumentCurrencyCode")]
    pub document_currency_code: cct::CodeType,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<cct::TextType>,
    #[serde(rename = "ParentDocumentReference")]
    pub parent_document_reference: cac::DocumentReferenceType,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "CustomerParty")]
    pub customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SubscriberParty")]
    pub subscriber_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "MainOnAccountPayment")]
    pub main_on_account_payment: ::std::vec::Vec<cac::OnAccountPaymentType>,
    #[serde(default, rename = "SubscriberConsumption")]
    pub subscriber_consumption: ::std::vec::Vec<cac::SubscriberConsumptionType>,
}

pub type UnawardedNotification = UnawardedNotificationType;

#[derive(Debug, Deserialize, Serialize)]
pub struct UnawardedNotificationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::IdentifierType,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "ContractName")]
    pub contract_name: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "MinutesDocumentReference")]
    pub minutes_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "TenderResult")]
    pub tender_result: ::std::vec::Vec<cac::TenderResultType>,
    #[serde(default, rename = "AppealTerms")]
    pub appeal_terms: ::core::option::Option<cac::AppealTermsType>,
}

pub type CallForTenders = CallForTendersType;

#[derive(Debug, Deserialize, Serialize)]
pub struct CallForTendersType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::IdentifierType,
    #[serde(default, rename = "ApprovalDate")]
    pub approval_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "LegalDocumentReference")]
    pub legal_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "TechnicalDocumentReference")]
    pub technical_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "RequiredDocumentReference")]
    pub required_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ProvidedDocumentReference")]
    pub provided_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: ::std::vec::Vec<cac::ContractingPartyType>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::std::vec::Vec<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "TenderingTerms")]
    pub tendering_terms: ::core::option::Option<cac::TenderingTermsType>,
    #[serde(default, rename = "TenderingProcess")]
    pub tendering_process: ::core::option::Option<cac::TenderingProcessType>,
    #[serde(rename = "ProcurementProject")]
    pub procurement_project: cac::ProcurementProjectType,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: ::std::vec::Vec<cac::ProcurementProjectLotType>,
}

pub type CatalogueRequest = CatalogueRequestType;

#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogueRequestType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<cct::TextType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "PricingUpdateRequestIndicator")]
    pub pricing_update_request_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "ItemUpdateRequestIndicator")]
    pub item_update_request_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(rename = "ProviderParty")]
    pub provider_party: cac::PartyType,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "RequestedCatalogueReference")]
    pub requested_catalogue_reference: ::core::option::Option<cac::CatalogueReferenceType>,
    #[serde(default, rename = "ReferencedContract")]
    pub referenced_contract: ::std::vec::Vec<cac::ContractType>,
    #[serde(default, rename = "TradingTerms")]
    pub trading_terms: ::std::vec::Vec<cac::TradingTermsType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ApplicableTerritoryAddress")]
    pub applicable_territory_address: ::std::vec::Vec<cac::AddressType>,
    #[serde(default, rename = "RequestedLanguage")]
    pub requested_language: ::core::option::Option<cac::LanguageType>,
    #[serde(default, rename = "RequestedClassificationScheme")]
    pub requested_classification_scheme: ::std::vec::Vec<cac::ClassificationSchemeType>,
    #[serde(default, rename = "CatalogueRequestLine")]
    pub catalogue_request_line: ::std::vec::Vec<cac::CatalogueRequestLineType>,
}

pub type StockAvailabilityReport = StockAvailabilityReportType;

#[derive(Debug, Deserialize, Serialize)]
pub struct StockAvailabilityReportType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "InventoryPeriod")]
    pub inventory_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierPartyType,
    #[serde(default, rename = "RetailerCustomerParty")]
    pub retailer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(rename = "InventoryReportingParty")]
    pub inventory_reporting_party: cac::PartyType,
    #[serde(default, rename = "StockAvailabilityReportLine")]
    pub stock_availability_report_line: ::std::vec::Vec<cac::StockAvailabilityReportLineType>,
}

pub type TransportServiceDescription = TransportServiceDescriptionType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TransportServiceDescriptionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ServiceName")]
    pub service_name: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "ResponseCode")]
    pub response_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(
        default,
        rename = "TransportServiceDescriptionRequestDocumentReference"
    )]
    pub transport_service_description_request_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "TransportServiceProviderParty")]
    pub transport_service_provider_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ServiceChargePaymentTerms")]
    pub service_charge_payment_terms: ::core::option::Option<cac::PaymentTermsType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "TransportationService")]
    pub transportation_service: ::std::vec::Vec<cac::TransportationServiceType>,
}

pub type ExpressionOfInterestResponse = ExpressionOfInterestResponseType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ExpressionOfInterestResponseType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ContractFolderID")]
    pub contract_folder_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "ContractName")]
    pub contract_name: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "TenderLanguageLocaleCode")]
    pub tender_language_locale_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ExpressionOfInterestDocumentReference")]
    pub expression_of_interest_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "EconomicOperatorParty")]
    pub economic_operator_party: cac::EconomicOperatorPartyType,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: ::std::vec::Vec<cac::ContractingPartyType>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: ::core::option::Option<cac::ProcurementProjectType>,
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: ::std::vec::Vec<cac::ProcurementProjectLotReferenceType>,
}

pub type TendererQualification = TendererQualificationType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TendererQualificationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::IdentifierType,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "TendererPartyQualification")]
    pub tenderer_party_qualification: ::std::vec::Vec<cac::TendererPartyQualificationType>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: ::core::option::Option<cac::ContractingPartyType>,
    #[serde(default, rename = "Evidence")]
    pub evidence: ::std::vec::Vec<cac::EvidenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
}

pub type InvoiceStatusRequest = InvoiceStatusRequestType;

#[derive(Debug, Deserialize, Serialize)]
pub struct InvoiceStatusRequestType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "RequestDate")]
    pub request_date: udt::DateTimeType,
    #[serde(default, rename = "RequestTime")]
    pub request_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: ::std::vec::Vec<cac::BillingReferenceType>,
}

pub type Reminder = ReminderType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ReminderType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "ReminderTypeCode")]
    pub reminder_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "ReminderSequenceNumeric")]
    pub reminder_sequence_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "TaxCurrencyCode")]
    pub tax_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PricingCurrencyCode")]
    pub pricing_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PaymentCurrencyCode")]
    pub payment_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PaymentAlternativeCurrencyCode")]
    pub payment_alternative_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "ReminderPeriod")]
    pub reminder_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierPartyType,
    #[serde(rename = "AccountingCustomerParty")]
    pub accounting_customer_party: cac::CustomerPartyType,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "TaxRepresentativeParty")]
    pub tax_representative_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: ::std::vec::Vec<cac::PaymentMeansType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<cac::PaymentTermsType>,
    #[serde(default, rename = "PrepaidPayment")]
    pub prepaid_payment: ::std::vec::Vec<cac::PaymentType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<cac::AllowanceChargeType>,
    #[serde(default, rename = "TaxExchangeRate")]
    pub tax_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PaymentExchangeRate")]
    pub payment_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PaymentAlternativeExchangeRate")]
    pub payment_alternative_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: cac::MonetaryTotalType,
    #[serde(default, rename = "ReminderLine")]
    pub reminder_line: ::std::vec::Vec<cac::ReminderLineType>,
}

pub type BillOfLading = BillOfLadingType;

#[derive(Debug, Deserialize, Serialize)]
pub struct BillOfLadingType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CarrierAssignedID")]
    pub carrier_assigned_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "ShippingOrderID")]
    pub shipping_order_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ToOrderIndicator")]
    pub to_order_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "AdValoremIndicator")]
    pub ad_valorem_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "DeclaredCarriageValueAmount")]
    pub declared_carriage_value_amount: ::core::option::Option<cct::AmountType>,
    #[serde(default, rename = "OtherInstruction")]
    pub other_instruction: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "Shipment")]
    pub shipment: ::core::option::Option<cac::ShipmentType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: ::std::vec::Vec<cac::ExchangeRateType>,
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: ::std::vec::Vec<cac::DocumentDistributionType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type DocumentStatusRequest = DocumentStatusRequestType;

#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentStatusRequestType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "TrackingID")]
    pub tracking_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "RequestedDocumentReference")]
    pub requested_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
}

pub type CataloguePricingUpdate = CataloguePricingUpdateType;

#[derive(Debug, Deserialize, Serialize)]
pub struct CataloguePricingUpdateType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<cct::TextType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "RevisionDate")]
    pub revision_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "RevisionTime")]
    pub revision_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "LineCountNumeric")]
    pub line_count_numeric: ::core::option::Option<cct::NumericType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(rename = "RelatedCatalogueReference")]
    pub related_catalogue_reference: cac::CatalogueReferenceType,
    #[serde(default, rename = "ReferencedContract")]
    pub referenced_contract: ::std::vec::Vec<cac::ContractType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "ProviderParty")]
    pub provider_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "TradingTerms")]
    pub trading_terms: ::std::vec::Vec<cac::TradingTermsType>,
    #[serde(default, rename = "DefaultLanguage")]
    pub default_language: ::core::option::Option<cac::LanguageType>,
    #[serde(default, rename = "CataloguePricingUpdateLine")]
    pub catalogue_pricing_update_line: ::std::vec::Vec<cac::CataloguePricingUpdateLineType>,
}

pub type TransportProgressStatus = TransportProgressStatusType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TransportProgressStatusType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(rename = "IssueTime")]
    pub issue_time: udt::DateTimeType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "StatusAvailableIndicator")]
    pub status_available_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "SourceIssuerParty")]
    pub source_issuer_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "TransportProgressStatusRequestDocumentReference")]
    pub transport_progress_status_request_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(rename = "TransportMeans")]
    pub transport_means: cac::TransportMeansType,
    #[serde(default, rename = "TransportSchedule")]
    pub transport_schedule: ::std::vec::Vec<cac::TransportScheduleType>,
}

pub type ProofOfReexportationRequest = ProofOfReexportationRequestType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProofOfReexportationRequestType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "GoodsItemPassportID")]
    pub goods_item_passport_id: cct::IdentifierType,
    #[serde(default, rename = "GoodsItemPassportCounterfoilID")]
    pub goods_item_passport_counterfoil_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ImportingGuarantorParty")]
    pub importing_guarantor_party: cac::PartyType,
    #[serde(rename = "ExportingGuarantorParty")]
    pub exporting_guarantor_party: cac::PartyType,
    #[serde(default, rename = "ImportingCustomsParty")]
    pub importing_customs_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type CatalogueDeletion = CatalogueDeletionType;

#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogueDeletionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<cct::TextType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "EffectiveDate")]
    pub effective_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "EffectiveTime")]
    pub effective_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::std::vec::Vec<cac::PeriodType>,
    #[serde(rename = "DeletedCatalogueReference")]
    pub deleted_catalogue_reference: cac::CatalogueReferenceType,
    #[serde(default, rename = "ReferencedContract")]
    pub referenced_contract: ::std::vec::Vec<cac::ContractType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(rename = "ProviderParty")]
    pub provider_party: cac::PartyType,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: ::core::option::Option<cac::CustomerPartyType>,
}

pub type BusinessCard = BusinessCardType;

#[derive(Debug, Deserialize, Serialize)]
pub struct BusinessCardType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "BriefDescription")]
    pub brief_description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "BusinessParty")]
    pub business_party: cac::PartyType,
    #[serde(default, rename = "BrochureDocumentReference")]
    pub brochure_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "BusinessCapability")]
    pub business_capability: ::std::vec::Vec<cac::CapabilityType>,
}

pub type TransitCustomsDeclaration = TransitCustomsDeclarationType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TransitCustomsDeclarationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "SubTypeCode")]
    pub sub_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "NatureOfTransactionCode")]
    pub nature_of_transaction_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "ExportCustomsExitOfficeLocation")]
    pub export_customs_exit_office_location: ::core::option::Option<cac::LocationType>,
    #[serde(default, rename = "TransitCustomsExitOfficeLocation")]
    pub transit_customs_exit_office_location: ::core::option::Option<cac::LocationType>,
    #[serde(default, rename = "ImportCustomsExitOfficeLocation")]
    pub import_customs_exit_office_location: ::core::option::Option<cac::LocationType>,
    #[serde(default, rename = "JurisdictionRegionAddress")]
    pub jurisdiction_region_address: ::core::option::Option<cac::AddressType>,
    #[serde(default, rename = "TransitExporterParty")]
    pub transit_exporter_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ConsigneeParty")]
    pub consignee_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "CustomsParty")]
    pub customs_party: cac::PartyType,
    #[serde(default, rename = "NotifierParty")]
    pub notifier_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "Shipment")]
    pub shipment: ::std::vec::Vec<cac::ShipmentType>,
    #[serde(default, rename = "PreviousCustomsDeclaration")]
    pub previous_customs_declaration: ::std::vec::Vec<cac::CustomsDeclarationType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type EnquiryResponse = EnquiryResponseType;

#[derive(Debug, Deserialize, Serialize)]
pub struct EnquiryResponseType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "RequestorParty")]
    pub requestor_party: cac::PartyType,
    #[serde(rename = "ResponderParty")]
    pub responder_party: cac::PartyType,
    #[serde(rename = "ParentDocumentReference")]
    pub parent_document_reference: cac::DocumentReferenceType,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Attachment")]
    pub attachment: ::std::vec::Vec<cac::AttachmentType>,
}

pub type Forecast = ForecastType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "BasedOnConsensusIndicator")]
    pub based_on_consensus_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(rename = "ForecastPurposeCode")]
    pub forecast_purpose_code: cct::CodeType,
    #[serde(rename = "ForecastPeriod")]
    pub forecast_period: cac::PeriodType,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "ForecastLine")]
    pub forecast_line: ::std::vec::Vec<cac::ForecastLineType>,
}

pub type TransportationStatusRequest = TransportationStatusRequestType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TransportationStatusRequestType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CarrierAssignedID")]
    pub carrier_assigned_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ShippingOrderID")]
    pub shipping_order_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "OtherInstruction")]
    pub other_instruction: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "TransportationStatusTypeCode")]
    pub transportation_status_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "TransportExecutionPlanDocumentReference")]
    pub transport_execution_plan_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "Consignment")]
    pub consignment: ::std::vec::Vec<cac::ConsignmentType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "RequestedStatusLocation")]
    pub requested_status_location: ::std::vec::Vec<cac::LocationType>,
    #[serde(default, rename = "RequestedStatusPeriod")]
    pub requested_status_period: ::std::vec::Vec<cac::PeriodType>,
}

pub type OrderCancellation = OrderCancellationType;

#[derive(Debug, Deserialize, Serialize)]
pub struct OrderCancellationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "CancellationNote")]
    pub cancellation_note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: ::std::vec::Vec<cac::OrderReferenceType>,
    #[serde(default, rename = "OriginatorDocumentReference")]
    pub originator_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Contract")]
    pub contract: ::std::vec::Vec<cac::ContractType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "BuyerCustomerParty")]
    pub buyer_customer_party: cac::CustomerPartyType,
    #[serde(rename = "SellerSupplierParty")]
    pub seller_supplier_party: cac::SupplierPartyType,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
}

pub type Waybill = WaybillType;

#[derive(Debug, Deserialize, Serialize)]
pub struct WaybillType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CarrierAssignedID")]
    pub carrier_assigned_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "StatusCode")]
    pub status_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ShippingOrderID")]
    pub shipping_order_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "WaybillTypeCode")]
    pub waybill_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "ConsolidatedIndicator")]
    pub consolidated_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "AdValoremIndicator")]
    pub ad_valorem_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "DeclaredCarriageValueAmount")]
    pub declared_carriage_value_amount: ::core::option::Option<cct::AmountType>,
    #[serde(default, rename = "OtherInstruction")]
    pub other_instruction: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "IssueLocation")]
    pub issue_location: ::core::option::Option<cac::LocationType>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "Shipment")]
    pub shipment: cac::ShipmentType,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: ::std::vec::Vec<cac::ExchangeRateType>,
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: ::std::vec::Vec<cac::DocumentDistributionType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type DigitalCapability = DigitalCapabilityType;

#[derive(Debug, Deserialize, Serialize)]
pub struct DigitalCapabilityType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "BusinessParty")]
    pub business_party: cac::PartyType,
    #[serde(default, rename = "DigitalProcess")]
    pub digital_process: ::std::vec::Vec<cac::DigitalProcessType>,
}

pub type AwardedNotification = AwardedNotificationType;

#[derive(Debug, Deserialize, Serialize)]
pub struct AwardedNotificationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::IdentifierType,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "ContractName")]
    pub contract_name: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "MinutesDocumentReference")]
    pub minutes_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "TenderResult")]
    pub tender_result: ::std::vec::Vec<cac::TenderResultType>,
    #[serde(default, rename = "FinalFinancialGuarantee")]
    pub final_financial_guarantee: ::std::vec::Vec<cac::FinancialGuaranteeType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
}

pub type TenderContract = TenderContractType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TenderContractType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ContractFolderID")]
    pub contract_folder_id: cct::IdentifierType,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "ContractName")]
    pub contract_name: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "RegulatoryDomain")]
    pub regulatory_domain: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "PublishAwardIndicator")]
    pub publish_award_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "PreviousDocumentReference")]
    pub previous_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: ::std::vec::Vec<cac::ContractingPartyType>,
    #[serde(default, rename = "EconomicOperatorParty")]
    pub economic_operator_party: ::std::vec::Vec<cac::EconomicOperatorPartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "TenderingTerms")]
    pub tendering_terms: ::core::option::Option<cac::TenderingTermsType>,
    #[serde(default, rename = "TenderingProcess")]
    pub tendering_process: ::core::option::Option<cac::TenderingProcessType>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: ::core::option::Option<cac::ProcurementProjectType>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: ::std::vec::Vec<cac::ProcurementProjectLotType>,
    #[serde(default, rename = "TenderResult")]
    pub tender_result: ::std::vec::Vec<cac::TenderResultType>,
}

pub type ProductActivity = ProductActivityType;

#[derive(Debug, Deserialize, Serialize)]
pub struct ProductActivityType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(rename = "ActivityPeriod")]
    pub activity_period: cac::PeriodType,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "SupplyChainActivityDataLine")]
    pub supply_chain_activity_data_line: ::std::vec::Vec<cac::ActivityDataLineType>,
}

pub type TenderStatus = TenderStatusType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TenderStatusType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ContractFolderID")]
    pub contract_folder_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "ContractName")]
    pub contract_name: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "ProcedureCode")]
    pub procedure_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "TenderSubmissionDeadlinePeriod")]
    pub tender_submission_deadline_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "InvitationSubmissionPeriod")]
    pub invitation_submission_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "ParticipationRequestReceptionPeriod")]
    pub participation_request_reception_period: ::core::option::Option<cac::PeriodType>,
    #[serde(default, rename = "ProcurementLegislationDocumentReference")]
    pub procurement_legislation_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "FiscalLegislationDocumentReference")]
    pub fiscal_legislation_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "EnvironmentalLegislationDocumentReference")]
    pub environmental_legislation_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "EmploymentLegislationDocumentReference")]
    pub employment_legislation_document_reference:
        ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "TenderStatusInquiryDocumentReference")]
    pub tender_status_inquiry_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "ContractingParty")]
    pub contracting_party: cac::ContractingPartyType,
    #[serde(rename = "EconomicOperatorParty")]
    pub economic_operator_party: cac::EconomicOperatorPartyType,
    #[serde(default, rename = "DocumentProviderParty")]
    pub document_provider_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "TenderRecipientParty")]
    pub tender_recipient_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: ::core::option::Option<cac::ProcurementProjectType>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: ::std::vec::Vec<cac::ProcurementProjectLotType>,
}

pub type UnsubscribeFromProcedureRequest = UnsubscribeFromProcedureRequestType;

#[derive(Debug, Deserialize, Serialize)]
pub struct UnsubscribeFromProcedureRequestType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ContractFolderID")]
    pub contract_folder_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "EconomicOperatorParty")]
    pub economic_operator_party: cac::EconomicOperatorPartyType,
    #[serde(rename = "ContractingParty")]
    pub contracting_party: cac::ContractingPartyType,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: ::core::option::Option<cac::ProcurementProjectType>,
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: ::std::vec::Vec<cac::ProcurementProjectLotReferenceType>,
}

pub type TransportProgressStatusRequest = TransportProgressStatusRequestType;

#[derive(Debug, Deserialize, Serialize)]
pub struct TransportProgressStatusRequestType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(rename = "IssueTime")]
    pub issue_time: udt::DateTimeType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "TransportMeans")]
    pub transport_means: cac::TransportMeansType,
    #[serde(default, rename = "StatusLocation")]
    pub status_location: ::std::vec::Vec<cac::LocationType>,
}

pub type PriorInformationNotice = PriorInformationNoticeType;

#[derive(Debug, Deserialize, Serialize)]
pub struct PriorInformationNoticeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ContractFolderID")]
    pub contract_folder_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "RequestedPublicationDate")]
    pub requested_publication_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "PlannedDate")]
    pub planned_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "RegulatoryDomain")]
    pub regulatory_domain: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "NoticeTypeCode")]
    pub notice_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "NoticeLanguageCode")]
    pub notice_language_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "AdditionalNoticeLanguage")]
    pub additional_notice_language: ::std::vec::Vec<cac::LanguageType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "ContractingParty")]
    pub contracting_party: ::std::vec::Vec<cac::ContractingPartyType>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::std::vec::Vec<cac::CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "TenderingTerms")]
    pub tendering_terms: ::core::option::Option<cac::TenderingTermsType>,
    #[serde(default, rename = "TenderingProcess")]
    pub tendering_process: ::core::option::Option<cac::TenderingProcessType>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: ::core::option::Option<cac::ProcurementProjectType>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: ::std::vec::Vec<cac::ProcurementProjectLotType>,
}

pub type GoodsItemItinerary = GoodsItemItineraryType;

#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsItemItineraryType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(rename = "IssueTime")]
    pub issue_time: udt::DateTimeType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(rename = "VersionID")]
    pub version_id: cct::IdentifierType,
    #[serde(rename = "TransportExecutionPlanReferenceID")]
    pub transport_execution_plan_reference_id: cct::IdentifierType,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ReferencedConsignment")]
    pub referenced_consignment: ::std::vec::Vec<cac::ConsignmentType>,
    #[serde(default, rename = "ReferencedTransportEquipment")]
    pub referenced_transport_equipment: ::std::vec::Vec<cac::TransportEquipmentType>,
    #[serde(default, rename = "ReferencedPackage")]
    pub referenced_package: ::std::vec::Vec<cac::PackageType>,
    #[serde(default, rename = "ReferencedGoodsItem")]
    pub referenced_goods_item: ::std::vec::Vec<cac::GoodsItemType>,
    #[serde(default, rename = "TransportationSegment")]
    pub transportation_segment: ::std::vec::Vec<cac::TransportationSegmentType>,
}

pub type InvoiceStatusResponse = InvoiceStatusResponseType;

#[derive(Debug, Deserialize, Serialize)]
pub struct InvoiceStatusResponseType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(default, rename = "SenderParty")]
    pub sender_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "ReceiverParty")]
    pub receiver_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "Payment")]
    pub payment: ::std::vec::Vec<cac::PaymentType>,
    #[serde(default, rename = "DocumentResponse")]
    pub document_response: ::std::vec::Vec<cac::DocumentResponseType>,
}

pub type WasteNotification = WasteNotificationType;

#[derive(Debug, Deserialize, Serialize)]
pub struct WasteNotificationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "WasteNotificationTypeCode")]
    pub waste_notification_type_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "ConsignmentQuantity")]
    pub consignment_quantity: ::core::option::Option<cct::QuantityType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(rename = "NotifierParty")]
    pub notifier_party: cac::PartyType,
    #[serde(default, rename = "CustomsParty")]
    pub customs_party: ::std::vec::Vec<cac::PartyType>,
    #[serde(default, rename = "DisposalFacilityParty")]
    pub disposal_facility_party: ::core::option::Option<cac::PartyType>,
    #[serde(default, rename = "RecoveryFacilityParty")]
    pub recovery_facility_party: ::core::option::Option<cac::PartyType>,
    #[serde(rename = "WasteProducerParty")]
    pub waste_producer_party: cac::PartyType,
    #[serde(rename = "Shipment")]
    pub shipment: cac::ShipmentType,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
}

pub type RetailEvent = RetailEventType;

#[derive(Debug, Deserialize, Serialize)]
pub struct RetailEventType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "RetailEventName")]
    pub retail_event_name: ::core::option::Option<cct::TextType>,
    #[serde(rename = "RetailEventStatusCode")]
    pub retail_event_status_code: cct::CodeType,
    #[serde(default, rename = "SellerEventID")]
    pub seller_event_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "BuyerEventID")]
    pub buyer_event_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<cct::TextType>,
    #[serde(rename = "Period")]
    pub period: cac::PeriodType,
    #[serde(default, rename = "OriginalDocumentReference")]
    pub original_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "SenderParty")]
    pub sender_party: cac::PartyType,
    #[serde(rename = "ReceiverParty")]
    pub receiver_party: cac::PartyType,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<cac::SupplierPartyType>,
    #[serde(default, rename = "EventComment")]
    pub event_comment: ::std::vec::Vec<cac::EventCommentType>,
    #[serde(default, rename = "PromotionalEvent")]
    pub promotional_event: ::core::option::Option<cac::PromotionalEventType>,
    #[serde(default, rename = "MiscellaneousEvent")]
    pub miscellaneous_event: ::core::option::Option<cac::MiscellaneousEventType>,
}

pub type PurchaseReceipt = PurchaseReceiptType;

#[derive(Debug, Deserialize, Serialize)]
pub struct PurchaseReceiptType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UBLVersionID")]
    pub ubl_version_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(default, rename = "ProfileExecutionID")]
    pub profile_execution_id: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "ID")]
    pub id: cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<cct::IdentifierType>,
    #[serde(rename = "IssueDate")]
    pub issue_date: udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "TransactionDate")]
    pub transaction_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "TransactionTime")]
    pub transaction_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "PurchaseDate")]
    pub purchase_date: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "PurchaseTime")]
    pub purchase_time: ::core::option::Option<udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<cct::TextType>,
    #[serde(default, rename = "DocumentCurrencyCode")]
    pub document_currency_code: ::core::option::Option<cct::CodeType>,
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: ::std::vec::Vec<cac::PurchaseReferenceType>,
    #[serde(default, rename = "SalesDocumentReference")]
    pub sales_document_reference: ::core::option::Option<cac::DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<cac::DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<cac::SignatureType>,
    #[serde(rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: cac::SupplierPartyType,
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: ::core::option::Option<cac::CustomerPartyType>,
    #[serde(default, rename = "CashierContact")]
    pub cashier_contact: ::core::option::Option<cac::ContactType>,
    #[serde(default, rename = "CashRegister")]
    pub cash_register: ::core::option::Option<cac::CashRegisterType>,
    #[serde(default, rename = "PointOfSaleLocation")]
    pub point_of_sale_location: ::core::option::Option<cac::LocationType>,
    #[serde(default, rename = "PointOfSaleContact")]
    pub point_of_sale_contact: ::core::option::Option<cac::ContactType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::core::option::Option<cac::DeliveryType>,
    #[serde(default, rename = "Payment")]
    pub payment: ::std::vec::Vec<cac::PaymentType>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: ::std::vec::Vec<cac::PaymentMeansType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<cac::AllowanceChargeType>,
    #[serde(default, rename = "TaxExchangeRate")]
    pub tax_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: ::core::option::Option<cac::ExchangeRateType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<cac::TaxTotalType>,
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: cac::MonetaryTotalType,
    #[serde(default, rename = "PurchaseReceiptLine")]
    pub purchase_receipt_line: ::std::vec::Vec<cac::PurchaseReceiptLineType>,
}

