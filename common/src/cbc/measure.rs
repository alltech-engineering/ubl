// UBL Measure types — physical measurements with required unit code.

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

/// Base Measure type with required unit code.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Measure {
    pub value: Decimal,
    pub unit_code: String,
}

impl Measure {
    pub fn new(value: Decimal, unit_code: impl Into<String>) -> Self {
        Self {
            value,
            unit_code: unit_code.into(),
        }
    }
}

macro_rules! define_measure {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
        pub struct $name(pub Measure);
        impl $name {
            pub fn new(value: Decimal, unit: impl Into<String>) -> Self {
                Self(Measure::new(value, unit))
            }
            pub fn value(&self) -> &Decimal {
                &self.0.value
            }
            pub fn unit_code(&self) -> &str {
                &self.0.unit_code
            }
        }
    };
}

define_measure!(AltitudeMeasure, "An altitude measure.");
define_measure!(BaseUnitMeasure, "A base unit measure.");
define_measure!(ChargeableWeightMeasure, "A chargeable weight measure.");
define_measure!(ComparedValueMeasure, "A compared value measure.");
define_measure!(ConsumptionMeasure, "A consumption measure.");
define_measure!(DurationMeasure, "A duration measure.");
define_measure!(GrossTonnageMeasure, "A gross tonnage measure.");
define_measure!(GrossVolumeMeasure, "A gross volume measure.");
define_measure!(GrossWeightMeasure, "A gross weight measure.");
define_measure!(LatitudeDegreesMeasure, "Latitude in degrees.");
define_measure!(LatitudeMinutesMeasure, "Latitude in minutes.");
define_measure!(LengthMeasure, "A length measure.");
define_measure!(LongitudeDegreesMeasure, "Longitude in degrees.");
define_measure!(LongitudeMinutesMeasure, "Longitude in minutes.");
define_measure!(MaximumMeasure, "A maximum measure.");
define_measure!(MinimumMeasure, "A minimum measure.");
define_measure!(NetNetWeightMeasure, "A net-net weight measure.");
define_measure!(NetTonnageMeasure, "A net tonnage measure.");
define_measure!(NetVolumeMeasure, "A net volume measure.");
define_measure!(NetWeightMeasure, "A net weight measure.");
define_measure!(SourceValueMeasure, "A source value measure.");
define_measure!(TareWeightMeasure, "A tare weight measure.");
define_measure!(TemperatureMeasure, "A temperature measure.");
define_measure!(ValueMeasure, "A value measure.");
define_measure!(WeightMeasure, "A weight measure.");
define_measure!(WidthMeasure, "A width measure.");

// --- Generated from UBL 2.5 XSD ---
define_measure!(ActivityLevelMeasure, "UBL CBC type: ActivityLevelMeasure.");
define_measure!(
    AllocatedEnergyMeasure,
    "UBL CBC type: AllocatedEnergyMeasure."
);
define_measure!(
    EstimatedGeneratedUntilNextPortMeasure,
    "UBL CBC type: EstimatedGeneratedUntilNextPortMeasure."
);
define_measure!(
    FuelConsumptionMeasure,
    "UBL CBC type: FuelConsumptionMeasure."
);
define_measure!(GrossMassMeasure, "UBL CBC type: GrossMassMeasure.");
define_measure!(LeadTimeMeasure, "UBL CBC type: LeadTimeMeasure.");
define_measure!(LoadingLengthMeasure, "UBL CBC type: LoadingLengthMeasure.");
define_measure!(
    MaxDedicatedStorageCapacityMeasure,
    "UBL CBC type: MaxDedicatedStorageCapacityMeasure."
);
define_measure!(
    MaximumDataLossDurationMeasure,
    "UBL CBC type: MaximumDataLossDurationMeasure."
);
define_measure!(
    MaximumIncidentNotificationDurationMeasure,
    "UBL CBC type: MaximumIncidentNotificationDurationMeasure."
);
define_measure!(
    MeanTimeToRecoverDurationMeasure,
    "UBL CBC type: MeanTimeToRecoverDurationMeasure."
);
define_measure!(
    MinimumDownTimeScheduleDurationMeasure,
    "UBL CBC type: MinimumDownTimeScheduleDurationMeasure."
);
define_measure!(
    MinimumResponseTimeDurationMeasure,
    "UBL CBC type: MinimumResponseTimeDurationMeasure."
);
define_measure!(
    PostEventNotificationDurationMeasure,
    "UBL CBC type: PostEventNotificationDurationMeasure."
);
define_measure!(
    PreEventNotificationDurationMeasure,
    "UBL CBC type: PreEventNotificationDurationMeasure."
);
define_measure!(RateOfTurnMeasure, "UBL CBC type: RateOfTurnMeasure.");
define_measure!(ResponseMeasure, "UBL CBC type: ResponseMeasure.");
define_measure!(
    RetainedOnBoardMeasure,
    "UBL CBC type: RetainedOnBoardMeasure."
);
define_measure!(SalinityMeasure, "UBL CBC type: SalinityMeasure.");
define_measure!(SeaHeightMeasure, "UBL CBC type: SeaHeightMeasure.");
define_measure!(
    SegregatedBallastMeasure,
    "UBL CBC type: SegregatedBallastMeasure."
);
define_measure!(
    SpeedOverGroundMeasure,
    "UBL CBC type: SpeedOverGroundMeasure."
);
define_measure!(ToBeDeliveredMeasure, "UBL CBC type: ToBeDeliveredMeasure.");
define_measure!(
    TotalBallastWaterCapacityMeasure,
    "UBL CBC type: TotalBallastWaterCapacityMeasure."
);
define_measure!(
    TotalBallastWaterOnBoardMeasure,
    "UBL CBC type: TotalBallastWaterOnBoardMeasure."
);
define_measure!(ValueBaseMeasure, "UBL CBC type: ValueBaseMeasure.");
define_measure!(VolumeMeasure, "UBL CBC type: VolumeMeasure.");
define_measure!(WasteMeasure, "UBL CBC type: WasteMeasure.");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_measure_roundtrip() {
        let m = GrossWeightMeasure::new(rust_decimal::Decimal::new(1500, 2), "KGM");
        let json = serde_json::to_string(&m).unwrap();
        let m2: GrossWeightMeasure = serde_json::from_str(&json).unwrap();
        assert_eq!(*m.value(), *m2.value());
    }

    #[test]
    fn test_net_weight_measure() {
        let m = NetWeightMeasure::new(rust_decimal::Decimal::new(1200, 2), "KGM");
        assert_eq!(m.unit_code(), "KGM");
    }
}
