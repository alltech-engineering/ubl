#[derive(Debug, Deserialize, Serialize)]
pub struct CardAccount {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "PrimaryAccountNumberID")]
    pub primary_account_number_id: cct::Identifier,
    #[serde(default, rename = "NetworkID")]
    pub network_id: Option<cct::Identifier>,
    #[serde(default, rename = "CardTypeCode")]
    pub card_type_code: Option<cct::Code>,
    #[serde(default, rename = "ValidityStartDate")]
    pub validity_start_date: Option<udt::DateTime>,
    #[serde(default, rename = "ExpiryDate")]
    pub expiry_date: Option<udt::DateTime>,
    #[serde(default, rename = "IssuerID")]
    pub issuer_id: Option<cct::Identifier>,
    #[serde(default, rename = "IssueNumberID")]
    pub issue_number_id: Option<cct::Identifier>,
    #[serde(default, rename = "CV2ID")]
    pub cv_2_id: Option<cct::Identifier>,
    #[serde(default, rename = "CardChipCode")]
    pub card_chip_code: Option<cct::Code>,
    #[serde(default, rename = "ChipApplicationID")]
    pub chip_application_id: Option<cct::Identifier>,
    #[serde(default, rename = "HolderName")]
    pub holder_name: Option<cct::Text>,
    #[serde(default, rename = "RoleCode")]
    pub role_code: Option<cct::Code>,
}
