#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a storage and storage requirements
///
/// UBL Dictionary Entry Name: `Storage. Details`
///
/// Generated from XSD type `StorageType`.
pub struct Storage {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this storage.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// The common name this storage
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// An identifier for the agreed gate to enter, deliver or pick up at this storage
    #[serde(default, rename = "GateID")]
    pub gate_id: Option<cct::Identifier>,
/// The percent of the airflow within this storage.
    #[serde(default, rename = "AirFlowPercent")]
    pub air_flow_percent: Option<cct::Numeric>,
/// The percent humidity within this storage.
    #[serde(default, rename = "HumidityPercent")]
    pub humidity_percent: Option<cct::Numeric>,
/// An indicator that this storage is approved for animal food (true) or not (false).
    #[serde(default, rename = "AnimalFoodApprovedIndicator")]
    pub animal_food_approved_indicator: Option<udt::Indicator>,
/// An indicator that this storage is approved for human food (true) or not (false).
    #[serde(default, rename = "HumanFoodApprovedIndicator")]
    pub human_food_approved_indicator: Option<udt::Indicator>,
/// An indicator that this stroage is approved for dangerous goods (true) or not (false).
    #[serde(default, rename = "DangerousGoodsApprovedIndicator")]
    pub dangerous_goods_approved_indicator: Option<udt::Indicator>,
/// An indicator that storage is refrigerated (true) or not (false).
    #[serde(default, rename = "RefrigeratedIndicator")]
    pub refrigerated_indicator: Option<udt::Indicator>,
/// An indicator that this storage can supply power (true) or not (false).
    #[serde(default, rename = "PowerIndicator")]
    pub power_indicator: Option<udt::Indicator>,
/// The minimum allowable operating temperature for this refriguated storage.
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: Option<Temperature>,
/// The maximum allowable operating temperature for this refriguated storage.
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: Option<Temperature>,
/// A certificate associated with this storage
    #[serde(default, rename = "Certificate")]
    pub certificate: Vec<Certificate>,
}
