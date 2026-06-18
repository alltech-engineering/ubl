#[derive(Debug, Deserialize, Serialize)]
pub struct WorkReportLine {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "WorkItemID")]
    pub work_item_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "WorkItemDescription")]
    pub work_item_description: Vec<super::cct::TextType>,
    #[serde(rename = "Quantity")]
    pub quantity: super::cct::QuantityType,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "CompletionPercent")]
    pub completion_percent: Option<super::cct::NumericType>,
    #[serde(default, rename = "ActivityOriginLocation")]
    pub activity_origin_location: Option<Location>,
    #[serde(default, rename = "Period")]
    pub period: Option<Period>,
    #[serde(default, rename = "PerformingParty")]
    pub performing_party: Option<Party>,
    #[serde(default, rename = "WorkPhaseReference")]
    pub work_phase_reference: Option<WorkPhaseReference>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<DocumentReference>,
    #[serde(default, rename = "Price")]
    pub price: Option<Price>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<TaxTotal>,
    #[serde(default, rename = "SubWorkReportLine")]
    pub sub_work_report_line: Vec<WorkReportLine>,
}
