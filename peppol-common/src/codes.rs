// Peppol Code Lists — validated code values for Peppol BIS documents.
//
// Peppol restricts UBL's open code lists to specific subsets.
// This module provides the known-good values and validation helpers.
//
// References:
//   - UNCL 1001: Document type codes
//   - UNCL 5305: Duty/tax/fee category codes
//   - UNCL 4461: Payment means codes
//   - UNCL 5189: Allowance reason codes
//   - UNCL 7161: Charge reason codes
//   - ISO 3166-1: Country codes (alpha-2)
//   - ISO 4217: Currency codes

use serde::{Deserialize, Serialize};

/// A code list entry — a known-valid code with its metadata.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CodeEntry {
    pub code: String,
    pub name: String,
    pub description: Option<String>,
}

/// A named code list with validation.
#[derive(Debug, Clone)]
pub struct CodeList {
    pub name: String,
    pub entries: Vec<CodeEntry>,
}

impl CodeList {
    pub fn new(name: &str, entries: Vec<CodeEntry>) -> Self {
        Self { name: name.to_string(), entries }
    }

    /// Check if a code value is valid for this list.
    pub fn is_valid(&self, code: &str) -> bool {
        self.entries.iter().any(|e| e.code == code)
    }

    /// Get the name of a code, if known.
    pub fn name_of(&self, code: &str) -> Option<&str> {
        self.entries.iter().find(|e| e.code == code).map(|e| e.name.as_str())
    }
}

// ── Pre-defined code lists ──

/// UNCL 1001 — Invoice Type Codes (subset used by Peppol Billing 3.0)
pub fn invoice_type_codes() -> CodeList {
    CodeList::new("UNCL1001-InvoiceType", vec![
        CodeEntry { code: "380".into(), name: "Commercial Invoice".into(), description: None },
        CodeEntry { code: "381".into(), name: "Credit Note".into(), description: None },
        CodeEntry { code: "383".into(), name: "Debit Note".into(), description: None },
        CodeEntry { code: "384".into(), name: "Corrected Invoice".into(), description: None },
        CodeEntry { code: "386".into(), name: "Prepayment Invoice".into(), description: None },
        CodeEntry { code: "388".into(), name: "Tax Invoice".into(), description: None },
        CodeEntry { code: "389".into(), name: "Self-billed Invoice".into(), description: None },
        CodeEntry { code: "395".into(), name: "Consolidated Invoice".into(), description: None },
    ])
}

/// UNCL 5305 — Duty/Tax/Fee Category Codes (Peppol Billing 3.0 subset)
pub fn tax_category_codes() -> CodeList {
    CodeList::new("UNCL5305-TaxCategory", vec![
        CodeEntry { code: "S".into(), name: "Standard Rate".into(), description: None },
        CodeEntry { code: "Z".into(), name: "Zero Rated".into(), description: None },
        CodeEntry { code: "E".into(), name: "Exempt from Tax".into(), description: None },
        CodeEntry { code: "AE".into(), name: "VAT Reverse Charge".into(), description: None },
        CodeEntry { code: "K".into(), name: "VAT exempt for intra-community supply".into(), description: None },
        CodeEntry { code: "G".into(), name: "Free export item, tax not charged".into(), description: None },
        CodeEntry { code: "O".into(), name: "Services outside scope of tax".into(), description: None },
        CodeEntry { code: "L".into(), name: "Canary Islands general indirect tax".into(), description: None },
        CodeEntry { code: "M".into(), name: "Tax for production, services and importation in Ceuta and Melilla".into(), description: None },
    ])
}

/// UNCL 4461 — Payment Means Codes (Peppol Billing 3.0 subset)
pub fn payment_means_codes() -> CodeList {
    CodeList::new("UNCL4461-PaymentMeans", vec![
        CodeEntry { code: "1".into(), name: "Instrument not defined".into(), description: None },
        CodeEntry { code: "10".into(), name: "In cash".into(), description: None },
        CodeEntry { code: "30".into(), name: "Credit transfer".into(), description: None },
        CodeEntry { code: "31".into(), name: "Debit transfer".into(), description: None },
        CodeEntry { code: "42".into(), name: "Payment to bank account".into(), description: None },
        CodeEntry { code: "48".into(), name: "Bank card".into(), description: None },
        CodeEntry { code: "49".into(), name: "Direct debit".into(), description: None },
        CodeEntry { code: "58".into(), name: "SEPA credit transfer".into(), description: None },
        CodeEntry { code: "59".into(), name: "SEPA direct debit".into(), description: None },
    ])
}

/// UNCL 2005 — Date/Time/Period Qualifier Codes (Peppol Billing 3.0 subset)
pub fn uncl2005_codes() -> CodeList {
    CodeList::new("UNCL2005-DateTimePeriodQualifier", vec![
        CodeEntry { code: "3".into(), name: "Invoice period".into(), description: None },
        CodeEntry { code: "35".into(), name: "Delivery date/time, actual".into(), description: None },
        CodeEntry { code: "432".into(), name: "Paid to date".into(), description: None },
    ])
}

/// ISO 4217 Currency Codes (most commonly used subset)
pub fn currency_codes() -> CodeList {
    CodeList::new("ISO4217-Currency", vec![
        CodeEntry { code: "ZAR".into(), name: "South African Rand".into(), description: None },
        CodeEntry { code: "EUR".into(), name: "Euro".into(), description: None },
        CodeEntry { code: "GBP".into(), name: "Pound Sterling".into(), description: None },
        CodeEntry { code: "USD".into(), name: "US Dollar".into(), description: None },
        CodeEntry { code: "DKK".into(), name: "Danish Krone".into(), description: None },
        CodeEntry { code: "SEK".into(), name: "Swedish Krona".into(), description: None },
        CodeEntry { code: "NOK".into(), name: "Norwegian Krone".into(), description: None },
        CodeEntry { code: "CHF".into(), name: "Swiss Franc".into(), description: None },
        CodeEntry { code: "AUD".into(), name: "Australian Dollar".into(), description: None },
        CodeEntry { code: "NZD".into(), name: "New Zealand Dollar".into(), description: None },
        CodeEntry { code: "CAD".into(), name: "Canadian Dollar".into(), description: None },
        CodeEntry { code: "PLN".into(), name: "Polish Zloty".into(), description: None },
        CodeEntry { code: "CZK".into(), name: "Czech Koruna".into(), description: None },
        CodeEntry { code: "HUF".into(), name: "Hungarian Forint".into(), description: None },
        CodeEntry { code: "BGN".into(), name: "Bulgarian Lev".into(), description: None },
        CodeEntry { code: "RON".into(), name: "Romanian Leu".into(), description: None },
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invoice_type_codes() {
        let list = invoice_type_codes();
        assert!(list.is_valid("380"));
        assert!(list.is_valid("381"));
        assert!(!list.is_valid("999"));
        assert_eq!(list.name_of("380"), Some("Commercial Invoice"));
    }

    #[test]
    fn test_tax_category_codes() {
        let list = tax_category_codes();
        assert!(list.is_valid("S"));
        assert!(list.is_valid("Z"));
        assert!(list.is_valid("E"));
        assert!(!list.is_valid("XX"));
    }

    #[test]
    fn test_payment_means_codes() {
        let list = payment_means_codes();
        assert!(list.is_valid("30"));
        assert!(list.is_valid("58"));
        assert!(!list.is_valid("99"));
    }

    #[test]
    fn test_currency_codes() {
        let list = currency_codes();
        assert!(list.is_valid("ZAR"));
        assert!(list.is_valid("EUR"));
        assert!(!list.is_valid("XXX"));
    }
}
