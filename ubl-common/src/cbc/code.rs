// UBL Code types — coded values with optional code list metadata.
//
// In UBL XML: <cbc:InvoiceTypeCode listID="UNCL1001">380</cbc:InvoiceTypeCode>

use serde::{Deserialize, Serialize};

/// The base Code type.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Code {
    pub value: String,
    pub list_id: Option<String>,
    pub list_agency_id: Option<String>,
    pub list_version_id: Option<String>,
    pub name: Option<String>,
}

impl Code {
    pub fn new(value: impl Into<String>) -> Self {
        Self { value: value.into(), list_id: None, list_agency_id: None, list_version_id: None, name: None }
    }
    pub fn with_list(mut self, list_id: impl Into<String>, agency_id: impl Into<String>) -> Self {
        self.list_id = Some(list_id.into());
        self.list_agency_id = Some(agency_id.into());
        self
    }
}

macro_rules! define_code {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        pub struct $name(pub Code);
        impl $name {
            pub fn new(value: impl Into<String>) -> Self { Self(Code::new(value)) }
            pub fn value(&self) -> &str { &self.0.value }
        }
    };
}

// Document type codes
define_code!(InvoiceTypeCode, "A code specifying the type of invoice.");
define_code!(CreditNoteTypeCode, "A code specifying the type of credit note.");
define_code!(DebitNoteTypeCode, "A code specifying the type of debit note.");
define_code!(OrderTypeCode, "A code specifying the type of order.");
define_code!(OrderResponseCode, "A code specifying the response to an order.");
define_code!(DespatchAdviceTypeCode, "A code specifying the type of despatch advice.");
define_code!(ReceiptAdviceTypeCode, "A code specifying the type of receipt advice.");
define_code!(DocumentTypeCode, "A code specifying a document type.");
define_code!(DocumentType, "The type of document being referenced.");
define_code!(ResponseCode, "A code specifying a response.");
define_code!(LineStatusCode, "A code specifying the status of a document line.");
define_code!(TenderTypeCode, "A code specifying the type of tender.");
define_code!(CatalogueTypeCode, "A code specifying the type of catalogue.");
define_code!(ContractTypeCode, "A code specifying the type of contract.");
define_code!(StatementTypeCode, "A code specifying the type of statement.");
define_code!(ReminderTypeCode, "A code specifying reminder type.");
define_code!(RemittanceAdviceTypeCode, "A code specifying remittance advice type.");

