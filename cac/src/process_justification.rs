#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a justification for the choice of tendering process.
///
/// UBL Dictionary Entry Name: `Process Justification. Details`
///
/// Generated from XSD type `ProcessJustificationType`.
pub struct ProcessJustification {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code signifying the type of the previous tendering process (which is now being cancelled).
    #[serde(default, rename = "PreviousCancellationReasonCode")]
    pub previous_cancellation_reason_code: Option<cct::Code>,
/// The reason why the contracting authority has followed a particular tendering procedure for the
/// awarding of a contract, expressed as a code.
    #[serde(default, rename = "ProcessReasonCode")]
    pub process_reason_code: Option<cct::Code>,
/// The reason why the contracting authority has followed a particular tendering procedure for the
/// awarding of a contract, expressed as text.
    #[serde(default, rename = "ProcessReason")]
    pub process_reason: Vec<cct::Text>,
/// Text providing justification for the selection of this process.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
}
