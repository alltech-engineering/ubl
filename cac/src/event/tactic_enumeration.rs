#[derive(Debug, Deserialize, Serialize)]
/// A class to define a set of codes that describes a retail tactic.
///
/// UBL Dictionary Entry Name: `Event Tactic Enumeration. Details`
///
/// Generated from XSD type `EventTacticEnumerationType`.
pub struct EventTacticEnumeration {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// A code signifying the type of consumer incentive. Examples include:Free Item, Temporary Price
/// reduction
    #[serde(default, rename = "ConsumerIncentiveTacticTypeCode")]
    pub consumer_incentive_tactic_type_code: Option<cct::Code>,
/// A code signifying the type of display. Examples Include: ON_COUNTER_DISPLAY, FLOOR_GRAPHICS
/// FLOOR_STACK_DISPLAY
    #[serde(default, rename = "DisplayTacticTypeCode")]
    pub display_tactic_type_code: Option<cct::Code>,
/// A code signifying a special feature. Examples Include: BILLBOARD DIRECT_MAIL_AD, FLYER
    #[serde(default, rename = "FeatureTacticTypeCode")]
    pub feature_tactic_type_code: Option<cct::Code>,
/// A code signifying the type of trade item packing and labeling. Examples Include: BONUS_SIZE
/// CO_BRANDED_TRADE_ITEM
    #[serde(default, rename = "TradeItemPackingLabelingTypeCode")]
    pub trade_item_packing_labeling_type_code: Option<cct::Code>,
}
