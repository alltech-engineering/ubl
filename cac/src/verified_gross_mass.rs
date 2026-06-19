#[derive(Debug, Deserialize, Serialize)]
pub struct VerifiedGrossMass {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "WeighingDate")]
    pub weighing_date: Option<udt::DateTime>,
    #[serde(default, rename = "WeighingTime")]
    pub weighing_time: Option<udt::DateTime>,
    #[serde(rename = "WeighingMethodCode")]
    pub weighing_method_code: cct::Code,
    #[serde(default, rename = "WeighingDeviceID")]
    pub weighing_device_id: Option<cct::Identifier>,
    #[serde(default, rename = "WeighingDeviceType")]
    pub weighing_device_type: Option<cct::Text>,
    #[serde(rename = "GrossMassMeasure")]
    pub gross_mass_measure: cct::Measure,
    #[serde(default, rename = "WeighingParty")]
    pub weighing_party: Option<Party>,
    #[serde(default, rename = "ShipperParty")]
    pub shipper_party: Option<Party>,
    #[serde(default, rename = "ResponsibleParty")]
    pub responsible_party: Option<Party>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
}
