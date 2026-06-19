#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a party as a legal entity.
///
/// UBL Dictionary Entry Name: `Party Legal Entity. Details`
///
/// Generated from XSD type `PartyLegalEntityType`.
pub struct PartyLegalEntity {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// The name of the party as registered with the relevant legal authority.
    #[serde(default, rename = "RegistrationName")]
    pub registration_name: Option<cct::Text>,
/// An identifier for the party as registered within a company registration scheme.
    #[serde(default, rename = "CompanyID")]
    pub company_id: Option<cct::Identifier>,
/// The registration date of the CompanyID.
    #[serde(default, rename = "RegistrationDate")]
    pub registration_date: Option<udt::DateTime>,
/// The date upon which a registration expires (e.g., registration for an import/export license).
    #[serde(default, rename = "RegistrationExpirationDate")]
    pub registration_expiration_date: Option<udt::DateTime>,
/// A code signifying the party's legal status.
    #[serde(default, rename = "CompanyLegalFormCode")]
    pub company_legal_form_code: Option<cct::Code>,
/// The company legal status, expressed as a text.
    #[serde(default, rename = "CompanyLegalForm")]
    pub company_legal_form: Vec<cct::Text>,
/// An indicator that the company is owned and controlled by one person (true) or not (false).
    #[serde(default, rename = "SoleProprietorshipIndicator")]
    pub sole_proprietorship_indicator: Option<udt::Indicator>,
/// A code signifying the size category of the legal entity (e.g., micro, small, medium, large).
    #[serde(default, rename = "EntitySizeCode")]
    pub entity_size_code: Option<cct::Code>,
/// A code signifying the party's liquidation status.
    #[serde(default, rename = "CompanyLiquidationStatusCode")]
    pub company_liquidation_status_code: Option<cct::Code>,
/// The number of shares in the capital stock of a corporation.
    #[serde(default, rename = "CorporateStockAmount")]
    pub corporate_stock_amount: Option<cct::Amount>,
/// An indicator that all shares of corporate stock have been paid by shareholders (true) or not
/// (false).
    #[serde(default, rename = "FullyPaidSharesIndicator")]
    pub fully_paid_shares_indicator: Option<udt::Indicator>,
/// The registered address of the party within a corporate registration scheme.
    #[serde(default, rename = "RegistrationAddress")]
    pub registration_address: Option<crate::Address>,
/// The corporate registration scheme used to register the party.
    #[serde(default, rename = "CorporateRegistrationScheme")]
    pub corporate_registration_scheme: Option<crate::CorporateRegistrationScheme>,
/// The head office of this Legal Entity.
    #[serde(default, rename = "HeadOfficeParty")]
    pub head_office_party: Option<Party>,
/// A Party that owns shares or equity in this Legal Entity.
    #[serde(default, rename = "ShareholderParty")]
    pub shareholder_party: Vec<crate::ShareholderParty>,
/// One or more securities issued by this Party Legal Entity that are listed on regulated markets.
    #[serde(default, rename = "SecurityListing")]
    pub security_listing: Vec<crate::SecurityListing>,
}
