// UBL Text and Name types.

use serde::{Deserialize, Serialize};

/// Base Text type with optional language identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Text {
    pub value: String,
    pub language_id: Option<String>,
}

impl Text {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into(), language_id: None }
    }
    pub fn with_language(mut self, lang: impl Into<String>) -> Self {
        self.language_id = Some(lang.into());
        self
    }
}

macro_rules! define_text {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(pub Text);
        impl $name {
            pub fn new(value: impl Into<String>) -> Self { Self(Text::new(value)) }
            pub fn value(&self) -> &str { &self.0.value }
        }
    };
}

define_text!(Description, "A textual description.");
define_text!(Note, "A free-form note or comment.");
define_text!(Information, "Additional information.");
define_text!(Instructions, "Instructions or directions.");
define_text!(Conditions, "Terms and conditions.");
define_text!(Remarks, "Free-form remarks or observations.");
define_text!(Comment, "A comment.");
define_text!(ChangeConditions, "Conditions for a change.");
define_text!(CancellationNote, "A note explaining a cancellation.");
define_text!(RejectionNote, "A note explaining a rejection.");
define_text!(OutstandingReason, "The reason something is outstanding.");
define_text!(BackorderReason, "The reason for a backorder.");
define_text!(DamageRemarks, "Remarks about damage.");
define_text!(DeliveryInstructions, "Delivery instructions.");
define_text!(HandlingInstructions, "Handling instructions.");
define_text!(SpecialInstructions, "Special instructions.");
define_text!(PackageLevelCode, "Package level code textual description.");
define_text!(PreviousMeterReadingMethod, "Previous meter reading method text.");
define_text!(SpecialTerms, "Special terms text.");
define_text!(TransportServiceProviderRemarks, "Transport service provider remarks.");

// Name types
macro_rules! define_name {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(pub String);
        impl $name {
            pub fn new(value: impl Into<String>) -> Self { Self(value.into()) }
            pub fn value(&self) -> &str { &self.0 }
        }
    };
}

define_name!(Name, "A generic name.");
define_name!(FirstName, "A person's first name.");
define_name!(FamilyName, "A person's family name.");
define_name!(MiddleName, "A person's middle name.");
define_name!(OtherName, "A person's other/additional name.");
define_name!(Title, "A person's title (Mr, Mrs, Dr).");
define_name!(JobTitle, "A person's job title.");
define_name!(OrganizationName, "The name of an organization.");
define_name!(Department, "A department name.");
define_name!(BrandName, "A brand name.");
define_name!(CityName, "The name of a city.");
define_name!(CountryName, "The name of a country.");
define_name!(StreetName, "The name of a street.");
define_name!(AdditionalStreetName, "An additional street name.");
define_name!(BuildingName, "A building name.");
define_name!(BuildingNumber, "A building number.");
define_name!(Room, "A room number or name.");
define_name!(Floor, "A floor identifier.");
define_name!(Postbox, "A post office box number.");
define_name!(BlockName, "A block name.");
define_name!(District, "A district name.");
define_name!(Region, "A region/state/province name.");
define_name!(RegistrationName, "A registration name.");
define_name!(AliasName, "An alias name.");
define_name!(BirthplaceName, "The name of a birthplace.");
define_name!(CategoryName, "A category name.");
define_name!(CertificateType, "A certificate type name.");
define_name!(Channel, "A channel name.");
define_name!(CompanyLegalForm, "A company legal form name.");
define_name!(ContractName, "A contract name.");
define_name!(ContractSubdivision, "A contract subdivision name.");
define_name!(CountrySubentity, "A country subentity name.");
define_name!(HolderName, "The name of a holder.");
define_name!(Location, "A location name.");
define_name!(PaymentNote, "A payment note.");
define_name!(PlotIdentification, "A plot identification.");
define_name!(Position, "A position name.");
define_name!(PostalZone, "A postal zone name.");
define_name!(RoleName, "A role name.");
define_name!(ServiceName, "A service name.");
define_name!(ShippingMarks, "Shipping marks.");
define_name!(Telephone, "A telephone number.");
define_name!(VesselName, "A vessel name.");
define_name!(XPath, "An XPath expression.");


