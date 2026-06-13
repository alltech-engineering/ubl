// UBL Quantity types — measured quantities with optional unit code.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Base Quantity type with optional unit code.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Quantity {
    pub value: Decimal,
    pub unit_code: Option<String>,
}

impl Quantity {
    pub fn new(value: Decimal) -> Self {
        Self {
            value,
            unit_code: None,
        }
    }
    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit_code = Some(unit.into());
        self
    }
}

macro_rules! define_quantity {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct $name(pub Quantity);
        impl $name {
            pub fn new(value: Decimal) -> Self {
                Self(Quantity::new(value))
            }
            pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
                self.0 = self.0.with_unit(unit);
                self
            }
            pub fn value(&self) -> &Decimal {
                &self.0.value
            }
        }
    };
}

define_quantity!(InvoicedQuantity, "The quantity being invoiced.");
define_quantity!(CreditedQuantity, "The quantity being credited.");
define_quantity!(DebitedQuantity, "The quantity being debited.");
define_quantity!(DeliveredQuantity, "The quantity delivered.");
define_quantity!(DespatchedQuantity, "The quantity despatched.");
define_quantity!(ReceivedQuantity, "The quantity received.");
define_quantity!(OrderedQuantity, "The quantity ordered.");
define_quantity!(BackorderQuantity, "The quantity on backorder.");
define_quantity!(RejectedQuantity, "The quantity rejected.");
define_quantity!(ReturnedQuantity, "The quantity returned.");
define_quantity!(BatchQuantity, "The batch quantity.");
define_quantity!(BaseQuantity, "The base quantity for pricing.");
define_quantity!(ChargeableQuantity, "The chargeable quantity.");
define_quantity!(ChildConsignmentQuantity, "The child consignment quantity.");
define_quantity!(ConsignmentQuantity, "The consignment quantity.");
define_quantity!(ConsumerUnitQuantity, "The consumer unit quantity.");
define_quantity!(ContentUnitQuantity, "The content unit quantity.");
define_quantity!(CustomsTariffQuantity, "Customs tariff quantity.");
define_quantity!(EstimatedDespatchQuantity, "Estimated despatch quantity.");
define_quantity!(EstimatedReceivedQuantity, "Estimated received quantity.");
define_quantity!(GoodsItemQuantity, "Goods item quantity.");
define_quantity!(MaximumBackorderQuantity, "Maximum backorder quantity.");
define_quantity!(MaximumOrderQuantity, "Maximum order quantity.");
define_quantity!(MinimumBackorderQuantity, "Minimum backorder quantity.");
define_quantity!(MinimumOrderQuantity, "Minimum order quantity.");
define_quantity!(MinimumQuantity, "Minimum quantity.");
define_quantity!(MultipleOrderQuantity, "Multiple order quantity.");
define_quantity!(PackageQuantity, "Package quantity.");
define_quantity!(PackQuantity, "Pack quantity.");
define_quantity!(PassengerQuantity, "Passenger quantity.");
// Quantity is the base type — no specific "Quantity" wrapper to avoid collision
define_quantity!(ReceiptLineQuantity, "Receipt line quantity.");
define_quantity!(RequestedQuantity, "Requested quantity.");
define_quantity!(ReturnableAssetQuantity, "Returnable asset quantity.");
define_quantity!(SalesOrderQuantity, "Sales order quantity.");
define_quantity!(ShareholderQuantity, "Shareholder quantity.");
define_quantity!(SharesNumberQuantity, "Shares number quantity.");
define_quantity!(ShortQuantity, "Short quantity.");
define_quantity!(TargetInventoryQuantity, "Target inventory quantity.");
define_quantity!(TariffQuantity, "Tariff quantity.");
define_quantity!(TotalConsumedQuantity, "Total consumed quantity.");
define_quantity!(TotalDeliveredQuantity, "Total delivered quantity.");
define_quantity!(TotalGoodsItemQuantity, "Total goods item quantity.");
define_quantity!(TotalMeteredQuantity, "Total metered quantity.");
define_quantity!(TotalOrderedQuantity, "Total ordered quantity.");
define_quantity!(TotalPackagesQuantity, "Total packages quantity.");
define_quantity!(TotalReceivedQuantity, "Total received quantity.");
define_quantity!(TotalRejectedQuantity, "Total rejected quantity.");
define_quantity!(TotalReturnedQuantity, "Total returned quantity.");
define_quantity!(
    TotalTransportHandlingUnitQuantity,
    "Total transport handling unit quantity."
);
define_quantity!(ValueQuantity, "Value quantity.");
define_quantity!(VarianceQuantity, "Variance quantity.");

// --- Missing ---
define_quantity!(MaximumQuantity, "The maximum quantity.");

