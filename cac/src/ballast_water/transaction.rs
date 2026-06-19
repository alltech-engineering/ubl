#[derive(Debug, Deserialize, Serialize)]
/// A class to define a ballast water transaction, such as the uptake, exchange or discharge of ballast
/// water.
///
/// UBL Dictionary Entry Name: `Ballast Water Transaction. Details`
///
/// Generated from XSD type `BallastWaterTransactionType`.
pub struct BallastWaterTransaction {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for the ballast water tank being used in this ballast water transaction.
    #[serde(default, rename = "TankID")]
    pub tank_id: Option<cct::Identifier>,
/// A code for the type of ballast water tank being used in the ballast water transaction.
    #[serde(default, rename = "TankTypeCode")]
    pub tank_type_code: Option<cct::Code>,
/// A code expressing how ballast water is being filled into or discharged from the tank.
    #[serde(default, rename = "ExchangeMethodCode")]
    pub exchange_method_code: Option<cct::Code>,
/// The percentage of the ballast water in the tank being exchanged in this ballast water transaction.
    #[serde(default, rename = "ExchangedPercent")]
    pub exchanged_percent: Option<cct::Numeric>,
/// The volume of ballast water being exchanged in this ballast water transaction.
    #[serde(default, rename = "VolumeMeasure")]
    pub volume_measure: Option<cct::Measure>,
/// A measure of the sea height at the time of the transaction.
    #[serde(default, rename = "SeaHeightMeasure")]
    pub sea_height_measure: Option<cct::Measure>,
/// A measure for the salinity of the water in the tank.
    #[serde(default, rename = "SalinityMeasure")]
    pub salinity_measure: Option<cct::Measure>,
/// The date when this ballast water transaction takes place.
    #[serde(default, rename = "TransactionDate")]
    pub transaction_date: Option<udt::DateTime>,
/// The location where this ballast water transaction takes place.
    #[serde(default, rename = "Location")]
    pub location: Option<crate::Location>,
/// The temperature of the ballast water at time of transaction.
    #[serde(default, rename = "BallastWaterTemperature")]
    pub ballast_water_temperature: Option<crate::Temperature>,
}
