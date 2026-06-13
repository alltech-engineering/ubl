// UBL Indicator types — boolean flags.

use serde::{Deserialize, Serialize};

macro_rules! define_indicator {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub bool);
        impl $name {
            pub fn new(value: bool) -> Self { Self(value) }
            pub fn is_true(&self) -> bool { self.0 }
        }
        impl From<bool> for $name {
            fn from(b: bool) -> Self { Self(b) }
        }
    };
}

define_indicator!(Indicator, "A generic indicator (true/false).");
define_indicator!(ChargeIndicator, "Whether this is a charge (true) or allowance (false).");
define_indicator!(CopyIndicator, "Whether this is a copy of the original document.");
define_indicator!(BackOrderAllowedIndicator, "Whether back orders are allowed.");
define_indicator!(CatalogueIndicator, "Whether a catalogue is referenced.");
define_indicator!(CompletionIndicator, "Whether something is complete.");
define_indicator!(ItemUpdateRequestIndicator, "Whether an item specification update is requested.");
define_indicator!(ConsolidatableIndicator, "Whether goods can be consolidated.");
define_indicator!(ConsolidatedIndicator, "Whether goods are consolidated.");
define_indicator!(ContainerizedIndicator, "Whether goods are containerized.");
define_indicator!(DangerousGoodsApprovedIndicator, "Whether dangerous goods approval exists.");
define_indicator!(HumanFoodIndicator, "Whether the item is for human consumption.");
define_indicator!(HazardousRiskIndicator, "Whether there is a hazardous risk.");
define_indicator!(LivestockIndicator, "Whether livestock is involved.");
define_indicator!(OrderableIndicator, "Whether the item is orderable.");
define_indicator!(PartialDeliveryIndicator, "Whether partial delivery is allowed.");
define_indicator!(ReturnableMaterialIndicator, "Whether material is returnable.");
define_indicator!(TaxIncludedIndicator, "Whether tax is included in the amount.");
define_indicator!(AcceptedIndicator, "Whether something was accepted.");
define_indicator!(AdValoremIndicator, "Ad valorem indicator.");
define_indicator!(BulkCargoIndicator, "Bulk cargo indicator.");
define_indicator!(CabotageIndicator, "Cabotage indicator.");
define_indicator!(DangerousGoodsIndicator, "Whether goods are dangerous.");
define_indicator!(GovernmentAgreementConstraintIndicator, "Government agreement constraint.");
define_indicator!(LegalStatusIndicator, "Legal status indicator.");
define_indicator!(OnCarriageIndicator, "On-carriage indicator.");
define_indicator!(PreCarriageIndicator, "Pre-carriage indicator.");
define_indicator!(PricingUpdateRequestIndicator, "Whether a pricing update was requested.");
define_indicator!(SmallBusinessInclusionIndicator, "Small business inclusion indicator.");
define_indicator!(SoleProprietorshipIndicator, "Sole proprietorship indicator.");
define_indicator!(SpecialSecurityIndicator, "Special security indicator.");
define_indicator!(ThirdPartyPayerIndicator, "Third-party payer indicator.");
define_indicator!(ToOrderIndicator, "To-order indicator.");
define_indicator!(TransitDirectionIndicator, "Transit direction indicator.");
define_indicator!(VariantConstraintIndicator, "Variant constraint indicator.");


// --- Missing ---
define_indicator!(TaxEvidenceIndicator, "Whether tax evidence is present.");
define_indicator!(FreeOfChargeIndicator, "Indicates whether something is free of charge (true = free).");
define_indicator!(BasedOnConsensusIndicator, "Whether a forecast is based on consensus.");
define_indicator!(IndicationIndicator, "A generic indicator flag.");
define_indicator!(PublishAwardIndicator, "Whether the award should be published.");
define_indicator!(WithdrawOfferIndicator, "Whether an offer/tender is withdrawn.");

