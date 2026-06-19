#[derive(Debug, Deserialize, Serialize)]
pub struct Storage {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "GateID")]
    pub gate_id: Option<cct::Identifier>,
    #[serde(default, rename = "AirFlowPercent")]
    pub air_flow_percent: Option<cct::Numeric>,
    #[serde(default, rename = "HumidityPercent")]
    pub humidity_percent: Option<cct::Numeric>,
    #[serde(default, rename = "AnimalFoodApprovedIndicator")]
    pub animal_food_approved_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "HumanFoodApprovedIndicator")]
    pub human_food_approved_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "DangerousGoodsApprovedIndicator")]
    pub dangerous_goods_approved_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "RefrigeratedIndicator")]
    pub refrigerated_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "PowerIndicator")]
    pub power_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: Option<Temperature>,
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: Option<Temperature>,
    #[serde(default, rename = "Certificate")]
    pub certificate: Vec<Certificate>,
}
