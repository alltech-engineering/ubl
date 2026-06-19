#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceLevelAgreement {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "ServiceTypeCode")]
    pub service_type_code: Option<cct::Code>,
    #[serde(default, rename = "ServiceType")]
    pub service_type: Vec<cct::Text>,
    #[serde(default, rename = "AvailabilityTimePercent")]
    pub availability_time_percent: Option<cct::Numeric>,
    #[serde(default, rename = "MondayAvailabilityIndicator")]
    pub monday_availability_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "TuesdayAvailabilityIndicator")]
    pub tuesday_availability_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "WednesdayAvailabilityIndicator")]
    pub wednesday_availability_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "ThursdayAvailabilityIndicator")]
    pub thursday_availability_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "FridayAvailabilityIndicator")]
    pub friday_availability_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "SaturdayAvailabilityIndicator")]
    pub saturday_availability_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "SundayAvailabilityIndicator")]
    pub sunday_availability_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "MinimumResponseTimeDurationMeasure")]
    pub minimum_response_time_duration_measure: Option<cct::Measure>,
    #[serde(default, rename = "MinimumDownTimeScheduleDurationMeasure")]
    pub minimum_down_time_schedule_duration_measure:
        Option<cct::Measure>,
    #[serde(default, rename = "MaximumIncidentNotificationDurationMeasure")]
    pub maximum_incident_notification_duration_measure:
        Option<cct::Measure>,
    #[serde(default, rename = "MaximumDataLossDurationMeasure")]
    pub maximum_data_loss_duration_measure: Option<cct::Measure>,
    #[serde(default, rename = "MeanTimeToRecoverDurationMeasure")]
    pub mean_time_to_recover_duration_measure: Option<cct::Measure>,
    #[serde(default, rename = "ServiceAvailabilityPeriod")]
    pub service_availability_period: Vec<crate::Period>,
    #[serde(default, rename = "ServiceMaintenancePeriod")]
    pub service_maintenance_period: Vec<crate::Period>,
}
