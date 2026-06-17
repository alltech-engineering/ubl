// UBL Item Price Extension aggregate.
// Extends the price of an item with additional monetary detail.
//
// UBL element: cac:ItemPriceExtension

use crate::cbc::*;
use serde::{Deserialize, Serialize};

/// Price extension for a line item — additional price detail.
/// UBL element: cac:ItemPriceExtension
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ItemPriceExtension {
    pub amount: Amount,
}
