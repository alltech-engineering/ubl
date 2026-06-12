// UBL Text and Name types.

use serde::{Deserialize, Serialize};

/// Base Text type with optional language identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Text {
    pub value: String,
    pub language_id: Option<String>,
}

impl Text {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into(), language_id: None }
    }
    pub fn with_language(mut self, lang: impl Into<String>) -> Self {
        self.language_id = Some(lang.into());
        self
    }
}

macro_rules! define_text {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(pub Text);
        impl $name {
            pub fn new(value: impl Into<String>) -> Self { Self(Text::new(value)) }
            pub fn value(&self) -> &str { &self.0.value }
        }
    };
}

define_text!(Description, "A textual description.");
define_text!(Note, "A free-form note or comment.");
define_text!(Information, "Additional information.");
define_text!(Instructions, "Instructions or directions.");
define_text!(Conditions, "Terms and conditions.");
define_text!(Remarks, "Free-form remarks or observations.");
define_text!(Comment, "A comment.");
define_text!(ChangeConditions, "Conditions for a change.");
define_text!(CancellationNote, "A note explaining a cancellation.");
define_text!(RejectionNote, "A note explaining a rejection.");
define_text!(OutstandingReason, "The reason something is outstanding.");
define_text!(BackorderReason, "The reason for a backorder.");
define_text!(DamageRemarks, "Remarks about damage.");
define_text!(DeliveryInstructions, "Delivery instructions.");
define_text!(HandlingInstructions, "Handling instructions.");
define_text!(SpecialInstructions, "Special instructions.");
define_text!(PackageLevelCode, "Package level code textual description.");
define_text!(PreviousMeterReadingMethod, "Previous meter reading method text.");
define_text!(SpecialTerms, "Special terms text.");
define_text!(TransportServiceProviderRemarks, "Transport service provider remarks.");
define_text!(OrderableUnit, "The unit that an item can be ordered in (e.g., Each, Box).");
define_text!(CanonicalizationMethod, "The canonicalization method for a digital signature.");
define_text!(SignatureMethod, "The signature method for a digital signature.");
define_text!(ModificationReasonDescription, "A description of the reason for a modification.");
define_text!(Reference, "A reference to associated information.");
define_text!(Purpose, "A textual statement of purpose.");
define_text!(BriefDescription, "A brief description.");
define_text!(RetailEventName, "The name of a retail event.");

// Name types
macro_rules! define_name {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(pub String);
        impl $name {
            pub fn new(value: impl Into<String>) -> Self { Self(value.into()) }
            pub fn value(&self) -> &str { &self.0 }
        }
    };
}

define_name!(Name, "A generic name.");
define_name!(FirstName, "A person's first name.");
define_name!(FamilyName, "A person's family name.");
define_name!(MiddleName, "A person's middle name.");
define_name!(OtherName, "A person's other/additional name.");
define_name!(Title, "A person's title (Mr, Mrs, Dr).");
define_name!(JobTitle, "A person's job title.");
define_name!(OrganizationName, "The name of an organization.");
define_name!(Department, "A department name.");
define_name!(BrandName, "A brand name.");
define_name!(CityName, "The name of a city.");
define_name!(CountryName, "The name of a country.");
define_name!(StreetName, "The name of a street.");
define_name!(AdditionalStreetName, "An additional street name.");
define_name!(BuildingName, "A building name.");
define_name!(BuildingNumber, "A building number.");
define_name!(Room, "A room number or name.");
define_name!(Floor, "A floor identifier.");
define_name!(Postbox, "A post office box number.");
define_name!(BlockName, "A block name.");
define_name!(District, "A district name.");
define_name!(Region, "A region/state/province name.");
define_name!(RegistrationName, "A registration name.");
define_name!(AliasName, "An alias name.");
define_name!(BirthplaceName, "The name of a birthplace.");
define_name!(CategoryName, "A category name.");
define_name!(CertificateType, "A certificate type name.");
define_name!(Channel, "A channel name.");
define_name!(CompanyLegalForm, "A company legal form name.");
define_name!(ContractName, "A contract name.");
define_name!(ContractSubdivision, "A contract subdivision name.");
define_name!(CountrySubentity, "A country subentity name.");
define_name!(HolderName, "The name of a holder.");
define_name!(Location, "A location name.");
define_name!(PaymentNote, "A payment note.");
define_name!(PlotIdentification, "A plot identification.");
define_name!(Position, "A position name.");
define_name!(PostalZone, "A postal zone name.");
define_name!(RoleName, "A role name.");
define_name!(ServiceName, "A service name.");
define_name!(ShippingMarks, "Shipping marks.");
define_name!(Telephone, "A telephone number.");
define_name!(VesselName, "A vessel name.");
define_name!(XPath, "An XPath expression.");



