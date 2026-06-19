#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a line in a Work Report, specifying the work performed.
///
/// UBL Dictionary Entry Name: `Work Report Line. Details`
///
/// Generated from XSD type `WorkReportLineType`.
pub struct WorkReportLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this Work Report Line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// An identifier for the specific work item to which this line relates.
    #[serde(default, rename = "WorkItemID")]
    pub work_item_id: Option<cct::Identifier>,
/// A description of the specific work item to which this line relates.
    #[serde(default, rename = "WorkItemDescription")]
    pub work_item_description: Vec<cct::Text>,
/// The quantity of work reported in this line.
    #[serde(rename = "Quantity")]
    pub quantity: cct::Quantity,
/// The total amount for this work report line, excluding taxes.
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<cct::Amount>,
/// The total amount for this work report line, including taxes.
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
/// The overall percentage of completion for the work item represented by this line.
    #[serde(default, rename = "CompletionPercent")]
    pub completion_percent: Option<cct::Numeric>,
/// The Location from which the work in this Work Report Line originated or was performed.
    #[serde(default, rename = "ActivityOriginLocation")]
    pub activity_origin_location: Option<crate::Location>,
/// The Period during which the work described in this Work Report Line was performed.
    #[serde(default, rename = "Period")]
    pub period: Option<crate::Period>,
/// The Party performing the work in this Work Report Line.
    #[serde(default, rename = "PerformingParty")]
    pub performing_party: Option<crate::Party>,
/// A reference to the Work Phase to which this Work Report Line relates.
    #[serde(default, rename = "WorkPhaseReference")]
    pub work_phase_reference: Option<WorkPhaseReference>,
/// A reference to an external document relevant to this Work Report Line.
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: Vec<crate::DocumentReference>,
/// The price applicable to this Work Report Line.
    #[serde(default, rename = "Price")]
    pub price: Option<crate::Price>,
/// A total amount of taxes of a particular kind applicable to this Work Report Line.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<crate::TaxTotal>,
/// A subsidiary Work Report Line related to this Work Report Line.
    #[serde(default, rename = "SubWorkReportLine")]
    pub sub_work_report_line: Vec<WorkReportLine>,
}