// --- Generated from UBL 2.5 XSD ---
define_indicator!(AcceptanceIndicator, "UBL CBC type: AcceptanceIndicator.");
define_indicator!(AnimalFoodApprovedIndicator, "UBL CBC type: AnimalFoodApprovedIndicator.");
define_indicator!(AnimalFoodIndicator, "UBL CBC type: AnimalFoodIndicator.");
define_indicator!(AtAnchorageIndicator, "UBL CBC type: AtAnchorageIndicator.");
define_indicator!(AuctionConstraintIndicator, "UBL CBC type: AuctionConstraintIndicator.");
define_indicator!(BalanceBroughtForwardIndicator, "UBL CBC type: BalanceBroughtForwardIndicator.");
define_indicator!(BindingOnBuyerIndicator, "UBL CBC type: BindingOnBuyerIndicator.");
define_indicator!(BuriedAtSeaIndicator, "UBL CBC type: BuriedAtSeaIndicator.");
define_indicator!(CandidateReductionConstraintIndicator, "UBL CBC type: CandidateReductionConstraintIndicator.");
define_indicator!(CustomsImportClassifiedIndicator, "UBL CBC type: CustomsImportClassifiedIndicator.");
define_indicator!(DiedIndicator, "UBL CBC type: DiedIndicator.");
define_indicator!(ElectronicCatalogueUsageIndicator, "UBL CBC type: ElectronicCatalogueUsageIndicator.");
define_indicator!(ElectronicInvoiceAcceptedIndicator, "UBL CBC type: ElectronicInvoiceAcceptedIndicator.");
define_indicator!(ElectronicOrderUsageIndicator, "UBL CBC type: ElectronicOrderUsageIndicator.");
define_indicator!(ElectronicPaymentUsageIndicator, "UBL CBC type: ElectronicPaymentUsageIndicator.");
define_indicator!(EvacuatedIndicator, "UBL CBC type: EvacuatedIndicator.");
define_indicator!(ExpectedAnchorageIndicator, "UBL CBC type: ExpectedAnchorageIndicator.");
define_indicator!(ExpectedIndicator, "UBL CBC type: ExpectedIndicator.");
define_indicator!(FollowupContractIndicator, "UBL CBC type: FollowupContractIndicator.");
define_indicator!(FridayAvailabilityIndicator, "UBL CBC type: FridayAvailabilityIndicator.");
define_indicator!(FrozenDocumentIndicator, "UBL CBC type: FrozenDocumentIndicator.");
define_indicator!(FulfilmentIndicator, "UBL CBC type: FulfilmentIndicator.");
define_indicator!(FullyPaidSharesIndicator, "UBL CBC type: FullyPaidSharesIndicator.");
define_indicator!(FumigatedCargoTransportIndicator, "UBL CBC type: FumigatedCargoTransportIndicator.");
define_indicator!(GeneralCargoIndicator, "UBL CBC type: GeneralCargoIndicator.");
define_indicator!(HumanFoodApprovedIndicator, "UBL CBC type: HumanFoodApprovedIndicator.");
define_indicator!(IMOGuidelinesOnBoardIndicator, "UBL CBC type: IMOGuidelinesOnBoardIndicator.");
define_indicator!(InfectiousDiseaseCaseOnBoardIndicator, "UBL CBC type: InfectiousDiseaseCaseOnBoardIndicator.");
define_indicator!(InitiatingPartyIndicator, "UBL CBC type: InitiatingPartyIndicator.");
define_indicator!(ManagementPlanImplementedIndicator, "UBL CBC type: ManagementPlanImplementedIndicator.");
define_indicator!(ManagementPlanOnBoardIndicator, "UBL CBC type: ManagementPlanOnBoardIndicator.");
define_indicator!(MarkAttentionIndicator, "UBL CBC type: MarkAttentionIndicator.");
define_indicator!(MarkCareIndicator, "UBL CBC type: MarkCareIndicator.");
define_indicator!(MedicalPractitionerConsultedIndicator, "UBL CBC type: MedicalPractitionerConsultedIndicator.");
define_indicator!(MondayAvailabilityIndicator, "UBL CBC type: MondayAvailabilityIndicator.");
define_indicator!(MoreIllThanExpectedIndicator, "UBL CBC type: MoreIllThanExpectedIndicator.");
define_indicator!(NoFurtherNegotiationIndicator, "UBL CBC type: NoFurtherNegotiationIndicator.");
define_indicator!(OptionalLineItemIndicator, "UBL CBC type: OptionalLineItemIndicator.");
define_indicator!(OtherConditionsIndicator, "UBL CBC type: OtherConditionsIndicator.");
define_indicator!(PowerIndicator, "UBL CBC type: PowerIndicator.");
define_indicator!(PrepaidIndicator, "UBL CBC type: PrepaidIndicator.");
define_indicator!(PrivatePartyIndicator, "UBL CBC type: PrivatePartyIndicator.");
define_indicator!(PrizeIndicator, "UBL CBC type: PrizeIndicator.");
define_indicator!(PublicPartyIndicator, "UBL CBC type: PublicPartyIndicator.");
define_indicator!(RecurringProcurementIndicator, "UBL CBC type: RecurringProcurementIndicator.");
define_indicator!(RefrigeratedIndicator, "UBL CBC type: RefrigeratedIndicator.");
define_indicator!(RefrigerationOnIndicator, "UBL CBC type: RefrigerationOnIndicator.");
define_indicator!(ReinspectionRequiredIndicator, "UBL CBC type: ReinspectionRequiredIndicator.");
define_indicator!(RenewalsIndicator, "UBL CBC type: RenewalsIndicator.");
define_indicator!(ReportedToMedicalOfficerIndicator, "UBL CBC type: ReportedToMedicalOfficerIndicator.");
define_indicator!(RequiredCurriculaIndicator, "UBL CBC type: RequiredCurriculaIndicator.");
define_indicator!(ResponseIndicator, "UBL CBC type: ResponseIndicator.");
define_indicator!(ReturnabilityIndicator, "UBL CBC type: ReturnabilityIndicator.");
define_indicator!(SMESuitableIndicator, "UBL CBC type: SMESuitableIndicator.");
define_indicator!(SSPOnBoardIndicator, "UBL CBC type: SSPOnBoardIndicator.");
define_indicator!(SSPSecurityMeasuresAppliedIndicator, "UBL CBC type: SSPSecurityMeasuresAppliedIndicator.");
define_indicator!(SanitaryMeasuresAppliedIndicator, "UBL CBC type: SanitaryMeasuresAppliedIndicator.");
define_indicator!(SaturdayAvailabilityIndicator, "UBL CBC type: SaturdayAvailabilityIndicator.");
define_indicator!(ServiceProviderPartyIndicator, "UBL CBC type: ServiceProviderPartyIndicator.");
define_indicator!(SickAnimalOnBoardIndicator, "UBL CBC type: SickAnimalOnBoardIndicator.");
define_indicator!(SplitConsignmentIndicator, "UBL CBC type: SplitConsignmentIndicator.");
define_indicator!(StatusAvailableIndicator, "UBL CBC type: StatusAvailableIndicator.");
define_indicator!(StillIllIndicator, "UBL CBC type: StillIllIndicator.");
define_indicator!(StillOnBoardIndicator, "UBL CBC type: StillOnBoardIndicator.");
define_indicator!(StowawaysFoundOnBoardIndicator, "UBL CBC type: StowawaysFoundOnBoardIndicator.");
define_indicator!(SundayAvailabilityIndicator, "UBL CBC type: SundayAvailabilityIndicator.");
define_indicator!(TerminatedIndicator, "UBL CBC type: TerminatedIndicator.");
define_indicator!(TestIndicator, "UBL CBC type: TestIndicator.");
define_indicator!(ThursdayAvailabilityIndicator, "UBL CBC type: ThursdayAvailabilityIndicator.");
define_indicator!(TuesdayAvailabilityIndicator, "UBL CBC type: TuesdayAvailabilityIndicator.");
define_indicator!(UnknownPriceIndicator, "UBL CBC type: UnknownPriceIndicator.");
define_indicator!(ValidISSCIndicator, "UBL CBC type: ValidISSCIndicator.");
define_indicator!(ValidSanitationCertificateOnBoardIndicator, "UBL CBC type: ValidSanitationCertificateOnBoardIndicator.");
define_indicator!(WednesdayAvailabilityIndicator, "UBL CBC type: WednesdayAvailabilityIndicator.");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_indicator_roundtrip() {
        let i = CopyIndicator::new(true);
        let json = serde_json::to_string(&i).unwrap();
        let i2: CopyIndicator = serde_json::from_str(&json).unwrap();
        assert_eq!(i.0, i2.0);
    }

    #[test]
    fn test_indicator_false() {
        let i = CopyIndicator::new(false);
        assert!(!i.0);
    }

    #[test]
    fn test_charge_indicator() {
        let i = ChargeIndicator::new(true);
        assert!(i.0);
    }
}
