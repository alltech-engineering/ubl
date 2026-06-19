#[derive(Debug, Deserialize, Serialize)]
/// A counted number of non-monetary units, possibly including a fractional part.
///
/// UBL Dictionary Entry Name: `Quantity. Type`
///
/// Generated from XSD type `QuantityType`.
pub struct Quantity {
    #[serde(default, rename = "@unitCode")]
    pub unit_code: Option<String>,
/// (Deprecated) The quantity unit code list.
    #[serde(default, rename = "@unitCodeListID")]
    pub unit_code_list_id: Option<String>,
/// (Deprecated) The identification of the agency that maintains the quantity unit code list
    #[serde(default, rename = "@unitCodeListAgencyID")]
    pub unit_code_list_agency_id: Option<String>,
/// (Deprecated) The name of the agency which maintains the quantity unit code list.
    #[serde(default, rename = "@unitCodeListAgencyName")]
    pub unit_code_list_agency_name: Option<String>,
    #[serde(rename = "$text")]
    pub content: f64,
}