// Industry standard codes
define_code!(CurrencyCode, "An ISO 4217 currency code (e.g., EUR, USD, ZAR).");
define_code!(CountryCode, "An ISO 3166-1 country code.");
define_code!(LanguageCode, "An ISO 639 language code.");
define_code!(UnitCode, "A unit of measure code (UN/ECE Rec. 20).");
define_code!(MimeCode, "A MIME type code (e.g., application/pdf).");
define_code!(TransportModeCode, "A code specifying the mode of transport.");
define_code!(TransportMeansTypeCode, "A code specifying the type of transport means.");
define_code!(PaymentMeansCode, "A code specifying the means of payment.");
define_code!(PaymentPurposeCode, "A code specifying the purpose of a payment.");
define_code!(TaxTypeCode, "A code specifying the type of tax.");
define_code!(TaxLevelCode, "A code specifying a tax level.");
define_code!(TaxExemptionReasonCode, "A code specifying the reason for tax exemption.");
define_code!(AllowanceChargeReasonCode, "A code specifying the reason for an allowance or charge.");
define_code!(CommodityCode, "A code specifying a commodity classification (e.g., HS code).");
define_code!(ItemClassificationCode, "A code specifying an item classification.");
define_code!(CargoTypeCode, "A code specifying the type of cargo.");
define_code!(DangerousGoodsCode, "A UNDG dangerous goods code.");
define_code!(PackagingTypeCode, "A code specifying the type of packaging.");
define_code!(HandlingCode, "A code specifying handling instructions.");
define_code!(InstructionCode, "A code specifying an instruction.");
define_code!(StatusCode, "A code specifying a status.");
define_code!(ActionCode, "A code specifying an action (add, change, delete).");
define_code!(ChannelCode, "A code specifying a communication channel.");
define_code!(CharacterSetCode, "A code specifying a character set.");
define_code!(ConditionCode, "A code specifying a condition.");
define_code!(ConfidentialityLevelCode, "A code specifying a confidentiality level.");
define_code!(CoordinateSystemCode, "A code specifying a coordinate system.");
define_code!(CorporateRegistrationTypeCode, "A code specifying corporate registration type.");
define_code!(CorrectionTypeCode, "A code specifying a correction type.");
define_code!(CustomsProcedureCode, "A code specifying a customs procedure.");
define_code!(CustomsStatusCode, "A customs status code.");
define_code!(DataSourceCode, "A code specifying a data source.");
define_code!(DeclarationTypeCode, "A code specifying the type of declaration.");
define_code!(DeliveryAcceptanceCode, "A code specifying delivery acceptance.");
define_code!(DirectionCode, "A code specifying direction.");
define_code!(DispositionCode, "A code specifying disposition.");
define_code!(EmergencyProceduresCode, "A code specifying emergency procedures.");
define_code!(EncodingCode, "A code specifying encoding.");
define_code!(EnvironmentalEmissionTypeCode, "A code specifying environmental emission type.");
define_code!(EvaluationCriterionTypeCode, "A code specifying evaluation criterion type.");
define_code!(EvidenceTypeCode, "A code specifying evidence type.");
define_code!(ExceptionResolutionCode, "A code specifying exception resolution.");
define_code!(ExceptionStatusCode, "A code specifying exception status.");
define_code!(ExemptionReasonCode, "A code specifying exemption reason.");
define_code!(ExpenseCode, "A code specifying an expense category.");
define_code!(ExportReasonCode, "A code specifying export reason.");
define_code!(FeatureTacticTypeCode, "A code specifying feature tactic type.");
define_code!(FinancingInstrumentCode, "A code specifying a financing instrument.");
define_code!(ForecastPurposeCode, "A code specifying forecast purpose.");
define_code!(ForecastTypeCode, "A code specifying forecast type.");
define_code!(FormatCode, "A code specifying a format.");
define_code!(FrequencyCode, "A code specifying a frequency.");
define_code!(GenderCode, "A code specifying gender.");
define_code!(GoodsItemStatusCode, "A code specifying goods item status.");
define_code!(GuaranteeTypeCode, "A code specifying guarantee type.");
define_code!(HazardousCategoryCode, "A code specifying hazardous category.");
define_code!(HazardousRegulationCode, "A code specifying hazardous regulation.");
define_code!(HeatingTypeCode, "A code specifying heating type.");
define_code!(IdentificationCode, "A code used for identification.");
define_code!(ImportanceCode, "A code specifying importance level.");
define_code!(IndustryClassificationCode, "A code for industry classification.");
define_code!(InhalationToxicityZoneCode, "A code specifying inhalation toxicity zone.");
define_code!(InspectionMethodCode, "A code specifying inspection method.");
define_code!(JobTitleCode, "A code specifying a job title.");
define_code!(JourneyTypeCode, "A code specifying journey type.");
define_code!(JustificationCode, "A code specifying justification.");
define_code!(LatitudeDirectionCode, "A code specifying latitude direction (N/S).");
define_code!(LifeCycleStatusCode, "A code specifying life cycle status.");
define_code!(LocaleCode, "A code specifying locale.");
define_code!(LocationTypeCode, "A code specifying location type.");
define_code!(LongitudeDirectionCode, "A code specifying longitude direction (E/W).");
define_code!(MandateTypeCode, "A code specifying mandate type.");
define_code!(MathematicOperatorCode, "A code specifying a mathematical operator.");
define_code!(MedicalFirstAidGuideCode, "A code specifying medical first aid guide.");
define_code!(MeterReadingTypeCode, "A code specifying meter reading type.");
define_code!(NatureOfTransactionCode, "A code specifying nature of transaction.");
define_code!(OptionTypeCode, "A code specifying option type.");
define_code!(OrganizationDepartmentCode, "A code specifying an organization department.");
define_code!(OutstandingReasonCode, "A code specifying outstanding reason.");
define_code!(OwnerTypeCode, "A code specifying owner type.");
define_code!(PackLevelCode, "A code specifying pack level.");
define_code!(PackagingDangerLevelCode, "A code specifying packaging danger level.");
define_code!(PartPresentationCode, "A code specifying part presentation.");
define_code!(PartyTypeCode, "A code specifying party type.");
define_code!(PerformanceMetricTypeCode, "A code specifying performance metric type.");
define_code!(PermitTypeCode, "A code specifying permit type.");
define_code!(PersonalSituationCode, "A code specifying personal situation.");
define_code!(PositionCode, "A code specifying position.");
define_code!(PostalZoneCode, "A postal zone code.");
define_code!(PreferenceCriterionCode, "A code specifying preference criterion.");
define_code!(PriceEvaluationCode, "A code specifying price evaluation.");
define_code!(PriceTypeCode, "A code specifying price type.");
define_code!(PriorityCode, "A code specifying priority.");
define_code!(ProcedureCode, "A code specifying a procedure.");
define_code!(ProcessReasonCode, "A code specifying process reason.");
define_code!(ProcurementSubTypeCode, "A code specifying procurement sub-type.");
define_code!(ProcurementTypeCode, "A code specifying procurement type.");
define_code!(ProductCode, "A product code.");
define_code!(ProfileStatusCode, "A profile status code.");
define_code!(PromotionalEventTypeCode, "A promotional event type code.");
define_code!(PropertyClassCode, "A property class code.");
define_code!(ProviderTypeCode, "A provider type code.");
define_code!(QualityControlCode, "A quality control code.");
define_code!(QuantityDiscrepancyCode, "A quantity discrepancy code.");
define_code!(RejectReasonCode, "A code specifying reject reason.");
define_code!(ResidenceTypeCode, "A residence type code.");
define_code!(ResolutionCode, "A resolution code.");
define_code!(ResponseTimeCode, "A response time code.");
define_code!(RetailEventNameCode, "A retail event name code.");
define_code!(RetailEventStatusCode, "A retail event status code.");
define_code!(ReturnabilityCode, "A returnability code.");
define_code!(RevisionStatusCode, "A revision status code.");
define_code!(RoleCode, "A code specifying a role.");
define_code!(RoundingDirectionCode, "A rounding direction code.");
define_code!(SealIssuerTypeCode, "A seal issuer type code.");
define_code!(SealStatusCode, "A seal status code.");
define_code!(SecurityClassificationCode, "A security classification code.");
define_code!(ServiceInformationPreferenceCode, "A service information preference code.");
define_code!(ServiceTypeCode, "A service type code.");
define_code!(SettlementMethodCode, "A settlement method code.");
define_code!(ShippingPriorityLevelCode, "A shipping priority level code.");
define_code!(ShortageActionCode, "A shortage action code.");
define_code!(SignatureMethodCode, "A signature method code.");
define_code!(SizeTypeCode, "A size type code.");
define_code!(SourceCurrencyCode, "A source currency code.");
define_code!(SpecialTransportRequirementsCode, "A special transport requirements code.");
define_code!(SpecificationTypeCode, "A specification type code.");
define_code!(SubmissionMethodCode, "A submission method code.");
define_code!(SubscriberTypeCode, "A subscriber type code.");
define_code!(SubstitutionStatusCode, "A substitution status code.");
define_code!(SupplyChainActivityTypeCode, "A supply chain activity type code.");
define_code!(TargetCurrencyCode, "A target currency code.");
define_code!(TariffClassCode, "A tariff class code.");
define_code!(TariffCode, "A tariff code (e.g., HS code).");
define_code!(TariffDescriptionCode, "A tariff description code.");
define_code!(TaxCurrencyCode, "A tax currency code.");
define_code!(TenderEnvelopeTypeCode, "A tender envelope type code.");
define_code!(TenderResultCode, "A tender result code.");
define_code!(TestMethodCode, "A test method code.");
define_code!(TextTypeCode, "A text type code.");
define_code!(TimeFrequencyCode, "A time frequency code.");
define_code!(TimingComplaintCode, "A timing complaint code.");
define_code!(TrackingDeviceCode, "A tracking device code.");
define_code!(TradeItemPackingLabelingTypeCode, "A trade item packing labeling type code.");
define_code!(TradeServiceCode, "A trade service code.");
define_code!(TransactionCurrencyCode, "A transaction currency code.");
define_code!(TransitDirectionCode, "A transit direction code.");
define_code!(TransportAuthorizationCode, "A transport authorization code.");
define_code!(TransportEmergencyCardCode, "A transport emergency card code.");
define_code!(TransportEquipmentTypeCode, "A transport equipment type code.");
define_code!(TransportEventTypeCode, "A transport event type code.");
define_code!(TransportHandlingUnitTypeCode, "A transport handling unit type code.");
define_code!(TransportServiceCode, "A transport service code.");
define_code!(TransportationServiceDescriptionCode, "A transportation service description code.");
define_code!(TransportationStatusTypeCode, "A transportation status type code.");
define_code!(TypeCode, "A generic type code.");
define_code!(UNDGCode, "A UN Dangerous Goods code.");
define_code!(ValidationResultCode, "A validation result code.");
define_code!(WaybillTypeCode, "A waybill type code.");
define_code!(WeightingAlgorithmCode, "A weighting algorithm code.");
define_code!(WorkPhaseCode, "A work phase code.");
define_code!(ZoneCode, "A zone code.");


// --- Missing types referenced by CAC modules ---
define_code!(AccountFormatCode, "A code specifying an account format.");
define_code!(AccountTypeCode, "A code specifying an account type.");
define_code!(AddressFormatCode, "A code specifying an address format.");
define_code!(AddressTypeCode, "A code specifying an address type.");
define_code!(CardChipCode, "A code specifying a card chip type.");
define_code!(CardTypeCode, "A code specifying a card type.");
define_code!(CountrySubentityCode, "A code for a country subentity.");
define_code!(AccountingCostCode, "A code for accounting cost.");
define_code!(DocumentStatusCode, "A code specifying the status of a document.");
define_code!(RejectActionCode, "A code specifying the action to take on rejection.");
define_code!(StatusReasonCode, "A code specifying the reason for a status.");
define_code!(ContractType, "A type of contract.");
