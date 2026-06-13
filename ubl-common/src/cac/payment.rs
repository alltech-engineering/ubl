// UBL Payment aggregates — payment means, terms, and financial account.

use crate::cbc::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMeans {
    pub id: Option<ID>,
    pub payment_means_code: PaymentMeansCode,
    pub payment_due_date: Option<PaymentDueDate>,
    pub payment_channel_code: Option<Code>,
    pub instruction_id: Option<InstructionID>,
    #[serde(default)]
    pub instruction_note: Vec<Text>,
    #[serde(default)]
    pub payment_id: Vec<PaymentID>,
    pub card_account: Option<CardAccount>,
    pub payer_financial_account: Option<FinancialAccount>,
    pub payee_financial_account: Option<FinancialAccount>,
    pub credit_account: Option<CreditAccount>,
    pub payment_mandate: Option<PaymentMandate>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardAccount {
    pub primary_account_number_id: ID,
    pub network_id: Option<NetworkID>,
    pub card_type_code: Option<CardTypeCode>,
    pub validity_start_date: Option<ValidityStartDate>,
    pub expiry_date: Option<ExpiryDate>,
    pub issuer_id: Option<IssuerID>,
    pub issue_number_id: Option<ID>,
    pub cv2_id: Option<ID>,
    pub card_chip_code: Option<CardChipCode>,
    pub chip_application_id: Option<ChipApplicationID>,
    pub holder_name: Option<HolderName>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccount {
    pub id: Option<ID>,
    pub name: Option<Name>,
    pub alias_name: Option<AliasName>,
    pub account_type_code: Option<AccountTypeCode>,
    pub account_format_code: Option<AccountFormatCode>,
    pub currency_code: Option<CurrencyCode>,
    #[serde(default)]
    pub payment_note: Vec<PaymentNote>,
    pub blockchain_id: Option<BlockchainID>,
    pub financial_institution_branch: Option<Branch>,
    pub country: Option<Country>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Branch {
    pub id: Option<ID>,
    pub name: Option<Name>,
    pub financial_institution: Option<FinancialInstitution>,
    pub address: Option<Address>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialInstitution {
    pub id: Option<ID>,
    pub name: Option<Name>,
    pub address: Option<Address>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditAccount {
    pub account_id: AccountID,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMandate {
    pub id: Option<ID>,
    pub mandate_type_code: Option<MandateTypeCode>,
    pub maximum_payment_instructions_numeric: Option<MaximumPaymentInstructionsNumeric>,
    pub maximum_paid_amount: Option<MaximumAmount>,
    pub signature_id: Option<SignatureID>,
    pub payer_party: Option<Party>,
    pub payer_financial_account: Option<FinancialAccount>,
}

use crate::cac::address::{Address, Country};
use crate::cac::party::Party;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentTerms {
    pub id: Option<ID>,
    #[serde(default)]
    pub payment_means_id: Vec<PaymentMeansID>,
    pub prepaid_payment_reference_id: Option<PrepaidPaymentReferenceID>,
    #[serde(default)]
    pub note: Vec<Note>,
    pub reference_event_code: Option<Code>,
    pub settlement_discount_percent: Option<SettlementDiscountPercent>,
    pub penalty_surcharge_percent: Option<PenaltySurchargePercent>,
    pub amount: Option<Amount>,
    pub settlement_period: Option<Period>,
    pub penalty_period: Option<Period>,
    pub payment_percent: Option<PaymentPercent>,
    pub settlement_discount_amount: Option<SettlementDiscountAmount>,
    pub penalty_amount: Option<PenaltyAmount>,
    pub payment_terms_details_uri: Option<PaymentTermsDetailsURI>,
    pub payment_due_date: Option<PaymentDueDate>,
    pub installment_due_date: Option<InstallmentDueDate>,
    pub invoicing_party_reference: Option<InvoicingPartyReference>,
    pub exchange_rate: Option<ExchangeRate>,
}

use crate::cac::exchange_rate::ExchangeRate;
use crate::cac::period::Period;

/// Information about a payment that has been made or will be made.
/// UBL element: cac:Payment
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Payment {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_amount: Option<PaidAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_date: Option<ReceivedDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_date: Option<PaidDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_time: Option<PaidTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction_id: Option<InstructionID>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_means: Option<PaymentMeans>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub note: Vec<Note>,
}
