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
        Self { value, unit_code: unit_code.into() }
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
            pub fn value(&self) -> &Decimal { &self.0.value }
            pub fn unit_code(&self) -> &str { &self.0.unit_code }
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

