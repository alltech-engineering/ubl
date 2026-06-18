#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceProviderParty {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ServiceTypeCode")]
    pub service_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "ServiceType")]
    pub service_type: Vec<super::cct::TextType>,
    #[serde(rename = "Party")]
    pub party: Box<Party>,
    #[serde(default, rename = "SellerContact")]
    pub seller_contact: Option<Contact>,
}
