// UBL Amount types — always carry a currencyID attribute.
//
// In UBL XML: <cbc:TaxAmount currencyID="EUR">100.00</cbc:TaxAmount>
//
// We model this as a struct with value + currency_id.
// Each named type is a distinct struct — you cannot accidentally mix
// TaxAmount with LineExtensionAmount.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// The base Amount type — a monetary value with its currency.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Amount {
    pub value: Decimal,
    pub currency_id: String,
}

impl Amount {
    pub fn new(value: Decimal, currency_id: impl Into<String>) -> Self {
        Self {
            value,
            currency_id: currency_id.into(),
        }
    }
}

macro_rules! define_amount {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct $name(pub Amount);

        impl $name {
            pub fn new(value: Decimal, currency_id: impl Into<String>) -> Self {
                Self(Amount::new(value, currency_id))
            }
            pub fn value(&self) -> &Decimal {
                &self.0.value
            }
            pub fn currency_id(&self) -> &str {
                &self.0.currency_id
            }
        }
    };
}

// --- Core billing amounts ---
define_amount!(TaxAmount, "The tax amount for a particular tax category.");
define_amount!(TaxExclusiveAmount, "The monetary amount exclusive of tax.");
define_amount!(TaxInclusiveAmount, "The monetary amount inclusive of tax.");
define_amount!(
    LineExtensionAmount,
    "The total of line extensions excluding tax."
);
define_amount!(
    TaxableAmount,
    "The monetary amount on which tax is calculated."
);
define_amount!(PayableAmount, "The amount to be paid.");
define_amount!(
    PayableRoundingAmount,
    "The rounding amount applied to the payable amount."
);
define_amount!(PrepaidAmount, "The amount that has been prepaid.");
define_amount!(AllowanceTotalAmount, "The total amount of all allowances.");
define_amount!(ChargeTotalAmount, "The total amount of all charges.");
define_amount!(BalanceAmount, "The outstanding balance amount.");
define_amount!(InvoiceTotalAmount, "The total amount of the invoice.");
define_amount!(
    BaseAmount,
    "The base amount, used as the basis for calculations."
);
define_amount!(CorrectionAmount, "The amount of a correction.");
define_amount!(PriceAmount, "The price amount of an item.");
define_amount!(UnitPriceAmount, "The unit price amount.");
define_amount!(TotalAmount, "The total amount.");
define_amount!(TotalInvoiceAmount, "The total invoice amount.");
define_amount!(TotalPaymentAmount, "The total payment amount.");
define_amount!(LiabilityAmount, "The amount of a liability.");
define_amount!(TotalTaxAmount, "The total tax amount.");
define_amount!(ValueAmount, "A generic value amount.");

// --- Extended amounts ---
define_amount!(
    AdvertisementAmount,
    "The monetary amount for advertisement."
);
define_amount!(AnnualAverageAmount, "The average annual monetary amount.");
define_amount!(AverageAmount, "The average monetary amount.");
define_amount!(
    AverageSubsequentContractAmount,
    "The average subsequent contract amount."
);
define_amount!(CallBaseAmount, "The base amount for a call.");
define_amount!(CallExtensionAmount, "The extension amount for a call.");
define_amount!(CorporateStockAmount, "The corporate stock amount.");
define_amount!(CreditLineAmount, "The amount of a credit line.");
define_amount!(DebitLineAmount, "The amount of a debit line.");
define_amount!(
    DeclaredCarriageValueAmount,
    "The declared value for carriage."
);
define_amount!(DeclaredCustomsValueAmount, "The declared customs value.");
define_amount!(
    DeclaredForCarriageValueAmount,
    "The declared value for carriage."
);
define_amount!(
    DeclaredStatisticsValueAmount,
    "The declared statistical value."
);
define_amount!(DeductibleAmount, "The deductible amount.");
define_amount!(
    EstimatedOverallContractAmount,
    "The estimated overall contract amount."
);
define_amount!(FaceValueAmount, "The face value amount.");
define_amount!(FeeAmount, "A fee amount.");
define_amount!(FreeOnBoardValueAmount, "The free-on-board value amount.");
define_amount!(HigherTenderAmount, "A higher tender amount.");
define_amount!(InsurancePremiumAmount, "The insurance premium amount.");
define_amount!(InsuranceValueAmount, "The insurance value amount.");
define_amount!(InventoryValueAmount, "The inventory value amount.");
define_amount!(InvoiceAmount, "The invoice amount.");
define_amount!(LowerTenderAmount, "A lower tender amount.");
define_amount!(MarketValueAmount, "The market value amount.");
define_amount!(MaximumAmount, "The maximum amount.");
define_amount!(MinimumAmount, "The minimum amount.");
define_amount!(
    OriginalContractValueAmount,
    "The original contract value amount."
);
define_amount!(PenaltyAmount, "A penalty amount.");
define_amount!(PerUnitAmount, "The per-unit amount.");
define_amount!(PriceChangeAmount, "The price change amount.");
define_amount!(RequiredFeeAmount, "The required fee amount.");
define_amount!(ResolutionAmount, "The resolution amount.");
define_amount!(ResolvedAmount, "The resolved amount.");
define_amount!(RoundingAmount, "A rounding amount.");
define_amount!(SalesTaxExemptAmount, "The sales tax exempt amount.");
define_amount!(
    ServiceAllowanceTotalAmount,
    "The total service allowance amount."
);
define_amount!(ServiceChargeTotalAmount, "The total service charge amount.");
define_amount!(SettlementAmount, "The settlement amount.");
define_amount!(TaxEnergyAmount, "The energy tax amount.");
define_amount!(TaxExclusiveAmountWithheld, "Tax exclusive amount withheld.");
define_amount!(TaxInclusiveAmountWithheld, "Tax inclusive amount withheld.");
define_amount!(TotalBalanceAmount, "The total balance amount.");
define_amount!(TotalCreditAmount, "The total credit amount.");
define_amount!(TotalDebitAmount, "The total debit amount.");
define_amount!(TotalTaskAmount, "The total task amount.");
define_amount!(
    TransactionCurrencyTaxAmount,
    "The tax amount in transaction currency."
);
define_amount!(WarrantyAmount, "The warranty amount.");
define_amount!(
    WithholdingTaxTotalAmount,
    "The total withholding tax amount."
);
define_amount!(
    ForeignExchangeRateAmount,
    "Foreign exchange rate expressed as amount."
);
define_amount!(
    RateOfExchangeBetweenTaxAndAlternativeCurrency,
    "Exchange rate between tax and alt currency."
);
define_amount!(PartyCapacityAmount, "The party capacity amount.");
define_amount!(
    PreferenceCriterionWeight,
    "Weight for a preference criterion."
);
define_amount!(
    PreviousCancellationAmount,
    "Amount from a previous cancellation."
);
define_amount!(
    TransportServiceProviderSpecialTermsAmount,
    "Transport service provider special terms amount."
);
define_amount!(TresholdAmount, "A threshold amount.");
define_amount!(PaidAmount, "The amount that has been paid.");

