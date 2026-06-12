// UBL Price aggregate — price with amount and quantity context.

use serde::{Deserialize, Serialize};
use crate::cbc::*;

use crate::cac::allowance::AllowanceCharge;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub price_amount: PriceAmount,
    pub tax_inclusive_price_amount: Option<TaxInclusivePriceAmount>,
    pub base_quantity: Option<BaseQuantity>,
    #[serde(default)]
    pub price_change_reason: Vec<Text>,
    pub price_type: Option<PriceTypeCode>,
    pub price_type_code: Option<PriceTypeCode>,
    pub orderable_unit_factor_rate: Option<OrderableUnitFactorRate>,
    #[serde(default)]
    pub validity_period: Vec<Period>,
    pub price_list: Option<PriceList>,
    #[serde(default)]
    pub allowance_charge: Vec<AllowanceCharge>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceList {
    pub id: Option<ID>,
    pub status_code: Option<StatusCode>,
    #[serde(default)]
    pub validity_period: Vec<Period>,
    pub previous_price_list: Option<Box<PriceList>>,
}

use crate::cac::period::Period;
