#[derive(Debug, Deserialize, Serialize)]
/// A class to describe the bond guarantee of a tenderer or bid submitter's actual entry into a contract
/// in the event that it is the successful bidder.
///
/// UBL Dictionary Entry Name: `Financial Guarantee. Details`
///
/// Generated from XSD type `FinancialGuaranteeType`.
pub struct FinancialGuarantee {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code signifying the type of financial guarantee. For instance "Provisional Guarantee" or "Final
/// Guarantee"
    #[serde(rename = "GuaranteeTypeCode")]
    pub guarantee_type_code: cct::Code,
/// Text describing this financial guarantee.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// The amount of liability in this financial guarantee.
    #[serde(default, rename = "LiabilityAmount")]
    pub liability_amount: Option<cct::Amount>,
/// The rate used to calculate the amount of liability in this financial guarantee.
    #[serde(default, rename = "AmountRate")]
    pub amount_rate: Option<cct::Numeric>,
/// The period during the tendering process to which this financial guarantee has to be settled.
    #[serde(default, rename = "ConstitutionPeriod")]
    pub constitution_period: Option<crate::Period>,
}
