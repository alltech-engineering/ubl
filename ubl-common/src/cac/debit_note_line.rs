// DebitNoteLine — UBL CAC aggregate
// A line in a Debit Note document.
use crate::cbc::*;

#[derive(Debug, Clone, Partialserde::Serialize, serde::Deserialize)]
pub struct DebitNoteLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<Uuid>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub note: Vec<Note>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debited_quantity: Option<Quantity>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_extension_amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_point_date: Option<TaxPointDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost_code: Option<AccountingCostCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_cost: Option<AccountingCost>,
}
