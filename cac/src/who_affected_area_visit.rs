#[derive(Debug, Deserialize, Serialize)]
pub struct WhoAffectedAreaVisit {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "VisitDate")]
    pub visit_date: udt::DateTime,
    #[serde(rename = "WHOAffectedAreaPortLocation")]
    pub who_affected_area_port_location: Location,
}
