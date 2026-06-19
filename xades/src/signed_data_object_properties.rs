#[derive(Debug, Deserialize, Serialize)]
pub struct SignedDataObjectProperties {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "DataObjectFormat")]
    pub data_object_format: Vec<DataObjectFormat>,
    #[serde(default, rename = "CommitmentTypeIndication")]
    pub commitment_type_indication: Vec<CommitmentTypeIndication>,
    #[serde(default, rename = "AllDataObjectsTimeStamp")]
    pub all_data_objects_time_stamp: Vec<GenericTimeStampType>,
    #[serde(default, rename = "IndividualDataObjectsTimeStamp")]
    pub individual_data_objects_time_stamp: Vec<GenericTimeStampType>,
    #[serde(default, rename = "any75")]
    pub any: Vec<String>,
}
