#[derive(Debug, Deserialize, Serialize)]
pub struct Despatch {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "RequestedDespatchDate")]
    pub requested_despatch_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "RequestedDespatchTime")]
    pub requested_despatch_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EstimatedDespatchDate")]
    pub estimated_despatch_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EstimatedDespatchTime")]
    pub estimated_despatch_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ActualDespatchDate")]
    pub actual_despatch_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ActualDespatchTime")]
    pub actual_despatch_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "GuaranteedDespatchDate")]
    pub guaranteed_despatch_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "GuaranteedDespatchTime")]
    pub guaranteed_despatch_time: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ReleaseID")]
    pub release_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Instructions")]
    pub instructions: Vec<super::cct::TextType>,
    #[serde(default, rename = "DespatchAddress")]
    pub despatch_address: Option<Address>,
    #[serde(default, rename = "DespatchLocation")]
    pub despatch_location: Option<Location>,
    #[serde(default, rename = "DespatchParty")]
    pub despatch_party: Option<Party>,
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: Option<Party>,
    #[serde(default, rename = "NotifyParty")]
    pub notify_party: Vec<Party>,
    #[serde(default, rename = "ResponsibleParty")]
    pub responsible_party: Option<Party>,
    #[serde(default, rename = "Contact")]
    pub contact: Option<Contact>,
    #[serde(default, rename = "EstimatedDespatchPeriod")]
    pub estimated_despatch_period: Option<Period>,
    #[serde(default, rename = "RequestedDespatchPeriod")]
    pub requested_despatch_period: Option<Period>,
}