// --- Generated from UBL 2.5 XSD ---
define_quantity!(
    ActualTemperatureReductionQuantity,
    "UBL CBC type: ActualTemperatureReductionQuantity."
);
define_quantity!(
    BasicConsumedQuantity,
    "UBL CBC type: BasicConsumedQuantity."
);
define_quantity!(
    ConsumptionEnergyQuantity,
    "UBL CBC type: ConsumptionEnergyQuantity."
);
define_quantity!(
    ConsumptionWaterQuantity,
    "UBL CBC type: ConsumptionWaterQuantity."
);
define_quantity!(CrewQuantity, "UBL CBC type: CrewQuantity.");
define_quantity!(
    DifferenceTemperatureReductionQuantity,
    "UBL CBC type: DifferenceTemperatureReductionQuantity."
);
define_quantity!(EmployeeQuantity, "UBL CBC type: EmployeeQuantity.");
define_quantity!(
    EstimatedConsumedQuantity,
    "UBL CBC type: EstimatedConsumedQuantity."
);
define_quantity!(
    EstimatedOverallContractQuantity,
    "UBL CBC type: EstimatedOverallContractQuantity."
);
define_quantity!(
    ExpectedOperatorQuantity,
    "UBL CBC type: ExpectedOperatorQuantity."
);
define_quantity!(ExpectedQuantity, "UBL CBC type: ExpectedQuantity.");
define_quantity!(GasPressureQuantity, "UBL CBC type: GasPressureQuantity.");
define_quantity!(LatestMeterQuantity, "UBL CBC type: LatestMeterQuantity.");
define_quantity!(
    MaximumOperatorQuantity,
    "UBL CBC type: MaximumOperatorQuantity."
);
define_quantity!(
    MaximumVariantQuantity,
    "UBL CBC type: MaximumVariantQuantity."
);
define_quantity!(
    MinimumInventoryQuantity,
    "UBL CBC type: MinimumInventoryQuantity."
);
define_quantity!(
    NormalTemperatureReductionQuantity,
    "UBL CBC type: NormalTemperatureReductionQuantity."
);
define_quantity!(
    OperatingYearsQuantity,
    "UBL CBC type: OperatingYearsQuantity."
);
define_quantity!(OutstandingQuantity, "UBL CBC type: OutstandingQuantity.");
define_quantity!(OversupplyQuantity, "UBL CBC type: OversupplyQuantity.");
define_quantity!(
    PerformanceValueQuantity,
    "UBL CBC type: PerformanceValueQuantity."
);
define_quantity!(
    PreviousMeterQuantity,
    "UBL CBC type: PreviousMeterQuantity."
);
define_quantity!(
    ReceivedElectronicTenderQuantity,
    "UBL CBC type: ReceivedElectronicTenderQuantity."
);
define_quantity!(
    ReceivedForeignTenderQuantity,
    "UBL CBC type: ReceivedForeignTenderQuantity."
);
define_quantity!(
    ReceivedTenderQuantity,
    "UBL CBC type: ReceivedTenderQuantity."
);
define_quantity!(ResponseQuantity, "UBL CBC type: ResponseQuantity.");
define_quantity!(ReturnableQuantity, "UBL CBC type: ReturnableQuantity.");
define_quantity!(
    TanksExchangedQuantity,
    "UBL CBC type: TanksExchangedQuantity."
);
define_quantity!(
    TanksInBallastQuantity,
    "UBL CBC type: TanksInBallastQuantity."
);
define_quantity!(
    TanksNotExchangedQuantity,
    "UBL CBC type: TanksNotExchangedQuantity."
);
define_quantity!(ThresholdQuantity, "UBL CBC type: ThresholdQuantity.");
define_quantity!(
    TimeDeltaDaysQuantity,
    "UBL CBC type: TimeDeltaDaysQuantity."
);
define_quantity!(
    TotalBallastTanksOnBoardQuantity,
    "UBL CBC type: TotalBallastTanksOnBoardQuantity."
);
define_quantity!(
    TotalDeadPersonQuantity,
    "UBL CBC type: TotalDeadPersonQuantity."
);
define_quantity!(
    TotalIllPersonQuantity,
    "UBL CBC type: TotalIllPersonQuantity."
);
define_quantity!(TotalPackageQuantity, "UBL CBC type: TotalPackageQuantity.");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_invoiced_quantity_roundtrip() {
        let q = InvoicedQuantity::new(rust_decimal::Decimal::new(5, 0));
        let json = serde_json::to_string(&q).unwrap();
        let q2: InvoicedQuantity = serde_json::from_str(&json).unwrap();
        assert_eq!(q.value(), q2.value());
    }

    #[test]
    fn test_quantity_defaults() {
        let q = InvoicedQuantity::new(rust_decimal::Decimal::new(100, 0));
        assert_eq!(q.value().to_string(), "100");
    }

    #[test]
    fn test_quantity_fractional() {
        let q = InvoicedQuantity::new(rust_decimal::Decimal::new(25, 1)); // 2.5
        assert_eq!(q.value().to_string(), "2.5");
    }
}
