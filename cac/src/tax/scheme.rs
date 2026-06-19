#[derive(Debug, Deserialize, Serialize)]
pub struct TaxScheme {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
    #[serde(default, rename = "TaxTypeCode")]
    pub tax_type_code: Option<cct::Code>,
    #[serde(default, rename = "CurrencyCode")]
    pub currency_code: Option<cct::Code>,
    #[serde(default, rename = "JurisdictionRegionAddress")]
    pub jurisdiction_region_address: Vec<crate::Address>,
}
