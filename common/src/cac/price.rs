// UBL Price aggregate — price with amount and quantity context.

use crate::cbc::*;
use serde::{Deserialize, Serialize};

use crate::cac::allowance::AllowanceCharge;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Price {
    pub price_amount: PriceAmount,
    #[serde(default)]
    pub tax_inclusive_price_amount: Option<TaxInclusivePriceAmount>,
    #[serde(default)]
    pub base_quantity: Option<BaseQuantity>,
    #[serde(default)]
    pub price_change_reason: Vec<Text>,
    #[serde(default)]
    pub price_type: Option<PriceTypeCode>,
    #[serde(default)]
    pub price_type_code: Option<PriceTypeCode>,
    #[serde(default)]
    pub orderable_unit_factor_rate: Option<OrderableUnitFactorRate>,
    #[serde(default)]
    pub validity_period: Vec<Period>,
    #[serde(default)]
    pub price_list: Option<PriceList>,
    #[serde(default)]
    pub allowance_charge: Vec<AllowanceCharge>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PriceList {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub status_code: Option<StatusCode>,
    #[serde(default)]
    pub validity_period: Vec<Period>,
    #[serde(default)]
    pub previous_price_list: Option<Box<PriceList>>,
}

use crate::cac::period::Period;
