#[derive(Debug, Deserialize, Serialize)]
pub struct AwardingTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "WeightingAlgorithmCode")]
    pub weighting_algorithm_code: Option<cct::Code>,
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
    #[serde(default, rename = "TechnicalCommitteeDescription")]
    pub technical_committee_description: Vec<cct::Text>,
    #[serde(default, rename = "LowTendersDescription")]
    pub low_tenders_description: Vec<cct::Text>,
    #[serde(default, rename = "PrizeIndicator")]
    pub prize_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "PrizeDescription")]
    pub prize_description: Vec<cct::Text>,
    #[serde(default, rename = "PaymentDescription")]
    pub payment_description: Vec<cct::Text>,
    #[serde(default, rename = "FollowupContractIndicator")]
    pub followup_contract_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "BindingOnBuyerIndicator")]
    pub binding_on_buyer_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "NoFurtherNegotiationIndicator")]
    pub no_further_negotiation_indicator: Option<udt::Indicator>,
    #[serde(default, rename = "AwardingCriterion")]
    pub awarding_criterion: Vec<AwardingCriterion>,
    #[serde(default, rename = "TechnicalCommitteePerson")]
    pub technical_committee_person: Vec<crate::Person>,
    #[serde(default, rename = "Prize")]
    pub prize: Vec<crate::Prize>,
}
