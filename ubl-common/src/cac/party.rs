// UBL Party aggregate — an individual, group, or body in a business function.
// One of the most heavily used aggregates across all UBL document types.

use serde::{Deserialize, Serialize};
use crate::cbc::*;

// Sibling CAC types used by Party
use crate::cac::address::{Address, PostalAddress};
use crate::cac::contact::Contact;
use crate::cac::tax::TaxScheme;

/// A party (organization, person, or role) involved in a business transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Party {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub party_identification: Vec<PartyIdentification>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub party_name: Vec<PartyName>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub postal_address: Option<PostalAddress>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub physical_location: Option<Location>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub party_tax_scheme: Vec<PartyTaxScheme>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub party_legal_entity: Vec<PartyLegalEntity>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact: Option<Contact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub person: Option<Person>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub agent_party: Option<Box<Party>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartyIdentification {
    pub id: ID,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartyName {
    pub name: Name,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Language {
    pub id: Option<ID>,
    pub name: Option<Name>,
    pub locale_code: Option<Code>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    pub id: Option<ID>,
    pub description: Option<Description>,
    pub conditions: Option<Conditions>,
    pub country_subentity: Option<CountrySubentity>,
    pub country_subentity_code: Option<Code>,
    pub address: Option<Address>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartyTaxScheme {
    pub registration_name: Option<RegistrationName>,
    pub company_id: Option<CompanyID>,
    pub tax_level_code: Option<TaxLevelCode>,
    pub exemption_reason_code: Option<TaxExemptionReasonCode>,
    pub exemption_reason: Option<TaxExemptionReason>,
    pub tax_scheme: TaxScheme,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartyLegalEntity {
    pub registration_name: Option<RegistrationName>,
    pub company_id: Option<CompanyID>,
    pub company_legal_form: Option<CompanyLegalForm>,
    pub corporate_registration_type_code: Option<CorporateRegistrationTypeCode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub first_name: Option<FirstName>,
    pub family_name: Option<FamilyName>,
    pub title: Option<Title>,
    pub middle_name: Option<MiddleName>,
    pub other_name: Option<OtherName>,
    pub job_title: Option<JobTitle>,
    pub nationality_id: Option<NationalityID>,
    pub birth_date: Option<BirthDate>,
    pub birth_place_name: Option<BirthplaceName>,
    pub gender_code: Option<GenderCode>,
}
