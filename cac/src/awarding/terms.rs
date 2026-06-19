#[derive(Debug, Deserialize, Serialize)]
/// A class to define the terms for awarding a contract.
///
/// UBL Dictionary Entry Name: `Awarding Terms. Details`
///
/// Generated from XSD type `AwardingTermsType`.
pub struct AwardingTerms {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions: Option<ext::UblExtensions>,
/// A code signifying the weighting algorithm for awarding criteria. When multiple awarding criteria is
/// used, different weighting and choices management algorithms based upon scores and weights of all
/// award criteria can be used. An algorithm for weighting criteria will be reported in the call for
/// tenders document. It is used to determine how to perform the final management of tenders based on
/// the results in each of the established award criteria
    #[serde(default, rename = "WeightingAlgorithmCode")]
    pub weighting_algorithm_code: Option<cct::Code>,
/// Text describing terms under which the contract is to be awarded.
    #[serde(default, rename = "Description")]
    pub description: Vec<cct::Text>,
/// Text describing the committee of experts evaluating the subjective criteria for awarding the
/// contract.
    #[serde(default, rename = "TechnicalCommitteeDescription")]
    pub technical_committee_description: Vec<cct::Text>,
/// Text describing the exclusion criterion for abnormally low tenders.
    #[serde(default, rename = "LowTendersDescription")]
    pub low_tenders_description: Vec<cct::Text>,
/// Indicates whether a prize will be awarded (true) or not (false).
    #[serde(default, rename = "PrizeIndicator")]
    pub prize_indicator: Option<udt::Indicator>,
/// Number and value of the prizes to be awarded.
    #[serde(default, rename = "PrizeDescription")]
    pub prize_description: Vec<cct::Text>,
/// Details of payments to all participants.
    #[serde(default, rename = "PaymentDescription")]
    pub payment_description: Vec<cct::Text>,
/// Indicates if any service contract following the contest will be awarded to the winner or one of the
/// winners of the contest (true) or not (false).
    #[serde(default, rename = "FollowupContractIndicator")]
    pub followup_contract_indicator: Option<udt::Indicator>,
/// Indicates if the decision is binding on the buyer (true) or not (false).
    #[serde(default, rename = "BindingOnBuyerIndicator")]
    pub binding_on_buyer_indicator: Option<udt::Indicator>,
/// Indicates if no further negotiation is allowed (true) or not (false).
    #[serde(default, rename = "NoFurtherNegotiationIndicator")]
    pub no_further_negotiation_indicator: Option<udt::Indicator>,
/// Defines a criterion for awarding this tender.
    #[serde(default, rename = "AwardingCriterion")]
    pub awarding_criterion: Vec<AwardingCriterion>,
/// A member of a committee of experts evaluating the subjective criteria for awarding the contract.
    #[serde(default, rename = "TechnicalCommitteePerson")]
    pub technical_committee_person: Vec<crate::Person>,
/// Information about the value amount that will be offered to the winner depending on his rank.
    #[serde(default, rename = "Prize")]
    pub prize: Vec<crate::Prize>,
}
