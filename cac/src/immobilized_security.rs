#[derive(Debug, Deserialize, Serialize)]
pub struct ImmobilizedSecurity {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ImmobilizationCertificateID")]
    pub immobilization_certificate_id: Option<cct::Identifier>,
    #[serde(default, rename = "SecurityID")]
    pub security_id: Option<cct::Identifier>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
    #[serde(default, rename = "FaceValueAmount")]
    pub face_value_amount: Option<cct::Amount>,
    #[serde(default, rename = "MarketValueAmount")]
    pub market_value_amount: Option<cct::Amount>,
    #[serde(default, rename = "SharesNumberQuantity")]
    pub shares_number_quantity: Option<cct::Quantity>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<Party>,
}
