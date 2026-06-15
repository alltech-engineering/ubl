// UBL Party aggregate — an individual, group, or body in a business function.
// One of the most heavily used aggregates across all UBL document types.

use crate::cbc::*;
use serde::{Deserialize, Serialize};

// Sibling CAC types used by Party
use crate::cac::address::{Address, PostalAddress};
use crate::cac::contact::Contact;
use crate::cac::tax::TaxScheme;

/// A party (organization, person, or role) involved in a business transaction.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Party {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mark_care_indicator: Option<MarkCareIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mark_attention_indicator: Option<MarkAttentionIndicator>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub website_uri: Option<WebsiteURI>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logo_reference_id: Option<LogoReferenceID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub endpoint_id: Option<EndpointID>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub industry_classification_code: Option<IndustryClassificationCode>,
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
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub name: Option<Name>,
    #[serde(default)]
    pub locale_code: Option<Code>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Location {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub description: Option<Description>,
    #[serde(default)]
    pub conditions: Option<Conditions>,
    #[serde(default)]
    pub country_subentity: Option<CountrySubentity>,
    #[serde(default)]
    pub country_subentity_code: Option<Code>,
    #[serde(default)]
    pub address: Option<Address>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartyTaxScheme {
    #[serde(default)]
    pub registration_name: Option<RegistrationName>,
    #[serde(default)]
    pub company_id: Option<CompanyID>,
    #[serde(default)]
    pub tax_level_code: Option<TaxLevelCode>,
    #[serde(default)]
    pub exemption_reason_code: Option<TaxExemptionReasonCode>,
    #[serde(default)]
    pub exemption_reason: Option<TaxExemptionReason>,
    pub tax_scheme: TaxScheme,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PartyLegalEntity {
    #[serde(default)]
    pub registration_name: Option<RegistrationName>,
    #[serde(default)]
    pub company_id: Option<CompanyID>,
    #[serde(default)]
    pub registration_date: Option<RegistrationDate>,
    #[serde(default)]
    pub registration_expiration_date: Option<RegistrationExpirationDate>,
    #[serde(default)]
    pub company_legal_form: Option<CompanyLegalForm>,
    #[serde(default)]
    pub company_legal_form_code: Option<CompanyLegalFormCode>,
    #[serde(default)]
    pub sole_proprietorship_indicator: Option<SoleProprietorshipIndicator>,
    #[serde(default)]
    pub corporate_stock_amount: Option<CorporateStockAmount>,
    #[serde(default)]
    pub fully_paid_shares_indicator: Option<FullyPaidSharesIndicator>,
    #[serde(default)]
    pub company_liquidation_status_code: Option<CompanyLiquidationStatusCode>,
    #[serde(default)]
    pub corporate_registration_type_code: Option<CorporateRegistrationTypeCode>,
    #[serde(default)]
    pub entity_size_code: Option<EntitySizeCode>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    #[serde(default)]
    pub id: Option<ID>,
    #[serde(default)]
    pub first_name: Option<FirstName>,
    #[serde(default)]
    pub family_name: Option<FamilyName>,
    #[serde(default)]
    pub title: Option<Title>,
    #[serde(default)]
    pub middle_name: Option<MiddleName>,
    #[serde(default)]
    pub other_name: Option<OtherName>,
    #[serde(default)]
    pub name_suffix: Option<NameSuffix>,
    #[serde(default)]
    pub job_title: Option<JobTitle>,
    #[serde(default)]
    pub nationality_id: Option<NationalityID>,
    #[serde(default)]
    pub national_id: Option<NationalID>,
    #[serde(default)]
    pub nationality_code: Option<NationalityCode>,
    #[serde(default)]
    pub birth_date: Option<BirthDate>,
    #[serde(default)]
    pub birth_place_name: Option<BirthplaceName>,
    #[serde(default)]
    pub organization_department: Option<OrganizationDepartment>,
    #[serde(default)]
    pub gender_code: Option<GenderCode>,
    #[serde(default)]
    pub birthplace_name: Option<BirthplaceName>,
    #[serde(default)]
    pub role_code: Option<RoleCode>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn empty_party() -> Party {
        Party {
            mark_care_indicator: None,
            mark_attention_indicator: None,
            website_uri: None,
            logo_reference_id: None,
            endpoint_id: None,
            industry_classification_code: None,
            party_identification: vec![],
            party_name: vec![],
            language: None,
            postal_address: None,
            physical_location: None,
            party_tax_scheme: vec![],
            party_legal_entity: vec![],
            contact: None,
            person: None,
            agent_party: None,
        }
    }

    #[test]
    fn test_party_roundtrip() {
        let mut party = empty_party();
        party.party_name = vec![PartyName {
            name: Name::new("Acme Corp"),
        }];
        let json = serde_json::to_string(&party).unwrap();
        let party2: Party = serde_json::from_str(&json).unwrap();
        assert_eq!(party.party_name[0].name.0, party2.party_name[0].name.0);
    }

    #[test]
    fn test_party_with_tax_scheme() {
        let mut party = empty_party();
        party.party_name = vec![PartyName {
            name: Name::new("Acme Corp"),
        }];
        party.party_tax_scheme = vec![PartyTaxScheme {
            registration_name: Some(RegistrationName::new("Acme Corp")),
            company_id: Some(CompanyID::new("9876543210")),
            tax_level_code: None,
            exemption_reason_code: None,
            exemption_reason: None,
            tax_scheme: TaxScheme {
                id: Some(ID::new("VAT")),
                name: Some(Name::new("VAT")),
                tax_type_code: None,
                currency_code: None,
                jurisdiction_region_address: vec![],
            },
        }];
        let json = serde_json::to_string(&party).unwrap();
        assert!(json.contains("VAT"));
        let party2: Party = serde_json::from_str(&json).unwrap();
        assert_eq!(
            party2.party_tax_scheme[0]
                .tax_scheme
                .name
                .as_ref()
                .unwrap()
                .0,
            "VAT"
        );
    }
}
