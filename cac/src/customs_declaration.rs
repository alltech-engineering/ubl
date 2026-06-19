#[derive(Debug, Deserialize, Serialize)]
pub struct CustomsDeclaration {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
    #[serde(default, rename = "FunctionCode")]
    pub function_code: Option<cct::Code>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<Period>,
    #[serde(default, rename = "ApplicableTerritoryAddress")]
    pub applicable_territory_address: Option<Address>,
    #[serde(default, rename = "Shipment")]
    pub shipment: Option<Shipment>,
    #[serde(default, rename = "CustomsExitOfficeLocation")]
    pub customs_exit_office_location: Option<Location>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<Party>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: Option<Party>,
    #[serde(default, rename = "ConsigneeParty")]
    pub consignee_party: Option<Party>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: Option<Party>,
    #[serde(default, rename = "CustomsParty")]
    pub customs_party: Option<Party>,
    #[serde(default, rename = "PreviousCustomsDeclaration")]
    pub previous_customs_declaration: Option<Box<CustomsDeclaration>>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<DocumentReference>,
}
