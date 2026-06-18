#[derive(Debug, Deserialize, Serialize)]
pub struct TaxScheme {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: Option<super::cct::TextType>,
    #[serde(default, rename = "TaxTypeCode")]
    pub tax_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CurrencyCode")]
    pub currency_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "JurisdictionRegionAddress")]
    pub jurisdiction_region_address: Vec<Address>,
}
