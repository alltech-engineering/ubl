use serde::{Deserialize, Serialize};
pub type AcceptanceTransportEvent = TransportEventType;
pub type AccessoryRelatedItem = RelatedItemType;
pub type AccountingContact = ContactType;
pub type AccountingCustomerParty = CustomerPartyType;
pub type AccountingSupplierParty = SupplierPartyType;
pub type ActivityDataLine = ActivityDataLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ActivityDataLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(rename = "SupplyChainActivityTypeCode")]
    pub supply_chain_activity_type_code: super::cct::CodeType,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<SupplierPartyType>,
    #[serde(default, rename = "ActivityPeriod")]
    pub activity_period: ::core::option::Option<PeriodType>,
    #[serde(rename = "ActivityOriginLocation")]
    pub activity_origin_location: LocationType,
    #[serde(default, rename = "ActivityFinalLocation")]
    pub activity_final_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "SalesItem")]
    pub sales_item: ::std::vec::Vec<SalesItemType>,
}
pub type ActivityFinalLocation = LocationType;
pub type ActivityOriginLocation = LocationType;
pub type ActivityPeriod = PeriodType;
pub type ActivityProperty = ActivityPropertyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ActivityPropertyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "Name")]
    pub name: super::cct::TextType,
    #[serde(rename = "Value")]
    pub value: super::cct::TextType,
}
pub type ActualArrivalTransportEvent = TransportEventType;
pub type ActualDeliveryTransportEvent = TransportEventType;
pub type ActualDepartureTransportEvent = TransportEventType;
pub type ActualPackage = PackageType;
pub type ActualPickupTransportEvent = TransportEventType;
pub type ActualWaypointTransportEvent = TransportEventType;
pub type AdditionalCommodityClassification = CommodityClassificationType;
pub type AdditionalDocumentReference = DocumentReferenceType;
pub type AdditionalDocumentResponse = DocumentResponseType;
pub type AdditionalFee = FeeType;
pub type AdditionalFuelProperty = FuelPropertyType;
pub type AdditionalInformationParty = PartyType;
pub type AdditionalInformationRequestPeriod = PeriodType;
pub type AdditionalItemIdentification = ItemIdentificationType;
pub type AdditionalItemProperty = ItemPropertyType;
pub type AdditionalNoticeLanguage = LanguageType;
pub type AdditionalPartyIdentification = PartyIdentificationType;
pub type AdditionalPortCallPurpose = PortCallPurposeType;
pub type AdditionalQualifyingParty = QualifyingPartyType;
pub type AdditionalSecurityMeasure = SecurityMeasureType;
pub type AdditionalTemperature = TemperatureType;
pub type AdditionalTransportationService = TransportationServiceType;
pub type AdditionalWebSite = WebSiteType;
pub type Address = AddressType;
pub type AddressLine = AddressLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct AddressLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Line")]
    pub line: ::std::vec::Vec<super::cct::TextType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AddressType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AddressTypeCode")]
    pub address_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "AddressFormatCode")]
    pub address_format_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Postbox")]
    pub postbox: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Floor")]
    pub floor: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Room")]
    pub room: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "StreetName")]
    pub street_name: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "AdditionalStreetName")]
    pub additional_street_name: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "BlockName")]
    pub block_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "BuildingName")]
    pub building_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "BuildingNumber")]
    pub building_number: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "InhouseMail")]
    pub inhouse_mail: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Department")]
    pub department: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "MarkAttention")]
    pub mark_attention: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "MarkCare")]
    pub mark_care: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "PlotIdentification")]
    pub plot_identification: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "CitySubdivisionName")]
    pub city_subdivision_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "CityName")]
    pub city_name: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PostalZone")]
    pub postal_zone: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CountrySubentity")]
    pub country_subentity: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CountrySubentityCode")]
    pub country_subentity_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Region")]
    pub region: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "District")]
    pub district: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "TimezoneOffset")]
    pub timezone_offset: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "AddressLine")]
    pub address_line: ::std::vec::Vec<AddressLineType>,
    #[serde(default, rename = "Country")]
    pub country: ::core::option::Option<CountryType>,
    #[serde(default, rename = "LocationCoordinate")]
    pub location_coordinate: ::std::vec::Vec<LocationCoordinateType>,
}
pub type AdoptionPeriod = PeriodType;
pub type AgentParty = PartyType;
pub type AgreementCountry = CountryType;
pub type AirTransport = AirTransportType;
#[derive(Debug, Deserialize, Serialize)]
pub struct AirTransportType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "AircraftID")]
    pub aircraft_id: super::cct::IdentifierType,
}
pub type AllowanceCharge = AllowanceChargeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct AllowanceChargeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(rename = "ChargeIndicator")]
    pub charge_indicator: super::udt::IndicatorType,
    #[serde(default, rename = "AllowanceChargeReasonCode")]
    pub allowance_charge_reason_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "AllowanceChargeReason")]
    pub allowance_charge_reason: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "MultiplierFactorNumeric")]
    pub multiplier_factor_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "PrepaidIndicator")]
    pub prepaid_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "SequenceNumeric")]
    pub sequence_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(rename = "Amount")]
    pub amount: super::cct::AmountType,
    #[serde(default, rename = "TaxInclusiveAmount")]
    pub tax_inclusive_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "BaseAmount")]
    pub base_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "PerUnitAmount")]
    pub per_unit_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxCategory")]
    pub tax_category: ::std::vec::Vec<TaxCategoryType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::core::option::Option<TaxTotalType>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: ::std::vec::Vec<PaymentMeansType>,
}
pub type AllowedSubcontractTerms = SubcontractTermsType;
pub type AlternativeConditionPrice = PriceType;
pub type AlternativeCurrencyPrice = PriceType;
pub type AlternativeDeliveryLocation = LocationType;
pub type AlternativeLineItem = LineItemType;
pub type Annotation = AnnotationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct AnnotationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "SubjectCode")]
    pub subject_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Subject")]
    pub subject: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "AnnotationContent")]
    pub annotation_content: ::std::vec::Vec<super::cct::TextType>,
}
pub type AnticipatedMonetaryTotal = MonetaryTotalType;
pub type AppealInformationParty = PartyType;
pub type AppealReceiverParty = PartyType;
pub type AppealTerms = AppealTermsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct AppealTermsType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PresentationPeriod")]
    pub presentation_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "AppealInformationParty")]
    pub appeal_information_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "AppealReceiverParty")]
    pub appeal_receiver_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "MediationParty")]
    pub mediation_party: ::core::option::Option<PartyType>,
}
pub type ApplicableAddress = AddressType;
pub type ApplicableDeliveryPeriod = PeriodType;
pub type ApplicablePeriod = PeriodType;
pub type ApplicableRadioactiveIsotope = RadioactiveIsotopeType;
pub type ApplicableRegulation = RegulationType;
pub type ApplicableTaxCategory = TaxCategoryType;
pub type ApplicableTerritoryAddress = AddressType;
pub type ApplicableTransportMeans = TransportMeansType;
pub type ApplicantParty = PartyType;
pub type AppliedSecurityMeasure = SecurityMeasureType;
pub type ApproverParty = PartyType;
pub type AtLocation = LocationType;
pub type AttachedTransportEquipment = TransportEquipmentType;
pub type Attachment = AttachmentType;
#[derive(Debug, Deserialize, Serialize)]
pub struct AttachmentType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "EmbeddedDocumentBinaryObject")]
    pub embedded_document_binary_object: ::core::option::Option<super::cct::BinaryObjectType>,
    #[serde(default, rename = "EmbeddedDocument")]
    pub embedded_document: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "FileName")]
    pub file_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ExternalReference")]
    pub external_reference: ::core::option::Option<ExternalReferenceType>,
}
pub type Attestation = AttestationType;
pub type AttestationLine = AttestationLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct AttestationLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CriterionItem")]
    pub criterion_item: ::std::vec::Vec<CriterionItemType>,
    #[serde(default, rename = "SubAttestationLine")]
    pub sub_attestation_line: ::std::vec::Vec<AttestationLineType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AttestationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "AcceptanceIndicator")]
    pub acceptance_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "AttestationLine")]
    pub attestation_line: ::std::vec::Vec<AttestationLineType>,
}
pub type AuctionTerms = AuctionTermsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct AuctionTermsType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "AuctionConstraintIndicator")]
    pub auction_constraint_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "JustificationDescription")]
    pub justification_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ProcessDescription")]
    pub process_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ConditionsDescription")]
    pub conditions_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ElectronicDeviceDescription")]
    pub electronic_device_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "AuctionURI")]
    pub auction_uri: ::core::option::Option<super::cct::IdentifierType>,
}
pub type AuthorityParty = PartyType;
pub type Authorization = AuthorizationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorizationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "PurposeCode")]
    pub purpose_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Purpose")]
    pub purpose: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "Certificate")]
    pub certificate: ::std::vec::Vec<CertificateType>,
}
pub type AvailabilityTransportEvent = TransportEventType;
pub type AwardedTenderedProject = TenderedProjectType;
pub type AwardingCriterion = AwardingCriterionType;
pub type AwardingCriterionResponse = AwardingCriterionResponseType;
#[derive(Debug, Deserialize, Serialize)]
pub struct AwardingCriterionResponseType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AwardingCriterionID")]
    pub awarding_criterion_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AwardingCriterionDescription")]
    pub awarding_criterion_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "Amount")]
    pub amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "SubordinateAwardingCriterionResponse")]
    pub subordinate_awarding_criterion_response: ::std::vec::Vec<AwardingCriterionResponseType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct AwardingCriterionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AwardingCriterionTypeCode")]
    pub awarding_criterion_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Name")]
    pub name: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "WeightNumeric")]
    pub weight_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "Weight")]
    pub weight: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CalculationExpression")]
    pub calculation_expression: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CalculationExpressionCode")]
    pub calculation_expression_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MinimumAmount")]
    pub minimum_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "MaximumAmount")]
    pub maximum_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "MinimumImprovementBid")]
    pub minimum_improvement_bid: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "SubordinateAwardingCriterion")]
    pub subordinate_awarding_criterion: ::std::vec::Vec<AwardingCriterionType>,
}
pub type AwardingTerms = AwardingTermsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct AwardingTermsType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "WeightingAlgorithmCode")]
    pub weighting_algorithm_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TechnicalCommitteeDescription")]
    pub technical_committee_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "LowTendersDescription")]
    pub low_tenders_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PrizeIndicator")]
    pub prize_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "PrizeDescription")]
    pub prize_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PaymentDescription")]
    pub payment_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "FollowupContractIndicator")]
    pub followup_contract_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "BindingOnBuyerIndicator")]
    pub binding_on_buyer_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "NoFurtherNegotiationIndicator")]
    pub no_further_negotiation_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "AwardingCriterion")]
    pub awarding_criterion: ::std::vec::Vec<AwardingCriterionType>,
    #[serde(default, rename = "TechnicalCommitteePerson")]
    pub technical_committee_person: ::std::vec::Vec<PersonType>,
    #[serde(default, rename = "Prize")]
    pub prize: ::std::vec::Vec<PrizeType>,
}
pub type BallastWaterSummary = BallastWaterSummaryType;
#[derive(Debug, Deserialize, Serialize)]
pub struct BallastWaterSummaryType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ManagementPlanOnBoardIndicator")]
    pub management_plan_on_board_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ManagementPlanImplementedIndicator")]
    pub management_plan_implemented_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "IMOGuidelinesOnBoardIndicator")]
    pub imo_guidelines_on_board_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "TotalBallastTanksOnBoardQuantity")]
    pub total_ballast_tanks_on_board_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "TanksInBallastQuantity")]
    pub tanks_in_ballast_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "TanksExchangedQuantity")]
    pub tanks_exchanged_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "TanksNotExchangedQuantity")]
    pub tanks_not_exchanged_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "TotalBallastWaterOnBoardMeasure")]
    pub total_ballast_water_on_board_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "TotalBallastWaterCapacityMeasure")]
    pub total_ballast_water_capacity_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "OtherControlActions")]
    pub other_control_actions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "NoControlActionsReason")]
    pub no_control_actions_reason: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "UptakeBallastWaterTransaction")]
    pub uptake_ballast_water_transaction: ::std::vec::Vec<BallastWaterTransactionType>,
    #[serde(default, rename = "ExchangeBallastWaterTransaction")]
    pub exchange_ballast_water_transaction: ::std::vec::Vec<BallastWaterTransactionType>,
    #[serde(default, rename = "DischargeBallastWaterTransaction")]
    pub discharge_ballast_water_transaction: ::std::vec::Vec<BallastWaterTransactionType>,
    #[serde(default, rename = "ResponsibleOfficerPerson")]
    pub responsible_officer_person: ::core::option::Option<PersonType>,
}
pub type BallastWaterTemperature = TemperatureType;
pub type BallastWaterTransaction = BallastWaterTransactionType;
#[derive(Debug, Deserialize, Serialize)]
pub struct BallastWaterTransactionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "TankID")]
    pub tank_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TankTypeCode")]
    pub tank_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ExchangeMethodCode")]
    pub exchange_method_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ExchangedPercent")]
    pub exchanged_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "VolumeMeasure")]
    pub volume_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "SeaHeightMeasure")]
    pub sea_height_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "SalinityMeasure")]
    pub salinity_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "TransactionDate")]
    pub transaction_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "Location")]
    pub location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "BallastWaterTemperature")]
    pub ballast_water_temperature: ::core::option::Option<TemperatureType>,
}
pub type BeneficiaryParty = PartyType;
pub type BillOfLadingHolderParty = PartyType;
pub type BillToParty = PartyType;
pub type BillingReference = BillingReferenceType;
pub type BillingReferenceLine = BillingReferenceLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct BillingReferenceLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Amount")]
    pub amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct BillingReferenceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "InvoiceDocumentReference")]
    pub invoice_document_reference: ::core::option::Option<DocumentReferenceType>,
    #[serde(default, rename = "SelfBilledInvoiceDocumentReference")]
    pub self_billed_invoice_document_reference: ::core::option::Option<DocumentReferenceType>,
    #[serde(default, rename = "CreditNoteDocumentReference")]
    pub credit_note_document_reference: ::core::option::Option<DocumentReferenceType>,
    #[serde(default, rename = "SelfBilledCreditNoteDocumentReference")]
    pub self_billed_credit_note_document_reference: ::core::option::Option<DocumentReferenceType>,
    #[serde(default, rename = "DebitNoteDocumentReference")]
    pub debit_note_document_reference: ::core::option::Option<DocumentReferenceType>,
    #[serde(default, rename = "ReminderDocumentReference")]
    pub reminder_document_reference: ::core::option::Option<DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::core::option::Option<DocumentReferenceType>,
    #[serde(default, rename = "BillingReferenceLine")]
    pub billing_reference_line: ::std::vec::Vec<BillingReferenceLineType>,
}
pub type BirthplaceLocation = LocationType;
pub type BondedWarehouseLocation = LocationType;
pub type BonusPaymentTerms = PaymentTermsType;
pub type Branch = BranchType;
#[derive(Debug, Deserialize, Serialize)]
pub struct BranchType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "FinancialInstitution")]
    pub financial_institution: ::core::option::Option<FinancialInstitutionType>,
    #[serde(default, rename = "Address")]
    pub address: ::core::option::Option<AddressType>,
}
pub type BrochureDocumentReference = DocumentReferenceType;
pub type BrokerParty = PartyType;
pub type BudgetAccount = BudgetAccountType;
pub type BudgetAccountLine = BudgetAccountLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct BudgetAccountLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TotalAmount")]
    pub total_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "BudgetAccount")]
    pub budget_account: ::std::vec::Vec<BudgetAccountType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct BudgetAccountType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "BudgetYearNumeric")]
    pub budget_year_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "RequiredClassificationScheme")]
    pub required_classification_scheme: ::core::option::Option<ClassificationSchemeType>,
}
pub type BusinessCapability = CapabilityType;
pub type BusinessClassificationScheme = ClassificationSchemeType;
pub type BusinessParty = PartyType;
pub type BusinessPartyGroup = PartyGroupType;
pub type BuyerAssignedReference = BuyerAssignedReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct BuyerAssignedReferenceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "BuyerReferenceCode")]
    pub buyer_reference_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "BuyerReference")]
    pub buyer_reference: ::std::vec::Vec<super::cct::TextType>,
}
pub type BuyerContact = ContactType;
pub type BuyerCustomerParty = CustomerPartyType;
pub type BuyerProposedSubstituteLineItem = LineItemType;
pub type BuyersItemIdentification = ItemIdentificationType;
pub type CallDuty = DutyType;
pub type CallForTenderDocumentReference = DocumentReferenceType;
pub type CallForTendersDocumentReference = DocumentReferenceType;
pub type CallForTendersLineReference = LineReferenceType;
pub type Capability = CapabilityType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CapabilityType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CapabilityTypeCode")]
    pub capability_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ValueAmount")]
    pub value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "ValueQuantity")]
    pub value_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "EvidenceSupplied")]
    pub evidence_supplied: ::std::vec::Vec<EvidenceSuppliedType>,
    #[serde(default, rename = "SuppliedEvidence")]
    pub supplied_evidence: ::std::vec::Vec<EvidenceType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "WebSite")]
    pub web_site: ::core::option::Option<WebSiteType>,
}
pub type CardAccount = CardAccountType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CardAccountType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "PrimaryAccountNumberID")]
    pub primary_account_number_id: super::cct::IdentifierType,
    #[serde(default, rename = "NetworkID")]
    pub network_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CardTypeCode")]
    pub card_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ValidityStartDate")]
    pub validity_start_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ExpiryDate")]
    pub expiry_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "IssuerID")]
    pub issuer_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssueNumberID")]
    pub issue_number_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CV2ID")]
    pub cv_2_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CardChipCode")]
    pub card_chip_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ChipApplicationID")]
    pub chip_application_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "HolderName")]
    pub holder_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "RoleCode")]
    pub role_code: ::core::option::Option<super::cct::CodeType>,
}
pub type CarrierParty = PartyType;
pub type CashRegister = CashRegisterType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CashRegisterType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "SerialNumberID")]
    pub serial_number_id: ::core::option::Option<super::cct::IdentifierType>,
}
pub type CashierContact = ContactType;
pub type CatalogueDocumentReference = DocumentReferenceType;
pub type CatalogueItemIdentification = ItemIdentificationType;
pub type CatalogueItemSpecificationUpdateLine = CatalogueItemSpecificationUpdateLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogueItemSpecificationUpdateLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: ::core::option::Option<CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<SupplierPartyType>,
    #[serde(rename = "Item")]
    pub item: ItemType,
}
pub type CatalogueLine = CatalogueLineType;
pub type CatalogueLineReference = LineReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogueLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "ActionCode")]
    pub action_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "LifeCycleStatusCode")]
    pub life_cycle_status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ContractSubdivision")]
    pub contract_subdivision: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "OrderableIndicator")]
    pub orderable_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "OrderableUnit")]
    pub orderable_unit: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ContentUnitQuantity")]
    pub content_unit_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "OrderQuantityIncrementNumeric")]
    pub order_quantity_increment_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "MinimumOrderQuantity")]
    pub minimum_order_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumOrderQuantity")]
    pub maximum_order_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "WarrantyInformation")]
    pub warranty_information: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PackLevelCode")]
    pub pack_level_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: ::core::option::Option<CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<SupplierPartyType>,
    #[serde(default, rename = "WarrantyParty")]
    pub warranty_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "LineValidityPeriod")]
    pub line_validity_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "ItemComparison")]
    pub item_comparison: ::std::vec::Vec<ItemComparisonType>,
    #[serde(default, rename = "ComponentRelatedItem")]
    pub component_related_item: ::std::vec::Vec<RelatedItemType>,
    #[serde(default, rename = "AccessoryRelatedItem")]
    pub accessory_related_item: ::std::vec::Vec<RelatedItemType>,
    #[serde(default, rename = "RequiredRelatedItem")]
    pub required_related_item: ::std::vec::Vec<RelatedItemType>,
    #[serde(default, rename = "ReplacementRelatedItem")]
    pub replacement_related_item: ::std::vec::Vec<RelatedItemType>,
    #[serde(default, rename = "ComplementaryRelatedItem")]
    pub complementary_related_item: ::std::vec::Vec<RelatedItemType>,
    #[serde(default, rename = "ReplacedRelatedItem")]
    pub replaced_related_item: ::std::vec::Vec<RelatedItemType>,
    #[serde(default, rename = "RequiredItemLocationQuantity")]
    pub required_item_location_quantity: ::std::vec::Vec<ItemLocationQuantityType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(rename = "Item")]
    pub item: ItemType,
    #[serde(default, rename = "KeywordItemProperty")]
    pub keyword_item_property: ::std::vec::Vec<ItemPropertyType>,
    #[serde(default, rename = "CallForTendersLineReference")]
    pub call_for_tenders_line_reference: ::core::option::Option<LineReferenceType>,
    #[serde(default, rename = "CallForTendersDocumentReference")]
    pub call_for_tenders_document_reference: ::std::vec::Vec<DocumentReferenceType>,
}
pub type CataloguePricingUpdateLine = CataloguePricingUpdateLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CataloguePricingUpdateLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "ContractorCustomerParty")]
    pub contractor_customer_party: ::core::option::Option<CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<SupplierPartyType>,
    #[serde(default, rename = "RequiredItemLocationQuantity")]
    pub required_item_location_quantity: ::std::vec::Vec<ItemLocationQuantityType>,
}
pub type CatalogueReference = CatalogueReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogueReferenceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "RevisionDate")]
    pub revision_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "RevisionTime")]
    pub revision_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PreviousVersionID")]
    pub previous_version_id: ::core::option::Option<super::cct::IdentifierType>,
}
pub type CatalogueRequestLine = CatalogueRequestLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CatalogueRequestLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "ContractSubdivision")]
    pub contract_subdivision: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "LineValidityPeriod")]
    pub line_validity_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "RequiredItemLocationQuantity")]
    pub required_item_location_quantity: ::std::vec::Vec<ItemLocationQuantityType>,
    #[serde(rename = "Item")]
    pub item: ItemType,
}
pub type CategorizesClassificationCategory = ClassificationCategoryType;
pub type Certificate = CertificateType;
pub type CertificateOfOriginApplication = CertificateOfOriginApplicationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CertificateOfOriginApplicationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ReferenceID")]
    pub reference_id: super::cct::IdentifierType,
    #[serde(rename = "CertificateType")]
    pub certificate_type: super::cct::TextType,
    #[serde(default, rename = "ApplicationStatusCode")]
    pub application_status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(rename = "OriginalJobID")]
    pub original_job_id: super::cct::IdentifierType,
    #[serde(default, rename = "PreviousJobID")]
    pub previous_job_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Remarks")]
    pub remarks: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "Shipment")]
    pub shipment: ShipmentType,
    #[serde(default, rename = "EndorserParty")]
    pub endorser_party: ::std::vec::Vec<EndorserPartyType>,
    #[serde(rename = "PreparationParty")]
    pub preparation_party: PartyType,
    #[serde(rename = "IssuerParty")]
    pub issuer_party: PartyType,
    #[serde(default, rename = "ExporterParty")]
    pub exporter_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "ImporterParty")]
    pub importer_party: ::core::option::Option<PartyType>,
    #[serde(rename = "IssuingCountry")]
    pub issuing_country: CountryType,
    #[serde(default, rename = "DocumentDistribution")]
    pub document_distribution: ::std::vec::Vec<DocumentDistributionType>,
    #[serde(default, rename = "SupportingDocumentReference")]
    pub supporting_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<SignatureType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct CertificateType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "CertificateTypeCode")]
    pub certificate_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CertificateType")]
    pub certificate_type: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CertificateReferenceID")]
    pub certificate_reference_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ApplicableCategoryCode")]
    pub applicable_category_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ApplicableCategory")]
    pub applicable_category: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "CertificateURI")]
    pub certificate_uri: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Remarks")]
    pub remarks: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "CertificateValidityPeriod")]
    pub certificate_validity_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<SignatureType>,
}
pub type CertificateValidityPeriod = PeriodType;
pub type CertificationDocumentReference = DocumentReferenceType;
pub type ChildConsignment = ConsignmentType;
pub type CircularityProfile = CircularityProfileType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CircularityProfileType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "CircularityTypeCode")]
    pub circularity_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "RecycledContentPercent")]
    pub recycled_content_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "RecyclabilityPercent")]
    pub recyclability_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "MaintenanceFrequencyCode")]
    pub maintenance_frequency_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "MaintenanceFrequencyDescription")]
    pub maintenance_frequency_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ResourceConsumption")]
    pub resource_consumption: ::std::vec::Vec<ResourceConsumptionType>,
    #[serde(default, rename = "WasteGenerated")]
    pub waste_generated: ::std::vec::Vec<WasteGeneratedType>,
    #[serde(default, rename = "RepairabilityScore")]
    pub repairability_score: ::std::vec::Vec<ScoreType>,
    #[serde(default, rename = "EndOfLifeTreatment")]
    pub end_of_life_treatment: ::core::option::Option<EndOfLifeTreatmentType>,
    #[serde(default, rename = "ProductDocumentationDocumentReference")]
    pub product_documentation_document_reference: ::std::vec::Vec<DocumentReferenceType>,
}
pub type CitizenshipCountry = CountryType;
pub type ClassificationCategory = ClassificationCategoryType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ClassificationCategoryType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "CodeValue")]
    pub code_value: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CategorizesClassificationCategory")]
    pub categorizes_classification_category: ::std::vec::Vec<ClassificationCategoryType>,
}
pub type ClassificationScheme = ClassificationSchemeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ClassificationSchemeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "LastRevisionDate")]
    pub last_revision_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "LastRevisionTime")]
    pub last_revision_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "AgencyID")]
    pub agency_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AgencyName")]
    pub agency_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "URI")]
    pub uri: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SchemeURI")]
    pub scheme_uri: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "LanguageID")]
    pub language_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ClassificationCategory")]
    pub classification_category: ::std::vec::Vec<ClassificationCategoryType>,
}
pub type ClassifiedTaxCategory = TaxCategoryType;
pub type Clause = ClauseType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ClauseType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Content")]
    pub content: ::std::vec::Vec<super::cct::TextType>,
}
pub type CollectPaymentTerms = PaymentTermsType;
pub type CollectedForParty = PartyType;
pub type CollectedPayment = PaymentType;
pub type CollectionCreditNoteLine = CreditNoteLineType;
pub type CollectionDebitNoteLine = DebitNoteLineType;
pub type CollectionInvoiceLine = InvoiceLineType;
pub type CommercialContact = ContactType;
pub type CommissionPaymentTerms = PaymentTermsType;
pub type CommodityClassification = CommodityClassificationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CommodityClassificationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "NatureCode")]
    pub nature_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CargoTypeCode")]
    pub cargo_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CommodityCode")]
    pub commodity_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ItemClassificationCode")]
    pub item_classification_code: ::core::option::Option<super::cct::CodeType>,
}
pub type Communication = CommunicationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CommunicationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ChannelCode")]
    pub channel_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Channel")]
    pub channel: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Value")]
    pub value: ::core::option::Option<super::cct::TextType>,
}
pub type ComplementaryRelatedItem = RelatedItemType;
pub type CompletedTask = CompletedTaskType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CompletedTaskType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "AnnualAverageAmount")]
    pub annual_average_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TotalTaskAmount")]
    pub total_task_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "PartyCapacityAmount")]
    pub party_capacity_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "EvidenceSupplied")]
    pub evidence_supplied: ::std::vec::Vec<EvidenceSuppliedType>,
    #[serde(default, rename = "SuppliedEvidence")]
    pub supplied_evidence: ::std::vec::Vec<EvidenceType>,
    #[serde(default, rename = "Period")]
    pub period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "RecipientCustomerParty")]
    pub recipient_customer_party: ::core::option::Option<CustomerPartyType>,
}
pub type ComponentRelatedItem = RelatedItemType;
pub type Condition = ConditionType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ConditionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "AttributeID")]
    pub attribute_id: super::cct::IdentifierType,
    #[serde(default, rename = "Measure")]
    pub measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "MinimumMeasure")]
    pub minimum_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "MaximumMeasure")]
    pub maximum_measure: ::core::option::Option<super::cct::MeasureType>,
}
pub type ConsigneeParty = PartyType;
pub type Consignment = ConsignmentType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ConsignmentType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "CarrierAssignedID")]
    pub carrier_assigned_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ConsigneeAssignedID")]
    pub consignee_assigned_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ConsignorAssignedID")]
    pub consignor_assigned_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "FreightForwarderAssignedID")]
    pub freight_forwarder_assigned_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "BrokerAssignedID")]
    pub broker_assigned_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ContractedCarrierAssignedID")]
    pub contracted_carrier_assigned_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PerformingCarrierAssignedID")]
    pub performing_carrier_assigned_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SummaryDescription")]
    pub summary_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TotalInvoiceAmount")]
    pub total_invoice_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "DeclaredCustomsValueAmount")]
    pub declared_customs_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TariffDescription")]
    pub tariff_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TariffCode")]
    pub tariff_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "InsurancePremiumAmount")]
    pub insurance_premium_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetWeightMeasure")]
    pub net_weight_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetNetWeightMeasure")]
    pub net_net_weight_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "ChargeableWeightMeasure")]
    pub chargeable_weight_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "GrossVolumeMeasure")]
    pub gross_volume_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetVolumeMeasure")]
    pub net_volume_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "LoadingLengthMeasure")]
    pub loading_length_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "Remarks")]
    pub remarks: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "AnimalFoodIndicator")]
    pub animal_food_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "HumanFoodIndicator")]
    pub human_food_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "LivestockIndicator")]
    pub livestock_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "BulkCargoIndicator")]
    pub bulk_cargo_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ContainerizedIndicator")]
    pub containerized_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "GeneralCargoIndicator")]
    pub general_cargo_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "SpecialSecurityIndicator")]
    pub special_security_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ThirdPartyPayerIndicator")]
    pub third_party_payer_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "CarrierServiceInstructions")]
    pub carrier_service_instructions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CustomsClearanceServiceInstructions")]
    pub customs_clearance_service_instructions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ForwarderServiceInstructions")]
    pub forwarder_service_instructions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "SpecialServiceInstructions")]
    pub special_service_instructions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "SequenceID")]
    pub sequence_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ShippingPriorityLevelCode")]
    pub shipping_priority_level_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "HandlingCode")]
    pub handling_code: ::std::vec::Vec<super::cct::CodeType>,
    #[serde(default, rename = "HandlingInstructions")]
    pub handling_instructions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Information")]
    pub information: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TotalGoodsItemQuantity")]
    pub total_goods_item_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "TotalTransportHandlingUnitQuantity")]
    pub total_transport_handling_unit_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "InsuranceValueAmount")]
    pub insurance_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "DeclaredForCarriageValueAmount")]
    pub declared_for_carriage_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "DeclaredStatisticsValueAmount")]
    pub declared_statistics_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "FreeOnBoardValueAmount")]
    pub free_on_board_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "SpecialInstructions")]
    pub special_instructions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "SplitConsignmentIndicator")]
    pub split_consignment_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "DeliveryInstructions")]
    pub delivery_instructions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ConsignmentQuantity")]
    pub consignment_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "ConsolidatableIndicator")]
    pub consolidatable_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "HaulageInstructions")]
    pub haulage_instructions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "LoadingSequenceID")]
    pub loading_sequence_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ChildConsignmentQuantity")]
    pub child_consignment_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "TotalPackagesQuantity")]
    pub total_packages_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "ConsolidatedShipment")]
    pub consolidated_shipment: ::std::vec::Vec<ShipmentType>,
    #[serde(default, rename = "CustomsDeclaration")]
    pub customs_declaration: ::std::vec::Vec<CustomsDeclarationType>,
    #[serde(default, rename = "RequestedPickupTransportEvent")]
    pub requested_pickup_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "RequestedDeliveryTransportEvent")]
    pub requested_delivery_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "PlannedPickupTransportEvent")]
    pub planned_pickup_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "PlannedDeliveryTransportEvent")]
    pub planned_delivery_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "ActualPickupTransportEvent")]
    pub actual_pickup_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "ActualDeliveryTransportEvent")]
    pub actual_delivery_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "Status")]
    pub status: ::std::vec::Vec<StatusType>,
    #[serde(default, rename = "ChildConsignment")]
    pub child_consignment: ::std::vec::Vec<ConsignmentType>,
    #[serde(default, rename = "ConsigneeParty")]
    pub consignee_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "ExporterParty")]
    pub exporter_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "ImporterParty")]
    pub importer_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "NotifyParty")]
    pub notify_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "OriginalDespatchParty")]
    pub original_despatch_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "FinalDeliveryParty")]
    pub final_delivery_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "PerformingCarrierParty")]
    pub performing_carrier_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "SubstituteCarrierParty")]
    pub substitute_carrier_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "LogisticsOperatorParty")]
    pub logistics_operator_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "TransportAdvisorParty")]
    pub transport_advisor_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "HazardousItemNotificationParty")]
    pub hazardous_item_notification_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "InsuranceParty")]
    pub insurance_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "MortgageHolderParty")]
    pub mortgage_holder_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "BillOfLadingHolderParty")]
    pub bill_of_lading_holder_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "OriginalDepartureCountry")]
    pub original_departure_country: ::core::option::Option<CountryType>,
    #[serde(default, rename = "FinalDestinationCountry")]
    pub final_destination_country: ::core::option::Option<CountryType>,
    #[serde(default, rename = "TransitCountry")]
    pub transit_country: ::std::vec::Vec<CountryType>,
    #[serde(default, rename = "TransportContract")]
    pub transport_contract: ::core::option::Option<ContractType>,
    #[serde(default, rename = "TransportEvent")]
    pub transport_event: ::std::vec::Vec<TransportEventType>,
    #[serde(default, rename = "OriginalDespatchTransportationService")]
    pub original_despatch_transportation_service: ::core::option::Option<TransportationServiceType>,
    #[serde(default, rename = "FinalDeliveryTransportationService")]
    pub final_delivery_transportation_service: ::core::option::Option<TransportationServiceType>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: ::core::option::Option<DeliveryTermsType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::core::option::Option<PaymentTermsType>,
    #[serde(default, rename = "CollectPaymentTerms")]
    pub collect_payment_terms: ::core::option::Option<PaymentTermsType>,
    #[serde(default, rename = "DisbursementPaymentTerms")]
    pub disbursement_payment_terms: ::core::option::Option<PaymentTermsType>,
    #[serde(default, rename = "PrepaidPaymentTerms")]
    pub prepaid_payment_terms: ::core::option::Option<PaymentTermsType>,
    #[serde(default, rename = "FreightAllowanceCharge")]
    pub freight_allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "ExtraAllowanceCharge")]
    pub extra_allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "MainCarriageShipmentStage")]
    pub main_carriage_shipment_stage: ::std::vec::Vec<ShipmentStageType>,
    #[serde(default, rename = "PreCarriageShipmentStage")]
    pub pre_carriage_shipment_stage: ::std::vec::Vec<ShipmentStageType>,
    #[serde(default, rename = "OnCarriageShipmentStage")]
    pub on_carriage_shipment_stage: ::std::vec::Vec<ShipmentStageType>,
    #[serde(default, rename = "TransportHandlingUnit")]
    pub transport_handling_unit: ::std::vec::Vec<TransportHandlingUnitType>,
    #[serde(default, rename = "FirstArrivalPortLocation")]
    pub first_arrival_port_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "LastExitPortLocation")]
    pub last_exit_port_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "OfficeOfEntryLocation")]
    pub office_of_entry_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "OfficeOfSubSequentiallyEntryLocation")]
    pub office_of_sub_sequentially_entry_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "OfficeOfExitLocation")]
    pub office_of_exit_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "OfficeOfDepartureLocation")]
    pub office_of_departure_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "OfficeOfDestinationLocation")]
    pub office_of_destination_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "OfficeOfImportLocation")]
    pub office_of_import_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "OfficeOfExportLocation")]
    pub office_of_export_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "OfficeOfTransitLocation")]
    pub office_of_transit_location: ::std::vec::Vec<LocationType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: ::std::vec::Vec<EnvironmentalEmissionType>,
    #[serde(default, rename = "InsurancePolicy")]
    pub insurance_policy: ::std::vec::Vec<InsurancePolicyType>,
}
pub type ConsignorParty = PartyType;
pub type ConsolidatedShipment = ShipmentType;
pub type ConstitutionPeriod = PeriodType;
pub type Consumption = ConsumptionType;
pub type ConsumptionAverage = ConsumptionAverageType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionAverageType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "AverageAmount")]
    pub average_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
}
pub type ConsumptionCorrection = ConsumptionCorrectionType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionCorrectionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "CorrectionType")]
    pub correction_type: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "CorrectionTypeCode")]
    pub correction_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "MeterNumber")]
    pub meter_number: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "GasPressureQuantity")]
    pub gas_pressure_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "ActualTemperatureReductionQuantity")]
    pub actual_temperature_reduction_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "NormalTemperatureReductionQuantity")]
    pub normal_temperature_reduction_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "DifferenceTemperatureReductionQuantity")]
    pub difference_temperature_reduction_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CorrectionUnitAmount")]
    pub correction_unit_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "ConsumptionEnergyQuantity")]
    pub consumption_energy_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "ConsumptionWaterQuantity")]
    pub consumption_water_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "CorrectionAmount")]
    pub correction_amount: ::core::option::Option<super::cct::AmountType>,
}
pub type ConsumptionHistory = ConsumptionHistoryType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionHistoryType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "MeterNumber")]
    pub meter_number: ::core::option::Option<super::cct::TextType>,
    #[serde(rename = "Quantity")]
    pub quantity: super::cct::QuantityType,
    #[serde(default, rename = "Amount")]
    pub amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "ConsumptionLevelCode")]
    pub consumption_level_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ConsumptionLevel")]
    pub consumption_level: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "Period")]
    pub period: PeriodType,
}
pub type ConsumptionLine = ConsumptionLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "ParentDocumentLineReferenceID")]
    pub parent_document_line_reference_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(rename = "InvoicedQuantity")]
    pub invoiced_quantity: super::cct::QuantityType,
    #[serde(rename = "LineExtensionAmount")]
    pub line_extension_amount: super::cct::AmountType,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "Period")]
    pub period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::std::vec::Vec<DeliveryType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<TaxTotalType>,
    #[serde(rename = "UtilityItem")]
    pub utility_item: UtilityItemType,
    #[serde(default, rename = "Price")]
    pub price: ::core::option::Option<PriceType>,
    #[serde(default, rename = "UnstructuredPrice")]
    pub unstructured_price: ::core::option::Option<UnstructuredPriceType>,
}
pub type ConsumptionPoint = ConsumptionPointType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionPointType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "SubscriberID")]
    pub subscriber_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SubscriberType")]
    pub subscriber_type: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "SubscriberTypeCode")]
    pub subscriber_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TotalDeliveredQuantity")]
    pub total_delivered_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "Address")]
    pub address: ::core::option::Option<AddressType>,
    #[serde(default, rename = "WebSiteAccess")]
    pub web_site_access: ::core::option::Option<WebSiteAccessType>,
    #[serde(default, rename = "UtilityMeter")]
    pub utility_meter: ::std::vec::Vec<MeterType>,
}
pub type ConsumptionReport = ConsumptionReportType;
pub type ConsumptionReportReference = ConsumptionReportReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionReportReferenceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ConsumptionReportID")]
    pub consumption_report_id: super::cct::IdentifierType,
    #[serde(default, rename = "ConsumptionType")]
    pub consumption_type: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ConsumptionTypeCode")]
    pub consumption_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(rename = "TotalConsumedQuantity")]
    pub total_consumed_quantity: super::cct::QuantityType,
    #[serde(rename = "Period")]
    pub period: PeriodType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionReportType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "ConsumptionType")]
    pub consumption_type: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ConsumptionTypeCode")]
    pub consumption_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TotalConsumedQuantity")]
    pub total_consumed_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "BasicConsumedQuantity")]
    pub basic_consumed_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "ResidentOccupantsNumeric")]
    pub resident_occupants_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "ConsumersEnergyLevelCode")]
    pub consumers_energy_level_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ConsumersEnergyLevel")]
    pub consumers_energy_level: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ResidenceType")]
    pub residence_type: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ResidenceTypeCode")]
    pub residence_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "HeatingType")]
    pub heating_type: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "HeatingTypeCode")]
    pub heating_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Period")]
    pub period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "GuidanceDocumentReference")]
    pub guidance_document_reference: ::core::option::Option<DocumentReferenceType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::core::option::Option<DocumentReferenceType>,
    #[serde(default, rename = "ConsumptionReportReference")]
    pub consumption_report_reference: ::std::vec::Vec<ConsumptionReportReferenceType>,
    #[serde(default, rename = "ConsumptionHistory")]
    pub consumption_history: ::std::vec::Vec<ConsumptionHistoryType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ConsumptionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "UtilityStatementTypeCode")]
    pub utility_statement_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "MainPeriod")]
    pub main_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<TaxTotalType>,
    #[serde(default, rename = "EnergyWaterSupply")]
    pub energy_water_supply: ::core::option::Option<EnergyWaterSupplyType>,
    #[serde(default, rename = "TelecommunicationsSupply")]
    pub telecommunications_supply: ::core::option::Option<TelecommunicationsSupplyType>,
    #[serde(rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: MonetaryTotalType,
}
pub type Contact = ContactType;
pub type ContactParty = PartyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ContactType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "JobTitle")]
    pub job_title: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Department")]
    pub department: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Telephone")]
    pub telephone: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Telefax")]
    pub telefax: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ElectronicMail")]
    pub electronic_mail: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "OtherCommunication")]
    pub other_communication: ::std::vec::Vec<CommunicationType>,
}
pub type ContainedGoodsItem = GoodsItemType;
pub type ContainedInTransportEquipment = TransportEquipmentType;
pub type ContainedPackage = PackageType;
pub type ContainingPackage = PackageType;
pub type ContainingTransportEquipment = TransportEquipmentType;
pub type Contract = ContractType;
pub type ContractAcceptancePeriod = PeriodType;
pub type ContractDocumentReference = DocumentReferenceType;
pub type ContractExecutionRequirement = ContractExecutionRequirementType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ContractExecutionRequirementType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Name")]
    pub name: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ExecutionRequirementCode")]
    pub execution_requirement_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
}
pub type ContractExtension = ContractExtensionType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ContractExtensionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "OptionsDescription")]
    pub options_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "MinimumNumberNumeric")]
    pub minimum_number_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "MaximumNumberNumeric")]
    pub maximum_number_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "RenewalsIndicator")]
    pub renewals_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "OptionValidityPeriod")]
    pub option_validity_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "Renewal")]
    pub renewal: ::std::vec::Vec<RenewalType>,
}
pub type ContractFormalizationPeriod = PeriodType;
pub type ContractResponsibleParty = PartyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ContractType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "NominationDate")]
    pub nomination_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "NominationTime")]
    pub nomination_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ContractTypeCode")]
    pub contract_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ContractType")]
    pub contract_type: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ModificationReasonCode")]
    pub modification_reason_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ModificationReasonDescription")]
    pub modification_reason_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "NominationPeriod")]
    pub nomination_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "ContractualDelivery")]
    pub contractual_delivery: ::core::option::Option<DeliveryType>,
}
pub type ContractingActivity = ContractingActivityType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ContractingActivityType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ActivityTypeCode")]
    pub activity_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ActivityType")]
    pub activity_type: ::std::vec::Vec<super::cct::TextType>,
}
pub type ContractingParty = ContractingPartyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ContractingPartyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "BuyerProfileURI")]
    pub buyer_profile_uri: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ContractingPartyType")]
    pub contracting_party_type: ::std::vec::Vec<ContractingPartyTypeType>,
    #[serde(default, rename = "ContractingActivity")]
    pub contracting_activity: ::std::vec::Vec<ContractingActivityType>,
    #[serde(default, rename = "ContractingRepresentationType")]
    pub contracting_representation_type: ::core::option::Option<ContractingRepresentationTypeType>,
    #[serde(rename = "Party")]
    pub party: PartyType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ContractingPartyTypeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "PartyTypeCode")]
    pub party_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PartyType")]
    pub party_type: ::std::vec::Vec<super::cct::TextType>,
}
pub type ContractingRepresentationType = ContractingRepresentationTypeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ContractingRepresentationTypeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "RepresentationTypeCode")]
    pub representation_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "RepresentationType")]
    pub representation_type: ::std::vec::Vec<super::cct::TextType>,
}
pub type ContractingSystem = ContractingSystemType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ContractingSystemType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ContractingSystemTypeCode")]
    pub contracting_system_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
}
pub type ContractorCustomerParty = CustomerPartyType;
pub type ContractualDelivery = DeliveryType;
pub type ContractualDocumentReference = DocumentReferenceType;
pub type CorporateRegistrationScheme = CorporateRegistrationSchemeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CorporateRegistrationSchemeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "CorporateRegistrationTypeCode")]
    pub corporate_registration_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "JurisdictionRegionAddress")]
    pub jurisdiction_region_address: ::std::vec::Vec<AddressType>,
}
pub type Country = CountryType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CountryType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "IdentificationCode")]
    pub identification_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
}
pub type CreditAccount = CreditAccountType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CreditAccountType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "AccountID")]
    pub account_id: super::cct::IdentifierType,
}
pub type CreditNoteDocumentReference = DocumentReferenceType;
pub type CreditNoteLine = CreditNoteLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CreditNoteLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CreditedQuantity")]
    pub credited_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "PaymentPurposeCode")]
    pub payment_purpose_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "FreeOfChargeIndicator")]
    pub free_of_charge_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: ::std::vec::Vec<PeriodType>,
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: ::std::vec::Vec<OrderLineReferenceType>,
    #[serde(default, rename = "DiscrepancyResponse")]
    pub discrepancy_response: ::std::vec::Vec<ResponseType>,
    #[serde(default, rename = "DespatchLineReference")]
    pub despatch_line_reference: ::std::vec::Vec<LineReferenceType>,
    #[serde(default, rename = "ReceiptLineReference")]
    pub receipt_line_reference: ::std::vec::Vec<LineReferenceType>,
    #[serde(default, rename = "WorkReportLineReference")]
    pub work_report_line_reference: ::std::vec::Vec<LineReferenceType>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: ::std::vec::Vec<BillingReferenceType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "PricingReference")]
    pub pricing_reference: ::core::option::Option<PricingReferenceType>,
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: ::core::option::Option<PurchaseReferenceType>,
    #[serde(default, rename = "OriginatorParty")]
    pub originator_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<PartyType>,
    #[serde(default, rename = "CollectedForParty")]
    pub collected_for_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::std::vec::Vec<DeliveryType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<PaymentTermsType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<TaxTotalType>,
    #[serde(default, rename = "WithholdingTaxTotal")]
    pub withholding_tax_total: ::std::vec::Vec<TaxTotalType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "Item")]
    pub item: ::core::option::Option<ItemType>,
    #[serde(default, rename = "Price")]
    pub price: ::core::option::Option<PriceType>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: ::std::vec::Vec<DeliveryTermsType>,
    #[serde(default, rename = "SubCreditNoteLine")]
    pub sub_credit_note_line: ::std::vec::Vec<CreditNoteLineType>,
    #[serde(default, rename = "ItemPriceExtension")]
    pub item_price_extension: ::core::option::Option<PriceExtensionType>,
}
pub type CrewMemberPerson = PersonType;
pub type CrewPerson = PersonType;
pub type CrewPersonEffect = CrewPersonEffectType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CrewPersonEffectType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "EffectDescription")]
    pub effect_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CrewPerson")]
    pub crew_person: ::core::option::Option<PersonType>,
}
pub type CriterionItem = CriterionItemType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CriterionItemType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CriterionDescription")]
    pub criterion_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "DeclaredPropertyItem")]
    pub declared_property_item: ItemType,
}
pub type CurrentStatus = StatusType;
pub type CustomerParty = CustomerPartyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CustomerPartyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "CustomerAssignedAccountID")]
    pub customer_assigned_account_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SupplierAssignedAccountID")]
    pub supplier_assigned_account_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AdditionalAccountID")]
    pub additional_account_id: ::std::vec::Vec<super::cct::IdentifierType>,
    #[serde(default, rename = "Party")]
    pub party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "DeliveryContact")]
    pub delivery_contact: ::core::option::Option<ContactType>,
    #[serde(default, rename = "AccountingContact")]
    pub accounting_contact: ::core::option::Option<ContactType>,
    #[serde(default, rename = "BuyerContact")]
    pub buyer_contact: ::core::option::Option<ContactType>,
}
pub type CustomsAgentParty = PartyType;
pub type CustomsDeclaration = CustomsDeclarationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct CustomsDeclarationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "FunctionCode")]
    pub function_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "ApplicableTerritoryAddress")]
    pub applicable_territory_address: ::core::option::Option<AddressType>,
    #[serde(default, rename = "Shipment")]
    pub shipment: ::core::option::Option<ShipmentType>,
    #[serde(default, rename = "CustomsExitOfficeLocation")]
    pub customs_exit_office_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "ConsignorParty")]
    pub consignor_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "ConsigneeParty")]
    pub consignee_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "FreightForwarderParty")]
    pub freight_forwarder_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "CustomsParty")]
    pub customs_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "PreviousCustomsDeclaration")]
    pub previous_customs_declaration:
        ::core::option::Option<::std::boxed::Box<CustomsDeclarationType>>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<DocumentReferenceType>,
}
pub type CustomsExitOfficeLocation = LocationType;
pub type CustomsOfficeLocation = LocationType;
pub type CustomsParty = PartyType;
pub type DamageDocumentationAttachment = AttachmentType;
pub type DebitNoteDocumentReference = DocumentReferenceType;
pub type DebitNoteLine = DebitNoteLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DebitNoteLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "DebitedQuantity")]
    pub debited_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(rename = "LineExtensionAmount")]
    pub line_extension_amount: super::cct::AmountType,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "PaymentPurposeCode")]
    pub payment_purpose_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "FreeOfChargeIndicator")]
    pub free_of_charge_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: ::std::vec::Vec<PeriodType>,
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: ::std::vec::Vec<OrderLineReferenceType>,
    #[serde(default, rename = "DiscrepancyResponse")]
    pub discrepancy_response: ::std::vec::Vec<ResponseType>,
    #[serde(default, rename = "DespatchLineReference")]
    pub despatch_line_reference: ::std::vec::Vec<LineReferenceType>,
    #[serde(default, rename = "ReceiptLineReference")]
    pub receipt_line_reference: ::std::vec::Vec<LineReferenceType>,
    #[serde(default, rename = "WorkReportLineReference")]
    pub work_report_line_reference: ::std::vec::Vec<LineReferenceType>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: ::std::vec::Vec<BillingReferenceType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "PricingReference")]
    pub pricing_reference: ::core::option::Option<PricingReferenceType>,
    #[serde(default, rename = "OriginatorParty")]
    pub originator_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<PartyType>,
    #[serde(default, rename = "CollectedForParty")]
    pub collected_for_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::std::vec::Vec<DeliveryType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<PaymentTermsType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<TaxTotalType>,
    #[serde(default, rename = "WithholdingTaxTotal")]
    pub withholding_tax_total: ::std::vec::Vec<TaxTotalType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "Item")]
    pub item: ::core::option::Option<ItemType>,
    #[serde(default, rename = "Price")]
    pub price: ::core::option::Option<PriceType>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: ::std::vec::Vec<DeliveryTermsType>,
    #[serde(default, rename = "SubDebitNoteLine")]
    pub sub_debit_note_line: ::std::vec::Vec<DebitNoteLineType>,
    #[serde(default, rename = "ItemPriceExtension")]
    pub item_price_extension: ::core::option::Option<PriceExtensionType>,
}
pub type Declaration = DeclarationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DeclarationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "DeclarationTypeCode")]
    pub declaration_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "EvidenceSupplied")]
    pub evidence_supplied: ::std::vec::Vec<EvidenceSuppliedType>,
    #[serde(default, rename = "SuppliedEvidence")]
    pub supplied_evidence: ::std::vec::Vec<EvidenceType>,
}
pub type DeclaredPropertyItem = ItemType;
pub type DefaultLanguage = LanguageType;
pub type DeletedCatalogueReference = CatalogueReferenceType;
pub type Delivery = DeliveryType;
pub type DeliveryAddress = AddressType;
pub type DeliveryChannel = DeliveryChannelType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DeliveryChannelType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "NetworkID")]
    pub network_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ParticipantID")]
    pub participant_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TestIndicator")]
    pub test_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "DigitalCertificate")]
    pub digital_certificate: ::core::option::Option<CertificateType>,
    #[serde(default, rename = "DigitalMessageDelivery")]
    pub digital_message_delivery: ::core::option::Option<MessageDeliveryType>,
}
pub type DeliveryContact = ContactType;
pub type DeliveryCustomerParty = CustomerPartyType;
pub type DeliveryLocation = LocationType;
pub type DeliveryNoteDocumentReference = DocumentReferenceType;
pub type DeliveryNoteLineReference = LineReferenceType;
pub type DeliveryParty = PartyType;
pub type DeliveryPeriod = PeriodType;
pub type DeliveryTerms = DeliveryTermsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DeliveryTermsType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SpecialTerms")]
    pub special_terms: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "LossRiskResponsibilityCode")]
    pub loss_risk_responsibility_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "LossRisk")]
    pub loss_risk: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Amount")]
    pub amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "DeliveryLocation")]
    pub delivery_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::core::option::Option<AllowanceChargeType>,
}
pub type DeliveryTransportEvent = TransportEventType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DeliveryType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "ActualDeliveryDate")]
    pub actual_delivery_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ActualDeliveryTime")]
    pub actual_delivery_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "LatestDeliveryDate")]
    pub latest_delivery_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "LatestDeliveryTime")]
    pub latest_delivery_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ReleaseID")]
    pub release_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TrackingID")]
    pub tracking_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "DeliveryAddress")]
    pub delivery_address: ::core::option::Option<AddressType>,
    #[serde(default, rename = "DeliveryLocation")]
    pub delivery_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "AlternativeDeliveryLocation")]
    pub alternative_delivery_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "RequestedDeliveryPeriod")]
    pub requested_delivery_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "PromisedDeliveryPeriod")]
    pub promised_delivery_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "EstimatedDeliveryPeriod")]
    pub estimated_delivery_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "DeliveryParty")]
    pub delivery_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "NotifyParty")]
    pub notify_party: ::std::vec::Vec<PartyType>,
    #[serde(default, rename = "Despatch")]
    pub despatch: ::core::option::Option<DespatchType>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: ::std::vec::Vec<DeliveryTermsType>,
    #[serde(default, rename = "MinimumDeliveryUnit")]
    pub minimum_delivery_unit: ::core::option::Option<DeliveryUnitType>,
    #[serde(default, rename = "MaximumDeliveryUnit")]
    pub maximum_delivery_unit: ::core::option::Option<DeliveryUnitType>,
    #[serde(default, rename = "Shipment")]
    pub shipment: ::core::option::Option<ShipmentType>,
    #[serde(default, rename = "FuelConsumption")]
    pub fuel_consumption: ::std::vec::Vec<FuelConsumptionType>,
    #[serde(default, rename = "DeliveryNoteDocumentReference")]
    pub delivery_note_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "DeliveryNoteLineReference")]
    pub delivery_note_line_reference: ::std::vec::Vec<LineReferenceType>,
}
pub type DeliveryUnit = DeliveryUnitType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DeliveryUnitType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "BatchQuantity")]
    pub batch_quantity: super::cct::QuantityType,
    #[serde(default, rename = "ConsumerUnitQuantity")]
    pub consumer_unit_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: ::core::option::Option<super::udt::IndicatorType>,
}
pub type DependentLineReference = LineReferenceType;
pub type DependentPriceReference = DependentPriceReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DependentPriceReferenceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Percent")]
    pub percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "LocationAddress")]
    pub location_address: ::core::option::Option<AddressType>,
    #[serde(default, rename = "DependentLineReference")]
    pub dependent_line_reference: ::core::option::Option<LineReferenceType>,
}
pub type Despatch = DespatchType;
pub type DespatchAddress = AddressType;
pub type DespatchContact = ContactType;
pub type DespatchDocumentReference = DocumentReferenceType;
pub type DespatchLine = DespatchLineType;
pub type DespatchLineReference = LineReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DespatchLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "LineStatusCode")]
    pub line_status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "DeliveredQuantity")]
    pub delivered_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "BackorderQuantity")]
    pub backorder_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "BackorderReason")]
    pub backorder_reason: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "OutstandingQuantity")]
    pub outstanding_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "OutstandingReason")]
    pub outstanding_reason: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "OversupplyQuantity")]
    pub oversupply_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: ::std::vec::Vec<OrderLineReferenceType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(rename = "Item")]
    pub item: ItemType,
    #[serde(default, rename = "Shipment")]
    pub shipment: ::std::vec::Vec<ShipmentType>,
    #[serde(default, rename = "SubDespatchLine")]
    pub sub_despatch_line: ::std::vec::Vec<DespatchLineType>,
}
pub type DespatchLocation = LocationType;
pub type DespatchParty = PartyType;
pub type DespatchSupplierParty = SupplierPartyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DespatchType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "RequestedDespatchDate")]
    pub requested_despatch_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "RequestedDespatchTime")]
    pub requested_despatch_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EstimatedDespatchDate")]
    pub estimated_despatch_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EstimatedDespatchTime")]
    pub estimated_despatch_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ActualDespatchDate")]
    pub actual_despatch_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ActualDespatchTime")]
    pub actual_despatch_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "GuaranteedDespatchDate")]
    pub guaranteed_despatch_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "GuaranteedDespatchTime")]
    pub guaranteed_despatch_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ReleaseID")]
    pub release_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Instructions")]
    pub instructions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "DespatchAddress")]
    pub despatch_address: ::core::option::Option<AddressType>,
    #[serde(default, rename = "DespatchLocation")]
    pub despatch_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "DespatchParty")]
    pub despatch_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "NotifyParty")]
    pub notify_party: ::std::vec::Vec<PartyType>,
    #[serde(default, rename = "ResponsibleParty")]
    pub responsible_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "Contact")]
    pub contact: ::core::option::Option<ContactType>,
    #[serde(default, rename = "EstimatedDespatchPeriod")]
    pub estimated_despatch_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "RequestedDespatchPeriod")]
    pub requested_despatch_period: ::core::option::Option<PeriodType>,
}
pub type DestinationCountry = CountryType;
pub type DestinationPortCall = PortCallType;
pub type DetentionTransportEvent = TransportEventType;
pub type DigitalAgreementTerms = DigitalAgreementTermsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DigitalAgreementTermsType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "AdoptionPeriod")]
    pub adoption_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "ServiceLevelAgreement")]
    pub service_level_agreement: ::std::vec::Vec<ServiceLevelAgreementType>,
}
pub type DigitalCertificate = CertificateType;
pub type DigitalCollaboration = DigitalCollaborationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DigitalCollaborationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SendingDigitalService")]
    pub sending_digital_service: ::core::option::Option<DigitalServiceType>,
    #[serde(default, rename = "ReceivingDigitalService")]
    pub receiving_digital_service: ::core::option::Option<DigitalServiceType>,
}
pub type DigitalDeliveryChannel = DeliveryChannelType;
pub type DigitalDocumentMetadata = DocumentMetadataType;
pub type DigitalMessageDelivery = MessageDeliveryType;
pub type DigitalProcess = DigitalProcessType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DigitalProcessType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ProfileID")]
    pub profile_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "DigitalCollaboration")]
    pub digital_collaboration: ::std::vec::Vec<DigitalCollaborationType>,
    #[serde(default, rename = "CertificationDocumentReference")]
    pub certification_document_reference: ::std::vec::Vec<DocumentReferenceType>,
}
pub type DigitalService = DigitalServiceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DigitalServiceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CustomizationID")]
    pub customization_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "DigitalDocumentMetadata")]
    pub digital_document_metadata: ::std::vec::Vec<DocumentMetadataType>,
    #[serde(default, rename = "DigitalDeliveryChannel")]
    pub digital_delivery_channel: ::std::vec::Vec<DeliveryChannelType>,
    #[serde(default, rename = "CertificationDocumentReference")]
    pub certification_document_reference: ::std::vec::Vec<DocumentReferenceType>,
}
pub type DigitalSignatureAttachment = AttachmentType;
pub type Dimension = DimensionType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DimensionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "AttributeID")]
    pub attribute_id: super::cct::IdentifierType,
    #[serde(default, rename = "Measure")]
    pub measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "MinimumMeasure")]
    pub minimum_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "MaximumMeasure")]
    pub maximum_measure: ::core::option::Option<super::cct::MeasureType>,
}
pub type DisbursementPaymentTerms = PaymentTermsType;
pub type DischargeBallastWaterTransaction = BallastWaterTransactionType;
pub type DischargeTransportEvent = TransportEventType;
pub type DiscrepancyResponse = ResponseType;
pub type DisposalFacilityParty = PartyType;
pub type DocumentAvailabilityPeriod = PeriodType;
pub type DocumentDistribution = DocumentDistributionType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentDistributionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "DocumentTypeCode")]
    pub document_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "DistributionTypeCode")]
    pub distribution_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "DistributionType")]
    pub distribution_type: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PrintQualifier")]
    pub print_qualifier: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "MaximumCopiesNumeric")]
    pub maximum_copies_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "MaximumOriginalsNumeric")]
    pub maximum_originals_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "Communication")]
    pub communication: ::core::option::Option<CommunicationType>,
    #[serde(rename = "Party")]
    pub party: PartyType,
}
pub type DocumentMetadata = DocumentMetadataType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentMetadataType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(rename = "FormatID")]
    pub format_id: super::cct::IdentifierType,
    #[serde(rename = "VersionID")]
    pub version_id: super::cct::IdentifierType,
    #[serde(default, rename = "SchemaURI")]
    pub schema_uri: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "DocumentTypeCode")]
    pub document_type_code: ::core::option::Option<super::cct::CodeType>,
}
pub type DocumentProviderParty = PartyType;
pub type DocumentReference = DocumentReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentReferenceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "DocumentTypeCode")]
    pub document_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "DocumentType")]
    pub document_type: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "XPath")]
    pub x_path: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ReferencedDocumentInternalAddress")]
    pub referenced_document_internal_address: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "LanguageID")]
    pub language_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "LocaleCode")]
    pub locale_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "VersionID")]
    pub version_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "DocumentStatusCode")]
    pub document_status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "DocumentDescription")]
    pub document_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Attachment")]
    pub attachment: ::core::option::Option<AttachmentType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "ResultOfVerification")]
    pub result_of_verification: ::core::option::Option<ResultOfVerificationType>,
}
pub type DocumentResponse = DocumentResponseType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DocumentResponseType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "Response")]
    pub response: ResponseType,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "RecipientParty")]
    pub recipient_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "LineResponse")]
    pub line_response: ::std::vec::Vec<LineResponseType>,
}
pub type DocumentTenderRequirement = TenderRequirementType;
pub type DocumentationAttachment = AttachmentType;
pub type DriverPerson = PersonType;
pub type DropoffTransportEvent = TransportEventType;
pub type DurationPeriod = PeriodType;
pub type Duty = DutyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct DutyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "Amount")]
    pub amount: super::cct::AmountType,
    #[serde(default, rename = "Duty")]
    pub duty: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "DutyCode")]
    pub duty_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TaxCategory")]
    pub tax_category: ::core::option::Option<TaxCategoryType>,
}
pub type EconomicOperatorParty = EconomicOperatorPartyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EconomicOperatorPartyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "QualifyingParty")]
    pub qualifying_party: ::std::vec::Vec<QualifyingPartyType>,
    #[serde(default, rename = "EconomicOperatorRole")]
    pub economic_operator_role: ::std::vec::Vec<EconomicOperatorRoleType>,
    #[serde(rename = "Party")]
    pub party: PartyType,
}
pub type EconomicOperatorRole = EconomicOperatorRoleType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EconomicOperatorRoleType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "RoleCode")]
    pub role_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "RoleDescription")]
    pub role_description: ::std::vec::Vec<super::cct::TextType>,
}
pub type EconomicOperatorShortList = EconomicOperatorShortListType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EconomicOperatorShortListType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "LimitationDescription")]
    pub limitation_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ExpectedQuantity")]
    pub expected_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "PreSelectedParty")]
    pub pre_selected_party: ::std::vec::Vec<PartyType>,
}
pub type EffectivePeriod = PeriodType;
pub type ElectronicAddress = ElectronicAddressType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ElectronicAddressType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ExchangeNetworkID")]
    pub exchange_network_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(rename = "ElectronicAddressID")]
    pub electronic_address_id: super::cct::IdentifierType,
}
pub type EmbassyEndorsement = EndorsementType;
pub type EmergencyTemperature = TemperatureType;
pub type EmissionCalculationLocation = LocationType;
pub type EmissionCalculationMethod = EmissionCalculationMethodType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EmissionCalculationMethodType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "CalculationMethodCode")]
    pub calculation_method_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "FullnessIndicationCode")]
    pub fullness_indication_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "EmissionFactorSource")]
    pub emission_factor_source: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "EmissionFactorDocumentReference")]
    pub emission_factor_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "MeasurementFromLocation")]
    pub measurement_from_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "MeasurementToLocation")]
    pub measurement_to_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "EmissionCalculationLocation")]
    pub emission_calculation_location: ::std::vec::Vec<LocationType>,
}
pub type EmissionFactorDocumentReference = DocumentReferenceType;
pub type EmploymentLegislationDocumentReference = DocumentReferenceType;
pub type EncryptionCertificateAttachment = AttachmentType;
pub type EncryptionCertificatePathChain = EncryptionCertificatePathChainType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptionCertificatePathChainType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Value")]
    pub value: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "URI")]
    pub uri: ::core::option::Option<super::cct::IdentifierType>,
}
pub type EncryptionData = EncryptionDataType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptionDataType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "MessageFormat")]
    pub message_format: super::cct::TextType,
    #[serde(default, rename = "EncryptionCertificateAttachment")]
    pub encryption_certificate_attachment: ::core::option::Option<AttachmentType>,
    #[serde(default, rename = "EncryptionCertificatePathChain")]
    pub encryption_certificate_path_chain: ::std::vec::Vec<EncryptionCertificatePathChainType>,
    #[serde(default, rename = "EncryptionSymmetricAlgorithm")]
    pub encryption_symmetric_algorithm: ::std::vec::Vec<EncryptionSymmetricAlgorithmType>,
}
pub type EncryptionSymmetricAlgorithm = EncryptionSymmetricAlgorithmType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EncryptionSymmetricAlgorithmType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "OID")]
    pub oid: ::core::option::Option<super::cct::IdentifierType>,
}
pub type EndOfLifeTreatment = EndOfLifeTreatmentType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EndOfLifeTreatmentType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "TreatmentPathwayCode")]
    pub treatment_pathway_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ProcessingTypeCode")]
    pub processing_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ImpactCode")]
    pub impact_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TreatmentLocation")]
    pub treatment_location: ::core::option::Option<LocationType>,
}
pub type Endorsement = EndorsementType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EndorsementType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "DocumentID")]
    pub document_id: super::cct::IdentifierType,
    #[serde(rename = "ApprovalStatus")]
    pub approval_status: super::cct::TextType,
    #[serde(default, rename = "Remarks")]
    pub remarks: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "EndorserParty")]
    pub endorser_party: EndorserPartyType,
    #[serde(default, rename = "Signature")]
    pub signature: ::std::vec::Vec<SignatureType>,
}
pub type EndorserParty = EndorserPartyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EndorserPartyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "RoleCode")]
    pub role_code: super::cct::CodeType,
    #[serde(rename = "SequenceNumeric")]
    pub sequence_numeric: super::cct::NumericType,
    #[serde(rename = "Party")]
    pub party: PartyType,
    #[serde(rename = "SignatoryContact")]
    pub signatory_contact: ContactType,
}
pub type EnergyConsumptionAllocation = EnergyConsumptionAllocationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EnergyConsumptionAllocationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "EnergySourceTypeCode")]
    pub energy_source_type_code: super::cct::CodeType,
    #[serde(rename = "AllocatedEnergyMeasure")]
    pub allocated_energy_measure: super::cct::MeasureType,
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: ::std::vec::Vec<EnvironmentalEmissionType>,
}
pub type EnergyTaxReport = EnergyTaxReportType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EnergyTaxReportType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "TaxEnergyAmount")]
    pub tax_energy_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxEnergyOnAccountAmount")]
    pub tax_energy_on_account_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxEnergyBalanceAmount")]
    pub tax_energy_balance_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(rename = "TaxScheme")]
    pub tax_scheme: TaxSchemeType,
}
pub type EnergyWaterConsumptionCorrection = ConsumptionCorrectionType;
pub type EnergyWaterSupply = EnergyWaterSupplyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EnergyWaterSupplyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ConsumptionReport")]
    pub consumption_report: ::std::vec::Vec<ConsumptionReportType>,
    #[serde(default, rename = "EnergyTaxReport")]
    pub energy_tax_report: ::std::vec::Vec<EnergyTaxReportType>,
    #[serde(default, rename = "ConsumptionAverage")]
    pub consumption_average: ::std::vec::Vec<ConsumptionAverageType>,
    #[serde(default, rename = "EnergyWaterConsumptionCorrection")]
    pub energy_water_consumption_correction: ::std::vec::Vec<ConsumptionCorrectionType>,
}
pub type EnvironmentalCertificate = CertificateType;
pub type EnvironmentalEmission = EnvironmentalEmissionType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EnvironmentalEmissionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "EnvironmentalEmissionTypeCode")]
    pub environmental_emission_type_code: super::cct::CodeType,
    #[serde(rename = "ValueMeasure")]
    pub value_measure: super::cct::MeasureType,
    #[serde(default, rename = "ValueFactorNumeric")]
    pub value_factor_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "ValueBaseMeasure")]
    pub value_base_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "EmissionStandardReference")]
    pub emission_standard_reference: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "LifecycleStageCode")]
    pub lifecycle_stage_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "LifecycleStageDescription")]
    pub lifecycle_stage_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "EmissionCalculationMethod")]
    pub emission_calculation_method: ::std::vec::Vec<EmissionCalculationMethodType>,
    #[serde(default, rename = "MeasurementPeriod")]
    pub measurement_period: ::core::option::Option<PeriodType>,
}
pub type EnvironmentalLegislationDocumentReference = DocumentReferenceType;
pub type EstimatedArrivalTransportEvent = TransportEventType;
pub type EstimatedDeliveryPeriod = PeriodType;
pub type EstimatedDepartureTransportEvent = TransportEventType;
pub type EstimatedDespatchPeriod = PeriodType;
pub type EstimatedDurationPeriod = PeriodType;
pub type EstimatedTransitPeriod = PeriodType;
pub type EvaluationCriterion = EvaluationCriterionType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EvaluationCriterionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "EvaluationCriterionTypeCode")]
    pub evaluation_criterion_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ThresholdAmount")]
    pub threshold_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "ThresholdQuantity")]
    pub threshold_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "ExpressionCode")]
    pub expression_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Expression")]
    pub expression: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "DurationPeriod")]
    pub duration_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "SuggestedEvidence")]
    pub suggested_evidence: ::std::vec::Vec<EvidenceType>,
}
pub type Event = EventType;
pub type EventComment = EventCommentType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EventCommentType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "Comment")]
    pub comment: super::cct::TextType,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<super::udt::DateTimeType>,
}
pub type EventLineItem = EventLineItemType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EventLineItemType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "LineNumberNumeric")]
    pub line_number_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "ParticipatingLocationsLocation")]
    pub participating_locations_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "RetailPlannedImpact")]
    pub retail_planned_impact: ::std::vec::Vec<RetailPlannedImpactType>,
    #[serde(rename = "SupplyItem")]
    pub supply_item: ItemType,
}
pub type EventTactic = EventTacticType;
pub type EventTacticEnumeration = EventTacticEnumerationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EventTacticEnumerationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ConsumerIncentiveTacticTypeCode")]
    pub consumer_incentive_tactic_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "DisplayTacticTypeCode")]
    pub display_tactic_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "FeatureTacticTypeCode")]
    pub feature_tactic_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TradeItemPackingLabelingTypeCode")]
    pub trade_item_packing_labeling_type_code: ::core::option::Option<super::cct::CodeType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct EventTacticType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Comment")]
    pub comment: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(rename = "EventTacticEnumeration")]
    pub event_tactic_enumeration: EventTacticEnumerationType,
    #[serde(default, rename = "Period")]
    pub period: ::core::option::Option<PeriodType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct EventType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "IdentificationID")]
    pub identification_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "OccurrenceDate")]
    pub occurrence_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "OccurrenceTime")]
    pub occurrence_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CompletionIndicator")]
    pub completion_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "CurrentStatus")]
    pub current_status: ::std::vec::Vec<StatusType>,
    #[serde(default, rename = "Contact")]
    pub contact: ::std::vec::Vec<ContactType>,
    #[serde(default, rename = "OccurenceLocation")]
    pub occurence_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "OccurrenceLocation")]
    pub occurrence_location: ::core::option::Option<LocationType>,
}
pub type Evidence = EvidenceType;
pub type EvidenceDocumentReference = DocumentReferenceType;
pub type EvidenceIssuingParty = PartyType;
pub type EvidenceSupplied = EvidenceSuppliedType;
#[derive(Debug, Deserialize, Serialize)]
pub struct EvidenceSuppliedType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct EvidenceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "EvidenceTypeCode")]
    pub evidence_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CandidateStatement")]
    pub candidate_statement: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ConfidentialityLevelCode")]
    pub confidentiality_level_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "EvidenceIssuingParty")]
    pub evidence_issuing_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "Language")]
    pub language: ::core::option::Option<LanguageType>,
}
pub type ExaminationTransportEvent = TransportEventType;
pub type ExceptionCriteriaLine = ExceptionCriteriaLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ExceptionCriteriaLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "ThresholdValueComparisonCode")]
    pub threshold_value_comparison_code: super::cct::CodeType,
    #[serde(rename = "ThresholdQuantity")]
    pub threshold_quantity: super::cct::QuantityType,
    #[serde(default, rename = "ExceptionStatusCode")]
    pub exception_status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CollaborationPriorityCode")]
    pub collaboration_priority_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ExceptionResolutionCode")]
    pub exception_resolution_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "SupplyChainActivityTypeCode")]
    pub supply_chain_activity_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PerformanceMetricTypeCode")]
    pub performance_metric_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "EffectivePeriod")]
    pub effective_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "SupplyItem")]
    pub supply_item: ::std::vec::Vec<ItemType>,
    #[serde(default, rename = "ForecastExceptionCriterionLine")]
    pub forecast_exception_criterion_line:
        ::core::option::Option<ForecastExceptionCriterionLineType>,
}
pub type ExceptionNotificationLine = ExceptionNotificationLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ExceptionNotificationLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ExceptionStatusCode")]
    pub exception_status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CollaborationPriorityCode")]
    pub collaboration_priority_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ResolutionCode")]
    pub resolution_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(rename = "ComparedValueMeasure")]
    pub compared_value_measure: super::cct::MeasureType,
    #[serde(rename = "SourceValueMeasure")]
    pub source_value_measure: super::cct::MeasureType,
    #[serde(default, rename = "VarianceQuantity")]
    pub variance_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "SupplyChainActivityTypeCode")]
    pub supply_chain_activity_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PerformanceMetricTypeCode")]
    pub performance_metric_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ExceptionObservationPeriod")]
    pub exception_observation_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "ForecastException")]
    pub forecast_exception: ::core::option::Option<ForecastExceptionType>,
    #[serde(rename = "SupplyItem")]
    pub supply_item: ItemType,
}
pub type ExceptionObservationPeriod = PeriodType;
pub type ExchangeBallastWaterTransaction = BallastWaterTransactionType;
pub type ExchangeRate = ExchangeRateType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ExchangeRateType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "SourceCurrencyCode")]
    pub source_currency_code: super::cct::CodeType,
    #[serde(default, rename = "SourceCurrencyBaseRate")]
    pub source_currency_base_rate: ::core::option::Option<super::cct::NumericType>,
    #[serde(rename = "TargetCurrencyCode")]
    pub target_currency_code: super::cct::CodeType,
    #[serde(default, rename = "TargetCurrencyBaseRate")]
    pub target_currency_base_rate: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "ExchangeMarketID")]
    pub exchange_market_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CalculationRate")]
    pub calculation_rate: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "MathematicOperatorCode")]
    pub mathematic_operator_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Date")]
    pub date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ForeignExchangeContract")]
    pub foreign_exchange_contract: ::core::option::Option<ContractType>,
}
pub type ExportCountry = CountryType;
pub type ExportCustomsExitOfficeLocation = LocationType;
pub type ExportationDocumentReference = DocumentReferenceType;
pub type ExportationTransportEvent = TransportEventType;
pub type ExporterParty = PartyType;
pub type ExportingCustomsParty = PartyType;
pub type ExportingGuarantorParty = PartyType;
pub type ExpressionOfInterestDocumentReference = DocumentReferenceType;
pub type ExternalReference = ExternalReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ExternalReferenceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "URI")]
    pub uri: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "DocumentHash")]
    pub document_hash: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "HashAlgorithmMethod")]
    pub hash_algorithm_method: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ExpiryDate")]
    pub expiry_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ExpiryTime")]
    pub expiry_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "MimeCode")]
    pub mime_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "FormatCode")]
    pub format_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "EncodingCode")]
    pub encoding_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CharacterSetCode")]
    pub character_set_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "FileName")]
    pub file_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
}
pub type ExtraAllowanceCharge = AllowanceChargeType;
pub type Fee = FeeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FeeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "FeeTypeCode")]
    pub fee_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "FeeAmount")]
    pub fee_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "FeeDescription")]
    pub fee_description: ::std::vec::Vec<super::cct::TextType>,
}
pub type FinalDeliveryParty = PartyType;
pub type FinalDeliveryTransportationService = TransportationServiceType;
pub type FinalDestinationCountry = CountryType;
pub type FinalFinancialGuarantee = FinancialGuaranteeType;
pub type FinancialAccount = FinancialAccountType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FinancialAccountType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "AliasName")]
    pub alias_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "AccountTypeCode")]
    pub account_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "AccountFormatCode")]
    pub account_format_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CurrencyCode")]
    pub currency_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "BlockchainID")]
    pub blockchain_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PaymentNote")]
    pub payment_note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "FinancialInstitutionBranch")]
    pub financial_institution_branch: ::core::option::Option<BranchType>,
    #[serde(default, rename = "Country")]
    pub country: ::core::option::Option<CountryType>,
}
pub type FinancialCapability = CapabilityType;
pub type FinancialEvaluationCriterion = EvaluationCriterionType;
pub type FinancialGuarantee = FinancialGuaranteeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FinancialGuaranteeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "GuaranteeTypeCode")]
    pub guarantee_type_code: super::cct::CodeType,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "LiabilityAmount")]
    pub liability_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "AmountRate")]
    pub amount_rate: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "ConstitutionPeriod")]
    pub constitution_period: ::core::option::Option<PeriodType>,
}
pub type FinancialInstitution = FinancialInstitutionType;
pub type FinancialInstitutionBranch = BranchType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FinancialInstitutionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Address")]
    pub address: ::core::option::Option<AddressType>,
}
pub type FinancingFinancialAccount = FinancialAccountType;
pub type FinancingParty = PartyType;
pub type FirstArrivalPortLocation = LocationType;
pub type FiscalLegislationDocumentReference = DocumentReferenceType;
pub type FlashpointTemperature = TemperatureType;
pub type FloorSpaceMeasurementDimension = DimensionType;
pub type ForecastException = ForecastExceptionType;
pub type ForecastExceptionCriterionLine = ForecastExceptionCriterionLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastExceptionCriterionLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ForecastPurposeCode")]
    pub forecast_purpose_code: super::cct::CodeType,
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: super::cct::CodeType,
    #[serde(default, rename = "ComparisonDataSourceCode")]
    pub comparison_data_source_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(rename = "DataSourceCode")]
    pub data_source_code: super::cct::CodeType,
    #[serde(default, rename = "TimeDeltaDaysQuantity")]
    pub time_delta_days_quantity: ::core::option::Option<super::cct::QuantityType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastExceptionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ForecastPurposeCode")]
    pub forecast_purpose_code: super::cct::CodeType,
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: super::cct::CodeType,
    #[serde(rename = "IssueDate")]
    pub issue_date: super::udt::DateTimeType,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(rename = "DataSourceCode")]
    pub data_source_code: super::cct::CodeType,
    #[serde(default, rename = "ComparisonDataCode")]
    pub comparison_data_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ComparisonForecastIssueTime")]
    pub comparison_forecast_issue_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ComparisonForecastIssueDate")]
    pub comparison_forecast_issue_date: ::core::option::Option<super::udt::DateTimeType>,
}
pub type ForecastLine = ForecastLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "FrozenDocumentIndicator")]
    pub frozen_document_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: super::cct::CodeType,
    #[serde(default, rename = "ForecastPeriod")]
    pub forecast_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "SalesItem")]
    pub sales_item: ::core::option::Option<SalesItemType>,
}
pub type ForecastPeriod = PeriodType;
pub type ForecastRevisionLine = ForecastRevisionLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ForecastRevisionLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "RevisedForecastLineID")]
    pub revised_forecast_line_id: super::cct::IdentifierType,
    #[serde(rename = "SourceForecastIssueDate")]
    pub source_forecast_issue_date: super::udt::DateTimeType,
    #[serde(rename = "SourceForecastIssueTime")]
    pub source_forecast_issue_time: super::udt::DateTimeType,
    #[serde(default, rename = "AdjustmentReasonCode")]
    pub adjustment_reason_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ForecastPeriod")]
    pub forecast_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "SalesItem")]
    pub sales_item: ::core::option::Option<SalesItemType>,
}
pub type ForeignExchangeContract = ContractType;
pub type FrameworkAgreement = FrameworkAgreementType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FrameworkAgreementType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ExpectedOperatorQuantity")]
    pub expected_operator_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumOperatorQuantity")]
    pub maximum_operator_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "Justification")]
    pub justification: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Frequency")]
    pub frequency: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "EstimatedMaximumValueAmount")]
    pub estimated_maximum_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "MaximumValueAmount")]
    pub maximum_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "DurationPeriod")]
    pub duration_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "SubsequentProcessTenderRequirement")]
    pub subsequent_process_tender_requirement: ::std::vec::Vec<TenderRequirementType>,
}
pub type FreightAllowanceCharge = AllowanceChargeType;
pub type FreightChargeLocation = LocationType;
pub type FreightForwarderParty = PartyType;
pub type FrequencyPeriod = PeriodType;
pub type FromLocation = LocationType;
pub type FuelConsumption = FuelConsumptionType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FuelConsumptionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "FuelTypeCode")]
    pub fuel_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "FuelType")]
    pub fuel_type: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "FuelConsumptionMeasure")]
    pub fuel_consumption_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "AdditionalFuelProperty")]
    pub additional_fuel_property: ::std::vec::Vec<FuelPropertyType>,
    #[serde(default, rename = "FuelMetering")]
    pub fuel_metering: ::std::vec::Vec<FuelMeteringType>,
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: ::std::vec::Vec<EnvironmentalEmissionType>,
    #[serde(default, rename = "FuelProviderParty")]
    pub fuel_provider_party: ::core::option::Option<PartyType>,
}
pub type FuelMetering = FuelMeteringType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FuelMeteringType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "TypeID")]
    pub type_id: super::cct::IdentifierType,
    #[serde(rename = "Value")]
    pub value: super::cct::TextType,
}
pub type FuelProperty = FuelPropertyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct FuelPropertyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "TypeID")]
    pub type_id: super::cct::IdentifierType,
    #[serde(rename = "Value")]
    pub value: super::cct::TextType,
}
pub type FuelProviderParty = PartyType;
pub type GoodsItem = GoodsItemType;
pub type GoodsItemContainer = GoodsItemContainerType;
#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsItemContainerType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Quantity")]
    pub quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "TransportEquipment")]
    pub transport_equipment: ::std::vec::Vec<TransportEquipmentType>,
}
pub type GoodsItemPassportAttachment = AttachmentType;
pub type GoodsItemPassportCounterfoil = GoodsItemPassportCounterfoilType;
#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsItemPassportCounterfoilType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "GoodsItemPassportID")]
    pub goods_item_passport_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "FinalReexportationDate")]
    pub final_reexportation_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CustomsOfficeLocation")]
    pub customs_office_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: ::core::option::Option<GoodsItemType>,
    #[serde(default, rename = "ExportationDocumentReference")]
    pub exportation_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "ImportationDocumentReference")]
    pub importation_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "ReexportationDocumentReference")]
    pub reexportation_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "ReimportationDocumentReference")]
    pub reimportation_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "VoucherDocumentReference")]
    pub voucher_document_reference: ::std::vec::Vec<DocumentReferenceType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsItemType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SequenceNumberID")]
    pub sequence_number_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "DeclaredCustomsValueAmount")]
    pub declared_customs_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "DeclaredForCarriageValueAmount")]
    pub declared_for_carriage_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "DeclaredStatisticsValueAmount")]
    pub declared_statistics_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "FreeOnBoardValueAmount")]
    pub free_on_board_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "InsuranceValueAmount")]
    pub insurance_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "ValueAmount")]
    pub value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetWeightMeasure")]
    pub net_weight_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetNetWeightMeasure")]
    pub net_net_weight_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "ChargeableWeightMeasure")]
    pub chargeable_weight_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "GrossVolumeMeasure")]
    pub gross_volume_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetVolumeMeasure")]
    pub net_volume_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "PreferenceCriterionCode")]
    pub preference_criterion_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "RequiredCustomsID")]
    pub required_customs_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CustomsStatusCode")]
    pub customs_status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CustomsProcedureCode")]
    pub customs_procedure_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CustomsTariffQuantity")]
    pub customs_tariff_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "CustomsImportClassifiedIndicator")]
    pub customs_import_classified_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ChargeableQuantity")]
    pub chargeable_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "ReturnableQuantity")]
    pub returnable_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "TraceID")]
    pub trace_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Item")]
    pub item: ::std::vec::Vec<ItemType>,
    #[serde(default, rename = "GoodsItemContainer")]
    pub goods_item_container: ::std::vec::Vec<GoodsItemContainerType>,
    #[serde(default, rename = "FreightAllowanceCharge")]
    pub freight_allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "InvoiceLine")]
    pub invoice_line: ::std::vec::Vec<InvoiceLineType>,
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: ::std::vec::Vec<OrderLineReferenceType>,
    #[serde(default, rename = "DespatchLineReference")]
    pub despatch_line_reference: ::core::option::Option<LineReferenceType>,
    #[serde(default, rename = "ReceiptLineReference")]
    pub receipt_line_reference: ::core::option::Option<LineReferenceType>,
    #[serde(default, rename = "Temperature")]
    pub temperature: ::std::vec::Vec<TemperatureType>,
    #[serde(default, rename = "ContainedGoodsItem")]
    pub contained_goods_item: ::std::vec::Vec<GoodsItemType>,
    #[serde(default, rename = "OriginAddress")]
    pub origin_address: ::core::option::Option<AddressType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::core::option::Option<DeliveryType>,
    #[serde(default, rename = "Pickup")]
    pub pickup: ::core::option::Option<PickupType>,
    #[serde(default, rename = "Despatch")]
    pub despatch: ::core::option::Option<DespatchType>,
    #[serde(default, rename = "BondedWarehouseLocation")]
    pub bonded_warehouse_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: ::std::vec::Vec<DimensionType>,
    #[serde(default, rename = "ContainingPackage")]
    pub containing_package: ::std::vec::Vec<PackageType>,
    #[serde(default, rename = "ShipmentDocumentReference")]
    pub shipment_document_reference: ::core::option::Option<DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: ::core::option::Option<TemperatureType>,
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: ::core::option::Option<TemperatureType>,
    #[serde(default, rename = "InsurancePolicy")]
    pub insurance_policy: ::std::vec::Vec<InsurancePolicyType>,
    #[serde(default, rename = "EnergyConsumptionAllocation")]
    pub energy_consumption_allocation: ::std::vec::Vec<EnergyConsumptionAllocationType>,
}
pub type GoodsProcessing = GoodsProcessingType;
#[derive(Debug, Deserialize, Serialize)]
pub struct GoodsProcessingType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Period")]
    pub period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "ProcessingParty")]
    pub processing_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "CriterionItem")]
    pub criterion_item: ::std::vec::Vec<CriterionItemType>,
    #[serde(default, rename = "SubGoodsProcessing")]
    pub sub_goods_processing: ::std::vec::Vec<GoodsProcessingType>,
}
pub type GovernorParty = PartyType;
pub type GuaranteeDocumentReference = DocumentReferenceType;
pub type GuarantorParty = PartyType;
pub type GuidanceDocumentReference = DocumentReferenceType;
pub type HandlingTransportEvent = TransportEventType;
pub type HandlingUnitDespatchLine = DespatchLineType;
pub type HaulageTradingTerms = TradingTermsType;
pub type HazardousGoodsTransit = HazardousGoodsTransitType;
#[derive(Debug, Deserialize, Serialize)]
pub struct HazardousGoodsTransitType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "TransportEmergencyCardCode")]
    pub transport_emergency_card_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PackingCriteriaCode")]
    pub packing_criteria_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "HazardousRegulationCode")]
    pub hazardous_regulation_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "InhalationToxicityZoneCode")]
    pub inhalation_toxicity_zone_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TransportAuthorizationCode")]
    pub transport_authorization_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TransitDescription")]
    pub transit_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: ::core::option::Option<TemperatureType>,
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: ::core::option::Option<TemperatureType>,
}
pub type HazardousItem = HazardousItemType;
pub type HazardousItemNotificationParty = PartyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct HazardousItemType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PlacardNotation")]
    pub placard_notation: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "PlacardEndorsement")]
    pub placard_endorsement: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "AdditionalInformation")]
    pub additional_information: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "UNDGCode")]
    pub undg_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "UNPackingGroupCode")]
    pub un_packing_group_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "UNPackingGroup")]
    pub un_packing_group: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "EmergencyProceduresCode")]
    pub emergency_procedures_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "MedicalFirstAidGuideCode")]
    pub medical_first_aid_guide_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TunnelRestrictionCode")]
    pub tunnel_restriction_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "MaritimePollutantCode")]
    pub maritime_pollutant_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TechnicalName")]
    pub technical_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "CategoryName")]
    pub category_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ProperShippingName")]
    pub proper_shipping_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "HazardousCategoryCode")]
    pub hazardous_category_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "UpperOrangeHazardPlacardID")]
    pub upper_orange_hazard_placard_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "LowerOrangeHazardPlacardID")]
    pub lower_orange_hazard_placard_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "MarkingID")]
    pub marking_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "HazardClassID")]
    pub hazard_class_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "HazardousTypeCode")]
    pub hazardous_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PackagingDangerLevelCode")]
    pub packaging_danger_level_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetWeightMeasure")]
    pub net_weight_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetVolumeMeasure")]
    pub net_volume_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "ContactParty")]
    pub contact_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "SecondaryHazard")]
    pub secondary_hazard: ::std::vec::Vec<SecondaryHazardType>,
    #[serde(default, rename = "HazardousGoodsTransit")]
    pub hazardous_goods_transit: ::std::vec::Vec<HazardousGoodsTransitType>,
    #[serde(default, rename = "EmergencyTemperature")]
    pub emergency_temperature: ::core::option::Option<TemperatureType>,
    #[serde(default, rename = "FlashpointTemperature")]
    pub flashpoint_temperature: ::core::option::Option<TemperatureType>,
    #[serde(default, rename = "AdditionalTemperature")]
    pub additional_temperature: ::std::vec::Vec<TemperatureType>,
    #[serde(default, rename = "PositionOnBoardStowage")]
    pub position_on_board_stowage: ::core::option::Option<StowageType>,
    #[serde(default, rename = "RadioactiveMaterial")]
    pub radioactive_material: ::std::vec::Vec<RadioactiveMaterialType>,
    #[serde(default, rename = "Package")]
    pub package: ::core::option::Option<PackageType>,
}
pub type HeadOfficeParty = PartyType;
pub type HolderParty = PartyType;
pub type IspsRequirements = IspsRequirementsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct IspsRequirementsType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "ValidISSCIndicator")]
    pub valid_issc_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ISSCAbsenceReason")]
    pub issc_absence_reason: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ISSCExpiryDate")]
    pub issc_expiry_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "SSPOnBoardIndicator")]
    pub ssp_on_board_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "SSPSecurityMeasuresAppliedIndicator")]
    pub ssp_security_measures_applied_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "CurrentOperatingSecurityLevelCode")]
    pub current_operating_security_level_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "AdditionalMattersDescription")]
    pub additional_matters_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "AdditionalSecurityMeasure")]
    pub additional_security_measure: ::std::vec::Vec<SecurityMeasureType>,
    #[serde(default, rename = "PortCallRecord")]
    pub port_call_record: ::std::vec::Vec<PortCallRecordType>,
    #[serde(default, rename = "ShipToShipActivityRecord")]
    pub ship_to_ship_activity_record: ::std::vec::Vec<ShipToShipActivityRecordType>,
    #[serde(default, rename = "ReportLocation")]
    pub report_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "ISSCIssuerParty")]
    pub issc_issuer_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "SecurityOfficerPerson")]
    pub security_officer_person: ::core::option::Option<PersonType>,
}
pub type IsscIssuerParty = PartyType;
pub type IdentityDocumentReference = DocumentReferenceType;
pub type ImmobilizedSecurity = ImmobilizedSecurityType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ImmobilizedSecurityType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ImmobilizationCertificateID")]
    pub immobilization_certificate_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SecurityID")]
    pub security_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "FaceValueAmount")]
    pub face_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "MarketValueAmount")]
    pub market_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "SharesNumberQuantity")]
    pub shares_number_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: ::core::option::Option<PartyType>,
}
pub type ImportCustomsExitOfficeLocation = LocationType;
pub type ImportationDocumentReference = DocumentReferenceType;
pub type ImporterParty = PartyType;
pub type ImportingCustomsParty = PartyType;
pub type ImportingGuarantorParty = PartyType;
pub type InformationContentProviderParty = PartyType;
pub type InstructionForReturnsLine = InstructionForReturnsLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct InstructionForReturnsLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "Quantity")]
    pub quantity: super::cct::QuantityType,
    #[serde(default, rename = "ManufacturerParty")]
    pub manufacturer_party: ::core::option::Option<PartyType>,
    #[serde(rename = "Item")]
    pub item: ItemType,
}
pub type InsuranceEndorsement = EndorsementType;
pub type InsuranceParty = PartyType;
pub type InsurancePolicy = InsurancePolicyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct InsurancePolicyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "InsuranceTypeCode")]
    pub insurance_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "InsuranceTypeDescription")]
    pub insurance_type_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "InsuredValueAmount")]
    pub insured_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "DeductibleAmount")]
    pub deductible_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "ExcessAmount")]
    pub excess_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "InsurancePremiumAmount")]
    pub insurance_premium_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "InsurerParty")]
    pub insurer_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "BrokerParty")]
    pub broker_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "PolicyHolderParty")]
    pub policy_holder_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<PartyType>,
    #[serde(default, rename = "PolicyDocumentReference")]
    pub policy_document_reference: ::core::option::Option<DocumentReferenceType>,
}
pub type InsurerParty = PartyType;
pub type InterestRate = InterestRateType;
#[derive(Debug, Deserialize, Serialize)]
pub struct InterestRateType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "InterestRatePercent")]
    pub interest_rate_percent: super::cct::NumericType,
    #[serde(default, rename = "TimeBasisCode")]
    pub time_basis_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CalculationMethodCode")]
    pub calculation_method_code: ::core::option::Option<super::cct::CodeType>,
}
pub type InterestedParty = PartyType;
pub type InterestedProcurementProjectLot = ProcurementProjectLotType;
pub type InventoryLocation = LocationType;
pub type InventoryPeriod = PeriodType;
pub type InventoryReportLine = InventoryReportLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct InventoryReportLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "Quantity")]
    pub quantity: super::cct::QuantityType,
    #[serde(default, rename = "InventoryValueAmount")]
    pub inventory_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "AvailabilityDate")]
    pub availability_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "AvailabilityStatusCode")]
    pub availability_status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(rename = "Item")]
    pub item: ItemType,
    #[serde(default, rename = "InventoryLocation")]
    pub inventory_location: ::core::option::Option<LocationType>,
}
pub type InventoryReportingParty = PartyType;
pub type InvitationSubmissionPeriod = PeriodType;
pub type InvoiceDocumentReference = DocumentReferenceType;
pub type InvoiceLine = InvoiceLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct InvoiceLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "InvoicedQuantity")]
    pub invoiced_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxPointDate")]
    pub tax_point_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "PaymentPurposeCode")]
    pub payment_purpose_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "FreeOfChargeIndicator")]
    pub free_of_charge_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: ::std::vec::Vec<PeriodType>,
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: ::std::vec::Vec<OrderLineReferenceType>,
    #[serde(default, rename = "DespatchLineReference")]
    pub despatch_line_reference: ::std::vec::Vec<LineReferenceType>,
    #[serde(default, rename = "ReceiptLineReference")]
    pub receipt_line_reference: ::std::vec::Vec<LineReferenceType>,
    #[serde(default, rename = "WorkReportLineReference")]
    pub work_report_line_reference: ::std::vec::Vec<LineReferenceType>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: ::std::vec::Vec<BillingReferenceType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "PricingReference")]
    pub pricing_reference: ::core::option::Option<PricingReferenceType>,
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: ::core::option::Option<PurchaseReferenceType>,
    #[serde(default, rename = "OriginatorParty")]
    pub originator_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<PartyType>,
    #[serde(default, rename = "CollectedForParty")]
    pub collected_for_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::std::vec::Vec<DeliveryType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<PaymentTermsType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<TaxTotalType>,
    #[serde(default, rename = "WithholdingTaxTotal")]
    pub withholding_tax_total: ::std::vec::Vec<TaxTotalType>,
    #[serde(rename = "Item")]
    pub item: ItemType,
    #[serde(default, rename = "Price")]
    pub price: ::core::option::Option<PriceType>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: ::core::option::Option<DeliveryTermsType>,
    #[serde(default, rename = "SubInvoiceLine")]
    pub sub_invoice_line: ::std::vec::Vec<InvoiceLineType>,
    #[serde(default, rename = "ItemPriceExtension")]
    pub item_price_extension: ::core::option::Option<PriceExtensionType>,
}
pub type InvoicePeriod = PeriodType;
pub type IssueLocation = LocationType;
pub type IssuerEndorsement = EndorsementType;
pub type IssuerParty = PartyType;
pub type IssuingCountry = CountryType;
pub type Item = ItemType;
pub type ItemComparison = ItemComparisonType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemComparisonType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "PriceAmount")]
    pub price_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: ::core::option::Option<super::cct::QuantityType>,
}
pub type ItemIdentification = ItemIdentificationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemIdentificationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "ExtendedID")]
    pub extended_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "BarcodeSymbologyID")]
    pub barcode_symbology_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssuerScopeID")]
    pub issuer_scope_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PhysicalAttribute")]
    pub physical_attribute: ::std::vec::Vec<PhysicalAttributeType>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: ::std::vec::Vec<DimensionType>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: ::core::option::Option<PartyType>,
}
pub type ItemInformationRequestLine = ItemInformationRequestLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemInformationRequestLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "TimeFrequencyCode")]
    pub time_frequency_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "SupplyChainActivityTypeCode")]
    pub supply_chain_activity_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ForecastTypeCode")]
    pub forecast_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PerformanceMetricTypeCode")]
    pub performance_metric_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Period")]
    pub period: ::std::vec::Vec<PeriodType>,
    #[serde(default, rename = "SalesItem")]
    pub sales_item: ::std::vec::Vec<SalesItemType>,
}
pub type ItemInstance = ItemInstanceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemInstanceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ProductTraceID")]
    pub product_trace_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ManufactureDate")]
    pub manufacture_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ManufactureTime")]
    pub manufacture_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "BestBeforeDate")]
    pub best_before_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "RegistrationID")]
    pub registration_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SerialID")]
    pub serial_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AdditionalItemProperty")]
    pub additional_item_property: ::std::vec::Vec<ItemPropertyType>,
    #[serde(default, rename = "LotIdentification")]
    pub lot_identification: ::core::option::Option<LotIdentificationType>,
}
pub type ItemLocationQuantity = ItemLocationQuantityType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemLocationQuantityType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "LeadTimeMeasure")]
    pub lead_time_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "TradingRestrictions")]
    pub trading_restrictions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ApplicableTerritoryAddress")]
    pub applicable_territory_address: ::std::vec::Vec<AddressType>,
    #[serde(default, rename = "Price")]
    pub price: ::core::option::Option<PriceType>,
    #[serde(default, rename = "DeliveryUnit")]
    pub delivery_unit: ::std::vec::Vec<DeliveryUnitType>,
    #[serde(default, rename = "ApplicableTaxCategory")]
    pub applicable_tax_category: ::std::vec::Vec<TaxCategoryType>,
    #[serde(default, rename = "Package")]
    pub package: ::core::option::Option<PackageType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "DependentPriceReference")]
    pub dependent_price_reference: ::core::option::Option<DependentPriceReferenceType>,
    #[serde(default, rename = "ApplicableDeliveryPeriod")]
    pub applicable_delivery_period: ::core::option::Option<PeriodType>,
}
pub type ItemManagementProfile = ItemManagementProfileType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemManagementProfileType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "FrozenPeriodDaysNumeric")]
    pub frozen_period_days_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "MinimumInventoryQuantity")]
    pub minimum_inventory_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MultipleOrderQuantity")]
    pub multiple_order_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "OrderIntervalDaysNumeric")]
    pub order_interval_days_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "ReplenishmentOwnerDescription")]
    pub replenishment_owner_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TargetServicePercent")]
    pub target_service_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "TargetInventoryQuantity")]
    pub target_inventory_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(rename = "EffectivePeriod")]
    pub effective_period: PeriodType,
    #[serde(rename = "Item")]
    pub item: ItemType,
    #[serde(default, rename = "ItemLocationQuantity")]
    pub item_location_quantity: ::core::option::Option<ItemLocationQuantityType>,
}
pub type ItemPriceExtension = PriceExtensionType;
pub type ItemProperty = ItemPropertyType;
pub type ItemPropertyGroup = ItemPropertyGroupType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemPropertyGroupType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ImportanceCode")]
    pub importance_code: ::core::option::Option<super::cct::CodeType>,
}
pub type ItemPropertyRange = ItemPropertyRangeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemPropertyRangeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "MinimumValue")]
    pub minimum_value: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "MaximumValue")]
    pub maximum_value: ::core::option::Option<super::cct::TextType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemPropertyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(rename = "Name")]
    pub name: super::cct::TextType,
    #[serde(default, rename = "NameCode")]
    pub name_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TestMethod")]
    pub test_method: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Value")]
    pub value: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ValueQuantity")]
    pub value_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "ValueQualifier")]
    pub value_qualifier: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ImportanceCode")]
    pub importance_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ListValue")]
    pub list_value: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "UsabilityPeriod")]
    pub usability_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "ItemPropertyGroup")]
    pub item_property_group: ::std::vec::Vec<ItemPropertyGroupType>,
    #[serde(default, rename = "RangeDimension")]
    pub range_dimension: ::core::option::Option<DimensionType>,
    #[serde(default, rename = "ItemPropertyRange")]
    pub item_property_range: ::core::option::Option<ItemPropertyRangeType>,
    #[serde(default, rename = "StandardPropertyIdentification")]
    pub standard_property_identification: ::core::option::Option<PropertyIdentificationType>,
    #[serde(default, rename = "SubItemProperty")]
    pub sub_item_property: ::std::vec::Vec<ItemPropertyType>,
}
pub type ItemSpecificationDocumentReference = DocumentReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ItemType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PackQuantity")]
    pub pack_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "PackSizeNumeric")]
    pub pack_size_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "CatalogueIndicator")]
    pub catalogue_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "Name")]
    pub name: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ItemTypeCode")]
    pub item_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "AdditionalInformation")]
    pub additional_information: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Keyword")]
    pub keyword: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "BrandName")]
    pub brand_name: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ModelName")]
    pub model_name: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "WarrantyInformation")]
    pub warranty_information: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "LifecycleStageCode")]
    pub lifecycle_stage_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "LifecycleStageDescription")]
    pub lifecycle_stage_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "BuyersItemIdentification")]
    pub buyers_item_identification: ::core::option::Option<ItemIdentificationType>,
    #[serde(default, rename = "SellersItemIdentification")]
    pub sellers_item_identification: ::core::option::Option<ItemIdentificationType>,
    #[serde(default, rename = "ManufacturersItemIdentification")]
    pub manufacturers_item_identification: ::std::vec::Vec<ItemIdentificationType>,
    #[serde(default, rename = "StandardItemIdentification")]
    pub standard_item_identification: ::core::option::Option<ItemIdentificationType>,
    #[serde(default, rename = "CatalogueItemIdentification")]
    pub catalogue_item_identification: ::core::option::Option<ItemIdentificationType>,
    #[serde(default, rename = "AdditionalItemIdentification")]
    pub additional_item_identification: ::std::vec::Vec<ItemIdentificationType>,
    #[serde(default, rename = "CatalogueDocumentReference")]
    pub catalogue_document_reference: ::core::option::Option<DocumentReferenceType>,
    #[serde(default, rename = "ItemSpecificationDocumentReference")]
    pub item_specification_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "OriginCountry")]
    pub origin_country: ::core::option::Option<CountryType>,
    #[serde(default, rename = "CommodityClassification")]
    pub commodity_classification: ::std::vec::Vec<CommodityClassificationType>,
    #[serde(default, rename = "TransactionConditions")]
    pub transaction_conditions: ::std::vec::Vec<TransactionConditionsType>,
    #[serde(default, rename = "HazardousItem")]
    pub hazardous_item: ::std::vec::Vec<HazardousItemType>,
    #[serde(default, rename = "ClassifiedTaxCategory")]
    pub classified_tax_category: ::std::vec::Vec<TaxCategoryType>,
    #[serde(default, rename = "AdditionalItemProperty")]
    pub additional_item_property: ::std::vec::Vec<ItemPropertyType>,
    #[serde(default, rename = "ManufacturerParty")]
    pub manufacturer_party: ::std::vec::Vec<PartyType>,
    #[serde(default, rename = "InformationContentProviderParty")]
    pub information_content_provider_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "OriginAddress")]
    pub origin_address: ::std::vec::Vec<AddressType>,
    #[serde(default, rename = "ItemInstance")]
    pub item_instance: ::std::vec::Vec<ItemInstanceType>,
    #[serde(default, rename = "Certificate")]
    pub certificate: ::std::vec::Vec<CertificateType>,
    #[serde(default, rename = "EnvironmentalCertificate")]
    pub environmental_certificate: ::std::vec::Vec<CertificateType>,
    #[serde(default, rename = "Dimension")]
    pub dimension: ::std::vec::Vec<DimensionType>,
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: ::std::vec::Vec<EnvironmentalEmissionType>,
    #[serde(default, rename = "CircularityProfile")]
    pub circularity_profile: ::core::option::Option<CircularityProfileType>,
}
pub type JurisdictionRegionAddress = AddressType;
pub type KeywordItemProperty = ItemPropertyType;
pub type Language = LanguageType;
#[derive(Debug, Deserialize, Serialize)]
pub struct LanguageType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "LocaleCode")]
    pub locale_code: ::core::option::Option<super::cct::CodeType>,
}
pub type LastExitPortLocation = LocationType;
pub type LegalAuthorityParty = PartyType;
pub type LegalContact = ContactType;
pub type LegalDocumentReference = DocumentReferenceType;
pub type LegalMonetaryTotal = MonetaryTotalType;
pub type Legislation = LegislationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct LegislationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Title")]
    pub title: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "JurisdictionLevel")]
    pub jurisdiction_level: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Article")]
    pub article: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "URI")]
    pub uri: ::std::vec::Vec<super::cct::IdentifierType>,
    #[serde(default, rename = "Language")]
    pub language: ::std::vec::Vec<LanguageType>,
    #[serde(default, rename = "JurisdictionRegionAddress")]
    pub jurisdiction_region_address: ::std::vec::Vec<AddressType>,
}
pub type LineItem = LineItemType;
#[derive(Debug, Deserialize, Serialize)]
pub struct LineItemType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "SalesOrderID")]
    pub sales_order_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "LineStatusCode")]
    pub line_status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TotalTaxAmount")]
    pub total_tax_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MinimumBackorderQuantity")]
    pub minimum_backorder_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumBackorderQuantity")]
    pub maximum_backorder_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "InspectionMethodCode")]
    pub inspection_method_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PartialDeliveryIndicator")]
    pub partial_delivery_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "BackOrderAllowedIndicator")]
    pub back_order_allowed_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "WarrantyInformation")]
    pub warranty_information: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::std::vec::Vec<DeliveryType>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: ::core::option::Option<DeliveryTermsType>,
    #[serde(default, rename = "OriginatorParty")]
    pub originator_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<PartyType>,
    #[serde(default, rename = "OrderedShipment")]
    pub ordered_shipment: ::std::vec::Vec<OrderedShipmentType>,
    #[serde(default, rename = "PricingReference")]
    pub pricing_reference: ::core::option::Option<PricingReferenceType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "Price")]
    pub price: ::core::option::Option<PriceType>,
    #[serde(rename = "Item")]
    pub item: ItemType,
    #[serde(default, rename = "SubLineItem")]
    pub sub_line_item: ::std::vec::Vec<LineItemType>,
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "WarrantyParty")]
    pub warranty_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<TaxTotalType>,
    #[serde(default, rename = "ItemPriceExtension")]
    pub item_price_extension: ::core::option::Option<PriceExtensionType>,
    #[serde(default, rename = "LineReference")]
    pub line_reference: ::std::vec::Vec<LineReferenceType>,
}
pub type LineReference = LineReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct LineReferenceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "LineID")]
    pub line_id: super::cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "LineStatusCode")]
    pub line_status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::core::option::Option<DocumentReferenceType>,
}
pub type LineResponse = LineResponseType;
#[derive(Debug, Deserialize, Serialize)]
pub struct LineResponseType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "LineReference")]
    pub line_reference: LineReferenceType,
    #[serde(default, rename = "Response")]
    pub response: ::std::vec::Vec<ResponseType>,
}
pub type LineValidityPeriod = PeriodType;
pub type LoadedHazardousItem = HazardousItemType;
pub type LoadingLocation = LocationType;
pub type LoadingPortLocation = LocationType;
pub type LoadingProofParty = PartyType;
pub type LoadingTransportEvent = TransportEventType;
pub type Location = LocationType;
pub type LocationAddress = AddressType;
pub type LocationCoordinate = LocationCoordinateType;
#[derive(Debug, Deserialize, Serialize)]
pub struct LocationCoordinateType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "CoordinateSystemCode")]
    pub coordinate_system_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "LatitudeDegreesMeasure")]
    pub latitude_degrees_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "LatitudeMinutesMeasure")]
    pub latitude_minutes_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "LatitudeDirectionCode")]
    pub latitude_direction_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "LongitudeDegreesMeasure")]
    pub longitude_degrees_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "LongitudeMinutesMeasure")]
    pub longitude_minutes_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "LongitudeDirectionCode")]
    pub longitude_direction_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "AltitudeMeasure")]
    pub altitude_measure: ::core::option::Option<super::cct::MeasureType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct LocationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Conditions")]
    pub conditions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CountrySubentity")]
    pub country_subentity: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "CountrySubentityCode")]
    pub country_subentity_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "LocationTypeCode")]
    pub location_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "InformationURI")]
    pub information_uri: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::std::vec::Vec<PeriodType>,
    #[serde(default, rename = "Address")]
    pub address: ::core::option::Option<AddressType>,
    #[serde(default, rename = "Storage")]
    pub storage: ::core::option::Option<StorageType>,
    #[serde(default, rename = "SubsidiaryLocation")]
    pub subsidiary_location: ::std::vec::Vec<LocationType>,
    #[serde(default, rename = "LocationCoordinate")]
    pub location_coordinate: ::std::vec::Vec<LocationCoordinateType>,
}
pub type LogisticsOperatorParty = PartyType;
pub type LotDistribution = LotDistributionType;
#[derive(Debug, Deserialize, Serialize)]
pub struct LotDistributionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "MaximumLotsAwardedNumeric")]
    pub maximum_lots_awarded_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "MaximumLotsSubmittedNumeric")]
    pub maximum_lots_submitted_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "GroupingLots")]
    pub grouping_lots: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "LotsGroup")]
    pub lots_group: ::std::vec::Vec<LotsGroupType>,
}
pub type LotIdentification = LotIdentificationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct LotIdentificationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "LotNumberID")]
    pub lot_number_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ExpiryDate")]
    pub expiry_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "AdditionalItemProperty")]
    pub additional_item_property: ::std::vec::Vec<ItemPropertyType>,
}
pub type LotsGroup = LotsGroupType;
#[derive(Debug, Deserialize, Serialize)]
pub struct LotsGroupType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "LotsGroupID")]
    pub lots_group_id: super::cct::IdentifierType,
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: ::std::vec::Vec<ProcurementProjectLotReferenceType>,
}
pub type MainCarriageShipmentStage = ShipmentStageType;
pub type MainCommodityClassification = CommodityClassificationType;
pub type MainOnAccountPayment = OnAccountPaymentType;
pub type MainPeriod = PeriodType;
pub type MainQualifyingParty = QualifyingPartyType;
pub type MainTransportationService = TransportationServiceType;
pub type MandateDocumentReference = DocumentReferenceType;
pub type ManufacturerParty = PartyType;
pub type ManufacturersItemIdentification = ItemIdentificationType;
pub type MaritimeHealthDeclaration = MaritimeHealthDeclarationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct MaritimeHealthDeclarationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "InfectiousDiseaseCaseOnBoardIndicator")]
    pub infectious_disease_case_on_board_indicator:
        ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "MoreIllThanExpectedIndicator")]
    pub more_ill_than_expected_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "MedicalPractitionerConsultedIndicator")]
    pub medical_practitioner_consulted_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "StowawaysFoundOnBoardIndicator")]
    pub stowaways_found_on_board_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "SickAnimalOnBoardIndicator")]
    pub sick_animal_on_board_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "FumigatedCargoTransportIndicator")]
    pub fumigated_cargo_transport_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "SanitaryMeasuresAppliedIndicator")]
    pub sanitary_measures_applied_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ValidSanitationCertificateOnBoardIndicator")]
    pub valid_sanitation_certificate_on_board_indicator:
        ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ReinspectionRequiredIndicator")]
    pub reinspection_required_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "TotalDeadPersonQuantity")]
    pub total_dead_person_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "TotalIllPersonQuantity")]
    pub total_ill_person_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "SickAnimalDescription")]
    pub sick_animal_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "StowawayDescription")]
    pub stowaway_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "LastDrinkingWaterAnalysisDate")]
    pub last_drinking_water_analysis_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "WHOAffectedAreaVisit")]
    pub who_affected_area_visit: ::std::vec::Vec<WhoAffectedAreaVisitType>,
    #[serde(default, rename = "PersonnelHealthIncident")]
    pub personnel_health_incident: ::std::vec::Vec<PersonnelHealthIncidentType>,
    #[serde(default, rename = "SanitaryMeasure")]
    pub sanitary_measure: ::std::vec::Vec<SanitaryMeasureType>,
    #[serde(default, rename = "PlaceOfReportLocation")]
    pub place_of_report_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "MedicalCertificate")]
    pub medical_certificate: ::core::option::Option<CertificateType>,
    #[serde(default, rename = "ShipSanitationControlCertificate")]
    pub ship_sanitation_control_certificate: ::core::option::Option<CertificateType>,
    #[serde(default, rename = "ShipSanitationControlExemptionDocumentReference")]
    pub ship_sanitation_control_exemption_document_reference:
        ::std::vec::Vec<DocumentReferenceType>,
}
pub type MaritimeTransport = MaritimeTransportType;
#[derive(Debug, Deserialize, Serialize)]
pub struct MaritimeTransportType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "VesselID")]
    pub vessel_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "VesselName")]
    pub vessel_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "RadioCallSignID")]
    pub radio_call_sign_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "MMSIRegistrationID")]
    pub mmsi_registration_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ShipsRequirements")]
    pub ships_requirements: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "GrossTonnageMeasure")]
    pub gross_tonnage_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetTonnageMeasure")]
    pub net_tonnage_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "SegregatedBallastMeasure")]
    pub segregated_ballast_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "ShipConfigurationCode")]
    pub ship_configuration_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "INFShipClassCode")]
    pub inf_ship_class_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "AntennaLocus")]
    pub antenna_locus: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "RegistryCertificateDocumentReference")]
    pub registry_certificate_document_reference: ::core::option::Option<DocumentReferenceType>,
    #[serde(default, rename = "RegistryPortLocation")]
    pub registry_port_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "VesselDynamics")]
    pub vessel_dynamics: ::core::option::Option<VesselDynamicsType>,
}
pub type MaritimeWaste = MaritimeWasteType;
#[derive(Debug, Deserialize, Serialize)]
pub struct MaritimeWasteType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "WasteTypeCode")]
    pub waste_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ToBeDeliveredMeasure")]
    pub to_be_delivered_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "RetainedOnBoardMeasure")]
    pub retained_on_board_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "MaxDedicatedStorageCapacityMeasure")]
    pub max_dedicated_storage_capacity_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "EstimatedGeneratedUntilNextPortMeasure")]
    pub estimated_generated_until_next_port_measure:
        ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "RemainingWasteDeliveryPortLocation")]
    pub remaining_waste_delivery_port_location: ::std::vec::Vec<LocationType>,
}
pub type MasterPerson = PersonType;
pub type MaximumDeliveryUnit = DeliveryUnitType;
pub type MaximumTemperature = TemperatureType;
pub type MeasurementDimension = DimensionType;
pub type MeasurementFromLocation = LocationType;
pub type MeasurementPeriod = PeriodType;
pub type MeasurementToLocation = LocationType;
pub type MediationParty = PartyType;
pub type MedicalCertificate = CertificateType;
pub type MessageDelivery = MessageDeliveryType;
#[derive(Debug, Deserialize, Serialize)]
pub struct MessageDeliveryType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ProtocolID")]
    pub protocol_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "EnvelopeTypeCode")]
    pub envelope_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "EndpointURI")]
    pub endpoint_uri: ::core::option::Option<super::cct::IdentifierType>,
}
pub type Meter = MeterType;
pub type MeterProperty = MeterPropertyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct MeterPropertyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "NameCode")]
    pub name_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Value")]
    pub value: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ValueQuantity")]
    pub value_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "ValueQualifier")]
    pub value_qualifier: ::std::vec::Vec<super::cct::TextType>,
}
pub type MeterReading = MeterReadingType;
#[derive(Debug, Deserialize, Serialize)]
pub struct MeterReadingType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "MeterReadingType")]
    pub meter_reading_type: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "MeterReadingTypeCode")]
    pub meter_reading_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(rename = "PreviousMeterReadingDate")]
    pub previous_meter_reading_date: super::udt::DateTimeType,
    #[serde(rename = "PreviousMeterQuantity")]
    pub previous_meter_quantity: super::cct::QuantityType,
    #[serde(rename = "LatestMeterReadingDate")]
    pub latest_meter_reading_date: super::udt::DateTimeType,
    #[serde(rename = "LatestMeterQuantity")]
    pub latest_meter_quantity: super::cct::QuantityType,
    #[serde(default, rename = "PreviousMeterReadingMethod")]
    pub previous_meter_reading_method: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "PreviousMeterReadingMethodCode")]
    pub previous_meter_reading_method_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "LatestMeterReadingMethod")]
    pub latest_meter_reading_method: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "LatestMeterReadingMethodCode")]
    pub latest_meter_reading_method_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "MeterReadingComments")]
    pub meter_reading_comments: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "DeliveredQuantity")]
    pub delivered_quantity: super::cct::QuantityType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct MeterType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "MeterNumber")]
    pub meter_number: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "MeterName")]
    pub meter_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "MeterConstant")]
    pub meter_constant: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "MeterConstantCode")]
    pub meter_constant_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TotalDeliveredQuantity")]
    pub total_delivered_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MeterReading")]
    pub meter_reading: ::std::vec::Vec<MeterReadingType>,
    #[serde(default, rename = "MeterProperty")]
    pub meter_property: ::std::vec::Vec<MeterPropertyType>,
}
pub type MinimumDeliveryUnit = DeliveryUnitType;
pub type MinimumTemperature = TemperatureType;
pub type MinutesDocumentReference = DocumentReferenceType;
pub type MiscellaneousEvent = MiscellaneousEventType;
#[derive(Debug, Deserialize, Serialize)]
pub struct MiscellaneousEventType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "MiscellaneousEventTypeCode")]
    pub miscellaneous_event_type_code: super::cct::CodeType,
    #[serde(default, rename = "EventLineItem")]
    pub event_line_item: ::std::vec::Vec<EventLineItemType>,
}
pub type MonetaryTotal = MonetaryTotalType;
#[derive(Debug, Deserialize, Serialize)]
pub struct MonetaryTotalType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxExclusiveAmount")]
    pub tax_exclusive_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusiveAmount")]
    pub tax_inclusive_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "AllowanceTotalAmount")]
    pub allowance_total_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "AllowanceTotalTaxInclusiveAmount")]
    pub allowance_total_tax_inclusive_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "ChargeTotalAmount")]
    pub charge_total_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "ChargeTotalTaxInclusiveAmount")]
    pub charge_total_tax_inclusive_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "WithholdingTaxTotalAmount")]
    pub withholding_tax_total_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "PrepaidAmount")]
    pub prepaid_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "PayableRoundingAmount")]
    pub payable_rounding_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(rename = "PayableAmount")]
    pub payable_amount: super::cct::AmountType,
    #[serde(default, rename = "PayableAlternativeAmount")]
    pub payable_alternative_amount: ::core::option::Option<super::cct::AmountType>,
}
pub type MortgageHolderParty = PartyType;
pub type NominationPeriod = PeriodType;
pub type NotaryParty = PartyType;
pub type NoticeDocumentReference = DocumentReferenceType;
pub type NoticeSubType = NoticeSubTypeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct NoticeSubTypeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "SubTypeCode")]
    pub sub_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "SubTypeDescription")]
    pub sub_type_description: ::std::vec::Vec<super::cct::TextType>,
}
pub type NotificationLocation = LocationType;
pub type NotificationPeriod = PeriodType;
pub type NotificationRequirement = NotificationRequirementType;
#[derive(Debug, Deserialize, Serialize)]
pub struct NotificationRequirementType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "NotificationTypeCode")]
    pub notification_type_code: super::cct::CodeType,
    #[serde(default, rename = "PostEventNotificationDurationMeasure")]
    pub post_event_notification_duration_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "PreEventNotificationDurationMeasure")]
    pub pre_event_notification_duration_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "NotifyParty")]
    pub notify_party: ::std::vec::Vec<PartyType>,
    #[serde(default, rename = "NotificationPeriod")]
    pub notification_period: ::std::vec::Vec<PeriodType>,
    #[serde(default, rename = "NotificationLocation")]
    pub notification_location: ::std::vec::Vec<LocationType>,
}
pub type NotifierParty = PartyType;
pub type NotifyParty = PartyType;
pub type OccurenceLocation = LocationType;
pub type OccurrenceLocation = LocationType;
pub type OfferedItemLocationQuantity = ItemLocationQuantityType;
pub type OfficeOfDepartureLocation = LocationType;
pub type OfficeOfDestinationLocation = LocationType;
pub type OfficeOfEntryLocation = LocationType;
pub type OfficeOfExitLocation = LocationType;
pub type OfficeOfExportLocation = LocationType;
pub type OfficeOfImportLocation = LocationType;
pub type OfficeOfSubSequentiallyEntryLocation = LocationType;
pub type OfficeOfTransitLocation = LocationType;
pub type OnAccountPayment = OnAccountPaymentType;
#[derive(Debug, Deserialize, Serialize)]
pub struct OnAccountPaymentType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "EstimatedConsumedQuantity")]
    pub estimated_consumed_quantity: super::cct::QuantityType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<PaymentTermsType>,
}
pub type OnCarriageShipmentStage = ShipmentStageType;
pub type OpenTenderEvent = EventType;
pub type OperatingParty = PartyType;
pub type OperationType = OperationTypeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct OperationTypeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Code")]
    pub code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
}
pub type OptionValidityPeriod = PeriodType;
pub type OptionalTakeoverTransportEvent = TransportEventType;
pub type OrderChangeDocumentReference = DocumentReferenceType;
pub type OrderDocumentReference = DocumentReferenceType;
pub type OrderLine = OrderLineType;
pub type OrderLineReference = OrderLineReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderLineReferenceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "LineID")]
    pub line_id: super::cct::IdentifierType,
    #[serde(default, rename = "SalesOrderLineID")]
    pub sales_order_line_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "LineStatusCode")]
    pub line_status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "OrderReference")]
    pub order_reference: ::core::option::Option<OrderReferenceType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "SubstitutionStatusCode")]
    pub substitution_status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "LineItem")]
    pub line_item: LineItemType,
    #[serde(default, rename = "SellerProposedSubstituteLineItem")]
    pub seller_proposed_substitute_line_item: ::std::vec::Vec<LineItemType>,
    #[serde(default, rename = "SellerSubstitutedLineItem")]
    pub seller_substituted_line_item: ::std::vec::Vec<LineItemType>,
    #[serde(default, rename = "BuyerProposedSubstituteLineItem")]
    pub buyer_proposed_substitute_line_item: ::std::vec::Vec<LineItemType>,
    #[serde(default, rename = "CatalogueLineReference")]
    pub catalogue_line_reference: ::core::option::Option<LineReferenceType>,
    #[serde(default, rename = "QuotationLineReference")]
    pub quotation_line_reference: ::core::option::Option<LineReferenceType>,
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: ::std::vec::Vec<OrderLineReferenceType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
}
pub type OrderReference = OrderReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderReferenceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "SalesOrderID")]
    pub sales_order_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CopyIndicator")]
    pub copy_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "CustomerReference")]
    pub customer_reference: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "OrderTypeCode")]
    pub order_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::core::option::Option<DocumentReferenceType>,
}
pub type OrderedShipment = OrderedShipmentType;
#[derive(Debug, Deserialize, Serialize)]
pub struct OrderedShipmentType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "Shipment")]
    pub shipment: ShipmentType,
    #[serde(default, rename = "Package")]
    pub package: ::std::vec::Vec<PackageType>,
}
pub type OriginAddress = AddressType;
pub type OriginCountry = CountryType;
pub type OriginalDepartureCountry = CountryType;
pub type OriginalDespatchParty = PartyType;
pub type OriginalDespatchTransportationService = TransportationServiceType;
pub type OriginalDocumentReference = DocumentReferenceType;
pub type OriginalItemLocationQuantity = ItemLocationQuantityType;
pub type OriginatorCustomerParty = CustomerPartyType;
pub type OriginatorDocumentReference = DocumentReferenceType;
pub type OriginatorParty = PartyType;
pub type OtherCommunication = CommunicationType;
pub type OwnerParty = PartyType;
pub type Package = PackageType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PackageType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "ReturnableMaterialIndicator")]
    pub returnable_material_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "PackageLevelCode")]
    pub package_level_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PackagingTypeCode")]
    pub packaging_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PackagingType")]
    pub packaging_type: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PackingMaterial")]
    pub packing_material: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TraceID")]
    pub trace_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ContainedPackage")]
    pub contained_package: ::std::vec::Vec<PackageType>,
    #[serde(default, rename = "ContainingTransportEquipment")]
    pub containing_transport_equipment: ::core::option::Option<TransportEquipmentType>,
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: ::std::vec::Vec<GoodsItemType>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: ::std::vec::Vec<DimensionType>,
    #[serde(default, rename = "DeliveryUnit")]
    pub delivery_unit: ::std::vec::Vec<DeliveryUnitType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::core::option::Option<DeliveryType>,
    #[serde(default, rename = "Pickup")]
    pub pickup: ::core::option::Option<PickupType>,
    #[serde(default, rename = "Despatch")]
    pub despatch: ::core::option::Option<DespatchType>,
    #[serde(default, rename = "Status")]
    pub status: ::std::vec::Vec<StatusType>,
}
pub type PackagedTransportHandlingUnit = TransportHandlingUnitType;
pub type PalletSpaceMeasurementDimension = DimensionType;
pub type ParentDocumentLineReference = LineReferenceType;
pub type ParentDocumentReference = DocumentReferenceType;
pub type ParticipantParty = ParticipantPartyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ParticipantPartyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "InitiatingPartyIndicator")]
    pub initiating_party_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "PrivatePartyIndicator")]
    pub private_party_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "PublicPartyIndicator")]
    pub public_party_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ServiceProviderPartyIndicator")]
    pub service_provider_party_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(rename = "Party")]
    pub party: PartyType,
    #[serde(default, rename = "LegalContact")]
    pub legal_contact: ::core::option::Option<ContactType>,
    #[serde(default, rename = "TechnicalContact")]
    pub technical_contact: ::core::option::Option<ContactType>,
    #[serde(default, rename = "SupportContact")]
    pub support_contact: ::core::option::Option<ContactType>,
    #[serde(default, rename = "CommercialContact")]
    pub commercial_contact: ::core::option::Option<ContactType>,
}
pub type ParticipatingLocationsLocation = LocationType;
pub type ParticipationInvitationPeriod = PeriodType;
pub type ParticipationRequestReceptionPeriod = PeriodType;
pub type Party = PartyType;
pub type PartyAuthorization = AuthorizationType;
pub type PartyGroup = PartyGroupType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PartyGroupType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "GroupTypeCode")]
    pub group_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "GroupType")]
    pub group_type: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Party")]
    pub party: ::std::vec::Vec<PartyType>,
}
pub type PartyIdentification = PartyIdentificationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PartyIdentificationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
}
pub type PartyLegalEntity = PartyLegalEntityType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PartyLegalEntityType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "RegistrationName")]
    pub registration_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "CompanyID")]
    pub company_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "RegistrationDate")]
    pub registration_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "RegistrationExpirationDate")]
    pub registration_expiration_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "CompanyLegalFormCode")]
    pub company_legal_form_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CompanyLegalForm")]
    pub company_legal_form: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "SoleProprietorshipIndicator")]
    pub sole_proprietorship_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "EntitySizeCode")]
    pub entity_size_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CompanyLiquidationStatusCode")]
    pub company_liquidation_status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CorporateStockAmount")]
    pub corporate_stock_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "FullyPaidSharesIndicator")]
    pub fully_paid_shares_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "RegistrationAddress")]
    pub registration_address: ::core::option::Option<AddressType>,
    #[serde(default, rename = "CorporateRegistrationScheme")]
    pub corporate_registration_scheme: ::core::option::Option<CorporateRegistrationSchemeType>,
    #[serde(default, rename = "HeadOfficeParty")]
    pub head_office_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "ShareholderParty")]
    pub shareholder_party: ::std::vec::Vec<ShareholderPartyType>,
    #[serde(default, rename = "SecurityListing")]
    pub security_listing: ::std::vec::Vec<SecurityListingType>,
}
pub type PartyName = PartyNameType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PartyNameType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "Name")]
    pub name: super::cct::TextType,
}
pub type PartyTaxScheme = PartyTaxSchemeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PartyTaxSchemeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "RegistrationName")]
    pub registration_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "CompanyID")]
    pub company_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TaxLevelCode")]
    pub tax_level_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ExemptionReasonCode")]
    pub exemption_reason_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ExemptionReason")]
    pub exemption_reason: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "RegistrationAddress")]
    pub registration_address: ::core::option::Option<AddressType>,
    #[serde(rename = "TaxScheme")]
    pub tax_scheme: TaxSchemeType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PartyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "MarkCareIndicator")]
    pub mark_care_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "MarkAttentionIndicator")]
    pub mark_attention_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "WebsiteURI")]
    pub website_uri: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "LogoReferenceID")]
    pub logo_reference_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "EndpointID")]
    pub endpoint_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IndustryClassificationCode")]
    pub industry_classification_code: ::std::vec::Vec<super::cct::CodeType>,
    #[serde(default, rename = "PartyIdentification")]
    pub party_identification: ::std::vec::Vec<PartyIdentificationType>,
    #[serde(default, rename = "AdditionalPartyIdentification")]
    pub additional_party_identification: ::std::vec::Vec<PartyIdentificationType>,
    #[serde(default, rename = "PartyName")]
    pub party_name: ::std::vec::Vec<PartyNameType>,
    #[serde(default, rename = "TradePartyName")]
    pub trade_party_name: ::std::vec::Vec<PartyNameType>,
    #[serde(default, rename = "Language")]
    pub language: ::core::option::Option<LanguageType>,
    #[serde(default, rename = "PostalAddress")]
    pub postal_address: ::core::option::Option<AddressType>,
    #[serde(default, rename = "PhysicalLocation")]
    pub physical_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "PartyTaxScheme")]
    pub party_tax_scheme: ::std::vec::Vec<PartyTaxSchemeType>,
    #[serde(default, rename = "PartyLegalEntity")]
    pub party_legal_entity: ::std::vec::Vec<PartyLegalEntityType>,
    #[serde(default, rename = "Contact")]
    pub contact: ::core::option::Option<ContactType>,
    #[serde(default, rename = "Person")]
    pub person: ::std::vec::Vec<PersonType>,
    #[serde(default, rename = "AgentParty")]
    pub agent_party: ::core::option::Option<::std::boxed::Box<PartyType>>,
    #[serde(default, rename = "ServiceProviderParty")]
    pub service_provider_party: ::std::vec::Vec<ServiceProviderPartyType>,
    #[serde(default, rename = "PowerOfAttorney")]
    pub power_of_attorney: ::std::vec::Vec<PowerOfAttorneyType>,
    #[serde(default, rename = "PartyAuthorization")]
    pub party_authorization: ::std::vec::Vec<AuthorizationType>,
    #[serde(default, rename = "FinancialAccount")]
    pub financial_account: ::core::option::Option<FinancialAccountType>,
    #[serde(default, rename = "AdditionalWebSite")]
    pub additional_web_site: ::std::vec::Vec<WebSiteType>,
    #[serde(default, rename = "SocialMediaProfile")]
    pub social_media_profile: ::std::vec::Vec<SocialMediaProfileType>,
    #[serde(default, rename = "ElectronicAddress")]
    pub electronic_address: ::std::vec::Vec<ElectronicAddressType>,
}
pub type PassengerPerson = PersonType;
pub type PayeeFinancialAccount = FinancialAccountType;
pub type PayeeParty = PartyType;
pub type PayerFinancialAccount = FinancialAccountType;
pub type PayerParty = PartyType;
pub type Payment = PaymentType;
pub type PaymentAlternativeExchangeRate = ExchangeRateType;
pub type PaymentExchangeRate = ExchangeRateType;
pub type PaymentInstructionAttachment = AttachmentType;
pub type PaymentMandate = PaymentMandateType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentMandateType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "MandateTypeCode")]
    pub mandate_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "MaximumPaymentInstructionsNumeric")]
    pub maximum_payment_instructions_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "MaximumPaidAmount")]
    pub maximum_paid_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "SignatureID")]
    pub signature_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PayerParty")]
    pub payer_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "PayerFinancialAccount")]
    pub payer_financial_account: ::core::option::Option<FinancialAccountType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "PaymentReversalPeriod")]
    pub payment_reversal_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "Clause")]
    pub clause: ::std::vec::Vec<ClauseType>,
}
pub type PaymentMeans = PaymentMeansType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentMeansType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(rename = "PaymentMeansCode")]
    pub payment_means_code: super::cct::CodeType,
    #[serde(default, rename = "PaymentMeansDescription")]
    pub payment_means_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PaymentDueDate")]
    pub payment_due_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "PaymentChannelCode")]
    pub payment_channel_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PaymentRailID")]
    pub payment_rail_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PaymentPlatformID")]
    pub payment_platform_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "InstructionID")]
    pub instruction_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "InstructionNote")]
    pub instruction_note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PaymentID")]
    pub payment_id: ::std::vec::Vec<super::cct::IdentifierType>,
    #[serde(default, rename = "ChargeBearerCode")]
    pub charge_bearer_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ServiceLevelCode")]
    pub service_level_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CardAccount")]
    pub card_account: ::std::vec::Vec<CardAccountType>,
    #[serde(default, rename = "PayerFinancialAccount")]
    pub payer_financial_account: ::core::option::Option<FinancialAccountType>,
    #[serde(default, rename = "PayeeFinancialAccount")]
    pub payee_financial_account: ::core::option::Option<FinancialAccountType>,
    #[serde(default, rename = "CreditAccount")]
    pub credit_account: ::core::option::Option<CreditAccountType>,
    #[serde(default, rename = "PaymentMandate")]
    pub payment_mandate: ::core::option::Option<PaymentMandateType>,
    #[serde(default, rename = "TradeFinancing")]
    pub trade_financing: ::core::option::Option<TradeFinancingType>,
    #[serde(default, rename = "RemittanceDocumentDistribution")]
    pub remittance_document_distribution: ::std::vec::Vec<DocumentDistributionType>,
    #[serde(default, rename = "PaymentInstructionAttachment")]
    pub payment_instruction_attachment: ::core::option::Option<AttachmentType>,
}
pub type PaymentReversalPeriod = PeriodType;
pub type PaymentTerms = PaymentTermsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentTermsType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PaymentMeansID")]
    pub payment_means_id: ::std::vec::Vec<super::cct::IdentifierType>,
    #[serde(default, rename = "PrepaidPaymentReferenceID")]
    pub prepaid_payment_reference_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ReferenceEventCode")]
    pub reference_event_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "SettlementDiscountPercent")]
    pub settlement_discount_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "PenaltySurchargePercent")]
    pub penalty_surcharge_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "PaymentPercent")]
    pub payment_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "Amount")]
    pub amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "SettlementDiscountAmount")]
    pub settlement_discount_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "PenaltyAmount")]
    pub penalty_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "PaymentTermsDetailsURI")]
    pub payment_terms_details_uri: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PaymentDueDate")]
    pub payment_due_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "InstallmentDueDate")]
    pub installment_due_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "InvoicingPartyReference")]
    pub invoicing_party_reference: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "SettlementPeriod")]
    pub settlement_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "PenaltyPeriod")]
    pub penalty_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "PenaltyInterestRate")]
    pub penalty_interest_rate: ::core::option::Option<InterestRateType>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: ::core::option::Option<ExchangeRateType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::core::option::Option<PeriodType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PaymentType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PaidAmount")]
    pub paid_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "PaidCashAmount")]
    pub paid_cash_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "CashChangeAmount")]
    pub cash_change_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "ReceivedDate")]
    pub received_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "PaidDate")]
    pub paid_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "PaidTime")]
    pub paid_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "InstructionID")]
    pub instruction_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "MerchantID")]
    pub merchant_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AuthorizationID")]
    pub authorization_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TransactionID")]
    pub transaction_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PaymentTerminalID")]
    pub payment_terminal_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "StatusCode")]
    pub status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: ::core::option::Option<ExchangeRateType>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: ::std::vec::Vec<BillingReferenceType>,
    #[serde(default, rename = "RemittanceDocumentReference")]
    pub remittance_document_reference: ::core::option::Option<DocumentReferenceType>,
}
pub type PenaltyClause = ClauseType;
pub type PenaltyInterestRate = InterestRateType;
pub type PenaltyPaymentTerms = PaymentTermsType;
pub type PenaltyPeriod = PeriodType;
pub type PerformanceDataLine = PerformanceDataLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PerformanceDataLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "PerformanceValueQuantity")]
    pub performance_value_quantity: super::cct::QuantityType,
    #[serde(rename = "PerformanceMetricTypeCode")]
    pub performance_metric_type_code: super::cct::CodeType,
    #[serde(default, rename = "Period")]
    pub period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "Item")]
    pub item: ::core::option::Option<ItemType>,
}
pub type PerformingCarrierParty = PartyType;
pub type PerformingParty = PartyType;
pub type Period = PeriodType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PeriodType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "StartDate")]
    pub start_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "StartTime")]
    pub start_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EndDate")]
    pub end_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EndTime")]
    pub end_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "DurationMeasure")]
    pub duration_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "DescriptionCode")]
    pub description_code: ::std::vec::Vec<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
}
pub type Person = PersonType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PersonType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "FirstName")]
    pub first_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "FamilyName")]
    pub family_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Title")]
    pub title: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "MiddleName")]
    pub middle_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "OtherName")]
    pub other_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "NameSuffix")]
    pub name_suffix: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "JobTitle")]
    pub job_title: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "NationalityID")]
    pub nationality_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "NationalID")]
    pub national_id: ::std::vec::Vec<super::cct::IdentifierType>,
    #[serde(default, rename = "NationalityCode")]
    pub nationality_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "GenderCode")]
    pub gender_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "BirthDate")]
    pub birth_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "BirthplaceName")]
    pub birthplace_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "OrganizationDepartment")]
    pub organization_department: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "RoleCode")]
    pub role_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "BirthplaceLocation")]
    pub birthplace_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "CitizenshipCountry")]
    pub citizenship_country: ::std::vec::Vec<CountryType>,
    #[serde(default, rename = "Contact")]
    pub contact: ::core::option::Option<ContactType>,
    #[serde(default, rename = "FinancialAccount")]
    pub financial_account: ::core::option::Option<FinancialAccountType>,
    #[serde(default, rename = "IdentityDocumentReference")]
    pub identity_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "ResidenceAddress")]
    pub residence_address: ::core::option::Option<AddressType>,
}
pub type PersonnelHealthIncident = PersonnelHealthIncidentType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PersonnelHealthIncidentType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "JoinedShipDate")]
    pub joined_ship_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "NatureOfIllnessDescription")]
    pub nature_of_illness_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "OnsetDate")]
    pub onset_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ReportedToMedicalOfficerIndicator")]
    pub reported_to_medical_officer_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "GivenTreatmentDescription")]
    pub given_treatment_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "StillIllIndicator")]
    pub still_ill_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "DiedIndicator")]
    pub died_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "StillOnBoardIndicator")]
    pub still_on_board_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "EvacuatedIndicator")]
    pub evacuated_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "BuriedAtSeaIndicator")]
    pub buried_at_sea_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Person")]
    pub person: ::core::option::Option<PersonType>,
}
pub type PhysicalAttribute = PhysicalAttributeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PhysicalAttributeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "AttributeID")]
    pub attribute_id: super::cct::IdentifierType,
    #[serde(default, rename = "PositionCode")]
    pub position_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "DescriptionCode")]
    pub description_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
}
pub type PhysicalLocation = LocationType;
pub type Pickup = PickupType;
pub type PickupLocation = LocationType;
pub type PickupParty = PartyType;
pub type PickupTransportEvent = TransportEventType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PickupType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ActualPickupDate")]
    pub actual_pickup_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ActualPickupTime")]
    pub actual_pickup_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EarliestPickupDate")]
    pub earliest_pickup_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EarliestPickupTime")]
    pub earliest_pickup_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "LatestPickupDate")]
    pub latest_pickup_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "LatestPickupTime")]
    pub latest_pickup_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "PickupLocation")]
    pub pickup_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "PickupParty")]
    pub pickup_party: ::core::option::Option<PartyType>,
}
pub type PlaceOfReportLocation = LocationType;
pub type PlannedArrivalTransportEvent = TransportEventType;
pub type PlannedDeliveryTransportEvent = TransportEventType;
pub type PlannedDepartureTransportEvent = TransportEventType;
pub type PlannedPeriod = PeriodType;
pub type PlannedPickupTransportEvent = TransportEventType;
pub type PlannedWaypointTransportEvent = TransportEventType;
pub type PointOfSaleContact = ContactType;
pub type PointOfSaleLocation = LocationType;
pub type PolicyDocumentReference = DocumentReferenceType;
pub type PolicyHolderParty = PartyType;
pub type PortCall = PortCallType;
pub type PortCallPurpose = PortCallPurposeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PortCallPurposeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "PurposeTypeCode")]
    pub purpose_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PurposeType")]
    pub purpose_type: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
}
pub type PortCallRecord = PortCallRecordType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PortCallRecordType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SecurityLevelCode")]
    pub security_level_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "SecurityMeasure")]
    pub security_measure: ::std::vec::Vec<SecurityMeasureType>,
    #[serde(default, rename = "PortFacilityLocation")]
    pub port_facility_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "Period")]
    pub period: ::core::option::Option<PeriodType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PortCallType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PlannedOperationsDescription")]
    pub planned_operations_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PlannedWorksDescription")]
    pub planned_works_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PlannedInspectionsDescription")]
    pub planned_inspections_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ExpectedAnchorageIndicator")]
    pub expected_anchorage_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "PositionInPortID")]
    pub position_in_port_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CargoAndBallastTankConditionDescription")]
    pub cargo_and_ballast_tank_condition_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ShipRequirement")]
    pub ship_requirement: ::std::vec::Vec<ShipRequirementType>,
    #[serde(default, rename = "PrimaryPortCallPurpose")]
    pub primary_port_call_purpose: ::core::option::Option<PortCallPurposeType>,
    #[serde(default, rename = "AdditionalPortCallPurpose")]
    pub additional_port_call_purpose: ::std::vec::Vec<PortCallPurposeType>,
    #[serde(default, rename = "RequestedArrivalEvent")]
    pub requested_arrival_event: ::core::option::Option<EventType>,
}
pub type PortFacilityLocation = LocationType;
pub type PositionOnBoardStowage = StowageType;
pub type PositioningTransportEvent = TransportEventType;
pub type PostAwardProcess = PostAwardProcessType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PostAwardProcessType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ElectronicCatalogueUsageIndicator")]
    pub electronic_catalogue_usage_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ElectronicInvoiceAcceptedIndicator")]
    pub electronic_invoice_accepted_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ElectronicOrderUsageIndicator")]
    pub electronic_order_usage_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ElectronicPaymentUsageIndicator")]
    pub electronic_payment_usage_indicator: ::std::vec::Vec<super::udt::IndicatorType>,
}
pub type PostalAddress = AddressType;
pub type PowerOfAttorney = PowerOfAttorneyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PowerOfAttorneyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "IssueTime")]
    pub issue_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "NotaryParty")]
    pub notary_party: ::core::option::Option<::std::boxed::Box<PartyType>>,
    #[serde(rename = "AgentParty")]
    pub agent_party: ::std::boxed::Box<PartyType>,
    #[serde(default, rename = "WitnessParty")]
    pub witness_party: ::std::vec::Vec<PartyType>,
    #[serde(default, rename = "MandateDocumentReference")]
    pub mandate_document_reference: ::std::vec::Vec<DocumentReferenceType>,
}
pub type PreCarriageShipmentStage = ShipmentStageType;
pub type PreSelectedParty = PartyType;
pub type PrepaidPayment = PaymentType;
pub type PrepaidPaymentTerms = PaymentTermsType;
pub type PreparationParty = PartyType;
pub type PresentationPeriod = PeriodType;
pub type PreviousCustomsDeclaration = CustomsDeclarationType;
pub type PreviousDocumentReference = DocumentReferenceType;
pub type PreviousPriceList = PriceListType;
pub type Price = PriceType;
pub type PriceExtension = PriceExtensionType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PriceExtensionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "Amount")]
    pub amount: super::cct::AmountType,
    #[serde(default, rename = "TaxInclusiveAmount")]
    pub tax_inclusive_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<TaxTotalType>,
}
pub type PriceList = PriceListType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PriceListType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "StatusCode")]
    pub status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::std::vec::Vec<PeriodType>,
    #[serde(default, rename = "PreviousPriceList")]
    pub previous_price_list: ::core::option::Option<::std::boxed::Box<PriceListType>>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PriceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "PriceAmount")]
    pub price_amount: super::cct::AmountType,
    #[serde(default, rename = "TaxInclusivePriceAmount")]
    pub tax_inclusive_price_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "BaseQuantity")]
    pub base_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "PriceChangeReason")]
    pub price_change_reason: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PriceTypeCode")]
    pub price_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PriceType")]
    pub price_type: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "OrderableUnitFactorRate")]
    pub orderable_unit_factor_rate: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "ValidityPeriod")]
    pub validity_period: ::std::vec::Vec<PeriodType>,
    #[serde(default, rename = "PriceList")]
    pub price_list: ::core::option::Option<PriceListType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "PricingExchangeRate")]
    pub pricing_exchange_rate: ::core::option::Option<ExchangeRateType>,
    #[serde(default, rename = "AlternativeCurrencyPrice")]
    pub alternative_currency_price: ::std::vec::Vec<PriceType>,
}
pub type PricingExchangeRate = ExchangeRateType;
pub type PricingReference = PricingReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PricingReferenceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "OriginalItemLocationQuantity")]
    pub original_item_location_quantity:
        ::core::option::Option<::std::boxed::Box<ItemLocationQuantityType>>,
    #[serde(default, rename = "AlternativeConditionPrice")]
    pub alternative_condition_price: ::std::vec::Vec<PriceType>,
}
pub type PrimaryPortCallPurpose = PortCallPurposeType;
pub type Prize = PrizeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PrizeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "RankCode")]
    pub rank_code: super::cct::CodeType,
    #[serde(default, rename = "ValueAmount")]
    pub value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
}
pub type ProcedureStatusRequestDocumentReference = DocumentReferenceType;
pub type ProcessJustification = ProcessJustificationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ProcessJustificationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "PreviousCancellationReasonCode")]
    pub previous_cancellation_reason_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ProcessReasonCode")]
    pub process_reason_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ProcessReason")]
    pub process_reason: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
}
pub type ProcessingParty = PartyType;
pub type ProcurementAdditionalType = ProcurementAdditionalTypeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ProcurementAdditionalTypeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ProcurementTypeCode")]
    pub procurement_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ProcurementType")]
    pub procurement_type: ::std::vec::Vec<super::cct::TextType>,
}
pub type ProcurementLegislationDocumentReference = DocumentReferenceType;
pub type ProcurementProject = ProcurementProjectType;
pub type ProcurementProjectLot = ProcurementProjectLotType;
pub type ProcurementProjectLotReference = ProcurementProjectLotReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ProcurementProjectLotReferenceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ProcurementProjectLotType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "LegalDocumentReference")]
    pub legal_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "TechnicalDocumentReference")]
    pub technical_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "RequiredDocumentReference")]
    pub required_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "ProvidedDocumentReference")]
    pub provided_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "TenderingTerms")]
    pub tendering_terms: ::core::option::Option<TenderingTermsType>,
    #[serde(default, rename = "TenderingProcess")]
    pub tendering_process: ::core::option::Option<TenderingProcessType>,
    #[serde(default, rename = "ProcurementProject")]
    pub procurement_project: ::core::option::Option<ProcurementProjectType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ProcurementProjectType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ProcurementTypeCode")]
    pub procurement_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ProcurementSubTypeCode")]
    pub procurement_sub_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "QualityControlCode")]
    pub quality_control_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "RequiredFeeAmount")]
    pub required_fee_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "FeeDescription")]
    pub fee_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "RequestedDeliveryDate")]
    pub requested_delivery_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EstimatedOverallContractQuantity")]
    pub estimated_overall_contract_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "SMESuitableIndicator")]
    pub sme_suitable_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ProcurementAdditionalType")]
    pub procurement_additional_type: ::std::vec::Vec<ProcurementAdditionalTypeType>,
    #[serde(default, rename = "RequestedTenderTotal")]
    pub requested_tender_total: ::core::option::Option<RequestedTenderTotalType>,
    #[serde(default, rename = "MainCommodityClassification")]
    pub main_commodity_classification: ::std::vec::Vec<CommodityClassificationType>,
    #[serde(default, rename = "AdditionalCommodityClassification")]
    pub additional_commodity_classification: ::std::vec::Vec<CommodityClassificationType>,
    #[serde(default, rename = "RealizedLocation")]
    pub realized_location: ::std::vec::Vec<LocationType>,
    #[serde(default, rename = "PlannedPeriod")]
    pub planned_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "ContractExtension")]
    pub contract_extension: ::core::option::Option<ContractExtensionType>,
    #[serde(default, rename = "RequestForTenderLine")]
    pub request_for_tender_line: ::std::vec::Vec<RequestForTenderLineType>,
}
pub type ProductDocumentationDocumentReference = DocumentReferenceType;
pub type ProjectReference = ProjectReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectReferenceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssueDate")]
    pub issue_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "WorkPhaseReference")]
    pub work_phase_reference: ::std::vec::Vec<WorkPhaseReferenceType>,
}
pub type PromisedDeliveryPeriod = PeriodType;
pub type PromotionalEvent = PromotionalEventType;
pub type PromotionalEventLineItem = PromotionalEventLineItemType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PromotionalEventLineItemType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "Amount")]
    pub amount: super::cct::AmountType,
    #[serde(rename = "EventLineItem")]
    pub event_line_item: EventLineItemType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PromotionalEventType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "PromotionalEventTypeCode")]
    pub promotional_event_type_code: super::cct::CodeType,
    #[serde(default, rename = "SubmissionDate")]
    pub submission_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "FirstShipmentAvailibilityDate")]
    pub first_shipment_availibility_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "FirstShipmentAvailabilityDate")]
    pub first_shipment_availability_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "LatestProposalAcceptanceDate")]
    pub latest_proposal_acceptance_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "PromotionalSpecification")]
    pub promotional_specification: ::std::vec::Vec<PromotionalSpecificationType>,
}
pub type PromotionalSpecification = PromotionalSpecificationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PromotionalSpecificationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "SpecificationID")]
    pub specification_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PromotionalEventLineItem")]
    pub promotional_event_line_item: ::std::vec::Vec<PromotionalEventLineItemType>,
    #[serde(default, rename = "EventTactic")]
    pub event_tactic: ::std::vec::Vec<EventTacticType>,
}
pub type ProofOfReexportationRequestDocumentReference = DocumentReferenceType;
pub type PropertyIdentification = PropertyIdentificationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PropertyIdentificationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "IssuerScopeID")]
    pub issuer_scope_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "IssuerParty")]
    pub issuer_party: ::core::option::Option<PartyType>,
}
pub type ProvidedDocumentReference = DocumentReferenceType;
pub type ProviderParty = PartyType;
pub type PurchaseLinePeriod = PeriodType;
pub type PurchaseReceiptLine = PurchaseReceiptLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PurchaseReceiptLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "PurchaseLinePeriod")]
    pub purchase_line_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "PurchaseReference")]
    pub purchase_reference: ::core::option::Option<PurchaseReferenceType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<TaxTotalType>,
    #[serde(rename = "Item")]
    pub item: ItemType,
    #[serde(default, rename = "Price")]
    pub price: ::core::option::Option<PriceType>,
}
pub type PurchaseReference = PurchaseReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct PurchaseReferenceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
}
pub type QualificationRequestRecipientParty = PartyType;
pub type QualificationResolution = QualificationResolutionType;
#[derive(Debug, Deserialize, Serialize)]
pub struct QualificationResolutionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "AdmissionCode")]
    pub admission_code: super::cct::CodeType,
    #[serde(default, rename = "ExclusionReason")]
    pub exclusion_reason: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Resolution")]
    pub resolution: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "ResolutionDate")]
    pub resolution_date: super::udt::DateTimeType,
    #[serde(default, rename = "ResolutionTime")]
    pub resolution_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: ::core::option::Option<ProcurementProjectLotType>,
}
pub type QualifyingParty = QualifyingPartyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct QualifyingPartyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ParticipationPercent")]
    pub participation_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "PersonalSituation")]
    pub personal_situation: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "OperatingYearsQuantity")]
    pub operating_years_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "EmployeeQuantity")]
    pub employee_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "BusinessClassificationEvidenceID")]
    pub business_classification_evidence_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "BusinessIdentityEvidenceID")]
    pub business_identity_evidence_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TendererRoleCode")]
    pub tenderer_role_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "BusinessClassificationScheme")]
    pub business_classification_scheme: ::core::option::Option<ClassificationSchemeType>,
    #[serde(default, rename = "TechnicalCapability")]
    pub technical_capability: ::std::vec::Vec<CapabilityType>,
    #[serde(default, rename = "FinancialCapability")]
    pub financial_capability: ::std::vec::Vec<CapabilityType>,
    #[serde(default, rename = "CompletedTask")]
    pub completed_task: ::std::vec::Vec<CompletedTaskType>,
    #[serde(default, rename = "Declaration")]
    pub declaration: ::std::vec::Vec<DeclarationType>,
    #[serde(default, rename = "Party")]
    pub party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "EconomicOperatorRole")]
    pub economic_operator_role: ::core::option::Option<EconomicOperatorRoleType>,
}
pub type QuarantineTransportEvent = TransportEventType;
pub type QuotationDocumentReference = DocumentReferenceType;
pub type QuotationLine = QuotationLineType;
pub type QuotationLineReference = LineReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct QuotationLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TotalTaxAmount")]
    pub total_tax_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "RequestForQuotationLineID")]
    pub request_for_quotation_line_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(rename = "LineItem")]
    pub line_item: LineItemType,
    #[serde(default, rename = "SellerProposedSubstituteLineItem")]
    pub seller_proposed_substitute_line_item: ::std::vec::Vec<LineItemType>,
    #[serde(default, rename = "AlternativeLineItem")]
    pub alternative_line_item: ::std::vec::Vec<LineItemType>,
    #[serde(default, rename = "RequestLineReference")]
    pub request_line_reference: ::core::option::Option<LineReferenceType>,
}
pub type QuotedMonetaryTotal = MonetaryTotalType;
pub type RadioactiveIsotope = RadioactiveIsotopeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct RadioactiveIsotopeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "Name")]
    pub name: super::cct::TextType,
    #[serde(rename = "ActivityLevelMeasure")]
    pub activity_level_measure: super::cct::MeasureType,
}
pub type RadioactiveMaterial = RadioactiveMaterialType;
#[derive(Debug, Deserialize, Serialize)]
pub struct RadioactiveMaterialType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "Name")]
    pub name: super::cct::TextType,
    #[serde(default, rename = "SpecialFormDescription")]
    pub special_form_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TransportIndexNumeric")]
    pub transport_index_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "FissileCriticalitySafetyIndexNumeric")]
    pub fissile_criticality_safety_index_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "ApplicableRadioactiveIsotope")]
    pub applicable_radioactive_isotope: ::core::option::Option<RadioactiveIsotopeType>,
}
pub type RailTransport = RailTransportType;
#[derive(Debug, Deserialize, Serialize)]
pub struct RailTransportType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "TrainID")]
    pub train_id: super::cct::IdentifierType,
    #[serde(default, rename = "RailCarID")]
    pub rail_car_id: ::core::option::Option<super::cct::IdentifierType>,
}
pub type RangeDimension = DimensionType;
pub type RealizedLocation = LocationType;
pub type ReceiptDocumentReference = DocumentReferenceType;
pub type ReceiptLine = ReceiptLineType;
pub type ReceiptLineReference = LineReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ReceiptLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ReceivedQuantity")]
    pub received_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "ShortQuantity")]
    pub short_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "ShortageActionCode")]
    pub shortage_action_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "RejectedQuantity")]
    pub rejected_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "RejectReasonCode")]
    pub reject_reason_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "RejectReason")]
    pub reject_reason: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "RejectActionCode")]
    pub reject_action_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "QuantityDiscrepancyCode")]
    pub quantity_discrepancy_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "OversupplyQuantity")]
    pub oversupply_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "ReceivedDate")]
    pub received_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ReceivedTime")]
    pub received_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "TimingComplaintCode")]
    pub timing_complaint_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TimingComplaint")]
    pub timing_complaint: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "OrderLineReference")]
    pub order_line_reference: ::core::option::Option<OrderLineReferenceType>,
    #[serde(default, rename = "DespatchLineReference")]
    pub despatch_line_reference: ::std::vec::Vec<LineReferenceType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "Item")]
    pub item: ::std::vec::Vec<ItemType>,
    #[serde(default, rename = "Shipment")]
    pub shipment: ::std::vec::Vec<ShipmentType>,
}
pub type ReceiptTransportEvent = TransportEventType;
pub type ReceivedHandlingUnitReceiptLine = ReceiptLineType;
pub type ReceiverParty = PartyType;
pub type ReceivingDigitalService = DigitalServiceType;
pub type RecipientCustomerParty = CustomerPartyType;
pub type RecipientParty = PartyType;
pub type RecoveryFacilityParty = PartyType;
pub type ReexportationDocumentReference = DocumentReferenceType;
pub type ReexportationEvidence = EvidenceType;
pub type ReferencedConsignment = ConsignmentType;
pub type ReferencedContract = ContractType;
pub type ReferencedGoodsItem = GoodsItemType;
pub type ReferencedPackage = PackageType;
pub type ReferencedShipment = ShipmentType;
pub type ReferencedTransportEquipment = TransportEquipmentType;
pub type RegistrationAddress = AddressType;
pub type RegistryCertificateDocumentReference = DocumentReferenceType;
pub type RegistryPortLocation = LocationType;
pub type Regulation = RegulationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct RegulationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Name")]
    pub name: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "LegalReference")]
    pub legal_reference: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "OntologyURI")]
    pub ontology_uri: ::core::option::Option<super::cct::IdentifierType>,
}
pub type ReimportationDocumentReference = DocumentReferenceType;
pub type RelatedCatalogueReference = CatalogueReferenceType;
pub type RelatedItem = RelatedItemType;
#[derive(Debug, Deserialize, Serialize)]
pub struct RelatedItemType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
}
pub type RemainingWasteDeliveryPortLocation = LocationType;
pub type ReminderDocumentReference = DocumentReferenceType;
pub type ReminderLine = ReminderLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ReminderLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "BalanceBroughtForwardIndicator")]
    pub balance_brought_forward_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "DebitLineAmount")]
    pub debit_line_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "CreditLineAmount")]
    pub credit_line_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "AccountingCostCode")]
    pub accounting_cost_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "AccountingCost")]
    pub accounting_cost: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "PenaltySurchargePercent")]
    pub penalty_surcharge_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "Amount")]
    pub amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "PaymentPurposeCode")]
    pub payment_purpose_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ReminderPeriod")]
    pub reminder_period: ::std::vec::Vec<PeriodType>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: ::std::vec::Vec<BillingReferenceType>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: ::core::option::Option<ExchangeRateType>,
}
pub type ReminderPeriod = PeriodType;
pub type RemittanceAdviceLine = RemittanceAdviceLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct RemittanceAdviceLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "DebitLineAmount")]
    pub debit_line_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "CreditLineAmount")]
    pub credit_line_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "BalanceAmount")]
    pub balance_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "PaymentPurposeCode")]
    pub payment_purpose_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "InvoicingPartyReference")]
    pub invoicing_party_reference: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: ::core::option::Option<SupplierPartyType>,
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: ::core::option::Option<CustomerPartyType>,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<SupplierPartyType>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::core::option::Option<CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<PartyType>,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: ::std::vec::Vec<PeriodType>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: ::std::vec::Vec<BillingReferenceType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: ::core::option::Option<ExchangeRateType>,
}
pub type RemittanceDocumentDistribution = DocumentDistributionType;
pub type RemittanceDocumentReference = DocumentReferenceType;
pub type Renewal = RenewalType;
#[derive(Debug, Deserialize, Serialize)]
pub struct RenewalType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Amount")]
    pub amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "Period")]
    pub period: ::core::option::Option<PeriodType>,
}
pub type RepairabilityScore = ScoreType;
pub type ReplacedNoticeDocumentReference = DocumentReferenceType;
pub type ReplacedRelatedItem = RelatedItemType;
pub type ReplacementRelatedItem = RelatedItemType;
pub type ReportLocation = LocationType;
pub type ReportedPeriod = PeriodType;
pub type ReportedShipment = ShipmentType;
pub type ReporterParty = PartyType;
pub type ReportingLocation = LocationType;
pub type ReportingPerson = PersonType;
pub type RepresentativeParty = PartyType;
pub type RequestForQuotationDocumentReference = DocumentReferenceType;
pub type RequestForQuotationLine = RequestForQuotationLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct RequestForQuotationLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "OptionalLineItemIndicator")]
    pub optional_line_item_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "PrivacyCode")]
    pub privacy_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "SecurityClassificationCode")]
    pub security_classification_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(rename = "LineItem")]
    pub line_item: LineItemType,
}
pub type RequestForTenderLine = RequestForTenderLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct RequestForTenderLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "TaxIncludedIndicator")]
    pub tax_included_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "MinimumAmount")]
    pub minimum_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "MaximumAmount")]
    pub maximum_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "EstimatedAmount")]
    pub estimated_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "DeliveryPeriod")]
    pub delivery_period: ::std::vec::Vec<PeriodType>,
    #[serde(default, rename = "RequiredItemLocationQuantity")]
    pub required_item_location_quantity: ::std::vec::Vec<ItemLocationQuantityType>,
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: ::core::option::Option<PeriodType>,
    #[serde(rename = "Item")]
    pub item: ItemType,
    #[serde(default, rename = "SubRequestForTenderLine")]
    pub sub_request_for_tender_line: ::std::vec::Vec<RequestForTenderLineType>,
}
pub type RequestLineReference = LineReferenceType;
pub type RequestedArrivalEvent = EventType;
pub type RequestedArrivalTransportEvent = TransportEventType;
pub type RequestedCatalogueReference = CatalogueReferenceType;
pub type RequestedClassificationScheme = ClassificationSchemeType;
pub type RequestedDeliveryPeriod = PeriodType;
pub type RequestedDeliveryTransportEvent = TransportEventType;
pub type RequestedDepartureTransportEvent = TransportEventType;
pub type RequestedDespatchPeriod = PeriodType;
pub type RequestedDocumentReference = DocumentReferenceType;
pub type RequestedLanguage = LanguageType;
pub type RequestedMonetaryTotal = MonetaryTotalType;
pub type RequestedPickupTransportEvent = TransportEventType;
pub type RequestedStatusLocation = LocationType;
pub type RequestedStatusPeriod = PeriodType;
pub type RequestedTenderTotal = RequestedTenderTotalType;
#[derive(Debug, Deserialize, Serialize)]
pub struct RequestedTenderTotalType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "EstimatedOverallContractAmount")]
    pub estimated_overall_contract_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "EstimatedOverallFrameworkContractsAmount")]
    pub estimated_overall_framework_contracts_amount:
        ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TotalAmount")]
    pub total_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxIncludedIndicator")]
    pub tax_included_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "MinimumAmount")]
    pub minimum_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "MaximumAmount")]
    pub maximum_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "MonetaryScope")]
    pub monetary_scope: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "AverageSubsequentContractAmount")]
    pub average_subsequent_contract_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "ApplicableTaxCategory")]
    pub applicable_tax_category: ::std::vec::Vec<TaxCategoryType>,
}
pub type RequestedValidityPeriod = PeriodType;
pub type RequestedWaypointTransportEvent = TransportEventType;
pub type RequestorParty = PartyType;
pub type RequiredBusinessClassificationScheme = ClassificationSchemeType;
pub type RequiredCertificationDocumentReference = DocumentReferenceType;
pub type RequiredClassificationScheme = ClassificationSchemeType;
pub type RequiredDocumentReference = DocumentReferenceType;
pub type RequiredFinancialGuarantee = FinancialGuaranteeType;
pub type RequiredItemLocationQuantity = ItemLocationQuantityType;
pub type RequiredRelatedItem = RelatedItemType;
pub type ResidenceAddress = AddressType;
pub type ResolutionDocumentReference = DocumentReferenceType;
pub type ResourceConsumption = ResourceConsumptionType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ResourceConsumptionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ResourceTypeCode")]
    pub resource_type_code: super::cct::CodeType,
    #[serde(rename = "ConsumptionMeasure")]
    pub consumption_measure: super::cct::MeasureType,
    #[serde(default, rename = "ResourceOriginDescription")]
    pub resource_origin_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "MeasurementPeriod")]
    pub measurement_period: ::core::option::Option<PeriodType>,
}
pub type ResponderParty = PartyType;
pub type Response = ResponseType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ReferenceID")]
    pub reference_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ResponseCode")]
    pub response_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "EffectiveDate")]
    pub effective_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EffectiveTime")]
    pub effective_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "Status")]
    pub status: ::std::vec::Vec<StatusType>,
}
pub type ResponseValue = ResponseValueType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ResponseValueType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Response")]
    pub response: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ResponseAmount")]
    pub response_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "ResponseBinaryObject")]
    pub response_binary_object: ::core::option::Option<super::cct::BinaryObjectType>,
    #[serde(default, rename = "ResponseCode")]
    pub response_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ResponseDate")]
    pub response_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ResponseID")]
    pub response_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ResponseIndicator")]
    pub response_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ResponseMeasure")]
    pub response_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "ResponseNumeric")]
    pub response_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "ResponseQuantity")]
    pub response_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "ResponseTime")]
    pub response_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ResponseURI")]
    pub response_uri: ::core::option::Option<super::cct::IdentifierType>,
}
pub type ResponsibleOfficerPerson = PersonType;
pub type ResponsibleParty = PartyType;
pub type ResponsibleTransportServiceProviderParty = PartyType;
pub type ResultOfVerification = ResultOfVerificationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ResultOfVerificationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ValidatorID")]
    pub validator_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ValidationResultCode")]
    pub validation_result_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ValidationDate")]
    pub validation_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ValidationTime")]
    pub validation_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ValidateProcess")]
    pub validate_process: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ValidateTool")]
    pub validate_tool: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ValidateToolVersion")]
    pub validate_tool_version: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "SignatoryParty")]
    pub signatory_party: ::core::option::Option<::std::boxed::Box<PartyType>>,
}
pub type RetailPlannedImpact = RetailPlannedImpactType;
#[derive(Debug, Deserialize, Serialize)]
pub struct RetailPlannedImpactType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "Amount")]
    pub amount: super::cct::AmountType,
    #[serde(rename = "ForecastPurposeCode")]
    pub forecast_purpose_code: super::cct::CodeType,
    #[serde(rename = "ForecastTypeCode")]
    pub forecast_type_code: super::cct::CodeType,
    #[serde(default, rename = "Period")]
    pub period: ::core::option::Option<PeriodType>,
}
pub type RetailerCustomerParty = CustomerPartyType;
pub type ReturnAddress = AddressType;
pub type RoadTransport = RoadTransportType;
#[derive(Debug, Deserialize, Serialize)]
pub struct RoadTransportType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "LicensePlateID")]
    pub license_plate_id: super::cct::IdentifierType,
    #[serde(default, rename = "TrailerLicensePlateID")]
    pub trailer_license_plate_id: ::core::option::Option<super::cct::IdentifierType>,
}
pub type SalesDocumentReference = DocumentReferenceType;
pub type SalesItem = SalesItemType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SalesItemType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "Quantity")]
    pub quantity: super::cct::QuantityType,
    #[serde(default, rename = "ActivityProperty")]
    pub activity_property: ::std::vec::Vec<ActivityPropertyType>,
    #[serde(default, rename = "TaxExclusivePrice")]
    pub tax_exclusive_price: ::std::vec::Vec<PriceType>,
    #[serde(default, rename = "TaxInclusivePrice")]
    pub tax_inclusive_price: ::std::vec::Vec<PriceType>,
    #[serde(rename = "Item")]
    pub item: ItemType,
}
pub type SanitaryMeasure = SanitaryMeasureType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SanitaryMeasureType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "SanitaryMeasureTypeCode")]
    pub sanitary_measure_type_code: super::cct::CodeType,
    #[serde(default, rename = "ApplicationDate")]
    pub application_date: ::core::option::Option<super::udt::DateTimeType>,
}
pub type ScheduledServiceFrequency = ServiceFrequencyType;
pub type Score = ScoreType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ScoreType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ScoreNumeric")]
    pub score_numeric: super::cct::NumericType,
    #[serde(rename = "ScoringSystemCode")]
    pub scoring_system_code: super::cct::CodeType,
}
pub type SecondaryHazard = SecondaryHazardType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SecondaryHazardType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "PlacardNotation")]
    pub placard_notation: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "PlacardEndorsement")]
    pub placard_endorsement: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "EmergencyProceduresCode")]
    pub emergency_procedures_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Extension")]
    pub extension: ::std::vec::Vec<super::cct::TextType>,
}
pub type SecurityClearanceTerm = SecurityClearanceTermType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityClearanceTermType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Code")]
    pub code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
}
pub type SecurityListing = SecurityListingType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityListingType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "MarketName")]
    pub market_name: super::cct::TextType,
    #[serde(default, rename = "MarketCode")]
    pub market_code: ::core::option::Option<super::cct::CodeType>,
}
pub type SecurityMeasure = SecurityMeasureType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityMeasureType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
}
pub type SecurityOfficerPerson = PersonType;
pub type SelfBilledCreditNoteDocumentReference = DocumentReferenceType;
pub type SelfBilledInvoiceDocumentReference = DocumentReferenceType;
pub type SellerContact = ContactType;
pub type SellerProposedSubstituteLineItem = LineItemType;
pub type SellerSubstitutedLineItem = LineItemType;
pub type SellerSupplierParty = SupplierPartyType;
pub type SellersItemIdentification = ItemIdentificationType;
pub type SenderParty = PartyType;
pub type SendingDigitalService = DigitalServiceType;
pub type SendingLogisticsOperatorParty = PartyType;
pub type ServiceAllowanceCharge = AllowanceChargeType;
pub type ServiceAvailabilityPeriod = PeriodType;
pub type ServiceChargePaymentTerms = PaymentTermsType;
pub type ServiceEndTimePeriod = PeriodType;
pub type ServiceFrequency = ServiceFrequencyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceFrequencyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "WeekDayCode")]
    pub week_day_code: super::cct::CodeType,
}
pub type ServiceLevelAgreement = ServiceLevelAgreementType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceLevelAgreementType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ServiceTypeCode")]
    pub service_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ServiceType")]
    pub service_type: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "AvailabilityTimePercent")]
    pub availability_time_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "MondayAvailabilityIndicator")]
    pub monday_availability_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "TuesdayAvailabilityIndicator")]
    pub tuesday_availability_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "WednesdayAvailabilityIndicator")]
    pub wednesday_availability_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ThursdayAvailabilityIndicator")]
    pub thursday_availability_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "FridayAvailabilityIndicator")]
    pub friday_availability_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "SaturdayAvailabilityIndicator")]
    pub saturday_availability_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "SundayAvailabilityIndicator")]
    pub sunday_availability_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "MinimumResponseTimeDurationMeasure")]
    pub minimum_response_time_duration_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "MinimumDownTimeScheduleDurationMeasure")]
    pub minimum_down_time_schedule_duration_measure:
        ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "MaximumIncidentNotificationDurationMeasure")]
    pub maximum_incident_notification_duration_measure:
        ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "MaximumDataLossDurationMeasure")]
    pub maximum_data_loss_duration_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "MeanTimeToRecoverDurationMeasure")]
    pub mean_time_to_recover_duration_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "ServiceAvailabilityPeriod")]
    pub service_availability_period: ::std::vec::Vec<PeriodType>,
    #[serde(default, rename = "ServiceMaintenancePeriod")]
    pub service_maintenance_period: ::std::vec::Vec<PeriodType>,
}
pub type ServiceMaintenancePeriod = PeriodType;
pub type ServiceProviderParty = ServiceProviderPartyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceProviderPartyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ServiceTypeCode")]
    pub service_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ServiceType")]
    pub service_type: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "Party")]
    pub party: ::std::boxed::Box<PartyType>,
    #[serde(default, rename = "SellerContact")]
    pub seller_contact: ::core::option::Option<ContactType>,
}
pub type ServiceStartTimePeriod = PeriodType;
pub type SettlementPeriod = PeriodType;
pub type ShareholderParty = ShareholderPartyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ShareholderPartyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "PartecipationPercent")]
    pub partecipation_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "ParticipationPercent")]
    pub participation_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "Party")]
    pub party: ::core::option::Option<::std::boxed::Box<PartyType>>,
}
pub type ShipRequirement = ShipRequirementType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ShipRequirementType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
}
pub type ShipSanitationControlCertificate = CertificateType;
pub type ShipSanitationControlExemptionDocumentReference = DocumentReferenceType;
pub type ShipStoreArticle = ShipStoreArticleType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ShipStoreArticleType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "OfficialUse")]
    pub official_use: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Stowage")]
    pub stowage: ::core::option::Option<StowageType>,
}
pub type ShipToShipActivityRecord = ShipToShipActivityRecordType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ShipToShipActivityRecordType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "AppliedSecurityMeasure")]
    pub applied_security_measure: ::std::vec::Vec<SecurityMeasureType>,
    #[serde(default, rename = "Period")]
    pub period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "Location")]
    pub location: ::core::option::Option<LocationType>,
}
pub type Shipment = ShipmentType;
pub type ShipmentDocumentReference = DocumentReferenceType;
pub type ShipmentStage = ShipmentStageType;
#[derive(Debug, Deserialize, Serialize)]
pub struct ShipmentStageType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ShipmentStageTypeCode")]
    pub shipment_stage_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ShipmentStageType")]
    pub shipment_stage_type: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TransportModeCode")]
    pub transport_mode_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TransportMeansTypeCode")]
    pub transport_means_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TransitDirectionCode")]
    pub transit_direction_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PreCarriageIndicator")]
    pub pre_carriage_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "OnCarriageIndicator")]
    pub on_carriage_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "CabotageIndicator")]
    pub cabotage_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "EstimatedDeliveryDate")]
    pub estimated_delivery_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EstimatedDeliveryTime")]
    pub estimated_delivery_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "RequiredDeliveryDate")]
    pub required_delivery_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "RequiredDeliveryTime")]
    pub required_delivery_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "LoadingSequenceID")]
    pub loading_sequence_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SuccessiveSequenceID")]
    pub successive_sequence_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Instructions")]
    pub instructions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "DemurrageInstructions")]
    pub demurrage_instructions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CrewQuantity")]
    pub crew_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "PassengerQuantity")]
    pub passenger_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "TransitPeriod")]
    pub transit_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "CarrierParty")]
    pub carrier_party: ::std::vec::Vec<PartyType>,
    #[serde(default, rename = "TransportMeans")]
    pub transport_means: ::core::option::Option<TransportMeansType>,
    #[serde(default, rename = "LoadingPortLocation")]
    pub loading_port_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "UnloadingPortLocation")]
    pub unloading_port_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "TransshipPortLocation")]
    pub transship_port_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "LoadingTransportEvent")]
    pub loading_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "ExaminationTransportEvent")]
    pub examination_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "AvailabilityTransportEvent")]
    pub availability_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "ExportationTransportEvent")]
    pub exportation_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "DischargeTransportEvent")]
    pub discharge_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "WarehousingTransportEvent")]
    pub warehousing_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "TakeoverTransportEvent")]
    pub takeover_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "OptionalTakeoverTransportEvent")]
    pub optional_takeover_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "DropoffTransportEvent")]
    pub dropoff_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "ActualPickupTransportEvent")]
    pub actual_pickup_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "DeliveryTransportEvent")]
    pub delivery_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "ReceiptTransportEvent")]
    pub receipt_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "StorageTransportEvent")]
    pub storage_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "AcceptanceTransportEvent")]
    pub acceptance_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "TerminalOperatorParty")]
    pub terminal_operator_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "CustomsAgentParty")]
    pub customs_agent_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "EstimatedTransitPeriod")]
    pub estimated_transit_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "FreightAllowanceCharge")]
    pub freight_allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "FreightChargeLocation")]
    pub freight_charge_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "DetentionTransportEvent")]
    pub detention_transport_event: ::std::vec::Vec<TransportEventType>,
    #[serde(default, rename = "RequestedDepartureTransportEvent")]
    pub requested_departure_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "RequestedArrivalTransportEvent")]
    pub requested_arrival_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "RequestedWaypointTransportEvent")]
    pub requested_waypoint_transport_event: ::std::vec::Vec<TransportEventType>,
    #[serde(default, rename = "PlannedDepartureTransportEvent")]
    pub planned_departure_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "PlannedArrivalTransportEvent")]
    pub planned_arrival_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "PlannedWaypointTransportEvent")]
    pub planned_waypoint_transport_event: ::std::vec::Vec<TransportEventType>,
    #[serde(default, rename = "ActualDepartureTransportEvent")]
    pub actual_departure_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "ActualWaypointTransportEvent")]
    pub actual_waypoint_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "ActualArrivalTransportEvent")]
    pub actual_arrival_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "TransportEvent")]
    pub transport_event: ::std::vec::Vec<TransportEventType>,
    #[serde(default, rename = "EstimatedDepartureTransportEvent")]
    pub estimated_departure_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "EstimatedArrivalTransportEvent")]
    pub estimated_arrival_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "PassengerPerson")]
    pub passenger_person: ::std::vec::Vec<PersonType>,
    #[serde(default, rename = "DriverPerson")]
    pub driver_person: ::std::vec::Vec<PersonType>,
    #[serde(default, rename = "ReportingPerson")]
    pub reporting_person: ::core::option::Option<PersonType>,
    #[serde(default, rename = "CrewMemberPerson")]
    pub crew_member_person: ::std::vec::Vec<PersonType>,
    #[serde(default, rename = "SecurityOfficerPerson")]
    pub security_officer_person: ::core::option::Option<PersonType>,
    #[serde(default, rename = "MasterPerson")]
    pub master_person: ::core::option::Option<PersonType>,
    #[serde(default, rename = "ShipsSurgeonPerson")]
    pub ships_surgeon_person: ::core::option::Option<PersonType>,
    #[serde(default, rename = "DestinationPortCall")]
    pub destination_port_call: ::core::option::Option<PortCallType>,
    #[serde(default, rename = "ShipStoreArticle")]
    pub ship_store_article: ::std::vec::Vec<ShipStoreArticleType>,
    #[serde(default, rename = "CrewPersonEffect")]
    pub crew_person_effect: ::std::vec::Vec<CrewPersonEffectType>,
    #[serde(default, rename = "MaritimeWaste")]
    pub maritime_waste: ::std::vec::Vec<MaritimeWasteType>,
    #[serde(default, rename = "BallastWaterSummary")]
    pub ballast_water_summary: ::core::option::Option<BallastWaterSummaryType>,
    #[serde(default, rename = "ISPSRequirements")]
    pub isps_requirements: ::core::option::Option<IspsRequirementsType>,
    #[serde(default, rename = "MaritimeHealthDeclaration")]
    pub maritime_health_declaration: ::core::option::Option<MaritimeHealthDeclarationType>,
    #[serde(default, rename = "FuelConsumption")]
    pub fuel_consumption: ::std::vec::Vec<FuelConsumptionType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct ShipmentType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ShippingPriorityLevelCode")]
    pub shipping_priority_level_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "HandlingCode")]
    pub handling_code: ::std::vec::Vec<super::cct::CodeType>,
    #[serde(default, rename = "HandlingInstructions")]
    pub handling_instructions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Information")]
    pub information: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetWeightMeasure")]
    pub net_weight_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetNetWeightMeasure")]
    pub net_net_weight_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "GrossVolumeMeasure")]
    pub gross_volume_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "NetVolumeMeasure")]
    pub net_volume_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "TotalGoodsItemQuantity")]
    pub total_goods_item_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "TotalTransportHandlingUnitQuantity")]
    pub total_transport_handling_unit_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "InsuranceValueAmount")]
    pub insurance_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "DeclaredCustomsValueAmount")]
    pub declared_customs_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "DeclaredForCarriageValueAmount")]
    pub declared_for_carriage_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "DeclaredStatisticsValueAmount")]
    pub declared_statistics_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "FreeOnBoardValueAmount")]
    pub free_on_board_value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "SpecialInstructions")]
    pub special_instructions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "DeliveryInstructions")]
    pub delivery_instructions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "SplitConsignmentIndicator")]
    pub split_consignment_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ConsignmentQuantity")]
    pub consignment_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "Consignment")]
    pub consignment: ::std::vec::Vec<ConsignmentType>,
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: ::std::vec::Vec<GoodsItemType>,
    #[serde(default, rename = "ShipmentStage")]
    pub shipment_stage: ::std::vec::Vec<ShipmentStageType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::core::option::Option<::std::boxed::Box<DeliveryType>>,
    #[serde(default, rename = "TransportHandlingUnit")]
    pub transport_handling_unit: ::std::vec::Vec<TransportHandlingUnitType>,
    #[serde(default, rename = "ReturnAddress")]
    pub return_address: ::core::option::Option<AddressType>,
    #[serde(default, rename = "OriginAddress")]
    pub origin_address: ::core::option::Option<AddressType>,
    #[serde(default, rename = "FirstArrivalPortLocation")]
    pub first_arrival_port_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "LastExitPortLocation")]
    pub last_exit_port_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "ExportCountry")]
    pub export_country: ::core::option::Option<CountryType>,
    #[serde(default, rename = "FreightAllowanceCharge")]
    pub freight_allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "InsurancePolicy")]
    pub insurance_policy: ::std::vec::Vec<InsurancePolicyType>,
}
pub type ShipperParty = PartyType;
pub type ShipsSurgeonPerson = PersonType;
pub type SignatoryContact = ContactType;
pub type SignatoryParty = PartyType;
pub type Signature = SignatureType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SignatureType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "ReasonCode")]
    pub reason_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ValidationDate")]
    pub validation_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ValidationTime")]
    pub validation_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ValidatorID")]
    pub validator_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CanonicalizationMethod")]
    pub canonicalization_method: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "SignatureMethod")]
    pub signature_method: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "SignatoryParty")]
    pub signatory_party: ::core::option::Option<::std::boxed::Box<PartyType>>,
    #[serde(default, rename = "DigitalSignatureAttachment")]
    pub digital_signature_attachment: ::core::option::Option<AttachmentType>,
    #[serde(default, rename = "OriginalDocumentReference")]
    pub original_document_reference:
        ::core::option::Option<::std::boxed::Box<DocumentReferenceType>>,
}
pub type SocialMediaProfile = SocialMediaProfileType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SocialMediaProfileType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "SocialMediaTypeCode")]
    pub social_media_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(rename = "URI")]
    pub uri: super::cct::IdentifierType,
}
pub type SourceCatalogueReference = CatalogueReferenceType;
pub type SourceIssuerParty = PartyType;
pub type SpecificTendererRequirement = TendererRequirementType;
pub type StandardItemIdentification = ItemIdentificationType;
pub type StandardPropertyIdentification = PropertyIdentificationType;
pub type StatementDocumentReference = DocumentReferenceType;
pub type StatementLine = StatementLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct StatementLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "UUID")]
    pub uuid: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "BalanceBroughtForwardIndicator")]
    pub balance_brought_forward_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "DebitLineAmount")]
    pub debit_line_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "CreditLineAmount")]
    pub credit_line_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "BalanceAmount")]
    pub balance_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "PaymentPurposeCode")]
    pub payment_purpose_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PaymentMeans")]
    pub payment_means: ::core::option::Option<PaymentMeansType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<PaymentTermsType>,
    #[serde(default, rename = "BuyerCustomerParty")]
    pub buyer_customer_party: ::core::option::Option<CustomerPartyType>,
    #[serde(default, rename = "SellerSupplierParty")]
    pub seller_supplier_party: ::core::option::Option<SupplierPartyType>,
    #[serde(default, rename = "OriginatorCustomerParty")]
    pub originator_customer_party: ::core::option::Option<CustomerPartyType>,
    #[serde(default, rename = "BeneficiaryParty")]
    pub beneficiary_party: ::std::vec::Vec<PartyType>,
    #[serde(default, rename = "AccountingCustomerParty")]
    pub accounting_customer_party: ::core::option::Option<CustomerPartyType>,
    #[serde(default, rename = "AccountingSupplierParty")]
    pub accounting_supplier_party: ::core::option::Option<SupplierPartyType>,
    #[serde(default, rename = "PayeeParty")]
    pub payee_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "InvoicePeriod")]
    pub invoice_period: ::std::vec::Vec<PeriodType>,
    #[serde(default, rename = "BillingReference")]
    pub billing_reference: ::std::vec::Vec<BillingReferenceType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: ::core::option::Option<ExchangeRateType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "CollectedPayment")]
    pub collected_payment: ::std::vec::Vec<PaymentType>,
}
pub type StatementMonetaryTotal = MonetaryTotalType;
pub type StatementPeriod = PeriodType;
pub type Status = StatusType;
pub type StatusLocation = LocationType;
pub type StatusPeriod = PeriodType;
#[derive(Debug, Deserialize, Serialize)]
pub struct StatusType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ConditionCode")]
    pub condition_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ReferenceDate")]
    pub reference_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ReferenceTime")]
    pub reference_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "StatusReasonCode")]
    pub status_reason_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "StatusReason")]
    pub status_reason: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "SequenceID")]
    pub sequence_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Text")]
    pub text: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "IndicationIndicator")]
    pub indication_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "Percent")]
    pub percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "ReliabilityPercent")]
    pub reliability_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "DocumentationAttachment")]
    pub documentation_attachment: ::std::vec::Vec<AttachmentType>,
    #[serde(default, rename = "SubStatus")]
    pub sub_status: ::std::vec::Vec<StatusType>,
    #[serde(default, rename = "Condition")]
    pub condition: ::std::vec::Vec<ConditionType>,
}
pub type StockAvailabilityReportLine = StockAvailabilityReportLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct StockAvailabilityReportLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "Quantity")]
    pub quantity: super::cct::QuantityType,
    #[serde(default, rename = "ValueAmount")]
    pub value_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "AvailabilityDate")]
    pub availability_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "AvailabilityStatusCode")]
    pub availability_status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(rename = "Item")]
    pub item: ItemType,
}
pub type Storage = StorageType;
pub type StorageLocation = LocationType;
pub type StorageTransportEvent = TransportEventType;
#[derive(Debug, Deserialize, Serialize)]
pub struct StorageType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "GateID")]
    pub gate_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AirFlowPercent")]
    pub air_flow_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "HumidityPercent")]
    pub humidity_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "AnimalFoodApprovedIndicator")]
    pub animal_food_approved_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "HumanFoodApprovedIndicator")]
    pub human_food_approved_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "DangerousGoodsApprovedIndicator")]
    pub dangerous_goods_approved_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "RefrigeratedIndicator")]
    pub refrigerated_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "PowerIndicator")]
    pub power_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: ::core::option::Option<TemperatureType>,
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: ::core::option::Option<TemperatureType>,
    #[serde(default, rename = "Certificate")]
    pub certificate: ::std::vec::Vec<CertificateType>,
}
pub type Stowage = StowageType;
#[derive(Debug, Deserialize, Serialize)]
pub struct StowageType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "LocationID")]
    pub location_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Location")]
    pub location: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: ::std::vec::Vec<DimensionType>,
}
pub type SubAttestationLine = AttestationLineType;
pub type SubCreditNoteLine = CreditNoteLineType;
pub type SubDebitNoteLine = DebitNoteLineType;
pub type SubDespatchLine = DespatchLineType;
pub type SubGoodsProcessing = GoodsProcessingType;
pub type SubInvoiceLine = InvoiceLineType;
pub type SubItemProperty = ItemPropertyType;
pub type SubLineItem = LineItemType;
pub type SubRequestForTenderLine = RequestForTenderLineType;
pub type SubStatus = StatusType;
pub type SubTenderLine = TenderLineType;
pub type SubTenderingCriterion = TenderingCriterionType;
pub type SubWorkReportLine = WorkReportLineType;
pub type SubcontractTerms = SubcontractTermsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SubcontractTermsType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Rate")]
    pub rate: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "UnknownPriceIndicator")]
    pub unknown_price_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Amount")]
    pub amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "SubcontractingConditionsCode")]
    pub subcontracting_conditions_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "MaximumPercent")]
    pub maximum_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "MinimumPercent")]
    pub minimum_percent: ::core::option::Option<super::cct::NumericType>,
}
pub type SubcontractorParty = PartyType;
pub type SubordinateAwardingCriterion = AwardingCriterionType;
pub type SubordinateAwardingCriterionResponse = AwardingCriterionResponseType;
pub type SubscriberConsumption = SubscriberConsumptionType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SubscriberConsumptionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ConsumptionID")]
    pub consumption_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SpecificationTypeCode")]
    pub specification_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TotalMeteredQuantity")]
    pub total_metered_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "SubscriberParty")]
    pub subscriber_party: ::core::option::Option<PartyType>,
    #[serde(rename = "UtilityConsumptionPoint")]
    pub utility_consumption_point: ConsumptionPointType,
    #[serde(default, rename = "OnAccountPayment")]
    pub on_account_payment: ::std::vec::Vec<OnAccountPaymentType>,
    #[serde(default, rename = "Consumption")]
    pub consumption: ::core::option::Option<ConsumptionType>,
    #[serde(default, rename = "SupplierConsumption")]
    pub supplier_consumption: ::std::vec::Vec<SupplierConsumptionType>,
}
pub type SubscriberParty = PartyType;
pub type SubsequentProcessTenderRequirement = TenderRequirementType;
pub type SubsidiaryLocation = LocationType;
pub type SubsidiaryTenderingCriterionPropertyGroup = TenderingCriterionPropertyGroupType;
pub type SubstituteCarrierParty = PartyType;
pub type SuggestedEvidence = EvidenceType;
pub type SuppliedEvidence = EvidenceType;
pub type SupplierConsumption = SupplierConsumptionType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SupplierConsumptionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "UtilitySupplierParty")]
    pub utility_supplier_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "UtilityCustomerParty")]
    pub utility_customer_party: ::core::option::Option<PartyType>,
    #[serde(rename = "Consumption")]
    pub consumption: ConsumptionType,
    #[serde(default, rename = "Contract")]
    pub contract: ::core::option::Option<ContractType>,
    #[serde(default, rename = "ConsumptionLine")]
    pub consumption_line: ::std::vec::Vec<ConsumptionLineType>,
}
pub type SupplierParty = SupplierPartyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct SupplierPartyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "CustomerAssignedAccountID")]
    pub customer_assigned_account_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "AdditionalAccountID")]
    pub additional_account_id: ::std::vec::Vec<super::cct::IdentifierType>,
    #[serde(default, rename = "DataSendingCapability")]
    pub data_sending_capability: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Party")]
    pub party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "DespatchContact")]
    pub despatch_contact: ::core::option::Option<ContactType>,
    #[serde(default, rename = "AccountingContact")]
    pub accounting_contact: ::core::option::Option<ContactType>,
    #[serde(default, rename = "SellerContact")]
    pub seller_contact: ::core::option::Option<ContactType>,
}
pub type SupplyChainActivityDataLine = ActivityDataLineType;
pub type SupplyItem = ItemType;
pub type SupportContact = ContactType;
pub type SupportedCommodityClassification = CommodityClassificationType;
pub type SupportedTransportEquipment = TransportEquipmentType;
pub type SupportingDocumentReference = DocumentReferenceType;
pub type TakeoverTransportEvent = TransportEventType;
pub type TaxCategory = TaxCategoryType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TaxCategoryType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Percent")]
    pub percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "BaseUnitMeasure")]
    pub base_unit_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "PerUnitAmount")]
    pub per_unit_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxExemptionReasonCode")]
    pub tax_exemption_reason_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TaxExemptionReason")]
    pub tax_exemption_reason: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TierRange")]
    pub tier_range: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "TierRatePercent")]
    pub tier_rate_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "SupplyTypeCode")]
    pub supply_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TaxScheme")]
    pub tax_scheme: ::core::option::Option<TaxSchemeType>,
}
pub type TaxDueCountry = CountryType;
pub type TaxExchangeRate = ExchangeRateType;
pub type TaxExclusivePrice = PriceType;
pub type TaxInclusivePrice = PriceType;
pub type TaxRepresentativeParty = PartyType;
pub type TaxScheme = TaxSchemeType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TaxSchemeType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "TaxTypeCode")]
    pub tax_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CurrencyCode")]
    pub currency_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "JurisdictionRegionAddress")]
    pub jurisdiction_region_address: ::std::vec::Vec<AddressType>,
}
pub type TaxSubtotal = TaxSubtotalType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TaxSubtotalType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "TaxableAmount")]
    pub taxable_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(rename = "TaxAmount")]
    pub tax_amount: super::cct::AmountType,
    #[serde(default, rename = "TaxInclusiveAmount")]
    pub tax_inclusive_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "CalculationSequenceNumeric")]
    pub calculation_sequence_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "TransactionCurrencyTaxAmount")]
    pub transaction_currency_tax_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "Percent")]
    pub percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "BaseUnitMeasure")]
    pub base_unit_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "PerUnitAmount")]
    pub per_unit_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TierRange")]
    pub tier_range: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "TierRatePercent")]
    pub tier_rate_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(rename = "TaxCategory")]
    pub tax_category: TaxCategoryType,
    #[serde(default, rename = "TaxDueCountry")]
    pub tax_due_country: ::core::option::Option<CountryType>,
}
pub type TaxTotal = TaxTotalType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TaxTotalType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "TaxAmount")]
    pub tax_amount: super::cct::AmountType,
    #[serde(default, rename = "CalculationSequenceNumeric")]
    pub calculation_sequence_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "RoundingAmount")]
    pub rounding_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxEvidenceIndicator")]
    pub tax_evidence_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "TaxIncludedIndicator")]
    pub tax_included_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "TaxSubtotal")]
    pub tax_subtotal: ::std::vec::Vec<TaxSubtotalType>,
}
pub type TechnicalCapability = CapabilityType;
pub type TechnicalCommitteePerson = PersonType;
pub type TechnicalContact = ContactType;
pub type TechnicalDocumentReference = DocumentReferenceType;
pub type TechnicalEvaluationCriterion = EvaluationCriterionType;
pub type TelecommunicationsService = TelecommunicationsServiceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TelecommunicationsServiceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(rename = "CallDate")]
    pub call_date: super::udt::DateTimeType,
    #[serde(rename = "CallTime")]
    pub call_time: super::udt::DateTimeType,
    #[serde(rename = "ServiceNumberCalled")]
    pub service_number_called: super::cct::TextType,
    #[serde(default, rename = "TelecommunicationsServiceCategory")]
    pub telecommunications_service_category: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "TelecommunicationsServiceCategoryCode")]
    pub telecommunications_service_category_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "MovieTitle")]
    pub movie_title: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "RoamingPartnerName")]
    pub roaming_partner_name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "PayPerView")]
    pub pay_per_view: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "TelecommunicationsServiceCall")]
    pub telecommunications_service_call: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "TelecommunicationsServiceCallCode")]
    pub telecommunications_service_call_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CallBaseAmount")]
    pub call_base_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "CallExtensionAmount")]
    pub call_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "Price")]
    pub price: ::core::option::Option<PriceType>,
    #[serde(default, rename = "Country")]
    pub country: ::core::option::Option<CountryType>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: ::std::vec::Vec<ExchangeRateType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<TaxTotalType>,
    #[serde(default, rename = "CallDuty")]
    pub call_duty: ::std::vec::Vec<DutyType>,
    #[serde(default, rename = "TimeDuty")]
    pub time_duty: ::std::vec::Vec<DutyType>,
}
pub type TelecommunicationsSupply = TelecommunicationsSupplyType;
pub type TelecommunicationsSupplyLine = TelecommunicationsSupplyLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TelecommunicationsSupplyLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(rename = "PhoneNumber")]
    pub phone_number: super::cct::TextType,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: ::std::vec::Vec<ExchangeRateType>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<TaxTotalType>,
    #[serde(default, rename = "TelecommunicationsService")]
    pub telecommunications_service: ::std::vec::Vec<TelecommunicationsServiceType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TelecommunicationsSupplyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "TelecommunicationsSupplyType")]
    pub telecommunications_supply_type: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "TelecommunicationsSupplyTypeCode")]
    pub telecommunications_supply_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(rename = "PrivacyCode")]
    pub privacy_code: super::cct::CodeType,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TotalAmount")]
    pub total_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TelecommunicationsSupplyLine")]
    pub telecommunications_supply_line: ::std::vec::Vec<TelecommunicationsSupplyLineType>,
}
pub type Temperature = TemperatureType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TemperatureType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "AttributeID")]
    pub attribute_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Measure")]
    pub measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "MeasureCode")]
    pub measure_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
}
pub type TemplateDocumentReference = DocumentReferenceType;
pub type TemplateEvidence = EvidenceType;
pub type TenderDocumentReference = DocumentReferenceType;
pub type TenderEncryptionData = EncryptionDataType;
pub type TenderEvaluationParty = PartyType;
pub type TenderLine = TenderLineType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TenderLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TotalTaxAmount")]
    pub total_tax_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "OrderableUnit")]
    pub orderable_unit: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ContentUnitQuantity")]
    pub content_unit_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "OrderQuantityIncrementNumeric")]
    pub order_quantity_increment_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "MinimumOrderQuantity")]
    pub minimum_order_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MaximumOrderQuantity")]
    pub maximum_order_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "WarrantyInformation")]
    pub warranty_information: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PackLevelCode")]
    pub pack_level_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "Item")]
    pub item: ::core::option::Option<ItemType>,
    #[serde(default, rename = "OfferedItemLocationQuantity")]
    pub offered_item_location_quantity: ::std::vec::Vec<ItemLocationQuantityType>,
    #[serde(default, rename = "ReplacementRelatedItem")]
    pub replacement_related_item: ::std::vec::Vec<RelatedItemType>,
    #[serde(default, rename = "WarrantyParty")]
    pub warranty_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "SubTenderLine")]
    pub sub_tender_line: ::std::vec::Vec<TenderLineType>,
    #[serde(default, rename = "CallForTendersLineReference")]
    pub call_for_tenders_line_reference: ::core::option::Option<LineReferenceType>,
    #[serde(default, rename = "CallForTendersDocumentReference")]
    pub call_for_tenders_document_reference: ::std::vec::Vec<DocumentReferenceType>,
}
pub type TenderNotificationDocumentReference = DocumentReferenceType;
pub type TenderPreparation = TenderPreparationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TenderPreparationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "TenderEnvelopeID")]
    pub tender_envelope_id: super::cct::IdentifierType,
    #[serde(default, rename = "TenderEnvelopeTypeCode")]
    pub tender_envelope_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "OpenTenderID")]
    pub open_tender_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: ::std::vec::Vec<ProcurementProjectLotType>,
    #[serde(default, rename = "DocumentTenderRequirement")]
    pub document_tender_requirement: ::std::vec::Vec<TenderRequirementType>,
    #[serde(default, rename = "TenderEncryptionData")]
    pub tender_encryption_data: ::std::vec::Vec<EncryptionDataType>,
}
pub type TenderRecipientParty = PartyType;
pub type TenderRequirement = TenderRequirementType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TenderRequirementType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Name")]
    pub name: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TemplateDocumentReference")]
    pub template_document_reference: ::core::option::Option<DocumentReferenceType>,
}
pub type TenderResult = TenderResultType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TenderResultType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "AwardID")]
    pub award_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TenderResultCode")]
    pub tender_result_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "AdvertisementAmount")]
    pub advertisement_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(rename = "AwardDate")]
    pub award_date: super::udt::DateTimeType,
    #[serde(default, rename = "AwardTime")]
    pub award_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ReceivedTenderQuantity")]
    pub received_tender_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "LowerTenderAmount")]
    pub lower_tender_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "HigherTenderAmount")]
    pub higher_tender_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "StartDate")]
    pub start_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ReceivedElectronicTenderQuantity")]
    pub received_electronic_tender_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "ReceivedForeignTenderQuantity")]
    pub received_foreign_tender_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "Contract")]
    pub contract: ::core::option::Option<ContractType>,
    #[serde(default, rename = "AwardedTenderedProject")]
    pub awarded_tendered_project: ::core::option::Option<TenderedProjectType>,
    #[serde(default, rename = "ContractFormalizationPeriod")]
    pub contract_formalization_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "SubcontractTerms")]
    pub subcontract_terms: ::std::vec::Vec<SubcontractTermsType>,
    #[serde(default, rename = "WinningParty")]
    pub winning_party: ::std::vec::Vec<WinningPartyType>,
}
pub type TenderStatusInquiryDocumentReference = DocumentReferenceType;
pub type TenderSubmissionDeadlinePeriod = PeriodType;
pub type TenderValidityPeriod = PeriodType;
pub type TenderedProject = TenderedProjectType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TenderedProjectType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "VariantID")]
    pub variant_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "FeeAmount")]
    pub fee_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "FeeDescription")]
    pub fee_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TenderEnvelopeID")]
    pub tender_envelope_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TenderEnvelopeTypeCode")]
    pub tender_envelope_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "AdditionalFee")]
    pub additional_fee: ::std::vec::Vec<FeeType>,
    #[serde(default, rename = "ProcurementProjectLot")]
    pub procurement_project_lot: ::std::vec::Vec<ProcurementProjectLotType>,
    #[serde(default, rename = "EvidenceDocumentReference")]
    pub evidence_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<TaxTotalType>,
    #[serde(default, rename = "LegalMonetaryTotal")]
    pub legal_monetary_total: ::core::option::Option<MonetaryTotalType>,
    #[serde(default, rename = "TenderLine")]
    pub tender_line: ::std::vec::Vec<TenderLineType>,
    #[serde(default, rename = "AwardingCriterionResponse")]
    pub awarding_criterion_response: ::std::vec::Vec<AwardingCriterionResponseType>,
}
pub type TendererParty = PartyType;
pub type TendererPartyQualification = TendererPartyQualificationType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TendererPartyQualificationType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "InterestedProcurementProjectLot")]
    pub interested_procurement_project_lot: ::std::vec::Vec<ProcurementProjectLotType>,
    #[serde(rename = "MainQualifyingParty")]
    pub main_qualifying_party: QualifyingPartyType,
    #[serde(default, rename = "AdditionalQualifyingParty")]
    pub additional_qualifying_party: ::std::vec::Vec<QualifyingPartyType>,
}
pub type TendererQualificationDocumentReference = DocumentReferenceType;
pub type TendererQualificationRequest = TendererQualificationRequestType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TendererQualificationRequestType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "CompanyLegalFormCode")]
    pub company_legal_form_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CompanyLegalForm")]
    pub company_legal_form: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PersonalSituation")]
    pub personal_situation: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "OperatingYearsQuantity")]
    pub operating_years_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "EmployeeQuantity")]
    pub employee_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "RequiredBusinessClassificationScheme")]
    pub required_business_classification_scheme: ::std::vec::Vec<ClassificationSchemeType>,
    #[serde(default, rename = "TechnicalEvaluationCriterion")]
    pub technical_evaluation_criterion: ::std::vec::Vec<EvaluationCriterionType>,
    #[serde(default, rename = "FinancialEvaluationCriterion")]
    pub financial_evaluation_criterion: ::std::vec::Vec<EvaluationCriterionType>,
    #[serde(default, rename = "SpecificTendererRequirement")]
    pub specific_tenderer_requirement: ::std::vec::Vec<TendererRequirementType>,
    #[serde(default, rename = "EconomicOperatorRole")]
    pub economic_operator_role: ::std::vec::Vec<EconomicOperatorRoleType>,
}
pub type TendererRequirement = TendererRequirementType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TendererRequirementType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Name")]
    pub name: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TendererRequirementTypeCode")]
    pub tenderer_requirement_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "LegalReference")]
    pub legal_reference: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "SuggestedEvidence")]
    pub suggested_evidence: ::std::vec::Vec<EvidenceType>,
}
pub type TenderingCriterion = TenderingCriterionType;
pub type TenderingCriterionProperty = TenderingCriterionPropertyType;
pub type TenderingCriterionPropertyGroup = TenderingCriterionPropertyGroupType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TenderingCriterionPropertyGroupType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PropertyGroupTypeCode")]
    pub property_group_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "FulfilmentIndicator")]
    pub fulfilment_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "FulfilmentIndicatorTypeCode")]
    pub fulfilment_indicator_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TenderingCriterionProperty")]
    pub tendering_criterion_property: ::std::vec::Vec<TenderingCriterionPropertyType>,
    #[serde(default, rename = "SubsidiaryTenderingCriterionPropertyGroup")]
    pub subsidiary_tendering_criterion_property_group:
        ::std::vec::Vec<TenderingCriterionPropertyGroupType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TenderingCriterionPropertyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TypeCode")]
    pub type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ValueDataTypeCode")]
    pub value_data_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ValueUnitCode")]
    pub value_unit_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ValueCurrencyCode")]
    pub value_currency_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ExpectedAmount")]
    pub expected_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "ExpectedID")]
    pub expected_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ExpectedIndicator")]
    pub expected_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ExpectedCode")]
    pub expected_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ExpectedValueNumeric")]
    pub expected_value_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "ExpectedDescription")]
    pub expected_description: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ExpectedURI")]
    pub expected_uri: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "MaximumAmount")]
    pub maximum_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "MinimumAmount")]
    pub minimum_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "MaximumValueNumeric")]
    pub maximum_value_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "MinimumValueNumeric")]
    pub minimum_value_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "MaximumQuantity")]
    pub maximum_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "MinimumQuantity")]
    pub minimum_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "TranslationTypeCode")]
    pub translation_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CertificationLevelDescription")]
    pub certification_level_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CopyQualityTypeCode")]
    pub copy_quality_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ApplicablePeriod")]
    pub applicable_period: ::std::vec::Vec<PeriodType>,
    #[serde(default, rename = "TemplateEvidence")]
    pub template_evidence: ::std::vec::Vec<EvidenceType>,
}
pub type TenderingCriterionResponse = TenderingCriterionResponseType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TenderingCriterionResponseType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ValidatedCriterionPropertyID")]
    pub validated_criterion_property_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ConfidentialityLevelCode")]
    pub confidentiality_level_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ResponseValue")]
    pub response_value: ::std::vec::Vec<ResponseValueType>,
    #[serde(default, rename = "ApplicablePeriod")]
    pub applicable_period: ::std::vec::Vec<PeriodType>,
    #[serde(default, rename = "EvidenceSupplied")]
    pub evidence_supplied: ::std::vec::Vec<EvidenceSuppliedType>,
    #[serde(default, rename = "SuppliedEvidence")]
    pub supplied_evidence: ::std::vec::Vec<EvidenceType>,
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: ::std::vec::Vec<ProcurementProjectLotReferenceType>,
    #[serde(default, rename = "CommodityClassification")]
    pub commodity_classification: ::std::vec::Vec<CommodityClassificationType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TenderingCriterionType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "CriterionTypeCode")]
    pub criterion_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Name")]
    pub name: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "WeightNumeric")]
    pub weight_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "FulfilmentIndicator")]
    pub fulfilment_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "FulfilmentIndicatorTypeCode")]
    pub fulfilment_indicator_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "EvaluationMethodTypeCode")]
    pub evaluation_method_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "WeightingConsiderationDescription")]
    pub weighting_consideration_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ProcurementProjectLotReference")]
    pub procurement_project_lot_reference: ::std::vec::Vec<ProcurementProjectLotReferenceType>,
    #[serde(default, rename = "CommodityClassification")]
    pub commodity_classification: ::std::vec::Vec<CommodityClassificationType>,
    #[serde(default, rename = "SubTenderingCriterion")]
    pub sub_tendering_criterion: ::std::vec::Vec<TenderingCriterionType>,
    #[serde(default, rename = "Legislation")]
    pub legislation: ::std::vec::Vec<LegislationType>,
    #[serde(default, rename = "TenderingCriterionPropertyGroup")]
    pub tendering_criterion_property_group: ::std::vec::Vec<TenderingCriterionPropertyGroupType>,
}
pub type TenderingProcess = TenderingProcessType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TenderingProcessType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "OriginalContractingSystemID")]
    pub original_contracting_system_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "NegotiationDescription")]
    pub negotiation_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ProcedureCode")]
    pub procedure_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "UrgencyCode")]
    pub urgency_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ExpenseCode")]
    pub expense_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PartPresentationCode")]
    pub part_presentation_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ContractingSystemCode")]
    pub contracting_system_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "SubmissionMethodCode")]
    pub submission_method_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CandidateReductionConstraintIndicator")]
    pub candidate_reduction_constraint_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "GovernmentAgreementConstraintIndicator")]
    pub government_agreement_constraint_indicator:
        ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "AccessToolsURI")]
    pub access_tools_uri: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TerminatedIndicator")]
    pub terminated_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "DocumentAvailabilityPeriod")]
    pub document_availability_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "TenderSubmissionDeadlinePeriod")]
    pub tender_submission_deadline_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "InvitationSubmissionPeriod")]
    pub invitation_submission_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "ParticipationInvitationPeriod")]
    pub participation_invitation_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "ParticipationRequestReceptionPeriod")]
    pub participation_request_reception_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "AdditionalInformationRequestPeriod")]
    pub additional_information_request_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "NoticeDocumentReference")]
    pub notice_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "AdditionalDocumentReference")]
    pub additional_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "ProcessJustification")]
    pub process_justification: ::std::vec::Vec<ProcessJustificationType>,
    #[serde(default, rename = "EconomicOperatorShortList")]
    pub economic_operator_short_list: ::std::vec::Vec<EconomicOperatorShortListType>,
    #[serde(default, rename = "OpenTenderEvent")]
    pub open_tender_event: ::std::vec::Vec<EventType>,
    #[serde(default, rename = "AuctionTerms")]
    pub auction_terms: ::core::option::Option<AuctionTermsType>,
    #[serde(default, rename = "FrameworkAgreement")]
    pub framework_agreement: ::core::option::Option<FrameworkAgreementType>,
    #[serde(default, rename = "ContractingSystem")]
    pub contracting_system: ::std::vec::Vec<ContractingSystemType>,
}
pub type TenderingTerms = TenderingTermsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TenderingTermsType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "AwardingMethodTypeCode")]
    pub awarding_method_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PriceEvaluationCode")]
    pub price_evaluation_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "MaximumVariantQuantity")]
    pub maximum_variant_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "VariantConstraintIndicator")]
    pub variant_constraint_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "AcceptedVariantsDescription")]
    pub accepted_variants_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "VariantConstraintCode")]
    pub variant_constraint_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PriceRevisionFormulaDescription")]
    pub price_revision_formula_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "FundingProgramCode")]
    pub funding_program_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "FundingProgram")]
    pub funding_program: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "MaximumAdvertisementAmount")]
    pub maximum_advertisement_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "Note")]
    pub note: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PaymentFrequencyCode")]
    pub payment_frequency_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "EconomicOperatorRegistryURI")]
    pub economic_operator_registry_uri: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "RequiredCurriculaIndicator")]
    pub required_curricula_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "RequiredCurriculaCode")]
    pub required_curricula_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "OtherConditionsIndicator")]
    pub other_conditions_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "RecurringProcurementIndicator")]
    pub recurring_procurement_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "RecurringProcurementDescription")]
    pub recurring_procurement_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "EstimatedTimingFurtherPublication")]
    pub estimated_timing_further_publication: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "AdditionalConditions")]
    pub additional_conditions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "LatestSecurityClearanceDate")]
    pub latest_security_clearance_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "DocumentationFeeAmount")]
    pub documentation_fee_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "MultipleTendersCode")]
    pub multiple_tenders_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PenaltyClause")]
    pub penalty_clause: ::std::vec::Vec<ClauseType>,
    #[serde(default, rename = "RequiredFinancialGuarantee")]
    pub required_financial_guarantee: ::std::vec::Vec<FinancialGuaranteeType>,
    #[serde(default, rename = "ProcurementLegislationDocumentReference")]
    pub procurement_legislation_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "FiscalLegislationDocumentReference")]
    pub fiscal_legislation_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "EnvironmentalLegislationDocumentReference")]
    pub environmental_legislation_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "EmploymentLegislationDocumentReference")]
    pub employment_legislation_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "ContractualDocumentReference")]
    pub contractual_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "CallForTendersDocumentReference")]
    pub call_for_tenders_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "WarrantyValidityPeriod")]
    pub warranty_validity_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<PaymentTermsType>,
    #[serde(default, rename = "TendererQualificationRequest")]
    pub tenderer_qualification_request: ::std::vec::Vec<TendererQualificationRequestType>,
    #[serde(default, rename = "AllowedSubcontractTerms")]
    pub allowed_subcontract_terms: ::std::vec::Vec<SubcontractTermsType>,
    #[serde(default, rename = "TenderPreparation")]
    pub tender_preparation: ::std::vec::Vec<TenderPreparationType>,
    #[serde(default, rename = "ContractExecutionRequirement")]
    pub contract_execution_requirement: ::std::vec::Vec<ContractExecutionRequirementType>,
    #[serde(default, rename = "AwardingTerms")]
    pub awarding_terms: ::core::option::Option<AwardingTermsType>,
    #[serde(default, rename = "AdditionalInformationParty")]
    pub additional_information_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "DocumentProviderParty")]
    pub document_provider_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "TenderRecipientParty")]
    pub tender_recipient_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "ContractResponsibleParty")]
    pub contract_responsible_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "TenderEvaluationParty")]
    pub tender_evaluation_party: ::std::vec::Vec<PartyType>,
    #[serde(default, rename = "QualificationRequestRecipientParty")]
    pub qualification_request_recipient_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "TenderValidityPeriod")]
    pub tender_validity_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "ContractAcceptancePeriod")]
    pub contract_acceptance_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "AppealTerms")]
    pub appeal_terms: ::core::option::Option<AppealTermsType>,
    #[serde(default, rename = "Language")]
    pub language: ::std::vec::Vec<LanguageType>,
    #[serde(default, rename = "BudgetAccountLine")]
    pub budget_account_line: ::std::vec::Vec<BudgetAccountLineType>,
    #[serde(default, rename = "ReplacedNoticeDocumentReference")]
    pub replaced_notice_document_reference: ::core::option::Option<DocumentReferenceType>,
    #[serde(default, rename = "LotDistribution")]
    pub lot_distribution: ::core::option::Option<LotDistributionType>,
    #[serde(default, rename = "PostAwardProcess")]
    pub post_award_process: ::core::option::Option<PostAwardProcessType>,
    #[serde(default, rename = "EconomicOperatorShortList")]
    pub economic_operator_short_list: ::core::option::Option<EconomicOperatorShortListType>,
    #[serde(default, rename = "SecurityClearanceTerm")]
    pub security_clearance_term: ::std::vec::Vec<SecurityClearanceTermType>,
}
pub type TerminalOperatorParty = PartyType;
pub type TimeDuty = DutyType;
pub type ToLocation = LocationType;
pub type TotalCapacityDimension = DimensionType;
pub type TradeFinancing = TradeFinancingType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TradeFinancingType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "FinancingInstrumentCode")]
    pub financing_instrument_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ContractDocumentReference")]
    pub contract_document_reference: ::core::option::Option<DocumentReferenceType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(rename = "FinancingParty")]
    pub financing_party: PartyType,
    #[serde(default, rename = "FinancingFinancialAccount")]
    pub financing_financial_account: ::core::option::Option<FinancialAccountType>,
    #[serde(default, rename = "Clause")]
    pub clause: ::std::vec::Vec<ClauseType>,
}
pub type TradePartyName = PartyNameType;
pub type TradingTerms = TradingTermsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TradingTermsType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Information")]
    pub information: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Reference")]
    pub reference: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ApplicableAddress")]
    pub applicable_address: ::core::option::Option<AddressType>,
}
pub type TransactionConditions = TransactionConditionsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TransactionConditionsType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ActionCode")]
    pub action_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
}
pub type TransitCountry = CountryType;
pub type TransitCustomsExitOfficeLocation = LocationType;
pub type TransitExporterParty = PartyType;
pub type TransitPeriod = PeriodType;
pub type TransportAdvisorParty = PartyType;
pub type TransportContract = ContractType;
pub type TransportEquipment = TransportEquipmentType;
pub type TransportEquipmentSeal = TransportEquipmentSealType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TransportEquipmentSealType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "SealIssuerTypeCode")]
    pub seal_issuer_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Condition")]
    pub condition: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "SealStatusCode")]
    pub seal_status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "SealingPartyType")]
    pub sealing_party_type: ::core::option::Option<super::cct::TextType>,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TransportEquipmentType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "ReferencedConsignmentID")]
    pub referenced_consignment_id: ::std::vec::Vec<super::cct::IdentifierType>,
    #[serde(default, rename = "TransportEquipmentTypeCode")]
    pub transport_equipment_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "ProviderTypeCode")]
    pub provider_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "OwnerTypeCode")]
    pub owner_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "SizeTypeCode")]
    pub size_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "DispositionCode")]
    pub disposition_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "FullnessIndicationCode")]
    pub fullness_indication_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "RefrigerationOnIndicator")]
    pub refrigeration_on_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "Information")]
    pub information: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ReturnabilityIndicator")]
    pub returnability_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "LegalStatusIndicator")]
    pub legal_status_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "AirFlowPercent")]
    pub air_flow_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "HumidityPercent")]
    pub humidity_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "AnimalFoodApprovedIndicator")]
    pub animal_food_approved_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "HumanFoodApprovedIndicator")]
    pub human_food_approved_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "DangerousGoodsApprovedIndicator")]
    pub dangerous_goods_approved_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "RefrigeratedIndicator")]
    pub refrigerated_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "Characteristics")]
    pub characteristics: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "DamageRemarks")]
    pub damage_remarks: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "SpecialTransportRequirements")]
    pub special_transport_requirements: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "GrossWeightMeasure")]
    pub gross_weight_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "GrossVolumeMeasure")]
    pub gross_volume_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "TareWeightMeasure")]
    pub tare_weight_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "TrackingDeviceCode")]
    pub tracking_device_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "PowerIndicator")]
    pub power_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "TraceID")]
    pub trace_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "StowagePositionID")]
    pub stowage_position_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: ::std::vec::Vec<DimensionType>,
    #[serde(default, rename = "TransportEquipmentSeal")]
    pub transport_equipment_seal: ::std::vec::Vec<TransportEquipmentSealType>,
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: ::core::option::Option<TemperatureType>,
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: ::core::option::Option<TemperatureType>,
    #[serde(default, rename = "ProviderParty")]
    pub provider_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "LoadingProofParty")]
    pub loading_proof_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "SupplierParty")]
    pub supplier_party: ::core::option::Option<SupplierPartyType>,
    #[serde(default, rename = "OwnerParty")]
    pub owner_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "OperatingParty")]
    pub operating_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "LoadingLocation")]
    pub loading_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "UnloadingLocation")]
    pub unloading_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "StorageLocation")]
    pub storage_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "PositioningTransportEvent")]
    pub positioning_transport_event: ::std::vec::Vec<TransportEventType>,
    #[serde(default, rename = "QuarantineTransportEvent")]
    pub quarantine_transport_event: ::std::vec::Vec<TransportEventType>,
    #[serde(default, rename = "DeliveryTransportEvent")]
    pub delivery_transport_event: ::std::vec::Vec<TransportEventType>,
    #[serde(default, rename = "PickupTransportEvent")]
    pub pickup_transport_event: ::std::vec::Vec<TransportEventType>,
    #[serde(default, rename = "HandlingTransportEvent")]
    pub handling_transport_event: ::std::vec::Vec<TransportEventType>,
    #[serde(default, rename = "LoadingTransportEvent")]
    pub loading_transport_event: ::std::vec::Vec<TransportEventType>,
    #[serde(default, rename = "TransportEvent")]
    pub transport_event: ::std::vec::Vec<TransportEventType>,
    #[serde(default, rename = "ApplicableTransportMeans")]
    pub applicable_transport_means: ::core::option::Option<TransportMeansType>,
    #[serde(default, rename = "HaulageTradingTerms")]
    pub haulage_trading_terms: ::std::vec::Vec<TradingTermsType>,
    #[serde(default, rename = "HazardousGoodsTransit")]
    pub hazardous_goods_transit: ::std::vec::Vec<HazardousGoodsTransitType>,
    #[serde(default, rename = "PackagedTransportHandlingUnit")]
    pub packaged_transport_handling_unit: ::std::vec::Vec<TransportHandlingUnitType>,
    #[serde(default, rename = "ServiceAllowanceCharge")]
    pub service_allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "FreightAllowanceCharge")]
    pub freight_allowance_charge: ::std::vec::Vec<AllowanceChargeType>,
    #[serde(default, rename = "AttachedTransportEquipment")]
    pub attached_transport_equipment: ::std::vec::Vec<TransportEquipmentType>,
    #[serde(default, rename = "Delivery")]
    pub delivery: ::core::option::Option<::std::boxed::Box<DeliveryType>>,
    #[serde(default, rename = "Pickup")]
    pub pickup: ::core::option::Option<PickupType>,
    #[serde(default, rename = "Despatch")]
    pub despatch: ::core::option::Option<DespatchType>,
    #[serde(default, rename = "ShipmentDocumentReference")]
    pub shipment_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "ContainedInTransportEquipment")]
    pub contained_in_transport_equipment: ::std::vec::Vec<TransportEquipmentType>,
    #[serde(default, rename = "Package")]
    pub package: ::std::vec::Vec<PackageType>,
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: ::std::vec::Vec<GoodsItemType>,
    #[serde(default, rename = "VerifiedGrossMass")]
    pub verified_gross_mass: ::core::option::Option<VerifiedGrossMassType>,
    #[serde(default, rename = "LoadedHazardousItem")]
    pub loaded_hazardous_item: ::std::vec::Vec<HazardousItemType>,
}
pub type TransportEvent = TransportEventType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TransportEventType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "IdentificationID")]
    pub identification_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "OccurrenceDate")]
    pub occurrence_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "OccurrenceTime")]
    pub occurrence_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "TransportEventTypeCode")]
    pub transport_event_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "CompletionIndicator")]
    pub completion_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "ReportedShipment")]
    pub reported_shipment: ::core::option::Option<::std::boxed::Box<ShipmentType>>,
    #[serde(default, rename = "CurrentStatus")]
    pub current_status: ::std::vec::Vec<StatusType>,
    #[serde(default, rename = "ResponsibleParty")]
    pub responsible_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "Contact")]
    pub contact: ::std::vec::Vec<ContactType>,
    #[serde(default, rename = "Location")]
    pub location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "Signature")]
    pub signature: ::core::option::Option<SignatureType>,
    #[serde(default, rename = "Period")]
    pub period: ::std::vec::Vec<PeriodType>,
}
pub type TransportExecutionPlanDocumentReference = DocumentReferenceType;
pub type TransportExecutionPlanRequestDocumentReference = DocumentReferenceType;
pub type TransportExecutionTerms = TransportExecutionTermsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TransportExecutionTermsType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "TransportUserSpecialTerms")]
    pub transport_user_special_terms: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TransportServiceProviderSpecialTerms")]
    pub transport_service_provider_special_terms: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ChangeConditions")]
    pub change_conditions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PaymentTerms")]
    pub payment_terms: ::std::vec::Vec<PaymentTermsType>,
    #[serde(default, rename = "DeliveryTerms")]
    pub delivery_terms: ::std::vec::Vec<DeliveryTermsType>,
    #[serde(default, rename = "BonusPaymentTerms")]
    pub bonus_payment_terms: ::core::option::Option<PaymentTermsType>,
    #[serde(default, rename = "CommissionPaymentTerms")]
    pub commission_payment_terms: ::core::option::Option<PaymentTermsType>,
    #[serde(default, rename = "PenaltyPaymentTerms")]
    pub penalty_payment_terms: ::core::option::Option<PaymentTermsType>,
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: ::std::vec::Vec<EnvironmentalEmissionType>,
    #[serde(default, rename = "NotificationRequirement")]
    pub notification_requirement: ::std::vec::Vec<NotificationRequirementType>,
    #[serde(default, rename = "ServiceChargePaymentTerms")]
    pub service_charge_payment_terms: ::core::option::Option<PaymentTermsType>,
}
pub type TransportHandlingUnit = TransportHandlingUnitType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TransportHandlingUnitType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "TransportHandlingUnitTypeCode")]
    pub transport_handling_unit_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "HandlingCode")]
    pub handling_code: ::std::vec::Vec<super::cct::CodeType>,
    #[serde(default, rename = "HandlingInstructions")]
    pub handling_instructions: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "HazardousRiskIndicator")]
    pub hazardous_risk_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "TotalGoodsItemQuantity")]
    pub total_goods_item_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "TotalPackageQuantity")]
    pub total_package_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "DamageRemarks")]
    pub damage_remarks: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ShippingMarks")]
    pub shipping_marks: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TraceID")]
    pub trace_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "HandlingUnitDespatchLine")]
    pub handling_unit_despatch_line: ::std::vec::Vec<DespatchLineType>,
    #[serde(default, rename = "ActualPackage")]
    pub actual_package: ::std::vec::Vec<PackageType>,
    #[serde(default, rename = "ReceivedHandlingUnitReceiptLine")]
    pub received_handling_unit_receipt_line: ::std::vec::Vec<ReceiptLineType>,
    #[serde(default, rename = "TransportEquipment")]
    pub transport_equipment: ::std::vec::Vec<TransportEquipmentType>,
    #[serde(default, rename = "TransportMeans")]
    pub transport_means: ::std::vec::Vec<TransportMeansType>,
    #[serde(default, rename = "HazardousGoodsTransit")]
    pub hazardous_goods_transit: ::std::vec::Vec<HazardousGoodsTransitType>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: ::std::vec::Vec<DimensionType>,
    #[serde(default, rename = "MinimumTemperature")]
    pub minimum_temperature: ::core::option::Option<TemperatureType>,
    #[serde(default, rename = "MaximumTemperature")]
    pub maximum_temperature: ::core::option::Option<TemperatureType>,
    #[serde(default, rename = "GoodsItem")]
    pub goods_item: ::std::vec::Vec<GoodsItemType>,
    #[serde(default, rename = "FloorSpaceMeasurementDimension")]
    pub floor_space_measurement_dimension: ::core::option::Option<DimensionType>,
    #[serde(default, rename = "PalletSpaceMeasurementDimension")]
    pub pallet_space_measurement_dimension: ::core::option::Option<DimensionType>,
    #[serde(default, rename = "ShipmentDocumentReference")]
    pub shipment_document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "Status")]
    pub status: ::std::vec::Vec<StatusType>,
    #[serde(default, rename = "CustomsDeclaration")]
    pub customs_declaration: ::std::vec::Vec<CustomsDeclarationType>,
    #[serde(default, rename = "ReferencedShipment")]
    pub referenced_shipment: ::std::vec::Vec<ShipmentType>,
    #[serde(default, rename = "Package")]
    pub package: ::std::vec::Vec<PackageType>,
    #[serde(default, rename = "DamageDocumentationAttachment")]
    pub damage_documentation_attachment: ::std::vec::Vec<AttachmentType>,
    #[serde(default, rename = "EnergyConsumptionAllocation")]
    pub energy_consumption_allocation: ::std::vec::Vec<EnergyConsumptionAllocationType>,
}
pub type TransportMeans = TransportMeansType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TransportMeansType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "JourneyID")]
    pub journey_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "RegistrationNationalityID")]
    pub registration_nationality_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "RegistrationNationality")]
    pub registration_nationality: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "DirectionCode")]
    pub direction_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TransportMeansTypeCode")]
    pub transport_means_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TradeServiceCode")]
    pub trade_service_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Stowage")]
    pub stowage: ::core::option::Option<StowageType>,
    #[serde(default, rename = "AirTransport")]
    pub air_transport: ::core::option::Option<AirTransportType>,
    #[serde(default, rename = "RoadTransport")]
    pub road_transport: ::core::option::Option<RoadTransportType>,
    #[serde(default, rename = "RailTransport")]
    pub rail_transport: ::core::option::Option<RailTransportType>,
    #[serde(default, rename = "MaritimeTransport")]
    pub maritime_transport: ::core::option::Option<MaritimeTransportType>,
    #[serde(default, rename = "OwnerParty")]
    pub owner_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "MeasurementDimension")]
    pub measurement_dimension: ::std::vec::Vec<DimensionType>,
}
pub type TransportProgressStatusRequestDocumentReference = DocumentReferenceType;
pub type TransportSchedule = TransportScheduleType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TransportScheduleType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "SequenceNumeric")]
    pub sequence_numeric: super::cct::NumericType,
    #[serde(default, rename = "ReferenceDate")]
    pub reference_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ReferenceTime")]
    pub reference_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "ReliabilityPercent")]
    pub reliability_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "Remarks")]
    pub remarks: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "StatusLocation")]
    pub status_location: LocationType,
    #[serde(default, rename = "ActualArrivalTransportEvent")]
    pub actual_arrival_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "ActualDepartureTransportEvent")]
    pub actual_departure_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "EstimatedDepartureTransportEvent")]
    pub estimated_departure_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "EstimatedArrivalTransportEvent")]
    pub estimated_arrival_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "PlannedDepartureTransportEvent")]
    pub planned_departure_transport_event: ::core::option::Option<TransportEventType>,
    #[serde(default, rename = "PlannedArrivalTransportEvent")]
    pub planned_arrival_transport_event: ::core::option::Option<TransportEventType>,
}
pub type TransportServiceDescriptionDocumentReference = DocumentReferenceType;
pub type TransportServiceDescriptionRequestDocumentReference = DocumentReferenceType;
pub type TransportServiceProviderParty = PartyType;
pub type TransportServiceProviderResponseDeadlinePeriod = PeriodType;
pub type TransportServiceProviderResponseRequiredPeriod = PeriodType;
pub type TransportUserParty = PartyType;
pub type TransportUserResponseRequiredPeriod = PeriodType;
pub type TransportationSegment = TransportationSegmentType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TransportationSegmentType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "SequenceNumeric")]
    pub sequence_numeric: super::cct::NumericType,
    #[serde(default, rename = "TransportExecutionPlanReferenceID")]
    pub transport_execution_plan_reference_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(rename = "TransportationService")]
    pub transportation_service: TransportationServiceType,
    #[serde(rename = "TransportServiceProviderParty")]
    pub transport_service_provider_party: PartyType,
    #[serde(default, rename = "ReferencedConsignment")]
    pub referenced_consignment: ::core::option::Option<ConsignmentType>,
    #[serde(default, rename = "ShipmentStage")]
    pub shipment_stage: ::std::vec::Vec<ShipmentStageType>,
}
pub type TransportationService = TransportationServiceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct TransportationServiceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "TransportServiceCode")]
    pub transport_service_code: super::cct::CodeType,
    #[serde(default, rename = "TariffClassCode")]
    pub tariff_class_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Priority")]
    pub priority: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "FreightRateClassCode")]
    pub freight_rate_class_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TransportationServiceDescription")]
    pub transportation_service_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "TransportationServiceDetailsURI")]
    pub transportation_service_details_uri: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "NominationDate")]
    pub nomination_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "NominationTime")]
    pub nomination_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "SequenceNumeric")]
    pub sequence_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "TransportEquipment")]
    pub transport_equipment: ::std::vec::Vec<TransportEquipmentType>,
    #[serde(default, rename = "SupportedTransportEquipment")]
    pub supported_transport_equipment: ::std::vec::Vec<TransportEquipmentType>,
    #[serde(default, rename = "UnsupportedTransportEquipment")]
    pub unsupported_transport_equipment: ::std::vec::Vec<TransportEquipmentType>,
    #[serde(default, rename = "CommodityClassification")]
    pub commodity_classification: ::std::vec::Vec<CommodityClassificationType>,
    #[serde(default, rename = "SupportedCommodityClassification")]
    pub supported_commodity_classification: ::std::vec::Vec<CommodityClassificationType>,
    #[serde(default, rename = "UnsupportedCommodityClassification")]
    pub unsupported_commodity_classification: ::std::vec::Vec<CommodityClassificationType>,
    #[serde(default, rename = "TotalCapacityDimension")]
    pub total_capacity_dimension: ::core::option::Option<DimensionType>,
    #[serde(default, rename = "ShipmentStage")]
    pub shipment_stage: ::std::vec::Vec<ShipmentStageType>,
    #[serde(default, rename = "TransportEvent")]
    pub transport_event: ::std::vec::Vec<TransportEventType>,
    #[serde(default, rename = "ResponsibleTransportServiceProviderParty")]
    pub responsible_transport_service_provider_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "EnvironmentalEmission")]
    pub environmental_emission: ::std::vec::Vec<EnvironmentalEmissionType>,
    #[serde(default, rename = "EstimatedDurationPeriod")]
    pub estimated_duration_period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "ScheduledServiceFrequency")]
    pub scheduled_service_frequency: ::std::vec::Vec<ServiceFrequencyType>,
}
pub type TransportationStatusRequestDocumentReference = DocumentReferenceType;
pub type TransshipPortLocation = LocationType;
pub type TreatmentLocation = LocationType;
pub type UnloadingLocation = LocationType;
pub type UnloadingPortLocation = LocationType;
pub type UnstructuredPrice = UnstructuredPriceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct UnstructuredPriceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "PriceAmount")]
    pub price_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusivePriceAmount")]
    pub tax_inclusive_price_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TimeAmount")]
    pub time_amount: ::core::option::Option<super::cct::TextType>,
}
pub type UnsubscribeToProcedureDocumentReference = DocumentReferenceType;
pub type UnsupportedCommodityClassification = CommodityClassificationType;
pub type UnsupportedTransportEquipment = TransportEquipmentType;
pub type UpdatedDeliveryTransportEvent = TransportEventType;
pub type UpdatedPickupTransportEvent = TransportEventType;
pub type UptakeBallastWaterTransaction = BallastWaterTransactionType;
pub type UsabilityPeriod = PeriodType;
pub type UtilityConsumptionPoint = ConsumptionPointType;
pub type UtilityCustomerParty = PartyType;
pub type UtilityItem = UtilityItemType;
#[derive(Debug, Deserialize, Serialize)]
pub struct UtilityItemType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "SubscriberID")]
    pub subscriber_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "SubscriberType")]
    pub subscriber_type: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "SubscriberTypeCode")]
    pub subscriber_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "PackQuantity")]
    pub pack_quantity: ::core::option::Option<super::cct::QuantityType>,
    #[serde(default, rename = "PackSizeNumeric")]
    pub pack_size_numeric: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "ConsumptionType")]
    pub consumption_type: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "ConsumptionTypeCode")]
    pub consumption_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "CurrentChargeType")]
    pub current_charge_type: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "CurrentChargeTypeCode")]
    pub current_charge_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "OneTimeChargeType")]
    pub one_time_charge_type: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "OneTimeChargeTypeCode")]
    pub one_time_charge_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "TaxCategory")]
    pub tax_category: ::core::option::Option<TaxCategoryType>,
    #[serde(default, rename = "Contract")]
    pub contract: ::core::option::Option<ContractType>,
}
pub type UtilityMeter = MeterType;
pub type UtilitySupplierParty = PartyType;
pub type ValidityPeriod = PeriodType;
pub type VerifiedGrossMass = VerifiedGrossMassType;
#[derive(Debug, Deserialize, Serialize)]
pub struct VerifiedGrossMassType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "WeighingDate")]
    pub weighing_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "WeighingTime")]
    pub weighing_time: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(rename = "WeighingMethodCode")]
    pub weighing_method_code: super::cct::CodeType,
    #[serde(default, rename = "WeighingDeviceID")]
    pub weighing_device_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "WeighingDeviceType")]
    pub weighing_device_type: ::core::option::Option<super::cct::TextType>,
    #[serde(rename = "GrossMassMeasure")]
    pub gross_mass_measure: super::cct::MeasureType,
    #[serde(default, rename = "WeighingParty")]
    pub weighing_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "ShipperParty")]
    pub shipper_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "ResponsibleParty")]
    pub responsible_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
}
pub type VesselDynamics = VesselDynamicsType;
#[derive(Debug, Deserialize, Serialize)]
pub struct VesselDynamicsType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "NavigationStatusCode")]
    pub navigation_status_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "AtAnchorageIndicator")]
    pub at_anchorage_indicator: ::core::option::Option<super::udt::IndicatorType>,
    #[serde(default, rename = "CourseOverGroundDirection")]
    pub course_over_ground_direction: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "SpeedOverGroundMeasure")]
    pub speed_over_ground_measure: ::core::option::Option<super::cct::MeasureType>,
    #[serde(default, rename = "RateOfTurnMeasure")]
    pub rate_of_turn_measure: ::core::option::Option<super::cct::MeasureType>,
}
pub type VoucherDocumentReference = DocumentReferenceType;
pub type WhoAffectedAreaPortLocation = LocationType;
pub type WhoAffectedAreaVisit = WhoAffectedAreaVisitType;
#[derive(Debug, Deserialize, Serialize)]
pub struct WhoAffectedAreaVisitType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "VisitDate")]
    pub visit_date: super::udt::DateTimeType,
    #[serde(rename = "WHOAffectedAreaPortLocation")]
    pub who_affected_area_port_location: LocationType,
}
pub type WarehouseParty = PartyType;
pub type WarehousingTransportEvent = TransportEventType;
pub type WarrantyParty = PartyType;
pub type WarrantyValidityPeriod = PeriodType;
pub type WasteGenerated = WasteGeneratedType;
#[derive(Debug, Deserialize, Serialize)]
pub struct WasteGeneratedType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "WasteTypeCode")]
    pub waste_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "WasteTypeDescription")]
    pub waste_type_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "WasteMeasure")]
    pub waste_measure: super::cct::MeasureType,
    #[serde(default, rename = "MeasurementPeriod")]
    pub measurement_period: ::core::option::Option<PeriodType>,
}
pub type WasteNotificationDocumentReference = DocumentReferenceType;
pub type WasteProducerParty = PartyType;
pub type WebSite = WebSiteType;
pub type WebSiteAccess = WebSiteAccessType;
#[derive(Debug, Deserialize, Serialize)]
pub struct WebSiteAccessType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "URI")]
    pub uri: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(rename = "Password")]
    pub password: super::cct::TextType,
    #[serde(rename = "Login")]
    pub login: super::cct::TextType,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct WebSiteType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "Name")]
    pub name: ::core::option::Option<super::cct::TextType>,
    #[serde(default, rename = "Description")]
    pub description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "WebSiteTypeCode")]
    pub web_site_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(rename = "URI")]
    pub uri: super::cct::IdentifierType,
    #[serde(default, rename = "WebSiteAccess")]
    pub web_site_access: ::std::vec::Vec<WebSiteAccessType>,
}
pub type WeighingParty = PartyType;
pub type WeightStatementDocumentReference = DocumentReferenceType;
pub type WinningParty = WinningPartyType;
#[derive(Debug, Deserialize, Serialize)]
pub struct WinningPartyType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "Rank")]
    pub rank: ::core::option::Option<super::cct::TextType>,
    #[serde(rename = "Party")]
    pub party: PartyType,
}
pub type WithholdingTaxTotal = TaxTotalType;
pub type WitnessParty = PartyType;
pub type WorkOrderDocumentReference = DocumentReferenceType;
pub type WorkPhaseReference = WorkPhaseReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct WorkPhaseReferenceType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(default, rename = "ID")]
    pub id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "WorkPhaseCode")]
    pub work_phase_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "WorkPhase")]
    pub work_phase: ::std::vec::Vec<super::cct::TextType>,
    #[serde(default, rename = "ProgressPercent")]
    pub progress_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "StartDate")]
    pub start_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "EndDate")]
    pub end_date: ::core::option::Option<super::udt::DateTimeType>,
    #[serde(default, rename = "WorkOrderDocumentReference")]
    pub work_order_document_reference: ::std::vec::Vec<DocumentReferenceType>,
}
pub type WorkQuantityTotal = WorkQuantityTotalType;
#[derive(Debug, Deserialize, Serialize)]
pub struct WorkQuantityTotalType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "Quantity")]
    pub quantity: super::cct::QuantityType,
    #[serde(default, rename = "WorkTypeCode")]
    pub work_type_code: ::core::option::Option<super::cct::CodeType>,
    #[serde(default, rename = "WorkTypeDescription")]
    pub work_type_description: ::std::vec::Vec<super::cct::TextType>,
}
pub type WorkReportDocumentReference = DocumentReferenceType;
pub type WorkReportLine = WorkReportLineType;
pub type WorkReportLineReference = LineReferenceType;
#[derive(Debug, Deserialize, Serialize)]
pub struct WorkReportLineType {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        ::core::option::Option<super::ext::ubl_common_extension_components_25::UblExtensionsType>,
    #[serde(rename = "ID")]
    pub id: super::cct::IdentifierType,
    #[serde(default, rename = "WorkItemID")]
    pub work_item_id: ::core::option::Option<super::cct::IdentifierType>,
    #[serde(default, rename = "WorkItemDescription")]
    pub work_item_description: ::std::vec::Vec<super::cct::TextType>,
    #[serde(rename = "Quantity")]
    pub quantity: super::cct::QuantityType,
    #[serde(default, rename = "LineExtensionAmount")]
    pub line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "TaxInclusiveLineExtensionAmount")]
    pub tax_inclusive_line_extension_amount: ::core::option::Option<super::cct::AmountType>,
    #[serde(default, rename = "CompletionPercent")]
    pub completion_percent: ::core::option::Option<super::cct::NumericType>,
    #[serde(default, rename = "ActivityOriginLocation")]
    pub activity_origin_location: ::core::option::Option<LocationType>,
    #[serde(default, rename = "Period")]
    pub period: ::core::option::Option<PeriodType>,
    #[serde(default, rename = "PerformingParty")]
    pub performing_party: ::core::option::Option<PartyType>,
    #[serde(default, rename = "WorkPhaseReference")]
    pub work_phase_reference: ::core::option::Option<WorkPhaseReferenceType>,
    #[serde(default, rename = "DocumentReference")]
    pub document_reference: ::std::vec::Vec<DocumentReferenceType>,
    #[serde(default, rename = "Price")]
    pub price: ::core::option::Option<PriceType>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: ::std::vec::Vec<TaxTotalType>,
    #[serde(default, rename = "SubWorkReportLine")]
    pub sub_work_report_line: ::std::vec::Vec<WorkReportLineType>,
}
