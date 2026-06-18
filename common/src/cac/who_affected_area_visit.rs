#[derive(Debug, Deserialize, Serialize)]
pub struct WhoAffectedAreaVisit {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "VisitDate")]
    pub visit_date: super::udt::DateTimeType,
    #[serde(rename = "WHOAffectedAreaPortLocation")]
    pub who_affected_area_port_location: Location,
}
