#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a person.
///
/// UBL Dictionary Entry Name: `Person. Details`
///
/// Generated from XSD type `PersonType`.
pub struct Person {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this person.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// This person's given name.
    #[serde(default, rename = "FirstName")]
    pub first_name: Option<cct::Text>,
/// This person's family name.
    #[serde(default, rename = "FamilyName")]
    pub family_name: Option<cct::Text>,
/// This person's title of address (e.g., Mr, Ms, Dr, Sir).
    #[serde(default, rename = "Title")]
    pub title: Option<cct::Text>,
/// This person's middle name(s) or initials.
    #[serde(default, rename = "MiddleName")]
    pub middle_name: Option<cct::Text>,
/// This person's second family name.
    #[serde(default, rename = "OtherName")]
    pub other_name: Option<cct::Text>,
/// A suffix to this person's name (e.g., PhD, OBE, Jr).
    #[serde(default, rename = "NameSuffix")]
    pub name_suffix: Option<cct::Text>,
/// This person's job title (for a particular role) within an organization.
    #[serde(default, rename = "JobTitle")]
    pub job_title: Option<cct::Text>,
/// (Deprecated) An identifier for this person's nationality.
    #[serde(default, rename = "NationalityID")]
    pub nationality_id: Option<cct::Identifier>,
/// An identifier issued by a national authority that uniquely identifies the person within that
/// country, such as a social security number or national registration number.
    #[serde(default, rename = "NationalID")]
    pub national_id: Vec<cct::Identifier>,
/// A code signifying the person’s nationality as defined by the applicable legal or administrative
/// framework.
    #[serde(default, rename = "NationalityCode")]
    pub nationality_code: Option<cct::Code>,
/// A code (e.g., ISO 5218, ICAO Doc 9303, etc.) signifying the gender of this person.
    #[serde(default, rename = "GenderCode")]
    pub gender_code: Option<cct::Code>,
/// This person's date of birth.
    #[serde(default, rename = "BirthDate")]
    pub birth_date: Option<udt::DateTime>,
/// The name of the place where this person was born, expressed as text.
    #[serde(default, rename = "BirthplaceName")]
    pub birthplace_name: Option<cct::Text>,
/// The department or subdivision of an organization that this person belongs to (in a particular role).
    #[serde(default, rename = "OrganizationDepartment")]
    pub organization_department: Option<cct::Text>,
/// A code stating the person's role
    #[serde(default, rename = "RoleCode")]
    pub role_code: Option<cct::Code>,
/// The location where this person was born.
    #[serde(default, rename = "BirthplaceLocation")]
    pub birthplace_location: Option<Location>,
/// The country of the person's citizenship.
    #[serde(default, rename = "CitizenshipCountry")]
    pub citizenship_country: Vec<Country>,
/// Contact information for this person.
    #[serde(default, rename = "Contact")]
    pub contact: Option<Contact>,
/// The financial account associated with this person.
    #[serde(default, rename = "FinancialAccount")]
    pub financial_account: Option<FinancialAccount>,
/// A reference to a document that can precisely identify this person (e.g., a driver's license).
    #[serde(default, rename = "IdentityDocumentReference")]
    pub identity_document_reference: Vec<DocumentReference>,
/// This person's address of residence.
    #[serde(default, rename = "ResidenceAddress")]
    pub residence_address: Option<Address>,
}
