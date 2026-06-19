#[derive(Debug, Deserialize, Serialize)]
/// A class to describe information about a charge or discount as applied to a price component.
///
/// UBL Dictionary Entry Name: `Allowance Charge. Details`
///
/// Generated from XSD type `AllowanceChargeType`.
pub struct AllowanceCharge {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this allowance or charge.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// An indicator that this AllowanceCharge describes a charge (true) or a discount (false).
    #[serde(rename = "ChargeIndicator")]
    pub charge_indicator: udt::Indicator,
/// A mutually agreed code signifying the reason for this allowance or charge.
    #[serde(default, rename = "AllowanceChargeReasonCode")]
    pub allowance_charge_reason_code: Option<cct::Code>,
/// The reason for this allowance or charge.
    #[serde(default, rename = "AllowanceChargeReason")]
    pub allowance_charge_reason: Vec<cct::Text>,
/// A number by which the base amount is multiplied to calculate the actual amount of this allowance or
/// charge.
    #[serde(default, rename = "MultiplierFactorNumeric")]
    pub multiplier_factor_numeric: Option<cct::Numeric>,
/// An indicator that this allowance or charge is prepaid (true) or not (false).
    #[serde(default, rename = "PrepaidIndicator")]
    pub prepaid_indicator: Option<udt::Indicator>,
/// A number indicating the order of this allowance or charge in the sequence of calculations applied
/// when there are multiple allowances or charges.
    #[serde(default, rename = "SequenceNumeric")]
    pub sequence_numeric: Option<cct::Numeric>,
/// The monetary amount of this allowance or charge to be applied.
    #[serde(rename = "Amount")]
    pub amount: cct::Amount,
/// The monetary amount of this allowance or charge inclusive of all taxes.
    #[serde(default, rename = "TaxInclusiveAmount")]
    pub tax_inclusive_amount: Option<cct::Amount>,
/// The monetary amount to which the multiplier factor is applied in calculating the amount of this
/// allowance or charge.
    #[serde(default, rename = "BaseAmount")]
    pub base_amount: Option<cct::Amount>,
/// The accounting cost centre used by the buyer to account for this allowance or charge, expressed as a
/// code.
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: Option<cct::Code>,
/// The accounting cost centre used by the buyer to account for this allowance or charge, expressed as
/// text.
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: Option<cct::Text>,
/// The allowance or charge per item; the total allowance or charge is calculated by multiplying the per
/// unit amount by the quantity of items, either at the level of the individual transaction line or for
/// the total number of items in the document, depending on the context in which it appears.
    #[serde(default, rename = "PerUnitAmount")]
    pub per_unit_amount: Option<cct::Amount>,
/// A tax category applicable to this allowance or charge.
    #[serde(default, rename = "TaxCategory")]
    pub tax_category: Vec<TaxCategory>,
/// The total of all the taxes applicable to this allowance or charge.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Option<TaxTotal>,
/// A means of payment for this allowance or charge.
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: Vec<PaymentMeans>,
}