// --- Missing types referenced by CAC modules ---
define_text!(AdditionalInformation, "Additional information text.");
define_text!(AllowanceChargeReason, "Reason for an allowance or charge.");
define_text!(Keyword, "A keyword for searching or classification.");
define_text!(TaxExemptionReason, "Reason for tax exemption.");
define_text!(AccountingCost, "The accounting cost.");
define_text!(StatusReason, "The reason for a particular status.");
define_text!(DocumentDescription, "A textual description of a document.");
define_text!(ReferencedDocumentInternalAddress, "An internal address referenced within a document.");
define_text!(WarrantyInformation, "Text describing warranty terms.");
define_text!(CustomerReference, "A customer-specific reference.");
define_text!(RejectReason, "The reason for rejection.");
define_text!(TimingComplaint, "A complaint about timing.");
define_text!(RegulatoryDomain, "The regulatory domain for a procurement procedure.");
define_text!(WeightScoringMethodologyNote, "A note describing the weight scoring methodology.");
define_name!(EconomicOperatorGroupName, "The name of an economic operator group.");

// --- Generated from UBL 2.5 XSD ---
define_name!(AgencyName, "UBL CBC type: AgencyName.");
define_name!(CitySubdivisionName, "UBL CBC type: CitySubdivisionName.");
define_name!(FileName, "UBL CBC type: FileName.");
define_name!(MarketName, "UBL CBC type: MarketName.");
define_name!(MeterName, "UBL CBC type: MeterName.");
define_name!(ModelName, "UBL CBC type: ModelName.");
define_name!(ProperShippingName, "UBL CBC type: ProperShippingName.");
define_name!(RoamingPartnerName, "UBL CBC type: RoamingPartnerName.");
define_name!(TechnicalName, "UBL CBC type: TechnicalName.");

