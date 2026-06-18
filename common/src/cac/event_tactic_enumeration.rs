#[derive(Debug, Deserialize, Serialize)]
pub struct EventTacticEnumeration {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ConsumerIncentiveTacticTypeCode")]
    pub consumer_incentive_tactic_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "DisplayTacticTypeCode")]
    pub display_tactic_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "FeatureTacticTypeCode")]
    pub feature_tactic_type_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "TradeItemPackingLabelingTypeCode")]
    pub trade_item_packing_labeling_type_code: Option<super::cct::CodeType>,
}
