#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a notification requirement.
///
/// UBL Dictionary Entry Name: `Notification Requirement. Details`
///
/// Generated from XSD type `NotificationRequirementType`.
pub struct NotificationRequirement {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code signifying the type of notification (e.g., pickup status).
    #[serde(rename = "NotificationTypeCode")]
    pub notification_type_code: cct::Code,
/// The length of time between the occurrence of a given event and the issuance of a notification.
    #[serde(default, rename = "PostEventNotificationDurationMeasure")]
    pub post_event_notification_duration_measure: Option<cct::Measure>,
/// The length of time to elapse between the issuance of a notification and the occurrence of the event
/// it relates to.
    #[serde(default, rename = "PreEventNotificationDurationMeasure")]
    pub pre_event_notification_duration_measure: Option<cct::Measure>,
/// The Party who is notified.
    #[serde(default, rename = "NotifyParty")]
    pub notify_party: Vec<Party>,
/// A period during which a notification will be issued.
    #[serde(default, rename = "NotificationPeriod")]
    pub notification_period: Vec<Period>,
/// A location at which a notification will be issued.
    #[serde(default, rename = "NotificationLocation")]
    pub notification_location: Vec<Location>,
}
