#[derive(Debug, Deserialize, Serialize)]
pub struct HazardousItem {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(default, rename = "PlacardNotation")]
    pub placard_notation: Option<cct::Text>,
    #[serde(default, rename = "PlacardEndorsement")]
    pub placard_endorsement: Option<cct::Text>,
    #[serde(default, rename = "AdditionalInformation")]
    pub additional_information: Vec<cct::Text>,
    #[serde(default, rename = "UNDGCode")]
    pub undg_code: Option<cct::Code>,
    #[serde(default, rename = "UNPackingGroupCode")]
    pub un_packing_group_code: Option<cct::Code>,
    #[serde(default, rename = "UNPackingGroup")]
    pub un_packing_group: Vec<cct::Text>,
    #[serde(default, rename = "EmergencyProceduresCode")]
    pub emergency_procedures_code: Option<cct::Code>,
    #[serde(default, rename = "MedicalFirstAidGuideCode")]
    pub medical_first_aid_guide_code: Option<cct::Code>,
    #[serde(default, rename = "TunnelRestrictionCode")]
    pub tunnel_restriction_code: Option<cct::Code>,
    #[serde(default, rename = "MaritimePollutantCode")]
    pub maritime_pollutant_code: Option<cct::Code>,
    #[serde(default, rename = "TechnicalName")]
    pub technical_name: Option<cct::Text>,
    #[serde(default, rename = "CategoryName")]
    pub category_name: Option<cct::Text>,
    #[serde(default, rename = "ProperShippingName")]
    pub proper_shipping_name: Option<cct::Text>,
    #[serde(default, rename = "HazardousCategoryCode")]
    pub hazardous_category_code: Option<cct::Code>,
    #[serde(default, rename = "UpperOrangeHazardPlacardID")]
    pub upper_orange_hazard_placard_id: Option<cct::Identifier>,
    #[serde(default, rename = "LowerOrangeHazardPlacardID")]
    pub lower_orange_hazard_placard_id: Option<cct::Identifier>,
    #[serde(default, rename = "MarkingID")]
    pub marking_id: Option<cct::Identifier>,
    #[serde(default, rename = "HazardClassID")]
    pub hazard_class_id: Option<cct::Identifier>,
    #[serde(default, rename = "HazardousTypeCode")]
    pub hazardous_type_code: Option<cct::Code>,
    #[serde(default, rename = "PackagingDangerLevelCode")]
    pub packaging_danger_level_code: Option<cct::Code>,
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: Option<cct::Measure>,
    #[serde(default, rename = "NetWeightMeasure")]
    pub net_weight_measure: Option<cct::Measure>,
    #[serde(default, rename = "NetVolumeMeasure")]
    pub net_volume_measure: Option<cct::Measure>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
    #[serde(default, rename = "ContactParty")]
    pub contact_party: Option<crate::Party>,
    #[serde(default, rename = "SecondaryHazard")]
    pub secondary_hazard: Vec<crate::SecondaryHazard>,
    #[serde(default, rename = "HazardousGoodsTransit")]
    pub hazardous_goods_transit: Vec<HazardousGoodsTransit>,
    #[serde(default, rename = "EmergencyTemperature")]
    pub emergency_temperature: Option<crate::Temperature>,
    #[serde(default, rename = "FlashpointTemperature")]
    pub flashpoint_temperature: Option<crate::Temperature>,
    #[serde(default, rename = "AdditionalTemperature")]
    pub additional_temperature: Vec<crate::Temperature>,
    #[serde(default, rename = "PositionOnBoardStowage")]
    pub position_on_board_stowage: Option<crate::Stowage>,
    #[serde(default, rename = "RadioactiveMaterial")]
    pub radioactive_material: Vec<crate::RadioactiveMaterial>,
    #[serde(default, rename = "Package")]
    pub package: Option<crate::Package>,
}
