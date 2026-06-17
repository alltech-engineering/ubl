// UBL Payment aggregates — payment means, terms, and financial account.

use crate::cbc::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMeans {
    #[serde(default)]
    pub id: Option<ID>,
    pub payment_means_code: PaymentMeansCode,
    #[serde(default)]
    pub payment_due_date: Option<PaymentDueDate>,
    #[serde(default)]
    pub payment_channel_code: Option<Code>,
    #[serde(default)]
    pub instruction_id: Option<InstructionID>,
    #[serde(default)]
    pub instruction_note: Vec<Text>,
    #[serde(default)]
    pub payment_id: Vec<PaymentID>,
    #[serde(default)]
    pub card_account: Option<CardAccount>,
    #[serde(default)]
    pub payer_financial_account: Option<FinancialAccount>,
    #[serde(default)]
    pub payee_financial_account: Option<FinancialAccount>,
    #[serde(default)]
    pub credit_account: Option<CreditAccount>,
    #[serde(default)]
    pub payment_mandate: Option<PaymentMandate>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CardAccount {
    pub primary_account_number_id: ID,
    #[serde(default)]
    pub network_id: Option<NetworkID>,
    #[serde(default)]
    pub card_type_code: Option<CardTypeCode>,
    #[serde(default)]
    pub validity_start_date: Option<ValidityStartDate>,
    #[serde(default)]
    pub expiry_date: Option<ExpiryDate>,
    #[serde(default)]
    pub issuer_id: Option<IssuerID>,
    #[serde(default)]
    pub issue_number_id: Option<ID>,
    #[serde(default)]
    pub cv2_id: Option<ID>,
    #[serde(default)]
    pub card_chip_code: Option<CardChipCode>,
    #[serde(default)]
    pub chip_application_id: Option<ChipApplicationID>,
    #[serde(default)]
    pub holder_name: Option<HolderName>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccount {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub name: Option<Name>,
    #[serde(default)]
    pub alias_name: Option<AliasName>,
    #[serde(default)]
    pub account_type_code: Option<AccountTypeCode>,
    #[serde(default)]
    pub account_format_code: Option<AccountFormatCode>,
    #[serde(default)]
    pub currency_code: Option<CurrencyCode>,
    #[serde(default)]
    pub payment_note: Vec<PaymentNote>,
    #[serde(default)]
    pub blockchain_id: Option<BlockchainID>,
    #[serde(default)]
    pub financial_institution_branch: Option<Branch>,
    #[serde(default)]
    pub country: Option<Country>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Branch {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub name: Option<Name>,
    #[serde(default)]
    pub financial_institution: Option<FinancialInstitution>,
    #[serde(default)]
    pub address: Option<Address>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialInstitution {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub name: Option<Name>,
    #[serde(default)]
    pub address: Option<Address>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CreditAccount {
    pub account_id: AccountID,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMandate {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub mandate_type_code: Option<MandateTypeCode>,
    #[serde(default)]
    pub maximum_payment_instructions_numeric: Option<MaximumPaymentInstructionsNumeric>,
    #[serde(default)]
    pub maximum_paid_amount: Option<MaximumAmount>,
    #[serde(default)]
    pub signature_id: Option<SignatureID>,
    #[serde(default)]
    pub payer_party: Option<Party>,
    #[serde(default)]
    pub payer_financial_account: Option<FinancialAccount>,
}

use crate::cac::address::{Address, Country};
use crate::cac::party::Party;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentTerms {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub payment_means_id: Vec<PaymentMeansID>,
    #[serde(default)]
    pub prepaid_payment_reference_id: Option<PrepaidPaymentReferenceID>,
    #[serde(default)]
    pub note: Vec<Note>,
    #[serde(default)]
    pub reference_event_code: Option<Code>,
    #[serde(default)]
    pub settlement_discount_percent: Option<SettlementDiscountPercent>,
    #[serde(default)]
    pub penalty_surcharge_percent: Option<PenaltySurchargePercent>,
    #[serde(default)]
    pub amount: Option<Amount>,
    #[serde(default)]
    pub settlement_period: Option<Period>,
    #[serde(default)]
    pub penalty_period: Option<Period>,
    #[serde(default)]
    pub payment_percent: Option<PaymentPercent>,
    #[serde(default)]
    pub settlement_discount_amount: Option<SettlementDiscountAmount>,
    #[serde(default)]
    pub penalty_amount: Option<PenaltyAmount>,
    #[serde(default)]
    pub payment_terms_details_uri: Option<PaymentTermsDetailsURI>,
    #[serde(default)]
    pub payment_due_date: Option<PaymentDueDate>,
    #[serde(default)]
    pub installment_due_date: Option<InstallmentDueDate>,
    #[serde(default)]
    pub invoicing_party_reference: Option<InvoicingPartyReference>,
    #[serde(default)]
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
