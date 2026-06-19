#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a service level agreement which regulates the quality, availability and
/// responsibilities of digital services.
///
/// UBL Dictionary Entry Name: `Service Level Agreement. Details`
///
/// Generated from XSD type `ServiceLevelAgreementType`.
pub struct ServiceLevelAgreement {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this service level agreement.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A specific type of service subject to this service level agreement.
    #[serde(default, rename = "ServiceTypeCode")]
    pub service_type_code: Option<cct::Code>,
/// A specific type of service subject to this service level agreement, expressed as text.
    #[serde(default, rename = "ServiceType")]
    pub service_type: Vec<cct::Text>,
/// The availability percentage (e.g. 98.5% of the time).
    #[serde(default, rename = "AvailabilityTimePercent")]
    pub availability_time_percent: Option<cct::Numeric>,
/// Indicates whether this service is available on monday (true) or not (false).
    #[serde(default, rename = "MondayAvailabilityIndicator")]
    pub monday_availability_indicator: Option<udt::Indicator>,
/// Indicates whether this service is available on tuesday (true) or not (false).
    #[serde(default, rename = "TuesdayAvailabilityIndicator")]
    pub tuesday_availability_indicator: Option<udt::Indicator>,
/// Indicates whether this service is available on wednesday (true) or not (false).
    #[serde(default, rename = "WednesdayAvailabilityIndicator")]
    pub wednesday_availability_indicator: Option<udt::Indicator>,
/// Indicates whether this service is available on thursday (true) or not (false).
    #[serde(default, rename = "ThursdayAvailabilityIndicator")]
    pub thursday_availability_indicator: Option<udt::Indicator>,
/// Indicates whether this service is available on friday (true) or not (false).
    #[serde(default, rename = "FridayAvailabilityIndicator")]
    pub friday_availability_indicator: Option<udt::Indicator>,
/// Indicates whether this service is available on saturday (true) or not (false).
    #[serde(default, rename = "SaturdayAvailabilityIndicator")]
    pub saturday_availability_indicator: Option<udt::Indicator>,
/// Indicates whether this service is available on sunday (true) or not (false).
    #[serde(default, rename = "SundayAvailabilityIndicator")]
    pub sunday_availability_indicator: Option<udt::Indicator>,
/// The response time for aknowledgment (e.g. to send a receipt to a sending Access Point within 300
/// seconds).
    #[serde(default, rename = "MinimumResponseTimeDurationMeasure")]
    pub minimum_response_time_duration_measure: Option<cct::Measure>,
/// The minimum down time schedule for programmed maintenance (e.g. scheduled 3 days before).
    #[serde(default, rename = "MinimumDownTimeScheduleDurationMeasure")]
    pub minimum_down_time_schedule_duration_measure:
        Option<cct::Measure>,
/// The maximum length of time between the occurrence of an incident and the issuance of a notification
/// (e.g. within 4 hours).
    #[serde(default, rename = "MaximumIncidentNotificationDurationMeasure")]
    pub maximum_incident_notification_duration_measure:
        Option<cct::Measure>,
/// The maximum data loss permitted (e.g. last 24 hours).
    #[serde(default, rename = "MaximumDataLossDurationMeasure")]
    pub maximum_data_loss_duration_measure: Option<cct::Measure>,
/// The time taken to recover after an outage of service (e.g. 3 hours).
    #[serde(default, rename = "MeanTimeToRecoverDurationMeasure")]
    pub mean_time_to_recover_duration_measure: Option<cct::Measure>,
/// The period for which the service is available.
    #[serde(default, rename = "ServiceAvailabilityPeriod")]
    pub service_availability_period: Vec<crate::Period>,
/// The period of time designated in advance by the technical staff, during which preventive maintenance
/// that could cause disruption of service may be performed.
    #[serde(default, rename = "ServiceMaintenancePeriod")]
    pub service_maintenance_period: Vec<crate::Period>,
}
