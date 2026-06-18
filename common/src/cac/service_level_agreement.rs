#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceLevelAgreement {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ServiceTypeCode")]
    pub service_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ServiceType")]
    pub service_type: Vec<super::cct::TextType>,
    #[serde(default, rename = "AvailabilityTimePercent")]
    pub availability_time_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "MondayAvailabilityIndicator")]
    pub monday_availability_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "TuesdayAvailabilityIndicator")]
    pub tuesday_availability_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "WednesdayAvailabilityIndicator")]
    pub wednesday_availability_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ThursdayAvailabilityIndicator")]
    pub thursday_availability_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "FridayAvailabilityIndicator")]
    pub friday_availability_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "SaturdayAvailabilityIndicator")]
    pub saturday_availability_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "SundayAvailabilityIndicator")]
    pub sunday_availability_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "MinimumResponseTimeDurationMeasure")]
    pub minimum_response_time_duration_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "MinimumDownTimeScheduleDurationMeasure")]
    pub minimum_down_time_schedule_duration_measure:
        Option<super::cct::MeasureType>,
    #[serde(default, rename = "MaximumIncidentNotificationDurationMeasure")]
    pub maximum_incident_notification_duration_measure:
        Option<super::cct::MeasureType>,
    #[serde(default, rename = "MaximumDataLossDurationMeasure")]
    pub maximum_data_loss_duration_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "MeanTimeToRecoverDurationMeasure")]
    pub mean_time_to_recover_duration_measure: Option<super::cct::MeasureType>,
    #[serde(default, rename = "ServiceAvailabilityPeriod")]
    pub service_availability_period: Vec<Period>,
    #[serde(default, rename = "ServiceMaintenancePeriod")]
    pub service_maintenance_period: Vec<Period>,
}
