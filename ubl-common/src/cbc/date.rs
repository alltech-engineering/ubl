// UBL Date and Time types.
// Date types use chrono::NaiveDate, Time uses chrono::NaiveTime,
// DateTime uses chrono::NaiveDateTime.

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Deserialize, Serialize};

// --- Date types ---
macro_rules! define_date {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub NaiveDate);
        impl $name { pub fn new(date: NaiveDate) -> Self { Self(date) } }
    };
}

define_date!(IssueDate, "The date on which the document was issued.");
define_date!(DueDate, "The date on which payment is due.");
define_date!(TaxPointDate, "The date on which tax becomes applicable.");
define_date!(StartDate, "The start date of a period.");
define_date!(EndDate, "The end date of a period.");
define_date!(ActualDeliveryDate, "The actual date of delivery.");
define_date!(ActualDespatchDate, "The actual date of despatch.");
define_date!(ActualPickupDate, "The actual date of pickup.");
define_date!(RequestedDeliveryDate, "The requested delivery date.");
define_date!(RequestedDespatchDate, "The requested despatch date.");
define_date!(PromisedDeliveryDate, "The promised delivery date.");
define_date!(EstimatedDeliveryDate, "The estimated delivery date.");
define_date!(LatestDeliveryDate, "The latest acceptable delivery date.");
define_date!(DeliveryDate, "The date of delivery.");
define_date!(DespatchDate, "The date of despatch.");
define_date!(ValidityStartDate, "The start of the validity period.");
define_date!(ValidityEndDate, "The end of the validity period.");
define_date!(RegistrationDate, "The date of registration.");
define_date!(ExpiryDate, "The expiry date.");
define_date!(EffectiveDate, "The date something becomes effective.");
define_date!(EffectiveEndDate, "The date something ceases to be effective.");
define_date!(ApprovalDate, "The date of approval.");
define_date!(ApplicationDate, "The date of application.");
define_date!(AwardDate, "The date of award.");
define_date!(BirthDate, "A person's birth date.");
define_date!(Date, "A generic date.");
define_date!(LastRevisionDate, "The date of last revision.");
define_date!(ManufactureDate, "The date of manufacture.");
define_date!(OccurrenceDate, "The date of an occurrence.");
define_date!(PaidDate, "The date payment was made.");
define_date!(PaymentDate, "The date of payment.");
define_date!(PaymentDueDate, "The date payment is due.");
define_date!(ReceivedDate, "The date something was received.");
define_date!(ReferenceDate, "A reference date.");
define_date!(ResponseDate, "The date of a response.");
define_date!(SubmissionDate, "The date of submission.");
define_date!(SubmissionDueDate, "The due date for submission.");
define_date!(TransactionDate, "The date of a transaction.");
define_date!(ValidationDate, "The date of validation.");
define_date!(SignatureDate, "The date of signature.");
define_date!(GuaranteedDespatchDate, "The guaranteed despatch date.");
define_date!(OpenTenderDate, "The open tender date.");
define_date!(PlannedDate, "A planned date.");

// --- Time types ---
macro_rules! define_time {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub NaiveTime);
        impl $name { pub fn new(time: NaiveTime) -> Self { Self(time) } }
    };
}

define_time!(IssueTime, "The time at which the document was issued.");
define_time!(ActualDeliveryTime, "The actual time of delivery.");
define_time!(ActualDespatchTime, "The actual time of despatch.");
define_time!(ActualPickupTime, "The actual time of pickup.");
define_time!(AwardTime, "The time of award.");
define_time!(ExpiryTime, "The time of expiry.");
define_time!(OccurrenceTime, "The time of an occurrence.");
define_time!(PaidTime, "The time payment was made.");
define_time!(PaymentTime, "The time of payment.");
define_time!(ReferenceTime, "A reference time.");
define_time!(ResponseTime, "The time of a response.");
define_time!(StartTime, "The start time.");
define_time!(SubmissionTime, "The time of submission.");
define_time!(ValidationTime, "The time of validation.");
define_time!(LatestDeliveryTime, "The latest acceptable delivery time.");
define_time!(EffectiveTime, "The time something becomes effective.");
define_time!(ReceivedTime, "The time something was received.");

// --- DateTime types ---
macro_rules! define_datetime {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub NaiveDateTime);
        impl $name { pub fn new(dt: NaiveDateTime) -> Self { Self(dt) } }
    };
}

define_datetime!(DateTime, "A generic date-time.");
define_datetime!(ExpiryDateTime, "The date-time of expiry.");
define_datetime!(OccurrenceDateTime, "The date-time of an occurrence.");



// --- Missing ---
define_date!(BestBeforeDate, "The best-before date for a product.");
define_time!(Time, "A generic time value.");
