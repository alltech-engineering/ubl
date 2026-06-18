#[derive(Debug, Deserialize, Serialize)]
pub struct PartyLegalEntity {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "RegistrationName")]
    pub registration_name: Option<super::cct::TextType>,
    #[serde(default, rename = "CompanyID")]
    pub company_id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "RegistrationDate")]
    pub registration_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "RegistrationExpirationDate")]
    pub registration_expiration_date: Option<super::udt::DateTimeType>,
    #[serde(default, rename = "CompanyLegalFormCode")]
    pub company_legal_form_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CompanyLegalForm")]
    pub company_legal_form: Vec<super::cct::TextType>,
    #[serde(default, rename = "SoleProprietorshipIndicator")]
    pub sole_proprietorship_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "EntitySizeCode")]
    pub entity_size_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CompanyLiquidationStatusCode")]
    pub company_liquidation_status_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CorporateStockAmount")]
    pub corporate_stock_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "FullyPaidSharesIndicator")]
    pub fully_paid_shares_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "RegistrationAddress")]
    pub registration_address: Option<Address>,
    #[serde(default, rename = "CorporateRegistrationScheme")]
    pub corporate_registration_scheme: Option<CorporateRegistrationScheme>,
    #[serde(default, rename = "HeadOfficeParty")]
    pub head_office_party: Option<Party>,
    #[serde(default, rename = "ShareholderParty")]
    pub shareholder_party: Vec<ShareholderParty>,
    #[serde(default, rename = "SecurityListing")]
    pub security_listing: Vec<SecurityListing>,
}
