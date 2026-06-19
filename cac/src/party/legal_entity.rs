#[derive(Debug, Deserialize, Serialize)]
pub struct PartyLegalEntity {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "RegistrationName")]
    pub registration_name: Option<cct::Text>,
    #[serde(default, rename = "CompanyID")]
    pub company_id: Option<cct::Identifier>,
    #[serde(default, rename = "RegistrationDate")]
    pub registration_date: Option<udt::DateTime>,
    #[serde(default, rename = "RegistrationExpirationDate")]
    pub registration_expiration_date: Option<udt::DateTime>,
    #[serde(default, rename = "CompanyLegalFormCode")]
    pub company_legal_form_code: Option<cct::Code>,
    #[serde(default, rename = "CompanyLegalForm")]
    pub company_legal_form: Vec<cct::Text>,
    #[serde(default, rename = "SoleProprietorshipIndicator")]
    pub sole_proprietorship_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "EntitySizeCode")]
    pub entity_size_code: Option<cct::Code>,
    #[serde(default, rename = "CompanyLiquidationStatusCode")]
    pub company_liquidation_status_code: Option<cct::Code>,
    #[serde(default, rename = "CorporateStockAmount")]
    pub corporate_stock_amount: Option<cct::Amount>,
    #[serde(default, rename = "FullyPaidSharesIndicator")]
    pub fully_paid_shares_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "RegistrationAddress")]
    pub registration_address: Option<crate::Address>,
    #[serde(default, rename = "CorporateRegistrationScheme")]
    pub corporate_registration_scheme: Option<crate::CorporateRegistrationScheme>,
    #[serde(default, rename = "HeadOfficeParty")]
    pub head_office_party: Option<Party>,
    #[serde(default, rename = "ShareholderParty")]
    pub shareholder_party: Vec<crate::ShareholderParty>,
    #[serde(default, rename = "SecurityListing")]
    pub security_listing: Vec<crate::SecurityListing>,
}
