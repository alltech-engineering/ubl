#[derive(Debug, Deserialize, Serialize)]
pub struct EventTacticEnumeration {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
    #[serde(default, rename = "ConsumerIncentiveTacticTypeCode")]
    pub consumer_incentive_tactic_type_code: Option<cct::Code>,
    #[serde(default, rename = "DisplayTacticTypeCode")]
    pub display_tactic_type_code: Option<cct::Code>,
    #[serde(default, rename = "FeatureTacticTypeCode")]
    pub feature_tactic_type_code: Option<cct::Code>,
    #[serde(default, rename = "TradeItemPackingLabelingTypeCode")]
    pub trade_item_packing_labeling_type_code: Option<cct::Code>,
}
