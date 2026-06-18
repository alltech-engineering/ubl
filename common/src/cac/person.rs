#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "FirstName")]
    pub first_name: Option<super::cct::TextType>,
    #[serde(default, rename = "FamilyName")]
    pub family_name: Option<super::cct::TextType>,
    #[serde(default, rename = "Title")]
    pub title: Option<super::cct::TextType>,
    #[serde(default, rename = "MiddleName")]
    pub middle_name: Option<super::cct::TextType>,
    #[serde(default, rename = "OtherName")]
    pub other_name: Option<super::cct::TextType>,
    #[serde(default, rename = "NameSuffix")]
    pub name_suffix: Option<super::cct::TextType>,
    #[serde(default, rename = "JobTitle")]
    pub job_title: Option<super::cct::TextType>,
    #[serde(default, rename = "NationalityID")]
    pub nationality_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "NationalID")]
    pub national_id: Vec<super::cct::IdentifierType>,
    #[serde(default, rename = "NationalityCode")]
    pub nationality_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "GenderCode")]
    pub gender_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "BirthDate")]
    pub birth_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "BirthplaceName")]
    pub birthplace_name: Option<super::cct::TextType>,
    #[serde(default, rename = "OrganizationDepartment")]
    pub organization_department: Option<super::cct::TextType>,
    #[serde(default, rename = "RoleCode")]
    pub role_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "BirthplaceLocation")]
    pub birthplace_location: Option<Location>,
    #[serde(default, rename = "CitizenshipCountry")]
    pub citizenship_country: Vec<Country>,
    #[serde(default, rename = "Contact")]
    pub contact: Option<Contact>,
    #[serde(default, rename = "FinancialAccount")]
    pub financial_account: Option<FinancialAccount>,
    #[serde(default, rename = "IdentityDocumentReference")]
    pub identity_document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "ResidenceAddress")]
    pub residence_address: Option<Address>,
}
