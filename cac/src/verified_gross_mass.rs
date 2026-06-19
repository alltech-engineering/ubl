#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a verified gross mass (VGM) measure and its documentation.
///
/// UBL Dictionary Entry Name: `Verified Gross Mass. Details`
///
/// Generated from XSD type `VerifiedGrossMassType`.
pub struct VerifiedGrossMass {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this mass measure.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The weighing date.
    #[serde(default, rename = "WeighingDate")]
    pub weighing_date: Option<udt::DateTime>,
/// The weighing time.
    #[serde(default, rename = "WeighingTime")]
    pub weighing_time: Option<udt::DateTime>,
/// A code signifying the weighing method used (e.g. according the SOLAS Convention).
    #[serde(rename = "WeighingMethodCode")]
    pub weighing_method_code: cct::Code,
/// An identifier for the weighing device used for executing the weight measurement.
    #[serde(default, rename = "WeighingDeviceID")]
    pub weighing_device_id: Option<cct::Identifier>,
/// Text describing the weighing device type used for executing the weight measurement.
    #[serde(default, rename = "WeighingDeviceType")]
    pub weighing_device_type: Option<cct::Text>,
/// The total verified gross mass of a packed container which includes the cargo weight, block and
/// bracing materials and container tare.
    #[serde(rename = "GrossMassMeasure")]
    pub gross_mass_measure: cct::Measure,
/// The Party who executes the weight measure.
    #[serde(default, rename = "WeighingParty")]
    pub weighing_party: Option<Party>,
/// The Party who is reponsible of the Verified Gross Mass (VGM) according to the SOLAS Convention. This
/// Party plays the role of the Shipper (BCO, FF or NVOCC).
    #[serde(default, rename = "ShipperParty")]
    pub shipper_party: Option<Party>,
/// The Party who signs the Verified Gross Mass (VGM) on behalf of the Shipper.
    #[serde(default, rename = "ResponsibleParty")]
    pub responsible_party: Option<Party>,
/// A reference to the VGM documentary evidence.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
}
