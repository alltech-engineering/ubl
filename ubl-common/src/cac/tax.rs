// Tax — UBL CAC aggregate (Tier 1 stubs)
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaxTotal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<Amount>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub tax_subtotal: Vec<TaxSubtotal>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaxSubtotal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxable_amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amount: Option<Amount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<Percent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_category: Option<TaxCategory>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaxCategory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<Name>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent: Option<Percent>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_scheme: Option<TaxScheme>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TaxScheme {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<TaxSchemeName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_type_code: Option<TaxTypeCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_code: Option<CurrencyCode>,
}
