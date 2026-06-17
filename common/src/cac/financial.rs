// UBL 2.5 CAC Tier 4: Financial types — FinancialInstitution, FinancialGuarantee, TradingTerms
//
// Reference: UBL-CommonAggregateComponents-2.5.xsd

use serde::{Deserialize, Serialize};

// ─── FinancialInstitution ────────────────────────────────────────────
// XSD: FinancialInstitutionType
// A financial institution (bank, credit union, etc.)

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialInstitution {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub name: Option<String>,
    // CAC: address: Option<Address>
}

// ─── FinancialGuarantee ──────────────────────────────────────────────
// XSD: FinancialGuaranteeType
// A financial guarantee for a transaction

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialGuarantee {
    #[serde(default)]
    pub guarantee_type_code: Option<String>,
    #[serde(default)]
    pub description: Vec<String>,
    #[serde(default)]
    pub liability_amount: Option<f64>,
    #[serde(default)]
    pub amount_rate: Option<f64>,
    // CAC: constitution_period: Option<Period>
}

// ─── TradingTerms (HaulageTradingTerms) ──────────────────────────────
// XSD: TradingTermsType
// Trading terms applied to a transaction (e.g., haulage trading terms)

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradingTerms {
    #[serde(default)]
    pub information: Vec<String>,
    #[serde(default)]
    pub reference: Option<String>,
    // CAC: applicable_address: Option<Address>
}
