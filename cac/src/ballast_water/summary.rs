#[derive(Debug, Deserialize, Serialize)]
/// A class to summarize the ballast water management of a vessel.
///
/// UBL Dictionary Entry Name: `Ballast Water Summary. Details`
///
/// Generated from XSD type `BallastWaterSummaryType`.
pub struct BallastWaterSummary {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identification of this ballast water summary.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// An indication of whether a waste water management plan is on board (true) or not (false).
    #[serde(default, rename = "ManagementPlanOnBoardIndicator")]
    pub management_plan_on_board_indicator: Option<udt::Indicator>,
/// An indication of whether a waste water management plan has been implemented (true) or not (false).
    #[serde(default, rename = "ManagementPlanImplementedIndicator")]
    pub management_plan_implemented_indicator: Option<udt::Indicator>,
/// An indication of whether International Maritime Organization (IMO) ballast water guidelines are on
/// board (true) or not (false).
    #[serde(default, rename = "IMOGuidelinesOnBoardIndicator")]
    pub imo_guidelines_on_board_indicator: Option<udt::Indicator>,
/// The total number of ballast tanks on board the vessel.
    #[serde(default, rename = "TotalBallastTanksOnBoardQuantity")]
    pub total_ballast_tanks_on_board_quantity: Option<cct::Quantity>,
/// The number of tanks in the ballast of the vessel.
    #[serde(default, rename = "TanksInBallastQuantity")]
    pub tanks_in_ballast_quantity: Option<cct::Quantity>,
/// The number of tanks being exchanged as part of this ballast water summary.
    #[serde(default, rename = "TanksExchangedQuantity")]
    pub tanks_exchanged_quantity: Option<cct::Quantity>,
/// The quantity of tanks not being exchanged.
    #[serde(default, rename = "TanksNotExchangedQuantity")]
    pub tanks_not_exchanged_quantity: Option<cct::Quantity>,
/// The messure of the total ballast water on board the vessel.
    #[serde(default, rename = "TotalBallastWaterOnBoardMeasure")]
    pub total_ballast_water_on_board_measure: Option<cct::Measure>,
/// The total ballast water capacity of the vessel.
    #[serde(default, rename = "TotalBallastWaterCapacityMeasure")]
    pub total_ballast_water_capacity_measure: Option<cct::Measure>,
/// A text describing any other control actions that are part of this ballast water summary.
    #[serde(default, rename = "OtherControlActions")]
    pub other_control_actions: Vec<cct::Text>,
/// A textual description of the reason if no control actions are being taken.
    #[serde(default, rename = "NoControlActionsReason")]
    pub no_control_actions_reason: Vec<cct::Text>,
/// The uptake ballast water transaction.
    #[serde(default, rename = "UptakeBallastWaterTransaction")]
    pub uptake_ballast_water_transaction: Vec<BallastWaterTransaction>,
/// The exchange ballast water transaction.
    #[serde(default, rename = "ExchangeBallastWaterTransaction")]
    pub exchange_ballast_water_transaction: Vec<BallastWaterTransaction>,
/// The discharge ballast water transaction.
    #[serde(default, rename = "DischargeBallastWaterTransaction")]
    pub discharge_ballast_water_transaction: Vec<BallastWaterTransaction>,
/// The officer responsible for this ballast water summery.
    #[serde(default, rename = "ResponsibleOfficerPerson")]
    pub responsible_officer_person: Option<crate::Person>,
}
