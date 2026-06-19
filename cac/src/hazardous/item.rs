#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a hazardous item.
///
/// UBL Dictionary Entry Name: `Hazardous Item. Details`
///
/// Generated from XSD type `HazardousItemType`.
pub struct HazardousItem {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this hazardous item.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// Text of the placard notation corresponding to the hazard class of this hazardous item. Can also be
/// the hazard identification number of the orange placard (upper part) required on the means of
/// transport.
    #[serde(default, rename = "PlacardNotation")]
    pub placard_notation: Option<cct::Text>,
/// Text of the placard endorsement that is to be shown on the shipping papers for this hazardous item.
/// Can also be used for the number of the orange placard (lower part) required on the means of
/// transport.
    #[serde(default, rename = "PlacardEndorsement")]
    pub placard_endorsement: Option<cct::Text>,
/// Text providing further information about the hazardous substance.
    #[serde(default, rename = "AdditionalInformation")]
    pub additional_information: Vec<cct::Text>,
/// The UN code for this kind of hazardous item.
    #[serde(default, rename = "UNDGCode")]
    pub undg_code: Option<cct::Code>,
/// A code signifying the UN Packing Group protective packaging requirements for this hazardous item.
    #[serde(default, rename = "UNPackingGroupCode")]
    pub un_packing_group_code: Option<cct::Code>,
/// A text describing the UN Packing Group protective packaging requirements for this hazardous item.
    #[serde(default, rename = "UNPackingGroup")]
    pub un_packing_group: Vec<cct::Text>,
/// A code signifying the emergency procedures for this hazardous item.
    #[serde(default, rename = "EmergencyProceduresCode")]
    pub emergency_procedures_code: Option<cct::Code>,
/// A code signifying a medical first aid guide appropriate to this hazardous item.
    #[serde(default, rename = "MedicalFirstAidGuideCode")]
    pub medical_first_aid_guide_code: Option<cct::Code>,
/// A code signifying the restrictions for this hazardous item for passing through a tunnel.
    #[serde(default, rename = "TunnelRestrictionCode")]
    pub tunnel_restriction_code: Option<cct::Code>,
/// A code for specifying the maritime pollutant for this hazardous item.
    #[serde(default, rename = "MaritimePollutantCode")]
    pub maritime_pollutant_code: Option<cct::Code>,
/// The full technical name of a specific hazardous substance contained in this goods item.
    #[serde(default, rename = "TechnicalName")]
    pub technical_name: Option<cct::Text>,
/// The name of the category of hazard that applies to the Item.
    #[serde(default, rename = "CategoryName")]
    pub category_name: Option<cct::Text>,
/// The proper shipping name supplemented.
    #[serde(default, rename = "ProperShippingName")]
    pub proper_shipping_name: Option<cct::Text>,
/// A code signifying a kind of hazard for a material.
    #[serde(default, rename = "HazardousCategoryCode")]
    pub hazardous_category_code: Option<cct::Code>,
/// The number for the upper part of the orange hazard placard required on the means of transport.
    #[serde(default, rename = "UpperOrangeHazardPlacardID")]
    pub upper_orange_hazard_placard_id: Option<cct::Identifier>,
/// The number for the lower part of the orange hazard placard required on the means of transport.
    #[serde(default, rename = "LowerOrangeHazardPlacardID")]
    pub lower_orange_hazard_placard_id: Option<cct::Identifier>,
/// An identifier to the marking of the Hazardous Item
    #[serde(default, rename = "MarkingID")]
    pub marking_id: Option<cct::Identifier>,
/// An identifier for the hazard class applicable to this hazardous item as defined by the relevant
/// regulation authority (e.g., the IMDG Class Number of the SOLAS Convention of IMO and the ADR/RID
/// Class Number for the road/rail environment).
    #[serde(default, rename = "HazardClassID")]
    pub hazard_class_id: Option<cct::Identifier>,
/// The code specifying the type of hazard for this hazardous item.
    #[serde(default, rename = "HazardousTypeCode")]
    pub hazardous_type_code: Option<cct::Code>,
/// The code specifying the level of danger that the packaging of these dangerous goods must cover for
/// transport purposes.
    #[serde(default, rename = "PackagingDangerLevelCode")]
    pub packaging_danger_level_code: Option<cct::Code>,
/// The measure of the gross weight (mass) of these transported hazardous items including packaging but
/// excluding the transport equipment.
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: Option<cct::Measure>,
/// The net weight of this hazardous item, excluding packaging.
    #[serde(default, rename = "NetWeightMeasure")]
    pub net_weight_measure: Option<cct::Measure>,
/// The volume of this hazardous item, excluding packaging and transport equipment.
    #[serde(default, rename = "NetVolumeMeasure")]
    pub net_volume_measure: Option<cct::Measure>,
/// The quantity of goods items in this hazardous item that are hazardous.
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
/// The individual, group, or body who is contacted in case of a hazardous incident associated with this
/// item.
    #[serde(default, rename = "ContactParty")]
    pub contact_party: Option<crate::Party>,
/// A secondary hazard associated with this hazardous item.
    #[serde(default, rename = "SecondaryHazard")]
    pub secondary_hazard: Vec<crate::SecondaryHazard>,
/// Information related to the transit of this kind of hazardous goods.
    #[serde(default, rename = "HazardousGoodsTransit")]
    pub hazardous_goods_transit: Vec<HazardousGoodsTransit>,
/// The threshold temperature at which emergency procedures apply in the handling of
/// temperature-controlled goods.
    #[serde(default, rename = "EmergencyTemperature")]
    pub emergency_temperature: Option<crate::Temperature>,
/// The flashpoint temperature of this hazardous item; i.e., the lowest temperature at which vapors
/// above a volatile combustible substance ignite in air when exposed to flame.
    #[serde(default, rename = "FlashpointTemperature")]
    pub flashpoint_temperature: Option<crate::Temperature>,
/// Another temperature relevant to the handling of this hazardous item.
    #[serde(default, rename = "AdditionalTemperature")]
    pub additional_temperature: Vec<crate::Temperature>,
/// A stowage indicating where to find this hazardous item.
    #[serde(default, rename = "PositionOnBoardStowage")]
    pub position_on_board_stowage: Option<crate::Stowage>,
/// The Radioactive Material (Class 7) of this Hazadous Item.
    #[serde(default, rename = "RadioactiveMaterial")]
    pub radioactive_material: Vec<crate::RadioactiveMaterial>,
/// The Package details for this Hazardous Item.
    #[serde(default, rename = "Package")]
    pub package: Option<crate::Package>,
}
