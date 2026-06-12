// UBL Payment aggregates — payment means, terms, and financial account.

use serde::{Deserialize, Serialize};
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaymentMeans {
    pub id: Option<ID>,
    pub payment_means_code: PaymentMeansCode,
    pub payment_due_date: Option<PaymentDueDate>,
    pub payment_channel_code: Option<Code>,
    pub instruction_id: Option<InstructionID>,
    pub instruction_note: Vec<Text>,
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
    pub payment_note: Vec<PaymentNote>,
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
