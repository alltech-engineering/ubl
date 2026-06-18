#[derive(Debug, Deserialize, Serialize)]
pub struct CardAccount {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "PrimaryAccountNumberID")]
    pub primary_account_number_id: super::cct::IdentifierType,
    #[serde(default, rename = "NetworkID")]
    pub network_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CardTypeCode")]
    pub card_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ValidityStartDate")]
    pub validity_start_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ExpiryDate")]
    pub expiry_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "IssuerID")]
    pub issuer_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssueNumberID")]
    pub issue_number_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CV2ID")]
    pub cv_2_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CardChipCode")]
    pub card_chip_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ChipApplicationID")]
    pub chip_application_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "HolderName")]
    pub holder_name: Option<super::cct::TextType>,
    #[serde(default, rename = "RoleCode")]
    pub role_code: Option<super::cct::CodeType>,
}
