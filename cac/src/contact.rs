#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a contactable person or department in an organization.
///
/// UBL Dictionary Entry Name: `Contact. Details`
///
/// Generated from XSD type `ContactType`.
pub struct Contact {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this contact.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The name of this contact. It is recommended that this be used for a functional name and not a
/// personal name.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// The job title or function of this contact
    #[serde(default, rename = "JobTitle")]
    pub job_title: Option<cct::Text>,
/// The department where this contact works
    #[serde(default, rename = "Department")]
    pub department: Option<cct::Text>,
/// The primary telephone number of this contact.
    #[serde(default, rename = "Telephone")]
    pub telephone: Option<cct::Text>,
/// The primary fax number of this contact.
    #[serde(default, rename = "Telefax")]
    pub telefax: Option<cct::Text>,
/// The primary email address of this contact.
    #[serde(default, rename = "ElectronicMail")]
    pub electronic_mail: Option<cct::Text>,
/// Free-form text conveying information that is not contained explicitly in other structures; in
/// particular, a textual description of the circumstances under which this contact can be used (e.g.,
/// "emergency" or "after hours").
    #[serde(default, rename = "Note")]
    pub note: Vec<cct::Text>,
/// Another means of communication with this contact.
    #[serde(default, rename = "OtherCommunication")]
    pub other_communication: Vec<Communication>,
}
