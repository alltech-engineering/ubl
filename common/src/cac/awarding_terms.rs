#[derive(Debug, Deserialize, Serialize)]
pub struct AwardingTerms {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "WeightingAlgorithmCode")]
    pub weighting_algorithm_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: Vec<super::cct::TextType>,
    #[serde(default, rename = "TechnicalCommitteeDescription")]
    pub technical_committee_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "LowTendersDescription")]
    pub low_tenders_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "PrizeIndicator")]
    pub prize_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "PrizeDescription")]
    pub prize_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "PaymentDescription")]
    pub payment_description: Vec<super::cct::TextType>,
    #[serde(default, rename = "FollowupContractIndicator")]
    pub followup_contract_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "BindingOnBuyerIndicator")]
    pub binding_on_buyer_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "NoFurtherNegotiationIndicator")]
    pub no_further_negotiation_indicator: Option<super::udt::IndicatorType>,
    #[serde(default, rename = "AwardingCriterion")]
    pub awarding_criterion: Vec<AwardingCriterion>,
    #[serde(default, rename = "TechnicalCommitteePerson")]
    pub technical_committee_person: Vec<Person>,
    #[serde(default, rename = "Prize")]
    pub prize: Vec<Prize>,
}
