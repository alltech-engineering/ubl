#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a visit to a port located in a geographical area considered an “affected area”
/// by the World Health Organization (WHO).
///
/// UBL Dictionary Entry Name: `WHO Affected Area Visit. Details`
///
/// Generated from XSD type `WHOAffectedAreaVisitType`.
pub struct WhoAffectedAreaVisit {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// The date that the WHO Affected Area was visited.
    #[serde(rename = "VisitDate")]
    pub visit_date: udt::DateTime,
/// The location of the port of the visited WHO Affected Area.
    #[serde(rename = "WHOAffectedAreaPortLocation")]
    pub who_affected_area_port_location: Location,
}
