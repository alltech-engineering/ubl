#[derive(Debug, Deserialize, Serialize)]
/// A class that outlines the telecommunication supply in details
///
/// UBL Dictionary Entry Name: `Telecommunications Supply Line. Details`
///
/// Generated from XSD type `TelecommunicationsSupplyLineType`.
pub struct TelecommunicationsSupplyLine {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this telecommunications supply line.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// The phone number used for this telecommunication supply line
    #[serde(rename = "PhoneNumber")]
    pub phone_number: cct::Text,
/// The description of the telecommunication supply line
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// An amount specifying the cost of this telecommunication line
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: Option<cct::Amount>,
/// The total amount for this telecommunications supply line, including all allowances, charges and
/// taxes.
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: Option<cct::Amount>,
/// Exchanges rates used to calculate the amount for this line.
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Vec<crate::ExchangeRate>,
/// An allowance or charge that applies to this telecommunication supply line.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
/// A total amount of taxes of a particular kind applicable to this telecommunications supply line
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<crate::TaxTotal>,
/// A telecommunications service (e.g., a telephone call).
    #[serde(default, rename = "TelecommunicationsService")]
    pub telecommunications_service: Vec<TelecommunicationsService>,
}
