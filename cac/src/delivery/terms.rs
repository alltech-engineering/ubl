#[derive(Debug, Deserialize, Serialize)]
/// A class for describing the terms and conditions applying to the delivery of goods.
///
/// UBL Dictionary Entry Name: `Delivery Terms. Details`
///
/// Generated from XSD type `DeliveryTermsType`.
pub struct DeliveryTerms {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier for this description of delivery terms.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// A description of any terms or conditions relating to the delivery items.
    #[serde(default, rename = "SpecialTerms")]
    pub special_terms: Vec<cct::Text>,
/// A code that identifies one of various responsibilities for loss risk in the execution of the
/// delivery.
    #[serde(default, rename = "LossRiskResponsibilityCode")]
    pub loss_risk_responsibility_code: Option<cct::Code>,
/// A description of responsibility for risk of loss in execution of the delivery, expressed as text.
    #[serde(default, rename = "LossRisk")]
    pub loss_risk: Vec<cct::Text>,
/// The monetary amount covered by these delivery terms.
    #[serde(default, rename = "Amount")]
    pub amount: Option<cct::Amount>,
/// The location for the contracted delivery.
    #[serde(default, rename = "DeliveryLocation")]
    pub delivery_location: Option<crate::Location>,
/// An allowance or charge covered by these delivery terms.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Option<crate::AllowanceCharge>,
}