// --- Generated from UBL 2.5 XSD ---
define_text!(AcceptedVariantsDescription, "UBL CBC type: AcceptedVariantsDescription.");
define_text!(ActivityType, "UBL CBC type: ActivityType.");
define_text!(AdditionalConditions, "UBL CBC type: AdditionalConditions.");
define_text!(AdditionalMattersDescription, "UBL CBC type: AdditionalMattersDescription.");
define_text!(AnnotationContent, "UBL CBC type: AnnotationContent.");
define_text!(AntennaLocus, "UBL CBC type: AntennaLocus.");
define_text!(ApplicableCategory, "UBL CBC type: ApplicableCategory.");
define_text!(ApprovalStatus, "UBL CBC type: ApprovalStatus.");
define_text!(Article, "UBL CBC type: Article.");
define_text!(AwardingCriterionDescription, "UBL CBC type: AwardingCriterionDescription.");
define_text!(BuyerReference, "UBL CBC type: BuyerReference.");
define_text!(CalculationExpression, "UBL CBC type: CalculationExpression.");
define_text!(CandidateStatement, "UBL CBC type: CandidateStatement.");
define_text!(CargoAndBallastTankConditionDescription, "UBL CBC type: CargoAndBallastTankConditionDescription.");
define_text!(CarrierServiceInstructions, "UBL CBC type: CarrierServiceInstructions.");
define_text!(CertificationLevelDescription, "UBL CBC type: CertificationLevelDescription.");
define_text!(Characteristics, "UBL CBC type: Characteristics.");
define_text!(CodeValue, "UBL CBC type: CodeValue.");
define_text!(Condition, "UBL CBC type: Condition.");
define_text!(ConditionsDescription, "UBL CBC type: ConditionsDescription.");
define_text!(ConsumersEnergyLevel, "UBL CBC type: ConsumersEnergyLevel.");
define_text!(ConsumptionLevel, "UBL CBC type: ConsumptionLevel.");
define_text!(ConsumptionType, "UBL CBC type: ConsumptionType.");
define_text!(Content, "UBL CBC type: Content.");
define_text!(ContractType, "UBL CBC type: ContractType.");
define_text!(CorrectionType, "UBL CBC type: CorrectionType.");
define_text!(CourseOverGroundDirection, "UBL CBC type: CourseOverGroundDirection.");
define_text!(CriterionDescription, "UBL CBC type: CriterionDescription.");
define_text!(CurrentChargeType, "UBL CBC type: CurrentChargeType.");
define_text!(CustomsClearanceServiceInstructions, "UBL CBC type: CustomsClearanceServiceInstructions.");
define_text!(DataSendingCapability, "UBL CBC type: DataSendingCapability.");
define_text!(DemurrageInstructions, "UBL CBC type: DemurrageInstructions.");
define_text!(DistributionType, "UBL CBC type: DistributionType.");
define_text!(DocumentHash, "UBL CBC type: DocumentHash.");
define_text!(DocumentStatusReasonDescription, "UBL CBC type: DocumentStatusReasonDescription.");
define_text!(DocumentType, "UBL CBC type: DocumentType.");
define_text!(Duty, "UBL CBC type: Duty.");
define_text!(EffectDescription, "UBL CBC type: EffectDescription.");
define_text!(ElectronicDeviceDescription, "UBL CBC type: ElectronicDeviceDescription.");
define_text!(ElectronicMail, "UBL CBC type: ElectronicMail.");
define_text!(EmbeddedDocument, "UBL CBC type: EmbeddedDocument.");
define_text!(EmissionFactorSource, "UBL CBC type: EmissionFactorSource.");
define_text!(EmissionStandardReference, "UBL CBC type: EmissionStandardReference.");
define_text!(EstimatedTimingFurtherPublication, "UBL CBC type: EstimatedTimingFurtherPublication.");
define_text!(ExclusionReason, "UBL CBC type: ExclusionReason.");
define_text!(ExemptionReason, "UBL CBC type: ExemptionReason.");
define_text!(ExpectedDescription, "UBL CBC type: ExpectedDescription.");
define_text!(ExportReason, "UBL CBC type: ExportReason.");
define_text!(Expression, "UBL CBC type: Expression.");
define_text!(Extension, "UBL CBC type: Extension.");
define_text!(FeeDescription, "UBL CBC type: FeeDescription.");
define_text!(ForwarderServiceInstructions, "UBL CBC type: ForwarderServiceInstructions.");
define_text!(Frequency, "UBL CBC type: Frequency.");
define_text!(FuelType, "UBL CBC type: FuelType.");
define_text!(FundingProgram, "UBL CBC type: FundingProgram.");
define_text!(GivenTreatmentDescription, "UBL CBC type: GivenTreatmentDescription.");
define_text!(GroupType, "UBL CBC type: GroupType.");
define_text!(GroupingLots, "UBL CBC type: GroupingLots.");
define_text!(HashAlgorithmMethod, "UBL CBC type: HashAlgorithmMethod.");
define_text!(HaulageInstructions, "UBL CBC type: HaulageInstructions.");
define_text!(HeatingType, "UBL CBC type: HeatingType.");
define_text!(ISSCAbsenceReason, "UBL CBC type: ISSCAbsenceReason.");
define_text!(InhouseMail, "UBL CBC type: InhouseMail.");
define_text!(InstructionNote, "UBL CBC type: InstructionNote.");
define_text!(InsuranceTypeDescription, "UBL CBC type: InsuranceTypeDescription.");
define_text!(InvoicingPartyReference, "UBL CBC type: InvoicingPartyReference.");
define_text!(JurisdictionLevel, "UBL CBC type: JurisdictionLevel.");
define_text!(Justification, "UBL CBC type: Justification.");
define_text!(JustificationDescription, "UBL CBC type: JustificationDescription.");
define_text!(LatestMeterReadingMethod, "UBL CBC type: LatestMeterReadingMethod.");
define_text!(LegalReference, "UBL CBC type: LegalReference.");
define_text!(LifecycleStageDescription, "UBL CBC type: LifecycleStageDescription.");
define_text!(LimitationDescription, "UBL CBC type: LimitationDescription.");
define_text!(Line, "UBL CBC type: Line.");
define_text!(ListValue, "UBL CBC type: ListValue.");
define_text!(Login, "UBL CBC type: Login.");
define_text!(LossRisk, "UBL CBC type: LossRisk.");
define_text!(LowTendersDescription, "UBL CBC type: LowTendersDescription.");
define_text!(MaintenanceFrequencyDescription, "UBL CBC type: MaintenanceFrequencyDescription.");
define_text!(ManifestType, "UBL CBC type: ManifestType.");
define_text!(MarkAttention, "UBL CBC type: MarkAttention.");
define_text!(MarkCare, "UBL CBC type: MarkCare.");
define_text!(MaximumValue, "UBL CBC type: MaximumValue.");
define_text!(MessageFormat, "UBL CBC type: MessageFormat.");
define_text!(MeterConstant, "UBL CBC type: MeterConstant.");
define_text!(MeterNumber, "UBL CBC type: MeterNumber.");
define_text!(MeterReadingComments, "UBL CBC type: MeterReadingComments.");
define_text!(MeterReadingType, "UBL CBC type: MeterReadingType.");
define_text!(MinimumImprovementBid, "UBL CBC type: MinimumImprovementBid.");
define_text!(MinimumValue, "UBL CBC type: MinimumValue.");
define_text!(MonetaryScope, "UBL CBC type: MonetaryScope.");
define_text!(MovieTitle, "UBL CBC type: MovieTitle.");
define_text!(NameSuffix, "UBL CBC type: NameSuffix.");
define_text!(NatureOfIllnessDescription, "UBL CBC type: NatureOfIllnessDescription.");
define_text!(NegotiationDescription, "UBL CBC type: NegotiationDescription.");
define_text!(NoControlActionsReason, "UBL CBC type: NoControlActionsReason.");
define_text!(OfficialUse, "UBL CBC type: OfficialUse.");
define_text!(OneTimeChargeType, "UBL CBC type: OneTimeChargeType.");
define_text!(OptionsDescription, "UBL CBC type: OptionsDescription.");
define_text!(OrganizationDepartment, "UBL CBC type: OrganizationDepartment.");
define_text!(OtherControlActions, "UBL CBC type: OtherControlActions.");
define_text!(OtherInstruction, "UBL CBC type: OtherInstruction.");
define_text!(PackagingType, "UBL CBC type: PackagingType.");
define_text!(PackingMaterial, "UBL CBC type: PackingMaterial.");
define_text!(PartyType, "UBL CBC type: PartyType.");
define_text!(Password, "UBL CBC type: Password.");
define_text!(PayPerView, "UBL CBC type: PayPerView.");
define_text!(PayerReference, "UBL CBC type: PayerReference.");
define_text!(PaymentDescription, "UBL CBC type: PaymentDescription.");
define_text!(PaymentMeansDescription, "UBL CBC type: PaymentMeansDescription.");
define_text!(PaymentOrderReference, "UBL CBC type: PaymentOrderReference.");
define_text!(PersonalSituation, "UBL CBC type: PersonalSituation.");
define_text!(PhoneNumber, "UBL CBC type: PhoneNumber.");
define_text!(PlacardEndorsement, "UBL CBC type: PlacardEndorsement.");
define_text!(PlacardNotation, "UBL CBC type: PlacardNotation.");
define_text!(PlannedInspectionsDescription, "UBL CBC type: PlannedInspectionsDescription.");
define_text!(PlannedOperationsDescription, "UBL CBC type: PlannedOperationsDescription.");
define_text!(PlannedWorksDescription, "UBL CBC type: PlannedWorksDescription.");
define_text!(PriceChangeReason, "UBL CBC type: PriceChangeReason.");
define_text!(PriceRevisionFormulaDescription, "UBL CBC type: PriceRevisionFormulaDescription.");
define_text!(PriceType, "UBL CBC type: PriceType.");
define_text!(PrintQualifier, "UBL CBC type: PrintQualifier.");
define_text!(Priority, "UBL CBC type: Priority.");
define_text!(PrizeDescription, "UBL CBC type: PrizeDescription.");
define_text!(ProcessDescription, "UBL CBC type: ProcessDescription.");
define_text!(ProcessReason, "UBL CBC type: ProcessReason.");
define_text!(ProcurementType, "UBL CBC type: ProcurementType.");
define_text!(PurposeType, "UBL CBC type: PurposeType.");
define_text!(Rank, "UBL CBC type: Rank.");
define_text!(RecurringProcurementDescription, "UBL CBC type: RecurringProcurementDescription.");
define_text!(RegistrationNationality, "UBL CBC type: RegistrationNationality.");
define_text!(ReplenishmentOwnerDescription, "UBL CBC type: ReplenishmentOwnerDescription.");
define_text!(ReportType, "UBL CBC type: ReportType.");
define_text!(RepresentationType, "UBL CBC type: RepresentationType.");
define_text!(ResidenceType, "UBL CBC type: ResidenceType.");
define_text!(Resolution, "UBL CBC type: Resolution.");
define_text!(ResourceOriginDescription, "UBL CBC type: ResourceOriginDescription.");
define_text!(Response, "UBL CBC type: Response.");
define_text!(RoleDescription, "UBL CBC type: RoleDescription.");
define_text!(SealingPartyType, "UBL CBC type: SealingPartyType.");
define_text!(ServiceNumberCalled, "UBL CBC type: ServiceNumberCalled.");
define_text!(ServiceType, "UBL CBC type: ServiceType.");
define_text!(ShipmentStageType, "UBL CBC type: ShipmentStageType.");
define_text!(ShipsRequirements, "UBL CBC type: ShipsRequirements.");
define_text!(SickAnimalDescription, "UBL CBC type: SickAnimalDescription.");
define_text!(SpecialFormDescription, "UBL CBC type: SpecialFormDescription.");
define_text!(SpecialServiceInstructions, "UBL CBC type: SpecialServiceInstructions.");
define_text!(SpecialTransportRequirements, "UBL CBC type: SpecialTransportRequirements.");
define_text!(Status, "UBL CBC type: Status.");
define_text!(StowawayDescription, "UBL CBC type: StowawayDescription.");
define_text!(SubTypeDescription, "UBL CBC type: SubTypeDescription.");
define_text!(Subject, "UBL CBC type: Subject.");
define_text!(SubscriberType, "UBL CBC type: SubscriberType.");
define_text!(SummaryDescription, "UBL CBC type: SummaryDescription.");
define_text!(TariffDescription, "UBL CBC type: TariffDescription.");
define_text!(TechnicalCommitteeDescription, "UBL CBC type: TechnicalCommitteeDescription.");
define_text!(TelecommunicationsServiceCall, "UBL CBC type: TelecommunicationsServiceCall.");
define_text!(TelecommunicationsServiceCategory, "UBL CBC type: TelecommunicationsServiceCategory.");
define_text!(TelecommunicationsSupplyType, "UBL CBC type: TelecommunicationsSupplyType.");
define_text!(Telefax, "UBL CBC type: Telefax.");
define_text!(TestMethod, "UBL CBC type: TestMethod.");
define_text!(TierRange, "UBL CBC type: TierRange.");
define_text!(TimeAmount, "UBL CBC type: TimeAmount.");
define_text!(TimezoneOffset, "UBL CBC type: TimezoneOffset.");
define_text!(TradingRestrictions, "UBL CBC type: TradingRestrictions.");
define_text!(TransitDescription, "UBL CBC type: TransitDescription.");
define_text!(TransportServiceProviderSpecialTerms, "UBL CBC type: TransportServiceProviderSpecialTerms.");
define_text!(TransportUserRemarks, "UBL CBC type: TransportUserRemarks.");
define_text!(TransportUserSpecialTerms, "UBL CBC type: TransportUserSpecialTerms.");
define_text!(TransportationServiceDescription, "UBL CBC type: TransportationServiceDescription.");
define_text!(UNPackingGroup, "UBL CBC type: UNPackingGroup.");
define_text!(ValidateProcess, "UBL CBC type: ValidateProcess.");
define_text!(ValidateTool, "UBL CBC type: ValidateTool.");
define_text!(ValidateToolVersion, "UBL CBC type: ValidateToolVersion.");
define_text!(Value, "UBL CBC type: Value.");
define_text!(ValueQualifier, "UBL CBC type: ValueQualifier.");
define_text!(WasteTypeDescription, "UBL CBC type: WasteTypeDescription.");
define_text!(WeighingDeviceType, "UBL CBC type: WeighingDeviceType.");
define_text!(Weight, "UBL CBC type: Weight.");
define_text!(WeightingConsiderationDescription, "UBL CBC type: WeightingConsiderationDescription.");
define_text!(WorkItemDescription, "UBL CBC type: WorkItemDescription.");
define_text!(WorkPhase, "UBL CBC type: WorkPhase.");
define_text!(WorkTypeDescription, "UBL CBC type: WorkTypeDescription.");
