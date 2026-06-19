#[derive(Debug, Deserialize, Serialize)]
/// A class to describe an immobilized security to be used as a guarantee.
///
/// UBL Dictionary Entry Name: `Immobilized Security. Details`
///
/// Generated from XSD type `ImmobilizedSecurityType`.
pub struct ImmobilizedSecurity {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for the certificate of this immobilized security.
    #[serde(default, rename = "ImmobilizationCertificateID")]
    pub immobilization_certificate_id: Option<cct::Identifier>,
/// An identifier for the security being immobilized.
    #[serde(default, rename = "SecurityID")]
    pub security_id: Option<cct::Identifier>,
/// The date on which this immobilized security was issued.
    #[serde(default, rename = "IssueDate")]
    pub issue_date: Option<udt::DateTime>,
/// The value of the security on the day it was immobilized.
    #[serde(default, rename = "FaceValueAmount")]
    pub face_value_amount: Option<cct::Amount>,
/// The current market value of the immobilized security.
    #[serde(default, rename = "MarketValueAmount")]
    pub market_value_amount: Option<cct::Amount>,
/// The number of shares immobilized.
    #[serde(default, rename = "SharesNumberQuantity")]
    pub shares_number_quantity: Option<cct::Quantity>,
/// The Party who issues the Immobilized Security Certificate.
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<Party>,
}
