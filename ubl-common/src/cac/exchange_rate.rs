// UBL Exchange Rate aggregate.

use crate::cbc::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ExchangeRate {
    pub source_currency_code: SourceCurrencyCode,
    pub source_currency_base_rate: Option<SourceCurrencyBaseRate>,
    pub target_currency_code: TargetCurrencyCode,
    pub target_currency_base_rate: Option<TargetCurrencyBaseRate>,
    pub exchange_market_id: Option<ExchangeMarketID>,
    pub calculation_rate: Option<CalculationRate>,
    pub mathematic_operator_code: Option<MathematicOperatorCode>,
    pub date: Option<Date>,
    pub foreign_exchange_contract: Option<Contract>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contract {
    pub id: Option<ID>,
    pub issue_date: Option<IssueDate>,
    pub issue_time: Option<IssueTime>,
    pub contract_type_code: Option<ContractTypeCode>,
    pub contract_type: Option<ContractType>,
    pub validity_period: Option<Period>,
    #[serde(default)]
    pub contract_document_reference: Vec<DocumentReference>,
}

use crate::cac::document::DocumentReference;
use crate::cac::period::Period;
