#[derive(Debug, Deserialize, Serialize)]
/// A class to define a Group of Parties
///
/// UBL Dictionary Entry Name: `Party Group. Details`
///
/// Generated from XSD type `PartyGroupType`.
pub struct PartyGroup {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code to specify the type of grouping (e.g. EEIG).
    #[serde(default, rename = "GroupTypeCode")]
    pub group_type_code: Option<cct::Code>,
/// Type of grouping as text.
    #[serde(default, rename = "GroupType")]
    pub group_type: Vec<cct::Text>,
/// A member of this Group of Parties.
    #[serde(default, rename = "Party")]
    pub party: Vec<Party>,
}
