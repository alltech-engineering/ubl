// UBL Numeric, Percent, and Rate types.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

// --- Numeric types ---
macro_rules! define_numeric {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub Decimal);
        impl $name {
            pub fn new(value: Decimal) -> Self { Self(value) }
            pub fn value(&self) -> &Decimal { &self.0 }
        }
    };
}

define_numeric!(Numeric, "A generic numeric value.");
define_numeric!(CalculationSequenceNumeric, "A calculation sequence number.");
define_numeric!(OrderQuantityIncrementNumeric, "Order quantity increment numeric.");
define_numeric!(OrderIntervalDaysNumeric, "The number of days between orders.");
define_numeric!(MaximumPaymentInstructionsNumeric, "Maximum payment instructions numeric.");
define_numeric!(MaximumValueNumeric, "Maximum value numeric.");
define_numeric!(MinimumValueNumeric, "Minimum value numeric.");
define_numeric!(LineNumberNumeric, "Line number numeric.");
define_numeric!(BudgetYearNumeric, "Budget year numeric.");
define_numeric!(WeightNumeric, "Weight numeric.");
define_numeric!(FrozenPeriodDaysNumeric, "Frozen period days numeric.");
define_numeric!(LineCountNumeric, "Line count numeric.");
define_numeric!(MaximumCopiesNumeric, "Maximum copies numeric.");
define_numeric!(ReminderSequenceNumeric, "Reminder sequence numeric.");
define_numeric!(MultiplierFactorNumeric, "A multiplier factor expressed as a numeric value.");

// --- Percent types ---
macro_rules! define_percent {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub Decimal);
        impl $name {
            pub fn new(value: Decimal) -> Self { Self(value) }
            pub fn value(&self) -> &Decimal { &self.0 }
        }
    };
}

define_percent!(Percent, "A generic percentage.");
define_percent!(CompletionPercent, "The completion percentage.");
define_percent!(ParticipationPercent, "The participation percentage.");
define_percent!(PartecipationPercent, "Deprecated — use ParticipationPercent.");
define_percent!(PaymentPercent, "The payment percentage.");
define_percent!(ProgressPercent, "The progress percentage.");
define_percent!(ReliabilityPercent, "The reliability percentage.");
define_percent!(SettlementDiscountPercent, "The settlement discount percentage.");
define_percent!(TargetServicePercent, "The target service percentage.");
define_percent!(AirFlowPercent, "Air flow percentage.");
define_percent!(AvailabilityTimePercent, "Availability time percentage.");
define_percent!(HumidityPercent, "Humidity percentage.");
define_percent!(PenaltySurchargePercent, "Penalty surcharge percentage.");
define_percent!(PriceChangePercent, "Price change percentage.");
define_percent!(SettlementPercent, "Settlement percentage.");
define_percent!(TierRatePercent, "Tier rate percentage.");

// --- Rate types ---
macro_rules! define_rate {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub Decimal);
        impl $name {
            pub fn new(value: Decimal) -> Self { Self(value) }
            pub fn value(&self) -> &Decimal { &self.0 }
        }
    };
}

define_rate!(Rate, "A generic rate.");
define_rate!(CalculationRate, "A calculation rate.");
define_rate!(AmountRate, "An amount-based rate.");
define_rate!(ExchangeRate, "An exchange rate.");
define_rate!(SourceCurrencyBaseRate, "The source currency base rate.");
define_rate!(TargetCurrencyBaseRate, "The target currency base rate.");
define_rate!(OrderableUnitFactorRate, "Orderable unit factor rate.");
define_rate!(PaymentAlternativeExchangeRate, "Payment alternative exchange rate.");
define_rate!(PaymentExchangeRate, "Payment exchange rate.");
define_rate!(PriceChangeRate, "Price change rate.");
define_rate!(TaxRate, "Tax rate.");
define_rate!(SettlementDiscountRate, "Settlement discount rate.");
define_rate!(ForeignExchangeRate, "Foreign exchange rate.");
define_rate!(InsurancePremiumRate, "Insurance premium rate.");
define_rate!(InterestRate, "Interest rate.");


