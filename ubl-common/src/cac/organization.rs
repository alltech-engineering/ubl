// UBL 2.5 CAC Tier 4: Financial Account, Branch, Person, ShareholderParty
//
// Reference: UBL-CommonAggregateComponents-2.5.xsd

use serde::{Deserialize, Serialize};

// ─── FinancialAccount ────────────────────────────────────────────────
// XSD: FinancialAccountType
// A financial account (bank account, credit card, etc.)

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FinancialAccount {
    pub id: Option<String>,
    pub name: Option<String>,
    pub alias_name: Option<String>,
    pub account_type_code: Option<String>,
    pub account_format_code: Option<String>,
    pub currency_code: Option<String>,
    pub blockchain_id: Option<String>,
    #[serde(default)]
    pub payment_note: Vec<String>,
    // CAC: financial_institution_branch: Option<Branch>
    // CAC: country: Option<Country>
}

// ─── Branch ──────────────────────────────────────────────────────────
// XSD: BranchType
// A branch or division of an organization

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Branch {
    pub id: Option<String>,
    pub name: Option<String>,
    // CAC: financial_institution: Option<FinancialInstitution>
    // CAC: address: Option<Address>
}

// ─── Person ──────────────────────────────────────────────────────────
// XSD: PersonType
// An individual person

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Person {
    pub id: Option<String>,
    pub first_name: Option<String>,
    pub family_name: Option<String>,
    pub title: Option<String>,
    pub middle_name: Option<String>,
    pub other_name: Option<String>,
    pub name_suffix: Option<String>,
    pub job_title: Option<String>,
    pub nationality_id: Option<String>,
    #[serde(default)]
    pub national_id: Vec<String>,
    pub nationality_code: Option<String>,
    pub gender_code: Option<String>,
    pub birth_date: Option<String>,
    pub birthplace_name: Option<String>,
    pub organization_department: Option<String>,
    pub role_code: Option<String>,
    // CAC: birthplace_location: Option<Location>
    // CAC: citizenship_country: Vec<Country>
    // CAC: contact: Option<Contact>
    // CAC: financial_account: Option<FinancialAccount>
    // CAC: identity_document_reference: Vec<DocumentReference>
    // CAC: residence_address: Option<Address>
}

// ─── ShareholderParty ────────────────────────────────────────────────
// XSD: ShareholderPartyType
// A party that holds shares in a company

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShareholderParty {
    pub partecipation_percent: Option<f64>,
    pub participation_percent: Option<f64>,
    // CAC: party: Option<Party>
}
