#[derive(Debug, Deserialize, Serialize)]
/// A number of monetary units specified using a given unit of currency.
///
/// UBL Dictionary Entry Name: `Amount. Type`
///
/// Generated from XSD type `AmountType`.
pub struct Amount {
/// The currency of the amount.
    #[serde(default, rename = "@currencyID")]
    pub currency_id: Option<String>,
/// (Deprecated) The VersionID of the UN/ECE Rec9 code list.
    #[serde(default, rename = "@currencyCodeListVersionID")]
    pub currency_code_list_version_id: Option<String>,
    #[serde(rename = "$text")]
    pub content: f64,
}