// --- Generated from UBL 2.5 XSD ---
define_amount!(
    AllowanceTotalTaxInclusiveAmount,
    "UBL CBC type: AllowanceTotalTaxInclusiveAmount."
);
define_amount!(CashChangeAmount, "UBL CBC type: CashChangeAmount.");
define_amount!(
    ChargeTotalTaxInclusiveAmount,
    "UBL CBC type: ChargeTotalTaxInclusiveAmount."
);
define_amount!(CorrectionUnitAmount, "UBL CBC type: CorrectionUnitAmount.");
define_amount!(
    DocumentationFeeAmount,
    "UBL CBC type: DocumentationFeeAmount."
);
define_amount!(EstimatedAmount, "UBL CBC type: EstimatedAmount.");
define_amount!(
    EstimatedMaximumValueAmount,
    "UBL CBC type: EstimatedMaximumValueAmount."
);
define_amount!(
    EstimatedOverallFrameworkContractsAmount,
    "UBL CBC type: EstimatedOverallFrameworkContractsAmount."
);
define_amount!(ExcessAmount, "UBL CBC type: ExcessAmount.");
define_amount!(ExpectedAmount, "UBL CBC type: ExpectedAmount.");
define_amount!(InsuredValueAmount, "UBL CBC type: InsuredValueAmount.");
define_amount!(
    MaximumAdvertisementAmount,
    "UBL CBC type: MaximumAdvertisementAmount."
);
define_amount!(MaximumPaidAmount, "UBL CBC type: MaximumPaidAmount.");
define_amount!(MaximumValueAmount, "UBL CBC type: MaximumValueAmount.");
define_amount!(PaidCashAmount, "UBL CBC type: PaidCashAmount.");
define_amount!(
    PayableAlternativeAmount,
    "UBL CBC type: PayableAlternativeAmount."
);
define_amount!(ResponseAmount, "UBL CBC type: ResponseAmount.");
define_amount!(
    SettlementDiscountAmount,
    "UBL CBC type: SettlementDiscountAmount."
);
define_amount!(
    TaxEnergyBalanceAmount,
    "UBL CBC type: TaxEnergyBalanceAmount."
);
define_amount!(
    TaxEnergyOnAccountAmount,
    "UBL CBC type: TaxEnergyOnAccountAmount."
);
define_amount!(
    TaxInclusiveLineExtensionAmount,
    "UBL CBC type: TaxInclusiveLineExtensionAmount."
);
define_amount!(
    TaxInclusivePriceAmount,
    "UBL CBC type: TaxInclusivePriceAmount."
);
define_amount!(ThresholdAmount, "UBL CBC type: ThresholdAmount.");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_amount_roundtrip() {
        let a = TaxAmount::new(rust_decimal::Decimal::new(1500, 2), "ZAR");
        let json = serde_json::to_string(&a).unwrap();
        let b: TaxAmount = serde_json::from_str(&json).unwrap();
        assert_eq!(*a.value(), *b.value());
        assert_eq!(a.currency_id(), b.currency_id());
    }

    #[test]
    fn test_line_extension_amount() {
        let a = LineExtensionAmount::new(rust_decimal::Decimal::new(9999, 2), "USD");
        let json = serde_json::to_string(&a).unwrap();
        let b: LineExtensionAmount = serde_json::from_str(&json).unwrap();
        assert_eq!(*a.value(), *b.value());
    }

    #[test]
    fn test_payable_amount() {
        let a = PayableAmount::new(rust_decimal::Decimal::new(11500, 2), "ZAR");
        assert_eq!(a.value().to_string(), "115.00");
        assert_eq!(a.currency_id(), "ZAR");
    }

    #[test]
    fn test_amount_defaults() {
        let a = TaxAmount::new(rust_decimal::Decimal::ZERO, "XXX");
        let json = serde_json::to_string(&a).unwrap();
        assert!(json.contains("0"));
        assert!(json.contains("XXX"));
    }
}
