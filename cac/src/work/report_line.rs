#[derive(Debug, Deserialize, Serialize)]
pub struct WorkReportLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "WorkItemID")]
    pub work_item_id: Option<cct::Identifier>,
    #[serde(default, rename = "WorkItemDescription")]
    pub work_item_description: Vec<cct::Text>,
    #[serde(rename = "Quantity")]
    pub quantity: cct::Quantity,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<cct::Amount>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
    #[serde(default, rename = "CompletionPercent")]
    pub completion_percent: Option<cct::Numeric>,
    #[serde(default, rename = "ActivityOriginLocation")]
    pub activity_origin_location: Option<crate::Location>,
    #[serde(default, rename = "Period")]
    pub period: Option<crate::Period>,
    #[serde(default, rename = "PerformingParty")]
    pub performing_party: Option<crate::Party>,
    #[serde(default, rename = "WorkPhaseReference")]
    pub work_phase_reference: Option<WorkPhaseReference>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
    #[serde(default, rename = "Price")]
    pub price: Option<crate::Price>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<crate::TaxTotal>,
    #[serde(default, rename = "SubWorkReportLine")]
    pub sub_work_report_line: Vec<WorkReportLine>,
}
