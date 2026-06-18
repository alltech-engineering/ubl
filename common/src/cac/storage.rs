#[derive(Debug, Deserialize, Serialize)]
pub struct Storage {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "GateID")]
    pub gate_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AirFlowPercent")]
    pub air_flow_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "HumidityPercent")]
    pub humidity_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "AnimalFoodApprovedIndicator")]
    pub animal_food_approved_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "HumanFoodApprovedIndicator")]
    pub human_food_approved_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "DangerousGoodsApprovedIndicator")]
    pub dangerous_goods_approved_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "RefrigeratedIndicator")]
    pub refrigerated_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "PowerIndicator")]
    pub power_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: Option<Temperature>,
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: Option<Temperature>,
    #[serde(default, rename = "Certificate")]
    pub certificate: Vec<Certificate>,
}
