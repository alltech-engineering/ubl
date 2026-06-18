#[derive(Debug, Deserialize, Serialize)]
pub struct VerifiedGrossMass {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "WeighingDate")]
    pub weighing_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "WeighingTime")]
    pub weighing_time: Option<super::udt::DateTimeType>,
    #[serde(rename = "WeighingMethodCode")]
    pub weighing_method_code: super::cct::CodeType,
    #[serde(default, rename = "WeighingDeviceID")]
    pub weighing_device_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "WeighingDeviceType")]
    pub weighing_device_type: Option<super::cct::TextType>,
    #[serde(rename = "GrossMassMeasure")]
    pub gross_mass_measure: super::cct::MeasureType,
    #[serde(default, rename = "WeighingParty")]
    pub weighing_party: Option<Party>,
    #[serde(default, rename = "ShipperParty")]
    pub shipper_party: Option<Party>,
    #[serde(default, rename = "ResponsibleParty")]
    pub responsible_party: Option<Party>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
}
