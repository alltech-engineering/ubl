#[derive(Debug, Deserialize, Serialize)]
pub struct ImmobilizedSecurity {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ImmobilizationCertificateID")]
    pub immobilization_certificate_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SecurityID")]
    pub security_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "FaceValueAmount")]
    pub face_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "MarketValueAmount")]
    pub market_value_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "SharesNumberQuantity")]
    pub shares_number_quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<Party>,
}
