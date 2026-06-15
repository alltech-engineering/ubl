// UBL Exchange Rate aggregate.

use crate::cbc::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub source_currency_code: SourceCurrencyCode,
    #[serde(default)]
    pub source_currency_base_rate: Option<SourceCurrencyBaseRate>,
    pub target_currency_code: TargetCurrencyCode,
    #[serde(default)]
    pub target_currency_base_rate: Option<TargetCurrencyBaseRate>,
    #[serde(default)]
    pub exchange_market_id: Option<ExchangeMarketID>,
    #[serde(default)]
    pub calculation_rate: Option<CalculationRate>,
    #[serde(default)]
    pub mathematic_operator_code: Option<MathematicOperatorCode>,
    #[serde(default)]
    pub date: Option<Date>,
    #[serde(default)]
    pub foreign_exchange_contract: Option<Contract>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contract {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub issue_date: Option<IssueDate>,
    #[serde(default)]
    pub issue_time: Option<IssueTime>,
    #[serde(default)]
    pub contract_type_code: Option<ContractTypeCode>,
    #[serde(default)]
    pub contract_type: Option<ContractType>,
    #[serde(default)]
    pub validity_period: Option<Period>,
    #[serde(default)]
    pub contract_document_reference: Vec<DocumentReference>,
}

use crate::cac::document::DocumentReference;
use crate::cac::period::Period;
