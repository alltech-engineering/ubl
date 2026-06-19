#[derive(Debug, Deserialize, Serialize)]
/// A class describing identifiers or references relating to customs procedures.
///
/// UBL Dictionary Entry Name: `Customs Declaration. Details`
///
/// Generated from XSD type `CustomsDeclarationType`.
pub struct CustomsDeclaration {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// An identifier associated with customs related procedures.
    #[serde(rename = "ID")]
    pub id: cct::Identifier,
/// A code describing the function of this customs declaration.
    #[serde(default, rename = "FunctionCode")]
    pub function_code: Option<cct::Code>,
/// The period during which this customs declaration is valid
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: Option<Period>,
/// The area or region where this customs declaration applies
    #[serde(default, rename = "ApplicableTerritoryAddress")]
    pub applicable_territory_address: Option<Address>,
/// A reference to the shipment of goods being declared
    #[serde(default, rename = "Shipment")]
    pub shipment: Option<Shipment>,
/// The location of the exit office from where the goods will leave or have left the customs territory
    #[serde(default, rename = "CustomsExitOfficeLocation")]
    pub customs_exit_office_location: Option<Location>,
/// The Party who issues this Customs Declaration.
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: Option<Party>,
/// The Party who is reponsible for sending the goods.
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: Option<Party>,
/// The Party who receives the goods.
    #[serde(default, rename = "ConsigneeParty")]
    pub consignee_party: Option<Party>,
/// The Party who combines individual smaller consignments into a single larger shipment (a so-called
/// consolidated consignment or shipment) which is sent to a counterpart who mirrors the consolidator's
/// activity by dividing the consolidated consignment into its original components.
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: Option<Party>,
/// The Authority who processes this Customs Declaration.
    #[serde(default, rename = "CustomsParty")]
    pub customs_party: Option<Party>,
/// A reference to a previous version of this customs declaration
    #[serde(default, rename = "PreviousCustomsDeclaration")]
    pub previous_customs_declaration: Option<Box<CustomsDeclaration>>,
/// A reference to additional documents relevant or related to this customs declaration
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: Vec<DocumentReference>,
}
