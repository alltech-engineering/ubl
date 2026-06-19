#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a taxation scheme (e.g., VAT, State tax, County tax).
///
/// UBL Dictionary Entry Name: `Tax Scheme. Details`
///
/// Generated from XSD type `TaxSchemeType`.
pub struct TaxScheme {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this taxation scheme.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The name of this taxation scheme.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// A code signifying the type of tax.
    #[serde(default, rename = "TaxTypeCode")]
    pub tax_type_code: Option<cct::Code>,
/// A code signifying the currency in which the tax is collected and reported.
    #[serde(default, rename = "CurrencyCode")]
    pub currency_code: Option<cct::Code>,
/// A geographic area in which this taxation scheme applies.
    #[serde(default, rename = "JurisdictionRegionAddress")]
    pub jurisdiction_region_address: Vec<crate::Address>,
}
