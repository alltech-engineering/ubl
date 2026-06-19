#[derive(Debug, Deserialize, Serialize)]
/// A class to define a credit card, debit card, or charge card account.
///
/// UBL Dictionary Entry Name: `Card Account. Details`
///
/// Generated from XSD type `CardAccountType`.
pub struct CardAccount {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier of the card (e.g., the Primary Account Number (PAN)).
    #[serde(rename = "PrimaryAccountNumberID")]
    pub primary_account_number_id: cct::Identifier,
/// An identifier for the financial service network provider of the card.
    #[serde(default, rename = "NetworkID")]
    pub network_id: Option<cct::Identifier>,
/// A mutually agreed code signifying the type of card. Examples of types are "debit", "credit" and
/// "purchasing"
    #[serde(default, rename = "CardTypeCode")]
    pub card_type_code: Option<cct::Code>,
/// The date from which the card is valid.
    #[serde(default, rename = "ValidityStartDate")]
    pub validity_start_date: Option<udt::DateTime>,
/// The date on which the card expires.
    #[serde(default, rename = "ExpiryDate")]
    pub expiry_date: Option<udt::DateTime>,
/// An identifier for the institution issuing the card.
    #[serde(default, rename = "IssuerID")]
    pub issuer_id: Option<cct::Identifier>,
/// An identifier for the card, specified by the issuer.
    #[serde(default, rename = "IssueNumberID")]
    pub issue_number_id: Option<cct::Identifier>,
/// An identifier for the Card Verification Value (often found on the reverse of the card itself).
    #[serde(default, rename = "CV2ID")]
    pub cv_2_id: Option<cct::Identifier>,
/// A mutually agreed code to distinguish between CHIP and MAG STRIPE cards.
    #[serde(default, rename = "CardChipCode")]
    pub card_chip_code: Option<cct::Code>,
/// An identifier on the chip card for the application that provides the quoted information; an AID
/// (application ID).
    #[serde(default, rename = "ChipApplicationID")]
    pub chip_application_id: Option<cct::Identifier>,
/// The name of the cardholder.
    #[serde(default, rename = "HolderName")]
    pub holder_name: Option<cct::Text>,
/// The role of this card or the card holder (e.g., the buyer, when the card is used as a payment means
/// to pay for an item), expressed as a code.
    #[serde(default, rename = "RoleCode")]
    pub role_code: Option<cct::Code>,
}
