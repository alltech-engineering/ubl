#[derive(Debug, Deserialize, Serialize)]
pub enum GenericTimeStampTypeContent {
    #[serde(rename = "Include")]
    Include(Include),
    #[serde(rename = "ReferenceInfo")]
    ReferenceInfo(ReferenceInfo),
    #[serde(rename = "CanonicalizationMethod")]
    CanonicalizationMethod(super::ds::CanonicalizationMethod),
    #[serde(rename = "EncapsulatedTimeStamp")]
    EncapsulatedTimeStamp(EncapsulatedPkiData),
    #[serde(rename = "XMLTimeStamp")]
    XmlTimeStamp(Any),
}
