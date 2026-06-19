#[derive(Debug, Deserialize, Serialize)]
pub struct BallastWaterSummary {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "ManagementPlanOnBoardIndicator")]
    pub management_plan_on_board_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "ManagementPlanImplementedIndicator")]
    pub management_plan_implemented_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "IMOGuidelinesOnBoardIndicator")]
    pub imo_guidelines_on_board_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "TotalBallastTanksOnBoardQuantity")]
    pub total_ballast_tanks_on_board_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "TanksInBallastQuantity")]
    pub tanks_in_ballast_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "TanksExchangedQuantity")]
    pub tanks_exchanged_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "TanksNotExchangedQuantity")]
    pub tanks_not_exchanged_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "TotalBallastWaterOnBoardMeasure")]
    pub total_ballast_water_on_board_measure: Option<cct::Measure>,
    #[serde(default, rename = "TotalBallastWaterCapacityMeasure")]
    pub total_ballast_water_capacity_measure: Option<cct::Measure>,
    #[serde(default, rename = "OtherControlActions")]
    pub other_control_actions: Vec<cct::Text>,
    #[serde(default, rename = "NoControlActionsReason")]
    pub no_control_actions_reason: Vec<cct::Text>,
    #[serde(default, rename = "UptakeBallastWaterTransaction")]
    pub uptake_ballast_water_transaction: Vec<BallastWaterTransaction>,
    #[serde(default, rename = "ExchangeBallastWaterTransaction")]
    pub exchange_ballast_water_transaction: Vec<BallastWaterTransaction>,
    #[serde(default, rename = "DischargeBallastWaterTransaction")]
    pub discharge_ballast_water_transaction: Vec<BallastWaterTransaction>,
    #[serde(default, rename = "ResponsibleOfficerPerson")]
    pub responsible_officer_person: Option<crate::Person>,
}
