//! Auto-generated documentation from UBL 2.5 XSD annotations.
//! 310 types from cac.

/// A class to associate a time period and locations (activity data) with an item for inventory planning purposes.
///
/// **UBL Dictionary Entry Name:** `Activity Data Line. Details`
///
/// Generated from XSD type `ActivityDataLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this activity data line.
/// - `SupplyChainActivityTypeCode` — A code signifying the type of supply chain activity.
/// - `BuyerCustomerParty` — The buyer of the item.
/// - `SellerSupplierParty` — The seller of the item.
/// - `ActivityPeriod` — The period during which the activity is realized.
/// - `ActivityOriginLocation` — Either the location where the movement of goods is observed or the location from which the goods are moved.
/// - `ActivityFinalLocation` — The location to which the goods are moved.
/// - `SalesItem` — Sales information for an item to which this line applies.
// pub struct ActivityDataLine { ... }

/// A class to define a name/value pair for a property of an inventory planning activity.
///
/// **UBL Dictionary Entry Name:** `Activity Property. Details`
///
/// Generated from XSD type `ActivityPropertyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Name` — The name of this activity property.
/// - `Value` — The value of this activity property.
// pub struct ActivityProperty { ... }

/// A class to define common information related to an address.
///
/// **UBL Dictionary Entry Name:** `Address. Details`
///
/// Generated from XSD type `AddressType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this address within an agreed scheme of address identifiers.
/// - `AddressTypeCode` — A mutually agreed code signifying the type of this address.
/// - `AddressFormatCode` — A mutually agreed code signifying the format of this address.
/// - `Postbox` — A post office box number registered for postal delivery by a postal service provider.
/// - `Floor` — An identifiable floor of a building.
/// - `Room` — An identifiable room, suite, or apartment of a building.
/// - `StreetName` — The name of the street, road, avenue, way, etc. to which the number of the building is attached (may be repeated only to provide the same content in multiple natural languages).
/// - `AdditionalStreetName` — An additional street name used to further clarify the address (may be repeated only to provide the same content in multiple natural languages).
/// - `BlockName` — The name of the block (an area surrounded by streets and usually containing several buildings) in which this address is located.
/// - `BuildingName` — The name of a building.
/// - `BuildingNumber` — The number of a building within the street.
/// - `Description` — Text describing this address for clarification or specificity
/// - `InhouseMail` — The specific identifable location within a building where mail is delivered.
/// - `Department` — The department of the addressee.
/// - `MarkAttention` — The name, expressed as text, of a person or department in an organization to whose attention incoming mail is directed; corresponds to the printed forms "for the attention of", "FAO", and ATTN:".
/// - `MarkCare` — The name, expressed as text, of a person or organization at this address into whose care incoming mail is entrusted; corresponds to the printed forms "care of" and "c/o".
/// - `PlotIdentification` — An identifier (e.g., a parcel number) for the piece of land associated with this address.
/// - `CitySubdivisionName` — The name of the subdivision of a city, town, or village in which this address is located, such as the name of its district or borough.
/// - `CityName` — The name of a city, town, or village (may be repeated only to provide the same content in multiple natural languages).
/// - `PostalZone` — The postal identifier for this address according to the relevant national postal service, such as a ZIP code or Post Code (may be repeated only to provide the same content in multiple natural languages).
/// - `CountrySubentity` — The political or administrative division of a country in which this address is located, such as the name of its county, province, or state, expressed as text (may be repeated only to provide the same content in multiple natural languages).
/// - `CountrySubentityCode` — The political or administrative division of a country in which this address is located, such as a county, province, or state, expressed as a code (typically nationally agreed).
/// - `Region` — The recognized geographic or economic region or group of countries in which this address is located.
/// - `District` — The district or geographical division of a country or region in which this address is located.
/// - `TimezoneOffset` — The time zone in which this address is located (as an offset from Universal Coordinated Time (UTC)) at the time of exchange.
/// - `AddressLine` — A single address line expressed as unstructured text.
/// - `Country` — The country in which this address is situated.
/// - `LocationCoordinate` — The geographical coordinates of this address.
// pub struct Address { ... }

/// A class to define an unstructured address line.
///
/// **UBL Dictionary Entry Name:** `Address Line. Details`
///
/// Generated from XSD type `AddressLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Line` — An address line expressed as unstructured text (may be repeated only to provide the same content in multiple natural languages).
// pub struct AddressLine { ... }

/// A class to identify a specific aircraft used for transportation.
///
/// **UBL Dictionary Entry Name:** `Air Transport. Details`
///
/// Generated from XSD type `AirTransportType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `AircraftID` — An identifer for a specific aircraft.
// pub struct AirTransport { ... }

/// A class to describe information about a charge or discount as applied to a price component.
///
/// **UBL Dictionary Entry Name:** `Allowance Charge. Details`
///
/// Generated from XSD type `AllowanceChargeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this allowance or charge.
/// - `ChargeIndicator` — An indicator that this AllowanceCharge describes a charge (true) or a discount (false).
/// - `AllowanceChargeReasonCode` — A mutually agreed code signifying the reason for this allowance or charge.
/// - `AllowanceChargeReason` — The reason for this allowance or charge.
/// - `MultiplierFactorNumeric` — A number by which the base amount is multiplied to calculate the actual amount of this allowance or charge.
/// - `PrepaidIndicator` — An indicator that this allowance or charge is prepaid (true) or not (false).
/// - `SequenceNumeric` — A number indicating the order of this allowance or charge in the sequence of calculations applied when there are multiple allowances or charges.
/// - `Amount` — The monetary amount of this allowance or charge to be applied.
/// - `TaxInclusiveAmount` — The monetary amount of this allowance or charge inclusive of all taxes.
/// - `BaseAmount` — The monetary amount to which the multiplier factor is applied in calculating the amount of this allowance or charge.
/// - `AccountingCostCode` — The accounting cost centre used by the buyer to account for this allowance or charge, expressed as a code.
/// - `AccountingCost` — The accounting cost centre used by the buyer to account for this allowance or charge, expressed as text.
/// - `PerUnitAmount` — The allowance or charge per item; the total allowance or charge is calculated by multiplying the per unit amount by the quantity of items, either at the level of the individual transaction line or for the total number of items in the document, depending on the context in which it appears.
/// - `TaxCategory` — A tax category applicable to this allowance or charge.
/// - `TaxTotal` — The total of all the taxes applicable to this allowance or charge.
/// - `PaymentMeans` — A means of payment for this allowance or charge.
// pub struct AllowanceCharge { ... }

/// A class to define a structured annotation providing contextual or explanatory information related to a document or other business object
///
/// **UBL Dictionary Entry Name:** `Annotation. Details`
///
/// Generated from XSD type `AnnotationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `SubjectCode` — A code identifying the subject of the Annotation.
/// - `Subject` — A textual description identifying the subject of the Annotation.
/// - `AnnotationContent` — The textual content of the annotation providing information or context.
// pub struct Annotation { ... }

/// A class to describe the terms and conditions, set by the contracting authority, under which an appeal can be lodged for a tender award.
///
/// **UBL Dictionary Entry Name:** `Appeal Terms. Details`
///
/// Generated from XSD type `AppealTermsType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Description` — Text describing the terms of an appeal.
/// - `PresentationPeriod` — The period during which an appeal can be presented.
/// - `AppealInformationParty` — The Party who presents the information for the appeal.
/// - `AppealReceiverParty` — The Party who receives the appeal.
/// - `MediationParty` — The Party who mediates any appeal.
// pub struct AppealTerms { ... }

/// A class to describe an attached document. An attachment can refer to an external document or be included with the document being exchanged.
///
/// **UBL Dictionary Entry Name:** `Attachment. Details`
///
/// Generated from XSD type `AttachmentType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `EmbeddedDocumentBinaryObject` — A binary large object containing an attached document.
/// - `EmbeddedDocument` — A clear text object containing an attached document.
/// - `FileName` — The filename of the attachment.
/// - `ExternalReference` — A reference to an attached document that is external to the document(s) being exchanged.
// pub struct Attachment { ... }

/// A class describing an attestation made for an item
///
/// **UBL Dictionary Entry Name:** `Attestation. Details`
///
/// Generated from XSD type `AttestationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this attestation.
/// - `Name` — A name of this attestation.
/// - `Description` — A textual description of this attestation.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `AcceptanceIndicator` — Indicates whether the attestation has been accepted or not.
/// - `ValidityPeriod` — The period during which this attestation is valid
/// - `IssuerParty` — The Party who issues this Attestation
/// - `AttestationLine` — An attestation or statement made and which forms part of this attestation
// pub struct Attestation { ... }

/// A class describing an attestation line
///
/// **UBL Dictionary Entry Name:** `Attestation Line. Details`
///
/// Generated from XSD type `AttestationLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this attestation line
/// - `TypeCode` — A code describing the type of attestation line or statement
/// - `Description` — A textual description of this attestation line
/// - `CriterionItem` — Criterion items associated with this attestation line
/// - `SubAttestationLine` — An attestation line subsidiary to this attestation line
// pub struct AttestationLine { ... }

/// A class to describe the terms to be fulfilled by tenderers if an auction is to be executed before the awarding of a tender.
///
/// **UBL Dictionary Entry Name:** `Auction Terms. Details`
///
/// Generated from XSD type `AuctionTermsType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `AuctionConstraintIndicator` — Indicates whether an electronic auction will be used before the awarding of a contract (true) or not (false).
/// - `JustificationDescription` — Text describing a justification for the use of an auction in awarding the tender.
/// - `Description` — Text for tenderers describing terms governing the auction.
/// - `ProcessDescription` — Text describing the auction process.
/// - `ConditionsDescription` — Text describing the conditions under which the tenderers will be able to bid as part of the auction.
/// - `ElectronicDeviceDescription` — Text describing an electronic device used for the auction, including associated connectivity specifications.
/// - `AuctionURI` — The Uniform Resource Identifier (URI) of the electronic device used for the auction.
// pub struct AuctionTerms { ... }

/// A class to define an authorization that as been issued
///
/// **UBL Dictionary Entry Name:** `Authorization. Details`
///
/// Generated from XSD type `AuthorizationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `PurposeCode` — A code defining the business purpose or scope of this authorization
/// - `Purpose` — The purpose or scope of this authorization expressed as a text
/// - `ValidityPeriod` — The period during which this authorization is valid
/// - `Certificate` — One or more certificates related to this authorization
// pub struct Authorization { ... }

/// A class to define a criterion from the contracting party that will be taken into account when awarding a contract. An awarding criterion can be objective, when it can be evaluated following a formula, or subjective, when human analysis is required.
///
/// **UBL Dictionary Entry Name:** `Awarding Criterion. Details`
///
/// Generated from XSD type `AwardingCriterionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — Identifies a specific awarding criterion.
/// - `AwardingCriterionTypeCode` — A code used to define this awarding criterion.
/// - `Name` — The name of this awarding criterion.
/// - `Description` — A description of the awarding criterion.
/// - `WeightNumeric` — A number defining the comparative weighting assigned to this awarding criterion, to enable formulaic evaluation.
/// - `Weight` — A description of the comparative weighting for this awarding criterion.
/// - `CalculationExpression` — The mathematical expression that will be used to evaluate this criterion.
/// - `CalculationExpressionCode` — A code identifying the mathematical expression that will be used to evaluate this criterion.
/// - `MinimumQuantity` — The minimum quantity for an awarding criterion.
/// - `MaximumQuantity` — The maximum quantity for an awarding criterion.
/// - `MinimumAmount` — The minimum monetary amount for an awarding criterion.
/// - `MaximumAmount` — The maximum monetary amount for an awarding criterion.
/// - `MinimumImprovementBid` — Describes the minimum improvement bid for this awarding criterion when used in an auction.
/// - `SubordinateAwardingCriterion` — Defines any subsidiary awarding criterion.
// pub struct AwardingCriterion { ... }

/// Defines the response for an awarding criterion from the tendering party.
///
/// **UBL Dictionary Entry Name:** `Awarding Criterion Response. Details`
///
/// Generated from XSD type `AwardingCriterionResponseType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identification of this awarding criterion response.
/// - `AwardingCriterionID` — An identifer of the awarding criterion being referred to.
/// - `AwardingCriterionDescription` — Describes the awarding criterion.
/// - `Description` — Describes the awarding criterion response.
/// - `Quantity` — Specifies the quantity tendered for this awarding criterion.
/// - `Amount` — Specifies the monetary amount tendered for this awarding criterion.
/// - `SubordinateAwardingCriterionResponse` — Defines responses to any subsidiary awarding criterion.
// pub struct AwardingCriterionResponse { ... }

/// A class to define the terms for awarding a contract.
///
/// **UBL Dictionary Entry Name:** `Awarding Terms. Details`
///
/// Generated from XSD type `AwardingTermsType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `WeightingAlgorithmCode` — A code signifying the weighting algorithm for awarding criteria. When multiple awarding criteria is used, different weighting and choices management algorithms based upon scores and weights of all award criteria can be used. An algorithm for weighting criteria will be reported in the call for tenders document. It is used to determine how to perform the final management of tenders based on the results in each of the established award criteria
/// - `Description` — Text describing terms under which the contract is to be awarded.
/// - `TechnicalCommitteeDescription` — Text describing the committee of experts evaluating the subjective criteria for awarding the contract.
/// - `LowTendersDescription` — Text describing the exclusion criterion for abnormally low tenders.
/// - `PrizeIndicator` — Indicates whether a prize will be awarded (true) or not (false).
/// - `PrizeDescription` — Number and value of the prizes to be awarded.
/// - `PaymentDescription` — Details of payments to all participants.
/// - `FollowupContractIndicator` — Indicates if any service contract following the contest will be awarded to the winner or one of the winners of the contest (true) or not (false).
/// - `BindingOnBuyerIndicator` — Indicates if the decision is binding on the buyer (true) or not (false).
/// - `NoFurtherNegotiationIndicator` — Indicates if no further negotiation is allowed (true) or not (false).
/// - `AwardingCriterion` — Defines a criterion for awarding this tender.
/// - `TechnicalCommitteePerson` — A member of a committee of experts evaluating the subjective criteria for awarding the contract.
/// - `Prize` — Information about the value amount that will be offered to the winner depending on his rank.
// pub struct AwardingTerms { ... }

/// A class to summarize the ballast water management of a vessel.
///
/// **UBL Dictionary Entry Name:** `Ballast Water Summary. Details`
///
/// Generated from XSD type `BallastWaterSummaryType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identification of this ballast water summary.
/// - `ManagementPlanOnBoardIndicator` — An indication of whether a waste water management plan is on board (true) or not (false).
/// - `ManagementPlanImplementedIndicator` — An indication of whether a waste water management plan has been implemented (true) or not (false).
/// - `IMOGuidelinesOnBoardIndicator` — An indication of whether International Maritime Organization (IMO) ballast water guidelines are on board (true) or not (false).
/// - `TotalBallastTanksOnBoardQuantity` — The total number of ballast tanks on board the vessel.
/// - `TanksInBallastQuantity` — The number of tanks in the ballast of the vessel.
/// - `TanksExchangedQuantity` — The number of tanks being exchanged as part of this ballast water summary.
/// - `TanksNotExchangedQuantity` — The quantity of tanks not being exchanged.
/// - `TotalBallastWaterOnBoardMeasure` — The messure of the total ballast water on board the vessel.
/// - `TotalBallastWaterCapacityMeasure` — The total ballast water capacity of the vessel.
/// - `OtherControlActions` — A text describing any other control actions that are part of this ballast water summary.
/// - `NoControlActionsReason` — A textual description of the reason if no control actions are being taken.
/// - `UptakeBallastWaterTransaction` — The uptake ballast water transaction.
/// - `ExchangeBallastWaterTransaction` — The exchange ballast water transaction.
/// - `DischargeBallastWaterTransaction` — The discharge ballast water transaction.
/// - `ResponsibleOfficerPerson` — The officer responsible for this ballast water summery.
// pub struct BallastWaterSummary { ... }

/// A class to define a ballast water transaction, such as the uptake, exchange or discharge of ballast water.
///
/// **UBL Dictionary Entry Name:** `Ballast Water Transaction. Details`
///
/// Generated from XSD type `BallastWaterTransactionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `TankID` — An identifier for the ballast water tank being used in this ballast water transaction.
/// - `TankTypeCode` — A code for the type of ballast water tank being used in the ballast water transaction.
/// - `ExchangeMethodCode` — A code expressing how ballast water is being filled into or discharged from the tank.
/// - `ExchangedPercent` — The percentage of the ballast water in the tank being exchanged in this ballast water transaction.
/// - `VolumeMeasure` — The volume of ballast water being exchanged in this ballast water transaction.
/// - `SeaHeightMeasure` — A measure of the sea height at the time of the transaction.
/// - `SalinityMeasure` — A measure for the salinity of the water in the tank.
/// - `TransactionDate` — The date when this ballast water transaction takes place.
/// - `Location` — The location where this ballast water transaction takes place.
/// - `BallastWaterTemperature` — The temperature of the ballast water at time of transaction.
// pub struct BallastWaterTransaction { ... }

/// A class to define a reference to a billing document.
///
/// **UBL Dictionary Entry Name:** `Billing Reference. Details`
///
/// Generated from XSD type `BillingReferenceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `InvoiceDocumentReference` — A reference to an invoice.
/// - `SelfBilledInvoiceDocumentReference` — A reference to a self billed invoice.
/// - `CreditNoteDocumentReference` — A reference to a credit note.
/// - `SelfBilledCreditNoteDocumentReference` — A reference to a self billed credit note.
/// - `DebitNoteDocumentReference` — A reference to a debit note.
/// - `ReminderDocumentReference` — A reference to a billing reminder.
/// - `AdditionalDocumentReference` — A reference to an additional document.
/// - `BillingReferenceLine` — A reference to a transaction line in the billing document.
// pub struct BillingReference { ... }

/// A class to define a reference to a transaction line in a billing document.
///
/// **UBL Dictionary Entry Name:** `Billing Reference Line. Details`
///
/// Generated from XSD type `BillingReferenceLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this transaction line in a billing document.
/// - `Amount` — The monetary amount of the transaction line, including any allowances and charges but excluding taxes.
/// - `AllowanceCharge` — An allowance or charge applicable to the transaction line.
// pub struct BillingReferenceLine { ... }

/// A class to describe a branch or a division of an organization.
///
/// **UBL Dictionary Entry Name:** `Branch. Details`
///
/// Generated from XSD type `BranchType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this branch or division of an organization.
/// - `Name` — The name of this branch or division of an organization.
/// - `FinancialInstitution` — The financial institution that this branch belongs to (if applicable).
/// - `Address` — The address of this branch or division.
// pub struct Branch { ... }

/// A class to define a budget account.
///
/// **UBL Dictionary Entry Name:** `Budget Account. Details`
///
/// Generated from XSD type `BudgetAccountType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for the budget account, typically an internal accounting reference.
/// - `BudgetYearNumeric` — The number of the year for this budget account, e.g. 2012
/// - `RequiredClassificationScheme` — A classification scheme required for this budget account.
// pub struct BudgetAccount { ... }

/// A class to define a budget account line.
///
/// **UBL Dictionary Entry Name:** `Budget Account Line. Details`
///
/// Generated from XSD type `BudgetAccountLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this budget account line.
/// - `TotalAmount` — The total monetary amount for this budget account line.
/// - `BudgetAccount` — An account covering this budget account line.
// pub struct BudgetAccountLine { ... }

/// A class to define a reference provided by the buyer for internal routing or classification.
///
/// **UBL Dictionary Entry Name:** `Buyer Assigned Reference. Details`
///
/// Generated from XSD type `BuyerAssignedReferenceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `BuyerReferenceCode` — A code identifying the buyer reference, such as a department or internal unit.
/// - `BuyerReference` — A textual description of the buyer reference.
// pub struct BuyerAssignedReference { ... }

/// A class to describe a specific capability of an organization.
///
/// **UBL Dictionary Entry Name:** `Capability. Details`
///
/// Generated from XSD type `CapabilityType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this Capability.
/// - `CapabilityTypeCode` — This class can be used as Financial or Technical capabilities. For instance, "Turnover" or "Qualified Engineers" are two possible codes.
/// - `Description` — Text describing this capability.
/// - `ValueAmount` — A monetary amount as a measure of this capability.
/// - `ValueQuantity` — A quantity as a measure of this capability.
/// - `EvidenceSupplied` — (Deprecated) The evidence that supports the capability claim.
/// - `SuppliedEvidence` — The Evidence that supports the capability claim.
/// - `ValidityPeriod` — The period of time for which this capability is (or has been) valid.
/// - `WebSite` — A web site where the capability is detailed.
// pub struct Capability { ... }

/// A class to define a credit card, debit card, or charge card account.
///
/// **UBL Dictionary Entry Name:** `Card Account. Details`
///
/// Generated from XSD type `CardAccountType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `PrimaryAccountNumberID` — An identifier of the card (e.g., the Primary Account Number (PAN)).
/// - `NetworkID` — An identifier for the financial service network provider of the card.
/// - `CardTypeCode` — A mutually agreed code signifying the type of card. Examples of types are "debit", "credit" and "purchasing"
/// - `ValidityStartDate` — The date from which the card is valid.
/// - `ExpiryDate` — The date on which the card expires.
/// - `IssuerID` — An identifier for the institution issuing the card.
/// - `IssueNumberID` — An identifier for the card, specified by the issuer.
/// - `CV2ID` — An identifier for the Card Verification Value (often found on the reverse of the card itself).
/// - `CardChipCode` — A mutually agreed code to distinguish between CHIP and MAG STRIPE cards.
/// - `ChipApplicationID` — An identifier on the chip card for the application that provides the quoted information; an AID (application ID).
/// - `HolderName` — The name of the cardholder.
/// - `RoleCode` — The role of this card or the card holder (e.g., the buyer, when the card is used as a payment means to pay for an item), expressed as a code.
// pub struct CardAccount { ... }

/// A class to define the cash register used in a commercial transaction.
///
/// **UBL Dictionary Entry Name:** `Cash Register. Details`
///
/// Generated from XSD type `CashRegisterType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — The identifier of this cash register.
/// - `SerialNumberID` — The serial number of this cash register.
// pub struct CashRegister { ... }

/// A class to define a line describing the transaction that updates the specification of an item in a catalogue.
///
/// **UBL Dictionary Entry Name:** `Catalogue Item Specification Update Line. Details`
///
/// Generated from XSD type `CatalogueItemSpecificationUpdateLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for the line to be updated in a catalogue.
/// - `ContractorCustomerParty` — The customer responsible for the contract associated with the catalogue item.
/// - `SellerSupplierParty` — The seller/supplier responsible for the contract associated with the catalogue item.
/// - `Item` — The catalogue item to be updated.
// pub struct CatalogueItemSpecificationUpdateLine { ... }

/// A class to define a line in a Catalogue describing a purchasable item.
///
/// **UBL Dictionary Entry Name:** `Catalogue Line. Details`
///
/// Generated from XSD type `CatalogueLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for the line in the catalogue.
/// - `ActionCode` — A code signifying the action required to synchronize this catalogue line. Recommend codes (delete, update, add)
/// - `LifeCycleStatusCode` — A code signifying the life cycle status of this catalogue line. Examples are pre-order, end of production
/// - `ContractSubdivision` — A subdivision of a contract or tender covering this catalogue line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `OrderableIndicator` — An indicator that this catalogue line describes an orderable item (true) or is included for reference purposes only (false).
/// - `OrderableUnit` — A textual description of the units in which the item described in this catalogue line can be ordered.
/// - `ContentUnitQuantity` — The numeric quantity of the ordering unit (and units of measure) of the catalogue line.
/// - `OrderQuantityIncrementNumeric` — The number of items that can set the order quantity increments.
/// - `MinimumOrderQuantity` — The minimum amount of the item described in this catalogue line that can be ordered.
/// - `MaximumOrderQuantity` — The maximum amount of the item described in this catalogue line that can be ordered.
/// - `WarrantyInformation` — Text about a warranty (provided by WarrantyParty) for the good or service described in this catalogue line.
/// - `PackLevelCode` — A mutually agreed code signifying the level of packaging associated with the item described in this catalogue line.
/// - `ContractorCustomerParty` — The customer responsible for the contract with which this catalogue line is associated.
/// - `SellerSupplierParty` — The seller/supplier responsible for the contract with which this catalogue line is associated.
/// - `WarrantyParty` — The Party who is responsible for any warranty associated with the item described in this Catalogue Line.
/// - `WarrantyValidityPeriod` — The period for which a warranty associated with the item in this catalogue line is valid.
/// - `LineValidityPeriod` — The period for which the information in this catalogue line is valid.
/// - `ItemComparison` — A combination of price and quantity used to provide price comparisons based on different sizes of order.
/// - `ComponentRelatedItem` — An item that may be a component of the item in this catalogue line.
/// - `AccessoryRelatedItem` — An item that may be an optional accessory of the item in this catalogue line.
/// - `RequiredRelatedItem` — An item that may be required for the item in this catalogue line.
/// - `ReplacementRelatedItem` — An item that may be a replacement for the item in this catalogue line.
/// - `ComplementaryRelatedItem` — An item that may complement the item in this catalogue line.
/// - `ReplacedRelatedItem` — An item in an existing catalogue that is being replaced by the item in this catalogue line.
/// - `RequiredItemLocationQuantity` — Properties of the item in this catalogue line that are dependent on location and quantity.
/// - `DocumentReference` — A reference to a document associated with this catalogue line.
/// - `Item` — A specification of the item itself.
/// - `KeywordItemProperty` — A property of the item in this catalogue line.
/// - `CallForTendersLineReference` — Reference to a Line on a Call For Tenders document.
/// - `CallForTendersDocumentReference` — One or more references to Call For Tenders documents.
// pub struct CatalogueLine { ... }

/// A class to define a line describing a pricing update to a catalogue line.
///
/// **UBL Dictionary Entry Name:** `Catalogue Pricing Update Line. Details`
///
/// Generated from XSD type `CataloguePricingUpdateLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for the catalogue line to be updated.
/// - `ContractorCustomerParty` — The customer responsible for the contract associated with the catalogue line being updated.
/// - `SellerSupplierParty` — The seller/supplier responsible for the contract associated with the catalogue line being updated.
/// - `RequiredItemLocationQuantity` — Updated properties of the item in this catalogue line that are dependent on location and quantity.
// pub struct CataloguePricingUpdateLine { ... }

/// A class to define a reference to a catalogue.
///
/// **UBL Dictionary Entry Name:** `Catalogue Reference. Details`
///
/// Generated from XSD type `CatalogueReferenceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for a specific catalogue.
/// - `UUID` — A universally unique identifier for a specific catalogue.
/// - `IssueDate` — The date on which the catalogue was issued.
/// - `IssueTime` — The time at which the catalogue was issued.
/// - `RevisionDate` — The date on which the information in the catalogue was last revised.
/// - `RevisionTime` — The time at which the information in the catalogue was last revised.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `Description` — Text describing the catalogue.
/// - `VersionID` — An identifier for the current version of the catalogue.
/// - `PreviousVersionID` — An identifier for the previous version of the catalogue that is superseded by this version.
// pub struct CatalogueReference { ... }

/// A class to define a line describing a request for a catalogue line.
///
/// **UBL Dictionary Entry Name:** `Catalogue Request Line. Details`
///
/// Generated from XSD type `CatalogueRequestLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for the requested catalogue line.
/// - `ContractSubdivision` — A subdivision of a contract or tender covering the line being requested.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `LineValidityPeriod` — The period for which the information in the requested catalogue line is valid.
/// - `RequiredItemLocationQuantity` — Properties of the item in the requested catalogue line that are dependent on location and quantity.
/// - `Item` — The item associated with the requested catalogue line.
// pub struct CatalogueRequestLine { ... }

/// A class to define a certificate applied to the item. Certificated can be a requirement to sell goods or services in a jurisdiction.
///
/// **UBL Dictionary Entry Name:** `Certificate. Details`
///
/// Generated from XSD type `CertificateType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this certificate.
/// - `CertificateTypeCode` — The type of this certificate, expressed as a code. The type specifies what array it belongs to, e.g.. Environmental, security, health improvement etc.
/// - `CertificateType` — The type of this certificate, expressed as a text.
/// - `CertificateReferenceID` — An identifier assigned by the issuing authority to reference this certificate in an external registry or official record.
/// - `ApplicableCategoryCode` — A code specifying the category of item or process to which this certificate applies.
/// - `ApplicableCategory` — A textual description of the category of item or process to which this certificate applies.
/// - `CertificateURI` — A textual description of the category of item or process to which this certificate applies.
/// - `Remarks` — Remarks by the applicant for this certificate.
/// - `IssuerParty` — The authorised Organisation who issues this Certificate.
/// - `CertificateValidityPeriod` — The period during which this certificate is valid.
/// - `DocumentReference` — A reference to a document relevant to this certificate or an application for this certificate.
/// - `Signature` — A signature applied to this certificate.
// pub struct Certificate { ... }

/// A class to define an application for a Certificate of Origin (CoO).
///
/// **UBL Dictionary Entry Name:** `Certificate Of Origin Application. Details`
///
/// Generated from XSD type `CertificateOfOriginApplicationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ReferenceID` — An identifier for a reference as part of the CoO application.
/// - `CertificateType` — The type of CoO being applied for (Ordinary, Re-export, Commonwealth Preferential, etc.).
/// - `ApplicationStatusCode` — A code signifying the status of the application (revision, replacement, etc.).
/// - `OriginalJobID` — The latest job number given to the CoO application. This is used by the system to keep track of amendments to or cancellation of any earlier applications.
/// - `PreviousJobID` — An identifier for the previous job used in case the application requires query or change.
/// - `Remarks` — Remarks by the applicant for the CoO.
/// - `Shipment` — The shipment of goods covered by the CoO.
/// - `EndorserParty` — A party providing an endorsement to the CoO.
/// - `PreparationParty` — The Party who prepares this Certificate of Origin Application. This Party is normally an individual, a group or a body.
/// - `IssuerParty` — The authorised Organisation who issues the Certificate of Origin requested by this application.
/// - `ExporterParty` — The Party who exports the goods or has similar right of disposal over them at the time of export.
/// - `ImporterParty` — The Party who imports the goods, or on whose behalf the goods are being imported.
/// - `IssuingCountry` — The country where the requested CoO will be issued.
/// - `DocumentDistribution` — An interested party to which the CoO is to be distributed.
/// - `SupportingDocumentReference` — A reference to a document supporting this application.
/// - `Signature` — A signature applied to this application.
// pub struct CertificateOfOriginApplication { ... }

/// A class to describe circular economy-related properties of a product, including its resource efficiency, reusability, repairability, and end-of-life treatment.
///
/// **UBL Dictionary Entry Name:** `Circularity Profile. Details`
///
/// Generated from XSD type `CircularityProfileType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `CircularityTypeCode` — A code indicating the type of circularity applicable to the profiled entity.
/// - `RecycledContentPercent` — The percentage of recycled material in the profiled entity.
/// - `RecyclabilityPercent` — The percentage of the product or material that can be recycled.
/// - `MaintenanceFrequencyCode` — A code specifying the recommended frequency for routine maintenance.
/// - `MaintenanceFrequencyDescription` — A text describing the recommended frequency for routine maintenance.
/// - `ResourceConsumption` — A record of a resource consumed, such as energy, water, or materials.
/// - `WasteGenerated` — The waste generated by the profiled entity.
/// - `RepairabilityScore` — An assigned Score indicating the ease of repair of the product.
/// - `EndOfLifeTreatment` — Information about how the profiled entity is expected to be treated at the end of its lifecycle
/// - `ProductDocumentationDocumentReference` — One or more product documentations such as repair instructions, sustainability disclosures, circularity declarations, or technical manuals.
// pub struct CircularityProfile { ... }

/// A class to define a category within a classification scheme.
///
/// **UBL Dictionary Entry Name:** `Classification Category. Details`
///
/// Generated from XSD type `ClassificationCategoryType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Name` — The name of this category within the classification scheme.
/// - `CodeValue` — The value of a code used to identify this category within the classification scheme.
/// - `Description` — Text describing this category.
/// - `CategorizesClassificationCategory` — A recursive description of a subcategory of this category.
// pub struct ClassificationCategory { ... }

/// A class to define a classification scheme, such as a taxonomy for classifying goods or services.
///
/// **UBL Dictionary Entry Name:** `Classification Scheme. Details`
///
/// Generated from XSD type `ClassificationSchemeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this classification scheme.
/// - `UUID` — A universally unique identifier for this classification scheme.
/// - `LastRevisionDate` — The date on which this classification scheme was last revised.
/// - `LastRevisionTime` — The time at which this classification scheme was last revised.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `Name` — The name of this classification scheme.
/// - `Description` — Text describing this classification scheme.
/// - `AgencyID` — An identifier for the agency that maintains this classification scheme.
/// - `AgencyName` — The name of the agency that maintains the classification scheme.
/// - `VersionID` — An identifier for the version of this classification scheme.
/// - `URI` — The Uniform Resource Identifier (URI) of the documentation for this classification scheme.
/// - `SchemeURI` — The Uniform Resource Identifier (URI) of this classification scheme.
/// - `LanguageID` — An identifier for the language of this classification scheme.
/// - `ClassificationCategory` — A description of a category within this classification scheme.
// pub struct ClassificationScheme { ... }

/// A class to define a clause (a distinct article or provision) in a contract, treaty, will, or other formal or legal written document requiring compliance.
///
/// **UBL Dictionary Entry Name:** `Clause. Details`
///
/// Generated from XSD type `ClauseType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this clause.
/// - `Content` — The text of this clause.
// pub struct Clause { ... }

/// A class to describe the classification of a commodity.
///
/// **UBL Dictionary Entry Name:** `Commodity Classification. Details`
///
/// Generated from XSD type `CommodityClassificationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `NatureCode` — A code defined by a specific maintenance agency signifying the high-level nature of the commodity.
/// - `CargoTypeCode` — A mutually agreed code signifying the type of cargo for purposes of commodity classification.
/// - `CommodityCode` — The harmonized international commodity code for cross border and regulatory (customs and trade statistics) purposes.
/// - `ItemClassificationCode` — A code signifying the trade classification of the commodity.
// pub struct CommodityClassification { ... }

/// A class to describe a means of communication.
///
/// **UBL Dictionary Entry Name:** `Communication. Details`
///
/// Generated from XSD type `CommunicationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ChannelCode` — The method of communication, expressed as a code.
/// - `Channel` — The method of communication, expressed as text.
/// - `Value` — An identifying value (phone number, email address, etc.) for this channel of communication
// pub struct Communication { ... }

/// A class to describe the completion of a specific task in the tendering process.
///
/// **UBL Dictionary Entry Name:** `Completed Task. Details`
///
/// Generated from XSD type `CompletedTaskType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `AnnualAverageAmount` — The average monetary amount of a task such as this completed task.
/// - `TotalTaskAmount` — The actual total monetary amount of this completed task.
/// - `PartyCapacityAmount` — A monetary amount corresponding to the financial capacity of the party that carried out this completed task.
/// - `Description` — Text describing this completed task.
/// - `EvidenceSupplied` — (Deprecated) The evidence justifying a designation of "complete" for this task.
/// - `SuppliedEvidence` — The Evidence justifying a designation of "complete" for this task.
/// - `Period` — The period in which this completed task was performed.
/// - `RecipientCustomerParty` — The original customer for this completed task.
// pub struct CompletedTask { ... }

/// A class to define a measurable condition of an object.
///
/// **UBL Dictionary Entry Name:** `Condition. Details`
///
/// Generated from XSD type `ConditionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `AttributeID` — An identifier for the attribute that applies to the condition.
/// - `Measure` — The measurement value.
/// - `Description` — Text describing the attribute that applies to the condition.
/// - `MinimumMeasure` — The minimum value in a range of measurement for this condition.
/// - `MaximumMeasure` — The maximum value in a range of measurement for this condition.
// pub struct Condition { ... }

/// A class to describe an identifiable collection of one or more goods items to be transported between the consignor and the consignee. This information may be defined within a transport contract. A consignment may comprise more than one shipment (e.g., when consolidated by a freight forwarder).
///
/// **UBL Dictionary Entry Name:** `Consignment. Details`
///
/// Generated from XSD type `ConsignmentType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier assigned to a collection of goods for both import and export.
/// - `CarrierAssignedID` — An identifier for this consignment, assigned by the carrier.
/// - `ConsigneeAssignedID` — An identifier for this consignment, assigned by the consignee.
/// - `ConsignorAssignedID` — An identifier for this consignment, assigned by the consignor.
/// - `FreightForwarderAssignedID` — An identifier for this consignment, assigned by the freight forwarder.
/// - `BrokerAssignedID` — An identifier for this consignment, assigned by the broker.
/// - `ContractedCarrierAssignedID` — An identifier for this consignment, assigned by the contracted carrier.
/// - `PerformingCarrierAssignedID` — An identifier for this consignment, assigned by the performing carrier.
/// - `SummaryDescription` — A textual summary description of the consignment.
/// - `TotalInvoiceAmount` — The total of all invoice amounts declared in this consignment.
/// - `DeclaredCustomsValueAmount` — The total declared value for customs purposes of all the goods in this consignment, regardless of whether they are subject to the same customs procedure, tariff/statistical categorization, country information, or duty regime.
/// - `TariffDescription` — Text describing the tariff applied to this consignment.
/// - `TariffCode` — A code signifying the tariff applied to this consignment.
/// - `InsurancePremiumAmount` — The amount of the premium payable to an insurance company for insuring the goods contained in this consignment.
/// - `GrossWeightMeasure` — The total declared weight of the goods in this consignment, including packaging but excluding the carrier's equipment.
/// - `NetWeightMeasure` — The total net weight of all the goods items referred to as one consignment.
/// - `NetNetWeightMeasure` — The total net weight of the goods in this consignment, exclusive of packaging.
/// - `ChargeableWeightMeasure` — The weight upon which a charge is to be based.
/// - `GrossVolumeMeasure` — The total volume of the goods referred to as one consignment.
/// - `NetVolumeMeasure` — The total net volume of all goods items referred to as one consignment.
/// - `LoadingLengthMeasure` — The total length in a means of transport or a piece of transport equipment which, given the width and height of the transport means, will accommodate all of the consignments in a single consolidation.
/// - `Remarks` — Remarks concerning the complete consignment, to be printed on the transport document.
/// - `HazardousRiskIndicator` — An indication that the transported goods in this consignment are subject to an international regulation concerning the carriage of dangerous goods (true) or not (false).
/// - `AnimalFoodIndicator` — An indication that the transported goods in this consignment are animal foodstuffs (true) or not (false).
/// - `HumanFoodIndicator` — An indication that the transported goods in this consignment are for human consumption (true) or not (false).
/// - `LivestockIndicator` — An indication that the transported goods are livestock (true) or not (false).
/// - `BulkCargoIndicator` — An indication that the transported goods in this consignment are bulk cargoes (true) or not (false).
/// - `ContainerizedIndicator` — An indication that the transported goods in this consignment are containerized cargoes (true) or not (false).
/// - `GeneralCargoIndicator` — An indication that the transported goods in this consignment are general cargoes (true) or not (false).
/// - `SpecialSecurityIndicator` — An indication that the transported goods in this consignment require special security (true) or not (false).
/// - `ThirdPartyPayerIndicator` — An indication that this consignment will be paid for by a third party (true) or not (false).
/// - `CarrierServiceInstructions` — Service instructions to the carrier, expressed as text.
/// - `CustomsClearanceServiceInstructions` — Service instructions for customs clearance, expressed as text.
/// - `ForwarderServiceInstructions` — Service instructions for the forwarder, expressed as text.
/// - `SpecialServiceInstructions` — Special service instructions, expressed as text.
/// - `SequenceID` — A sequence identifier for this consignment.
/// - `ShippingPriorityLevelCode` — A code signifying the priority or level of service required for this consignment.
/// - `HandlingCode` — The handling required for this consignment, expressed as a code.
/// - `HandlingInstructions` — The handling required for this consignment, expressed as text.
/// - `Information` — Free-form text pertinent to this consignment, conveying information that is not contained explicitly in other structures.
/// - `TotalGoodsItemQuantity` — The total number of goods items in this consignment.
/// - `TotalTransportHandlingUnitQuantity` — The number of pieces of transport handling equipment (pallets, boxes, cases, etc.) in this consignment.
/// - `InsuranceValueAmount` — The amount covered by insurance for this consignment.
/// - `DeclaredForCarriageValueAmount` — The value of this consignment, declared by the shipper or his agent solely for the purpose of varying the carrier's level of liability from that provided in the contract of carriage, in case of loss or damage to goods or delayed delivery.
/// - `DeclaredStatisticsValueAmount` — The value, declared for statistical purposes, of those goods in this consignment that have the same statistical heading.
/// - `FreeOnBoardValueAmount` — The monetary amount that has to be or has been paid as calculated under the applicable trade delivery.
/// - `SpecialInstructions` — Special instructions relating to this consignment.
/// - `SplitConsignmentIndicator` — An indicator that this consignment has been split in transit (true) or not (false).
/// - `DeliveryInstructions` — A set of delivery instructions relating to this consignment.
/// - `ConsignmentQuantity` — The count in this consignment considering goods items, child consignments, shipments
/// - `ConsolidatableIndicator` — An indicator that this consignment can be consolidated (true) or not (false).
/// - `HaulageInstructions` — Instructions regarding haulage of this consignment, expressed as text.
/// - `LoadingSequenceID` — An identifier for the loading sequence of this consignment.
/// - `ChildConsignmentQuantity` — The quantity of (consolidated) child consignments
/// - `TotalPackagesQuantity` — The total number of packages associated with a Consignment.
/// - `ConsolidatedShipment` — A consolidated shipment (a shipment created by an act of consolidation).
/// - `CustomsDeclaration` — A class describing identifiers or references relating to customs procedures.
/// - `RequestedPickupTransportEvent` — The pickup of this consignment requested by the party requesting a transportation service (the transport user).
/// - `RequestedDeliveryTransportEvent` — The delivery of this consignment requested by the party requesting a transportation service (the transport user).
/// - `PlannedPickupTransportEvent` — The pickup of this consignment planned by the party responsible for providing the transportation service (the transport service provider).
/// - `PlannedDeliveryTransportEvent` — The delivery of this consignment planned by the party responsible for providing the transportation service (the transport service provider).
/// - `ActualPickupTransportEvent` — The actual pickup of this consignment by the party responsible for providing the transportation service (the transport service provider).
/// - `ActualDeliveryTransportEvent` — The actual delivery of this consignment by the party responsible for providing the transportation service (the transport service provider).
/// - `Status` — The status of a particular condition associated with this consignment.
/// - `ChildConsignment` — One of the child consignments of which a consolidated consignment is composed.
/// - `ConsigneeParty` — The Party who receives the goods.
/// - `ExporterParty` — The Party who exports the goods or has similar right of disposal over them at the time of export.
/// - `ConsignorParty` — The Party who is reponsible for sending the goods.
/// - `ImporterParty` — The Party who imports the goods, or on whose behalf the goods are being imported.
/// - `CarrierParty` — The Party who provides the transport of goods between named points.
/// - `FreightForwarderParty` — The Party who combines individual smaller consignments into a single larger shipment (a so-called consolidated consignment or shipment) which is sent to a counterpart who mirrors the consolidator's activity by dividing the consolidated consignment into its original components.
/// - `NotifyParty` — The Party who is notified upon arrival of Goods and when special occurrences (usually pre-defined) take place during a transportation service.
/// - `OriginalDespatchParty` — The Party who originally sends this Consignment.
/// - `FinalDeliveryParty` — The final delivery party for this consignment.
/// - `PerformingCarrierParty` — The Party who performs the carriage of this Consignment.
/// - `SubstituteCarrierParty` — The Party who subtitutes the carrier of this Consignment.
/// - `LogisticsOperatorParty` — The Party who operates the logistics for this Consignment.
/// - `TransportAdvisorParty` — The Party who provides transport advice in this Consignment.
/// - `HazardousItemNotificationParty` — The Party who is notified of a Hazardous Item in this Consignment.
/// - `InsuranceParty` — The Party who holds the insurance for this Consignment.
/// - `MortgageHolderParty` — The Party who holds the mortgage for this Consignment.
/// - `BillOfLadingHolderParty` — The Party who holds the Bill of Lading for this Consignment.
/// - `OriginalDepartureCountry` — The country from which the goods in this consignment were originally exported, without any commercial transaction taking place in intermediate countries.
/// - `FinalDestinationCountry` — The country in which the goods in this consignment are to be delivered to the final consignee or buyer.
/// - `TransitCountry` — One of the countries through which goods or passengers in this consignment are routed between the country of original departure and the country of final destination.
/// - `TransportContract` — A transport contract relating to this consignment.
/// - `TransportEvent` — A class for describing any additional significant occurrences or happenings related to the transportation of goods not specified elsewhere in this Consignment.
/// - `OriginalDespatchTransportationService` — The service for pickup from the consignor under the transport contract for this consignment.
/// - `FinalDeliveryTransportationService` — The service for delivery to the consignee under the transport contract for this consignment.
/// - `DeliveryTerms` — The conditions agreed upon between a seller and a buyer with regard to the delivery of goods and/or services (e.g., CIF, FOB, or EXW from the INCOTERMS Terms of Delivery).
/// - `PaymentTerms` — The terms of payment between the parties (such as logistics service client, logistics service provider) in a transaction.
/// - `CollectPaymentTerms` — The terms of payment that apply to the collection of this consignment.
/// - `DisbursementPaymentTerms` — The terms of payment for disbursement.
/// - `PrepaidPaymentTerms` — The terms of payment for prepayment.
/// - `FreightAllowanceCharge` — A cost incurred by the shipper in moving goods, by whatever means, from one place to another under the terms of the contract of carriage for this consignment. In addition to transport costs, this may include such elements as packing, documentation, loading, unloading, and insurance to the extent that they relate to the freight costs.
/// - `ExtraAllowanceCharge` — A charge for extra allowance.
/// - `MainCarriageShipmentStage` — A shipment stage during main carriage.
/// - `PreCarriageShipmentStage` — A shipment stage during precarriage (usually refers to movement activity that takes place prior to the container being loaded at a port of loading).
/// - `OnCarriageShipmentStage` — A shipment stage during on-carriage (usually refers to movement activity that takes place after the container is discharged at a port of discharge).
/// - `TransportHandlingUnit` — A transport handling unit used for loose and containerized goods.
/// - `FirstArrivalPortLocation` — The first arrival location in a transport. This would be a port for sea, an airport for air, a terminal for rail, or a border post for land crossing.
/// - `LastExitPortLocation` — The final exporting location in a transport. This would be a port for sea, an airport for air, a terminal for rail, or a border post for land crossing.
/// - `OfficeOfEntryLocation` — The customs office or offices indicated in the authorisation as empowered to accept declarations entering goods for the arrangements.
/// - `OfficeOfSubSequentiallyEntryLocation` — A location that is involved in the subsequent entry of goods in a consignment. This could mean a customs office or facility where goods are processed after their initial entry point, possibly for further clearance, inspection, or transit.
/// - `OfficeOfExitLocation` — The customs office of the actual exit of the goods at which the goods are placed in the export procedure and released for exit.
/// - `OfficeOfDepartureLocation` — A Customs Office where the customs declaration placing goods under transit is accepted.
/// - `OfficeOfDestinationLocation` — Any customs office at which a customs transit operation is terminated.
/// - `OfficeOfImportLocation` — The customs office where the formalities for assigning goods brought into the customs territory of the Community to a customs-approved treatment or use are to be carried out.
/// - `OfficeOfExportLocation` — The customs office at which an export declaration or a re-export declaration is made.
/// - `OfficeOfTransitLocation` — A location that finds the reference numbers for the departure, transit and destination offices.
/// - `DocumentReference` — A reference to a document related to or relevant for this consignment.
/// - `EnvironmentalEmission` — One or more environmental emissions of this consignment.
/// - `InsurancePolicy` — One or more Insurance Policies that apply to this consignment.
// pub struct Consignment { ... }

/// A class to describe the consumption of a utility.
///
/// **UBL Dictionary Entry Name:** `Consumption. Details`
///
/// Generated from XSD type `ConsumptionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UtilityStatementTypeCode` — A code identifying the type of the Utility Statement required for this consumption. Explains the kind of utility the statement is about, e.g.. "gas", "electricity", "telephone"
/// - `MainPeriod` — The period of consumption.
/// - `AllowanceCharge` — An allowance or charges that may apply with this consumption.
/// - `TaxTotal` — The total of taxes for each tax type covering the consumption.
/// - `EnergyWaterSupply` — The details of any energy or water consumption.
/// - `TelecommunicationsSupply` — The details of any telecommunications consumption.
/// - `LegalMonetaryTotal` — The total amount payable on this consumption, including any allowances, charges, or taxes.
// pub struct Consumption { ... }

/// A class to define an average consumption as a monetary amount.
///
/// **UBL Dictionary Entry Name:** `Consumption Average. Details`
///
/// Generated from XSD type `ConsumptionAverageType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `AverageAmount` — The average monetary amount of the consumption.
/// - `Description` — A description of the average consumed.
// pub struct ConsumptionAverage { ... }

/// The Statement of correction, for examples heating correction.
///
/// **UBL Dictionary Entry Name:** `Consumption Correction. Details`
///
/// Generated from XSD type `ConsumptionCorrectionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `CorrectionType` — Statement for the correction type.
/// - `CorrectionTypeCode` — Statement at the code for the correction type.
/// - `MeterNumber` — Statement for meter number.
/// - `GasPressureQuantity` — Correction of the gas pressure.
/// - `ActualTemperatureReductionQuantity` — Statement for the actuel heating correction temperature.
/// - `NormalTemperatureReductionQuantity` — Statement for the standard for heating correction temperature.
/// - `DifferenceTemperatureReductionQuantity` — Deviation from standard heating correction.
/// - `Description` — Description related to the corrections.
/// - `CorrectionUnitAmount` — Correction per MWH per degree C.
/// - `ConsumptionEnergyQuantity` — Your consumpt for district heating energy.
/// - `ConsumptionWaterQuantity` — Your consumpt for district heating water.
/// - `CorrectionAmount` — Your correction for heating correction.
// pub struct ConsumptionCorrection { ... }

/// A class to describe the measurement of a type of consumption during a particular period, used for the subscriber to get an overview of his consumption
///
/// **UBL Dictionary Entry Name:** `Consumption History. Details`
///
/// Generated from XSD type `ConsumptionHistoryType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `MeterNumber` — A text identifier for the meter measuring the consumption.
/// - `Quantity` — The quantity consumed.
/// - `Amount` — The monetary amount to be charged for the quantity consumed.
/// - `ConsumptionLevelCode` — The consumption level, expressed as a code used explain the consumption quantity, e.g.. diversion from the normal.
/// - `ConsumptionLevel` — The consumption level, expressed as text, used explain the consumption quantity, e.g.. diversion from the normal.
/// - `Description` — Text describing the consumption itself.
/// - `Period` — The period during which the consumption took place.
// pub struct ConsumptionHistory { ... }

/// A class to describe a line item for utility consumption. To specify more than one utility item, use separate consumption lines.
///
/// **UBL Dictionary Entry Name:** `Consumption Line. Details`
///
/// Generated from XSD type `ConsumptionLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this consumption line.
/// - `ParentDocumentLineReferenceID` — An identifier for the transaction line on a related document (such as an invoice) that covers this consumption line.
/// - `InvoicedQuantity` — The quantity invoiced.
/// - `LineExtensionAmount` — The monetary amount, including discount, to be charged for this consumption line.
/// - `TaxInclusiveLineExtensionAmount` — The total amount for this consumption line, including all allowances, charges and taxes.
/// - `Period` — The period of time covered by this consumption line.
/// - `Delivery` — A delivery of the utility item on this consumption line.
/// - `AllowanceCharge` — An allowance or charge that applies to this consumption line.
/// - `TaxTotal` — A total amount of taxes of a particular kind applicable to this consumption line.
/// - `UtilityItem` — The utility item consumed.
/// - `Price` — The price associated with this consumption line, expressed in a data structure containing multiple properties.
/// - `UnstructuredPrice` — The price associated with this consumption line expressed in a less structured form that includes just the amount and the time of use.
// pub struct ConsumptionLine { ... }

/// A class to define the point of consumption for a utility, such as a meter.
///
/// **UBL Dictionary Entry Name:** `Consumption Point. Details`
///
/// Generated from XSD type `ConsumptionPointType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this point of consumption.
/// - `Description` — Text describing this consumption point.
/// - `SubscriberID` — An identifier for the subscriber responsible for the consumption at this consumption point.
/// - `SubscriberType` — The type of subscriber, expressed as text.
/// - `SubscriberTypeCode` — The type of subscriber, expressed as a code.
/// - `TotalDeliveredQuantity` — The total quantity delivered, calculated at this consumption point.
/// - `Address` — The address of this consumption point.
/// - `WebSiteAccess` — Access information for the website of this consumption point.
/// - `UtilityMeter` — A meter at this consumption point.
// pub struct ConsumptionPoint { ... }

/// A class to describe utility consumption, including details of the environment in which consumption takes place.
///
/// **UBL Dictionary Entry Name:** `Consumption Report. Details`
///
/// Generated from XSD type `ConsumptionReportType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this consumption report.
/// - `ConsumptionType` — The type of consumption, expressed as text.
/// - `ConsumptionTypeCode` — The type of consumption, expressed as a code.
/// - `Description` — Text reporting utility consumption.
/// - `TotalConsumedQuantity` — The total quantity consumed.
/// - `BasicConsumedQuantity` — The basic quantity consumed, excluding additional consumption.
/// - `ResidentOccupantsNumeric` — The number of people occupying the residence covered by this report.
/// - `ConsumersEnergyLevelCode` — The level of energy consumed, compared to the average for this residence type and the number of people living in the residence, expressed as a code.
/// - `ConsumersEnergyLevel` — The level of energy consumed, compared to the average for this residence type and the number of people living in the residence, expressed as text.
/// - `ResidenceType` — The type of residence (house, apartment, etc.) covered in this report, expressed as text.
/// - `ResidenceTypeCode` — The type of residence (house, apartment, etc.) covered in this report, expressed as a code.
/// - `HeatingType` — The type of heating in the residence covered in this report, expressed as text.
/// - `HeatingTypeCode` — The type of heating in the residence covered in this report, expressed as a code.
/// - `Period` — The period of consumption covered in this report.
/// - `GuidanceDocumentReference` — A reference to a document providing an explanation of this kind of report.
/// - `DocumentReference` — A reference to some other document (for example, this report in another format).
/// - `ConsumptionReportReference` — A reference to a previous consumption report.
/// - `ConsumptionHistory` — A report describing historical parameters relating to a specific instance of consumption.
// pub struct ConsumptionReport { ... }

/// A class to define a reference to an earlier consumption report (e.g., last year's consumption).
///
/// **UBL Dictionary Entry Name:** `Consumption Report Reference. Details`
///
/// Generated from XSD type `ConsumptionReportReferenceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ConsumptionReportID` — An identifier for the referenced consumption report.
/// - `ConsumptionType` — The reported consumption type, expressed as text.
/// - `ConsumptionTypeCode` — The reported consumption type, expressed as a code.
/// - `TotalConsumedQuantity` — The total quantity consumed during the period of the referenced report.
/// - `Period` — The period of consumption covered by the referenced report.
// pub struct ConsumptionReportReference { ... }

/// A class to describe a contactable person or department in an organization.
///
/// **UBL Dictionary Entry Name:** `Contact. Details`
///
/// Generated from XSD type `ContactType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this contact.
/// - `Name` — The name of this contact. It is recommended that this be used for a functional name and not a personal name.
/// - `JobTitle` — The job title or function of this contact
/// - `Department` — The department where this contact works
/// - `Telephone` — The primary telephone number of this contact.
/// - `Telefax` — The primary fax number of this contact.
/// - `ElectronicMail` — The primary email address of this contact.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures; in particular, a textual description of the circumstances under which this contact can be used (e.g., "emergency" or "after hours").
/// - `OtherCommunication` — Another means of communication with this contact.
// pub struct Contact { ... }

/// A class to describe a contract.
///
/// **UBL Dictionary Entry Name:** `Contract. Details`
///
/// Generated from XSD type `ContractType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this contract.
/// - `IssueDate` — The date on which this contract was issued.
/// - `IssueTime` — The time at which this contract was issued.
/// - `NominationDate` — In a transportation contract, the deadline date by which the services referred to in the transport execution plan have to be booked. For example, if this service is a carrier service scheduled for Wednesday 16 February 2011 at 10 a.m. CET, the nomination date might be Tuesday15 February 2011.
/// - `NominationTime` — In a transportation contract, the deadline time by which the services referred to in the transport execution plan have to be booked. For example, if this service is a carrier service scheduled for Wednesday 16 February 2011 at 10 a.m. CET, the nomination date might be Tuesday15 February 2011 and the nomination time 4 p.m. at the latest.
/// - `ContractTypeCode` — The type of this contract, expressed as a code, such as "Cost plus award fee" and "Cost plus fixed fee" from UNCEFACT Contract Type code list.
/// - `ContractType` — The type of this contract, expressed as text, such as "Cost plus award fee" and "Cost plus fixed fee" from UNCEFACT Contract Type code list.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `VersionID` — An identifier for the current version of this contract.
/// - `ModificationReasonCode` — The main reason for modifying the contract expressed as a code.
/// - `ModificationReasonDescription` — Text describing the main reason for modifying the contract
/// - `Description` — Text describing this contract.
/// - `ValidityPeriod` — The period during which this contract is valid.
/// - `ContractDocumentReference` — A reference to a contract document.
/// - `NominationPeriod` — In a transportation contract, the period required to book the services specified in the contract before the services can begin.
/// - `ContractualDelivery` — In a transportation contract, the delivery of the services required to book the services specified in the contract.
// pub struct Contract { ... }

/// A class to describe a requirement for execution of a contract.
///
/// **UBL Dictionary Entry Name:** `Contract Execution Requirement. Details`
///
/// Generated from XSD type `ContractExecutionRequirementType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Name` — A name for this requirement.
/// - `ExecutionRequirementCode` — A code signifying a type of requirement to be fulfiled by the economic operator.
/// - `Description` — Text describing this requirement.
// pub struct ContractExecutionRequirement { ... }

/// A class to describe possible extensions to a contract.
///
/// **UBL Dictionary Entry Name:** `Contract Extension. Details`
///
/// Generated from XSD type `ContractExtensionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `OptionsDescription` — A description for the possible options that can be carried out during the execution of the contract.
/// - `MinimumNumberNumeric` — The fixed minimum number of contract extensions or renewals.
/// - `MaximumNumberNumeric` — The maximum allowed number of contract extensions.
/// - `RenewalsIndicator` — Indicates that the contract can be extended using renewals.
/// - `OptionValidityPeriod` — The period during which the option for extending the contract is available.
/// - `Renewal` — The period allowed for each contract extension.
// pub struct ContractExtension { ... }

/// The nature of the type of business of the organization.
///
/// **UBL Dictionary Entry Name:** `Contracting Activity. Details`
///
/// Generated from XSD type `ContractingActivityType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ActivityTypeCode` — A code specifying the nature of the type of business of the organization.
/// - `ActivityType` — The nature of the type of business of the organization, expressed as text.
// pub struct ContractingActivity { ... }

/// A class to describe an individual, a group, or a body having a procurement role in a tendering process.
///
/// **UBL Dictionary Entry Name:** `Contracting Party. Details`
///
/// Generated from XSD type `ContractingPartyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `BuyerProfileURI` — The buyer profile is typically located on a web site where the contracting party publishes its procurement opportunities
/// - `ContractingPartyType` — The type of contracting party that is independent of its role.
/// - `ContractingActivity` — The nature of the type of business of the organization
/// - `ContractingRepresentationType` — The type of represention empowering the party to act on behalf of a third party
/// - `Party` — The Party who is reponsible for the Contract.
// pub struct ContractingParty { ... }

/// The type of contracting party that is independent of its role.
///
/// **UBL Dictionary Entry Name:** `Contracting Party Type. Details`
///
/// Generated from XSD type `ContractingPartyTypeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `PartyTypeCode` — A code specifying the type of party that is independent of its role.
/// - `PartyType` — The type of party that is independent of its role, expressed as text.
// pub struct ContractingPartyType { ... }

/// The type of representation the party has acting for the Contracting party
///
/// **UBL Dictionary Entry Name:** `Contracting Representation Type. Details`
///
/// Generated from XSD type `ContractingRepresentationTypeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `RepresentationTypeCode` — A code specifying the type of representation empowering the party to act on behalf of a third party
/// - `RepresentationType` — The type of representation empowering the party to act on behalf of a third party, expressed as text.
// pub struct ContractingRepresentationType { ... }

/// A class to describe the contracting system. If the procedure is individual (nonrepetitive), this class ought not be used.
///
/// **UBL Dictionary Entry Name:** `Contracting System. Details`
///
/// Generated from XSD type `ContractingSystemType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for the contracting system.
/// - `ContractingSystemTypeCode` — A code signifying the type of contracting system (e.g., framework agreement, dynamic purchasing system).
/// - `Description` — The description of the contracting system
// pub struct ContractingSystem { ... }

/// A class to describe a scheme for corporate registration.
///
/// **UBL Dictionary Entry Name:** `Corporate Registration Scheme. Details`
///
/// Generated from XSD type `CorporateRegistrationSchemeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this registration scheme.
/// - `Name` — The name of this registration scheme.
/// - `CorporateRegistrationTypeCode` — A code signifying the type of this registration scheme.
/// - `JurisdictionRegionAddress` — A geographic area in which this registration scheme applies.
// pub struct CorporateRegistrationScheme { ... }

/// A class to describe a country.
///
/// **UBL Dictionary Entry Name:** `Country. Details`
///
/// Generated from XSD type `CountryType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `IdentificationCode` — A code signifying this country.
/// - `Name` — The name of this country.
// pub struct Country { ... }

/// A class to identify a credit account for sales on account.
///
/// **UBL Dictionary Entry Name:** `Credit Account. Details`
///
/// Generated from XSD type `CreditAccountType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `AccountID` — An identifier for this credit account.
// pub struct CreditAccount { ... }

/// A class to define a line in a Credit Note or Self Billed Credit Note.
///
/// **UBL Dictionary Entry Name:** `Credit Note Line. Details`
///
/// Generated from XSD type `CreditNoteLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this credit note line.
/// - `UUID` — A universally unique identifier for this credit note line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `CreditedQuantity` — The quantity of items credited in this credit note line.
/// - `LineExtensionAmount` — The total amount for this credit note line, including allowance charges but exclusive of taxes.
/// - `TaxInclusiveLineExtensionAmount` — The total amount for this credit note line, including all allowances, charges and taxes.
/// - `TaxPointDate` — The date of this credit note line, used to indicate the point at which tax becomes applicable.
/// - `AccountingCostCode` — The buyer's accounting cost centre for this credit note line, expressed as a code.
/// - `AccountingCost` — The buyer's accounting cost centre for this credit note line, expressed as text.
/// - `PaymentPurposeCode` — A code signifying the business purpose for this payment.
/// - `FreeOfChargeIndicator` — An indicator that this credit note line is free of charge (true) or not (false). The default is false.
/// - `InvoicePeriod` — An invoice period to which this credit note line applies.
/// - `OrderLineReference` — A reference to an order line associated with this credit note line.
/// - `DiscrepancyResponse` — A reason for the credit.
/// - `DespatchLineReference` — A reference to a despatch line associated with this credit note line.
/// - `ReceiptLineReference` — A reference to a receipt line associated with this credit note line.
/// - `WorkReportLineReference` — A reference to a work report line associated with this credit note line.
/// - `BillingReference` — A reference to a billing document associated with this credit note line.
/// - `DocumentReference` — A reference to a document associated with this credit note line.
/// - `PricingReference` — A reference to pricing and item location information associated with this credit note line.
/// - `PurchaseReference` — A reference to an object, such as a subscription number, telephone number, meter, vehicle, person, etc., to which this Credit Note Line relates.
/// - `OriginatorParty` — The Party who originates the Order to which the Credit Note is related.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `CollectedForParty` — The Party on whose behalf this item or amount is collected.
/// - `Delivery` — A delivery associated with this credit note line.
/// - `PaymentTerms` — A specification of payment terms associated with this credit note line.
/// - `TaxTotal` — A total amount of taxes of a particular kind applicable to this credit note line.
/// - `WithholdingTaxTotal` — A reference to a TaxTotal class describing the amount that has been withhold by the authorities, e.g. if the creditor is in dept because of non paid taxes.
/// - `AllowanceCharge` — An allowance or charge associated with this credit note.
/// - `Item` — The item associated with this credit note line.
/// - `Price` — The price of the item associated with this credit note line.
/// - `DeliveryTerms` — Terms and conditions of a delivery associated with this credit note line.
/// - `SubCreditNoteLine` — A class defining one or more Credit Note Lines detailing the credit note line.
/// - `ItemPriceExtension` — The price extension, calculated by multiplying the price per unit by the quantity of items on this credit note line.
// pub struct CreditNoteLine { ... }

/// A class describing the effect or belonging of a Crew Person
///
/// **UBL Dictionary Entry Name:** `Crew Person Effect. Details`
///
/// Generated from XSD type `CrewPersonEffectType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `EffectDescription` — The description of the crew effect.
/// - `CrewPerson` — The crew person to whom the effect belongs.
// pub struct CrewPersonEffect { ... }

/// A class describing a criteria
///
/// **UBL Dictionary Entry Name:** `Criterion Item. Details`
///
/// Generated from XSD type `CriterionItemType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this criteria
/// - `TypeCode` — A code describing the type of criteria
/// - `CriterionDescription` — The criteria for this item, expressed as a text
/// - `DeclaredPropertyItem` — The item associated with this criteria
// pub struct CriterionItem { ... }

/// A class to describe a customer party.
///
/// **UBL Dictionary Entry Name:** `Customer Party. Details`
///
/// Generated from XSD type `CustomerPartyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `CustomerAssignedAccountID` — An identifier for the customer's account, assigned by the customer itself.
/// - `SupplierAssignedAccountID` — An identifier for the customer's account, assigned by the supplier.
/// - `AdditionalAccountID` — An identifier for the customer's account, assigned by a third party.
/// - `Party` — The Customer Party itself.
/// - `DeliveryContact` — A customer contact for deliveries.
/// - `AccountingContact` — A customer contact for accounting.
/// - `BuyerContact` — A customer contact for purchasing.
// pub struct CustomerParty { ... }

/// A class describing identifiers or references relating to customs procedures.
///
/// **UBL Dictionary Entry Name:** `Customs Declaration. Details`
///
/// Generated from XSD type `CustomsDeclarationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier associated with customs related procedures.
/// - `FunctionCode` — A code describing the function of this customs declaration.
/// - `ValidityPeriod` — The period during which this customs declaration is valid
/// - `ApplicableTerritoryAddress` — The area or region where this customs declaration applies
/// - `Shipment` — A reference to the shipment of goods being declared
/// - `CustomsExitOfficeLocation` — The location of the exit office from where the goods will leave or have left the customs territory
/// - `IssuerParty` — The Party who issues this Customs Declaration.
/// - `ConsignorParty` — The Party who is reponsible for sending the goods.
/// - `ConsigneeParty` — The Party who receives the goods.
/// - `FreightForwarderParty` — The Party who combines individual smaller consignments into a single larger shipment (a so-called consolidated consignment or shipment) which is sent to a counterpart who mirrors the consolidator's activity by dividing the consolidated consignment into its original components.
/// - `CustomsParty` — The Authority who processes this Customs Declaration.
/// - `PreviousCustomsDeclaration` — A reference to a previous version of this customs declaration
/// - `AdditionalDocumentReference` — A reference to additional documents relevant or related to this customs declaration
// pub struct CustomsDeclaration { ... }

/// A class to define a line in a Debit Note.
///
/// **UBL Dictionary Entry Name:** `Debit Note Line. Details`
///
/// Generated from XSD type `DebitNoteLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this debit note line.
/// - `UUID` — A universally unique identifier for this debit note line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `DebitedQuantity` — The quantity of Items debited in this debit note line.
/// - `LineExtensionAmount` — The total amount for this debit note line, including allowance charges but net of taxes.
/// - `TaxInclusiveLineExtensionAmount` — The total amount for this debit note line, including all allowances, charges and taxes.
/// - `TaxPointDate` — The date of this debit note line, used to indicate the point at which tax becomes applicable.
/// - `AccountingCostCode` — The buyer's accounting cost centre for this debit note line, expressed as a code.
/// - `AccountingCost` — The buyer's accounting cost centre for this debit note line, expressed as text.
/// - `PaymentPurposeCode` — A code signifying the business purpose for this payment.
/// - `FreeOfChargeIndicator` — An indicator that this Debit Note Line is free of charge (true) or not (false). The default is false.
/// - `InvoicePeriod` — An invoice period to which this Debit Note Line applies.
/// - `OrderLineReference` — A reference to an Order Line associated with this Debit Note Line.
/// - `DiscrepancyResponse` — A reason for the debit.
/// - `DespatchLineReference` — A reference to a despatch line associated with this debit note line.
/// - `ReceiptLineReference` — A reference to a receipt line associated with this debit note line.
/// - `WorkReportLineReference` — A reference to a work report line associated with this debit note line.
/// - `BillingReference` — A reference to a billing document associated with this debit note line.
/// - `DocumentReference` — A reference to a document associated with this debit note line.
/// - `PricingReference` — A reference to pricing and item location information associated with this debit note line.
/// - `OriginatorParty` — The Party who originated the Order to which the Debit Note is related.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `CollectedForParty` — The Party on whose behalf this item or amount is collected.
/// - `Delivery` — A delivery associated with this debit note line.
/// - `PaymentTerms` — A specification of payment terms associated with this Debit Note Line.
/// - `TaxTotal` — A total amount of taxes of a particular kind applicable to this debit note line.
/// - `WithholdingTaxTotal` — A reference to a TaxTotal class describing the amount that has been withhold by the authorities, e.g. if the creditor is in dept because of non paid taxes.
/// - `AllowanceCharge` — An allowance or charge associated with this debit note.
/// - `Item` — The item associated with this debit note line.
/// - `Price` — The price of the item associated with this debit note line.
/// - `DeliveryTerms` — Terms and conditions of a delivery associated with this Credit Note Line.
/// - `SubDebitNoteLine` — A recursive description of a debit note line subsidiary to this debit note line.
/// - `ItemPriceExtension` — The price extension, calculated by multiplying the price per unit by the quantity of items on this Debit Note Line.
// pub struct DebitNoteLine { ... }

/// A class to describe a declaration by an economic operator of certain characteristics or capabilities in fulfilment of requirements specified in a call for tenders.
///
/// **UBL Dictionary Entry Name:** `Declaration. Details`
///
/// Generated from XSD type `DeclarationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this Declaration.
/// - `Name` — The name of this declaration.
/// - `DeclarationTypeCode` — A code signifying the type of this declaration.
/// - `Description` — Text describing this declaration.
/// - `EvidenceSupplied` — (Deprecated) The evidence supporting this declaration.
/// - `SuppliedEvidence` — The Evidence supporting this declaration.
// pub struct Declaration { ... }

/// A class to describe a delivery.
///
/// **UBL Dictionary Entry Name:** `Delivery. Details`
///
/// Generated from XSD type `DeliveryType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this delivery.
/// - `Quantity` — The quantity of items, child consignments, shipments in this delivery.
/// - `MinimumQuantity` — The minimum quantity of items, child consignments, shipments in this delivery.
/// - `MaximumQuantity` — The maximum quantity of items, child consignments, shipments in this delivery.
/// - `ActualDeliveryDate` — The actual date of delivery.
/// - `ActualDeliveryTime` — The actual time of delivery.
/// - `LatestDeliveryDate` — The latest date of delivery allowed by the buyer.
/// - `LatestDeliveryTime` — The latest time of delivery allowed by the buyer.
/// - `ReleaseID` — An identifier used for approval of access to delivery locations (e.g., port terminals).
/// - `TrackingID` — The delivery Tracking ID (for transport tracking).
/// - `DeliveryAddress` — The delivery address.
/// - `DeliveryLocation` — The delivery location.
/// - `AlternativeDeliveryLocation` — An alternative delivery location.
/// - `RequestedDeliveryPeriod` — The period requested for delivery.
/// - `PromisedDeliveryPeriod` — The period promised for delivery.
/// - `EstimatedDeliveryPeriod` — The period estimated for delivery.
/// - `CarrierParty` — The Party who provides the transport of goods between named points.
/// - `DeliveryParty` — The Party who receives the goods.
/// - `NotifyParty` — The Party who is notified of this Delivery.
/// - `Despatch` — The despatch (pickup) associated with this delivery.
/// - `DeliveryTerms` — Terms and conditions relating to the delivery.
/// - `MinimumDeliveryUnit` — The minimum delivery unit for this delivery.
/// - `MaximumDeliveryUnit` — The maximum delivery unit for this delivery.
/// - `Shipment` — The shipment being delivered.
/// - `FuelConsumption` — One or more fuel consumptions of this delivery.
/// - `DeliveryNoteDocumentReference` — A reference to a Delivery Note associated with this Delivery.
/// - `DeliveryNoteLineReference` — A reference to a Delivery Note Line associated with this Delivery.
// pub struct Delivery { ... }

/// A class to describe a delivery channel.
///
/// **UBL Dictionary Entry Name:** `Delivery Channel. Details`
///
/// Generated from XSD type `DeliveryChannelType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `NetworkID` — An identifier for the network where messages are delivered (e.g. a business network).
/// - `ParticipantID` — An identifier for a registered participant in the network (e.g. according a precise scheme such as IT:VAT, DK:CVR, GLN).
/// - `TestIndicator` — An indicator that the channel is a test channel (true).
/// - `DigitalCertificate` — A digital certificate associated with this delivery channel.
/// - `DigitalMessageDelivery` — A digital message delivery associated with this delivery channel (aka routing information).
// pub struct DeliveryChannel { ... }

/// A class for describing the terms and conditions applying to the delivery of goods.
///
/// **UBL Dictionary Entry Name:** `Delivery Terms. Details`
///
/// Generated from XSD type `DeliveryTermsType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this description of delivery terms.
/// - `SpecialTerms` — A description of any terms or conditions relating to the delivery items.
/// - `LossRiskResponsibilityCode` — A code that identifies one of various responsibilities for loss risk in the execution of the delivery.
/// - `LossRisk` — A description of responsibility for risk of loss in execution of the delivery, expressed as text.
/// - `Amount` — The monetary amount covered by these delivery terms.
/// - `DeliveryLocation` — The location for the contracted delivery.
/// - `AllowanceCharge` — An allowance or charge covered by these delivery terms.
// pub struct DeliveryTerms { ... }

/// A class to describe a delivery unit.
///
/// **UBL Dictionary Entry Name:** `Delivery Unit. Details`
///
/// Generated from XSD type `DeliveryUnitType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `BatchQuantity` — The quantity of ordered Items that constitutes a batch for delivery purposes.
/// - `ConsumerUnitQuantity` — The quantity of units in the Delivery Unit expressed in the units used by the consumer.
/// - `HazardousRiskIndicator` — An indication that the transported goods are subject to an international regulation concerning the carriage of dangerous goods (true) or not (false).
// pub struct DeliveryUnit { ... }

/// A class to define the price of an item as a percentage of the price of a different item.
///
/// **UBL Dictionary Entry Name:** `Dependent Price Reference. Details`
///
/// Generated from XSD type `DependentPriceReferenceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Percent` — The percentage by which the price of the different item is multiplied to calculate the price of the item.
/// - `LocationAddress` — The reference location for this dependent price reference.
/// - `DependentLineReference` — A reference to a line that the price is depended of.
// pub struct DependentPriceReference { ... }

/// A class to describe the despatching of goods (their pickup for delivery).
///
/// **UBL Dictionary Entry Name:** `Despatch. Details`
///
/// Generated from XSD type `DespatchType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this despatch event.
/// - `RequestedDespatchDate` — The despatch (pickup) date requested, normally by the buyer.
/// - `RequestedDespatchTime` — The despatch (pickup) time requested, normally by the buyer.
/// - `EstimatedDespatchDate` — The estimated despatch (pickup) date.
/// - `EstimatedDespatchTime` — The estimated despatch (pickup) time.
/// - `ActualDespatchDate` — The actual despatch (pickup) date.
/// - `ActualDespatchTime` — The actual despatch (pickup) time.
/// - `GuaranteedDespatchDate` — The date guaranteed for the despatch (pickup).
/// - `GuaranteedDespatchTime` — The time guaranteed for the despatch (pickup).
/// - `ReleaseID` — An identifier for the release of the despatch used as security control or cargo control (pick-up).
/// - `Instructions` — Text describing any special instructions applying to the despatch (pickup).
/// - `DespatchAddress` — The address of the despatch (pickup).
/// - `DespatchLocation` — The location of the despatch (pickup).
/// - `DespatchParty` — The Party who despatches the goods.
/// - `CarrierParty` — The Party who provides the transport of goods between named points.
/// - `NotifyParty` — The Party who is notified of this Despatch.
/// - `ResponsibleParty` — The party who picks up the goods.
/// - `Contact` — The primary contact for this despatch (pickup).
/// - `EstimatedDespatchPeriod` — The period estimated for the despatch (pickup) of goods.
/// - `RequestedDespatchPeriod` — The period requested for the despatch (pickup) of goods.
// pub struct Despatch { ... }

/// A class to define a line in a Despatch Advice.
///
/// **UBL Dictionary Entry Name:** `Despatch Line. Details`
///
/// Generated from XSD type `DespatchLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this despatch line.
/// - `UUID` — A universally unique identifier for this despatch line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `LineStatusCode` — A code signifying the status of this despatch line with respect to its original state.
/// - `DeliveredQuantity` — The quantity despatched (picked up).
/// - `BackorderQuantity` — The quantity on back order at the supplier.
/// - `BackorderReason` — The reason for the back order.
/// - `OutstandingQuantity` — The quantity outstanding (which will follow in a later despatch).
/// - `OutstandingReason` — The reason for the outstanding quantity.
/// - `OversupplyQuantity` — The quantity over-supplied, i.e., the quantity over and above that ordered.
/// - `AccountingCostCode` — The accounting cost centre, applied to the Despatch Advice Line, expressed as a code.
/// - `AccountingCost` — The accounting cost centre, applied to the Despatch Advice Line, expressed as text.
/// - `OrderLineReference` — A reference to an order line associated with this despatch line.
/// - `DocumentReference` — A reference to a document associated with this despatch line.
/// - `Item` — The item associated with this despatch line.
/// - `Shipment` — A shipment associated with this despatch line.
/// - `SubDespatchLine` — A despatch line subsidiary to this despatch line.
// pub struct DespatchLine { ... }

/// A class to describe the terms and conditions of a digital agreement.
///
/// **UBL Dictionary Entry Name:** `Digital Agreement Terms. Details`
///
/// Generated from XSD type `DigitalAgreementTermsType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Description` — Text describing the terms and conditions of a digital agreement.
/// - `ValidityPeriod` — The period of time for which this digital agreement is valid.
/// - `AdoptionPeriod` — The period during which a digital agreement must be adopted.
/// - `ServiceLevelAgreement` — The service level agreement which regulates the quality, availability and responsibilities of digital services.
// pub struct DigitalAgreementTerms { ... }

/// A class to describe a digital trade collaboration.
///
/// **UBL Dictionary Entry Name:** `Digital Collaboration. Details`
///
/// Generated from XSD type `DigitalCollaborationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for the digital collaboration.
/// - `SendingDigitalService` — The sending digital service associated with this digital collaboration.
/// - `ReceivingDigitalService` — The receiving digital service associated with this digital collaboration.
// pub struct DigitalCollaboration { ... }

/// A class to describe a digital trade process.
///
/// **UBL Dictionary Entry Name:** `Digital Process. Details`
///
/// Generated from XSD type `DigitalProcessType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for the digital collaboration.
/// - `Description` — Text describing the digital process.
/// - `ProfileID` — Identifies a user-defined profile of this digital process (e.g. an UBL profile).
/// - `DigitalCollaboration` — The digital collaboration associated with this digital process.
/// - `CertificationDocumentReference` — A reference to a certification document associated with this digital process.
// pub struct DigitalProcess { ... }

/// A class to describe a specific digital trade service supported by an organization for either sending or receiving business documents on different formats (e.g. UBL, ISO20022, EDIFACT, ...).
///
/// **UBL Dictionary Entry Name:** `Digital Service. Details`
///
/// Generated from XSD type `DigitalServiceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for the digital service (aka transaction ID).
/// - `CustomizationID` — Identifies a user-defined customization of this digital service (e.g. a PEPPOL customization).
/// - `DigitalDocumentMetadata` — The digital document metadata associated with this digital service.
/// - `DigitalDeliveryChannel` — The digital delivery channel associated with this digital service.
/// - `CertificationDocumentReference` — A reference to a certification document associated with this digital service.
// pub struct DigitalService { ... }

/// A class to define a measurable dimension (length, mass, weight, volume, or area) of an item.
///
/// **UBL Dictionary Entry Name:** `Dimension. Details`
///
/// Generated from XSD type `DimensionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `AttributeID` — An identifier for the attribute to which the measure applies.
/// - `Measure` — The measurement value.
/// - `Description` — Text describing the measurement attribute.
/// - `MinimumMeasure` — The minimum value in a range of measurement of this dimension.
/// - `MaximumMeasure` — The maximum value in a range of measurement of this dimension.
// pub struct Dimension { ... }

/// A class to describe the distribution of a document to an interested party.
///
/// **UBL Dictionary Entry Name:** `Document Distribution. Details`
///
/// Generated from XSD type `DocumentDistributionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this document distribution.
/// - `DocumentTypeCode` — The type of document, expressed as a code.
/// - `DistributionTypeCode` — The type of distribution, expressed as a code.
/// - `DistributionType` — The type of distribution, expressed as text.
/// - `PrintQualifier` — (Deprecated) Text describing the interested party’s rights and limitations for distributing originals and copies of this document.
/// - `CopyIndicator` — (Deprecated) An indicator that the document in this ditribution is a copy (true) or the original (false).
/// - `MaximumCopiesNumeric` — (Deprecated) The maximum number of printed copies of the document that the interested party is allowed to make.
/// - `MaximumOriginalsNumeric` — (Deprecated) The maximum number of printed originals of the document that the interested party is allowed to make.
/// - `Communication` — A Communication used for this document distribution.
/// - `Party` — The interested Party who receives this Document.
// pub struct DocumentDistribution { ... }

/// A class to describe the metadata of a specific business document based on any document format (e.g. UBL, EDIFACT, ...).
///
/// **UBL Dictionary Entry Name:** `Document Metadata. Details`
///
/// Generated from XSD type `DocumentMetadataType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for the document.
/// - `FormatID` — An identifier for the document format (e.g. standard business vocabularies).
/// - `VersionID` — An identifier for a precise version of a document format.
/// - `SchemaURI` — The Uniform Resource Identifier (URI) of a schema definition for the business document (e.g. a namespace URI for XML schemas, a message ID for non-xml legacy documents).
/// - `DocumentTypeCode` — The type of document, expressed as a code.
// pub struct DocumentMetadata { ... }

/// A class to define a reference to a document.
///
/// **UBL Dictionary Entry Name:** `Document Reference. Details`
///
/// Generated from XSD type `DocumentReferenceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for the referenced document.
/// - `CopyIndicator` — (Deprecated) An indicator that the referenced document is a copy (true) or the original (false).
/// - `UUID` — A universally unique identifier for this document reference.
/// - `IssueDate` — The date, assigned by the sender of the referenced document, on which the document was issued.
/// - `IssueTime` — The time, assigned by the sender of the referenced document, at which the document was issued.
/// - `DocumentTypeCode` — The type of document being referenced, expressed as a code.
/// - `DocumentType` — The type of document being referenced, expressed as text.
/// - `XPath` — An unambiguous location within the bounding document or the document referenced by the parent DocumentReference, expressed as an XPath
/// - `ReferencedDocumentInternalAddress` — A pointer to a location within the document being referenced
/// - `LanguageID` — An identifier for the language used in the referenced document.
/// - `LocaleCode` — A code signifying the locale in which the language in the referenced document is used.
/// - `VersionID` — An identifier for the current version of the referenced document.
/// - `DocumentStatusCode` — A code signifying the status of the reference document with respect to its original state.
/// - `DocumentDescription` — Text describing the referenced document.
/// - `Attachment` — The referenced document as an attachment to the document from which it is referenced.
/// - `ValidityPeriod` — The period for which the document referenced by this Document Rreference is valid.
/// - `IssuerParty` — The Party who issues the Referenced Document.
/// - `ResultOfVerification` — The result of an attempt to verify a signature associated with the referenced document.
// pub struct DocumentReference { ... }

/// A class to describe an application-level response to a document.
///
/// **UBL Dictionary Entry Name:** `Document Response. Details`
///
/// Generated from XSD type `DocumentResponseType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Response` — A response to the document as a whole.
/// - `DocumentReference` — A referenced document.
/// - `IssuerParty` — The Party who issues this Document.
/// - `RecipientParty` — The Party who is the intended recipient of this Document.
/// - `LineResponse` — A response to a particular line in the document.
// pub struct DocumentResponse { ... }

/// The charging rate used for both call charging and time dependent charging
///
/// **UBL Dictionary Entry Name:** `Duty. Details`
///
/// Generated from XSD type `DutyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Amount` — The amount of this duty.
/// - `Duty` — Text describing this duty.
/// - `DutyCode` — The type of this charge rate, expressed as a code.
/// - `TaxCategory` — The tax category applicable to this duty.
// pub struct Duty { ... }

/// A class to describe a potential contractor, supplier and service provider responding to a tender.
///
/// **UBL Dictionary Entry Name:** `Economic Operator Party. Details`
///
/// Generated from XSD type `EconomicOperatorPartyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `QualifyingParty` — The party qualifying this economic operator.
/// - `EconomicOperatorRole` — The role of the party in a tender consortium.
/// - `Party` — The party information about the economic operator in a tender.
// pub struct EconomicOperatorParty { ... }

/// A class to describe the tenderer contracting role.
///
/// **UBL Dictionary Entry Name:** `Economic Operator Role. Details`
///
/// Generated from XSD type `EconomicOperatorRoleType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `RoleCode` — A code specifying the role of the party.
/// - `RoleDescription` — A textual description of the party role.
// pub struct EconomicOperatorRole { ... }

/// A class to provide information about the preselection of a short list of economic operators for consideration as possible candidates in a tendering process.
///
/// **UBL Dictionary Entry Name:** `Economic Operator Short List. Details`
///
/// Generated from XSD type `EconomicOperatorShortListType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `LimitationDescription` — Text describing the criteria used to restrict the number of candidates.
/// - `ExpectedQuantity` — The number of economic operators expected to be on the short list.
/// - `MaximumQuantity` — The maximum number of economic operators on the short list.
/// - `MinimumQuantity` — The minimum number of economic operators on the short list.
/// - `PreSelectedParty` — The Party who is preselected to submit a Tender in a negotiated procedure. Negotiated procedure is a type of procedure where the Buyer can set the Parties to be invited in the procurement project.
// pub struct EconomicOperatorShortList { ... }

/// A class to describe an Electronic Address where a Party is registered on a given exchange network.
///
/// **UBL Dictionary Entry Name:** `Electronic Address. Details`
///
/// Generated from XSD type `ElectronicAddressType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ExchangeNetworkID` — An identifier for the exchange network where the Party is registered.
/// - `ElectronicAddressID` — An identifier for the Electronic Address of the Party on the given exchange network.
// pub struct ElectronicAddress { ... }

/// A class to define how an environmental emission is calculated.
///
/// **UBL Dictionary Entry Name:** `Emission Calculation Method. Details`
///
/// Generated from XSD type `EmissionCalculationMethodType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `CalculationMethodCode` — A code signifying the method used to calculate the emission.
/// - `FullnessIndicationCode` — A code signifying whether a piece of transport equipment is full, partially full, or empty. This indication is used as a parameter when calculating the environmental emission.
/// - `EmissionFactorSource` — A reference to the source of the emission factor data used in the calculation of this emission.
/// - `EmissionFactorDocumentReference` — A reference to a document that defines, publishes, or justifies the emission factor or calculation method used for this emission.
/// - `MeasurementFromLocation` — A start location from which an environmental emission is calculated.
/// - `MeasurementToLocation` — An end location to which an environmental emission is calculated.
/// - `EmissionCalculationLocation` — The geographical context in which this environmental emission was calculated or for which the emission factor applies, such as the country of a national database or regulatory regime.
// pub struct EmissionCalculationMethod { ... }

/// Details of a certificate path chain used in encryption.
///
/// **UBL Dictionary Entry Name:** `Encryption Certificate Path Chain. Details`
///
/// Generated from XSD type `EncryptionCertificatePathChainType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Value` — The path chain value manifest in the instance.
/// - `URI` — The path chain value references external to the instance.
// pub struct EncryptionCertificatePathChain { ... }

/// Details of an encryption process
///
/// **UBL Dictionary Entry Name:** `Encryption Data. Details`
///
/// Generated from XSD type `EncryptionDataType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `MessageFormat` — The format of the encrypted message.
/// - `EncryptionCertificateAttachment` — A reference to the certificate used in the encryption process.
/// - `EncryptionCertificatePathChain` — A reference to the path chain defined for the encryption process.
/// - `EncryptionSymmetricAlgorithm` — A reference to the symmetric algorithm used for the encryption process.
// pub struct EncryptionData { ... }

/// Details of a symmetric algorithm used in encryption.
///
/// **UBL Dictionary Entry Name:** `Encryption Symmetric Algorithm. Details`
///
/// Generated from XSD type `EncryptionSymmetricAlgorithmType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — A human-readable identifier the algorithm.
/// - `OID` — The object identifier for the algorithm.
// pub struct EncryptionSymmetricAlgorithm { ... }

/// A class to describe how an entity is expected to be treated at the end of its lifecycle, including treatment pathway, processing type, location, and environmental considerations.
///
/// **UBL Dictionary Entry Name:** `End Of Life Treatment. Details`
///
/// Generated from XSD type `EndOfLifeTreatmentType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `TreatmentPathwayCode` — A code identifying the end-of-life pathway.
/// - `ProcessingTypeCode` — A code indicating how the product is processed at end-of-life.
/// - `ImpactCode` — A text description of the environmental impact of the selected end-of-life option.
/// - `TreatmentLocation` — A country or location where end-of-life treatment occurs.
// pub struct EndOfLifeTreatment { ... }

/// A class to describe an endorsement of a document.
///
/// **UBL Dictionary Entry Name:** `Endorsement. Details`
///
/// Generated from XSD type `EndorsementType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `DocumentID` — An identifier for this endorsement.
/// - `ApprovalStatus` — The status of this endorsement.
/// - `Remarks` — Remarks provided by the endorsing party.
/// - `EndorserParty` — The type of party providing this endorsement.
/// - `Signature` — A signature applied to this endorsement.
// pub struct Endorsement { ... }

/// A class to describe the party endorsing a document.
///
/// **UBL Dictionary Entry Name:** `Endorser Party. Details`
///
/// Generated from XSD type `EndorserPartyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `RoleCode` — A code specifying the role of the party providing the endorsement (e.g., issuer, embassy, insurance, etc.).
/// - `SequenceNumeric` — A number indicating the order of the endorsement provided by this party in the sequence in which endorsements are to be applied.
/// - `Party` — The Party who endorses the application.
/// - `SignatoryContact` — The individual representing the exporter who signs the Certificate of Origin application before submitting it to the issuer party.
// pub struct EndorserParty { ... }

/// A class to describe an allocation of energy consumption and its associated environmental emissions
///
/// **UBL Dictionary Entry Name:** `Energy Consumption Allocation. Details`
///
/// Generated from XSD type `EnergyConsumptionAllocationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `EnergySourceTypeCode` — A code indicating the type of energy used (e.g., diesel, electricity, etc.).
/// - `AllocatedEnergyMeasure` — The amount of energy allocated.
/// - `EnvironmentalEmission` — The corresponding Environmental Emissions associated with this allocation.
// pub struct EnergyConsumptionAllocation { ... }

/// A class to describe energy taxes.
///
/// **UBL Dictionary Entry Name:** `Energy Tax Report. Details`
///
/// Generated from XSD type `EnergyTaxReportType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `TaxEnergyAmount` — The monetary amount of taxes (and duties).
/// - `TaxEnergyOnAccountAmount` — The monetary amount of taxes (and duties) paid on account.
/// - `TaxEnergyBalanceAmount` — The monetary amount of the balance of taxes owing.
/// - `TaxScheme` — The relevant taxation scheme.
// pub struct EnergyTaxReport { ... }

/// A class to describe the supply (and therefore consumption) of an amount of energy or water.
///
/// **UBL Dictionary Entry Name:** `Energy Water Supply. Details`
///
/// Generated from XSD type `EnergyWaterSupplyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ConsumptionReport` — An amount of energy or water consumed.
/// - `EnergyTaxReport` — A tax on the consumption of energy or water.
/// - `ConsumptionAverage` — A consumption average.
/// - `EnergyWaterConsumptionCorrection` — Describes any corrections or adjustments to the supply of energy or water.
// pub struct EnergyWaterSupply { ... }

/// A class to describe an environmental emission.
///
/// **UBL Dictionary Entry Name:** `Environmental Emission. Details`
///
/// Generated from XSD type `EnvironmentalEmissionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `EnvironmentalEmissionTypeCode` — A code specifying the type of environmental emission.
/// - `ValueMeasure` — A value measurement for the environmental emission (e.g., total emissions in kg CO2)
/// - `ValueFactorNumeric` — A numeric factor used to calculate the value measurement (e.g., emissions per unit of activity).
/// - `ValueBaseMeasure` — The base quantity to which the value factor applies (e.g., per km, per kg, per unit produced).
/// - `EmissionStandardReference` — A reference to the emission reporting standard or methodological framework used to calculate and report this emission.
/// - `LifecycleStageCode` — A code indicating the lifecycle stage to which this emission applies.
/// - `LifecycleStageDescription` — The lifecycle stage to which this emission applies, expressed as a text.
/// - `Description` — Text describing this environmental emission.
/// - `EmissionCalculationMethod` — A method used to calculate the amount of this emission.
/// - `MeasurementPeriod` — The period during which this environmental emission was measured or calculated.
// pub struct EnvironmentalEmission { ... }

/// A class defining the required criterion for a tenderer to be elligible in a tendering process.
///
/// **UBL Dictionary Entry Name:** `Evaluation Criterion. Details`
///
/// Generated from XSD type `EvaluationCriterionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `EvaluationCriterionTypeCode` — A code that specifies the criterion; it may be financial, technical or organizational.
/// - `Description` — A description of the criterion.
/// - `ThresholdAmount` — Estimated monetary amount of the threshold for the criterion.
/// - `ThresholdQuantity` — Estimated quantity of the threshold for the criterion.
/// - `ExpressionCode` — A code identifying the expression that will be used to evaluate the criterion.
/// - `Expression` — The expression that will be used to evaluate the criterion.
/// - `DurationPeriod` — Describes the period for which the evaluation criterion is valid.
/// - `SuggestedEvidence` — Describes any evidences that ought to be used to satisfy the criterion.
// pub struct EvaluationCriterion { ... }

/// A class to describe a significant occurrence relating to an object, process, or person.
///
/// **UBL Dictionary Entry Name:** `Event. Details`
///
/// Generated from XSD type `EventType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `IdentificationID` — An identifier for this event within an agreed event identification scheme.
/// - `OccurrenceDate` — The date of this event.
/// - `OccurrenceTime` — The time of this event.
/// - `TypeCode` — A code signifying the type of this event.
/// - `Description` — Text describing this event.
/// - `CompletionIndicator` — An indicator that this event has been completed (true) or not (false).
/// - `CurrentStatus` — The current status of this event.
/// - `Contact` — Contacts associated with this event.
/// - `OccurenceLocation` — (Deprecated) The location of this event.
/// - `OccurrenceLocation` — The location of this event.
// pub struct Event { ... }

/// A class to define comments about a retail event.
///
/// **UBL Dictionary Entry Name:** `Event Comment. Details`
///
/// Generated from XSD type `EventCommentType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Comment` — Text commenting on the event.
/// - `IssueDate` — The date on which this comment was made.
/// - `IssueTime` — The time at which this comment was made.
// pub struct EventComment { ... }

/// A class to define a line item describing the expected impacts associated with a retail event involving a specific product at a specific location.
///
/// **UBL Dictionary Entry Name:** `Event Line Item. Details`
///
/// Generated from XSD type `EventLineItemType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `LineNumberNumeric` — The number of this event line item.
/// - `ParticipatingLocationsLocation` — The location of the stores involved in the event described in this line item.
/// - `RetailPlannedImpact` — A planned impact of the event described in this line item.
/// - `SupplyItem` — The product with which the event is associated.
// pub struct EventLineItem { ... }

/// A class defining a specific type of action or situation arranged by the Buyer or the Seller to promote the product or products.
///
/// **UBL Dictionary Entry Name:** `Event Tactic. Details`
///
/// Generated from XSD type `EventTacticType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Comment` — Generic field to add additional information or to specify mutually defined eventTacticTypes that are not currently listed.
/// - `Quantity` — The currencies, units, etc. that describes what is need for the event or promotion Usage example: Number of pallets per store for a stack display
/// - `EventTacticEnumeration` — The set of codes that describes this event tactic.
/// - `Period` — The period covered by this event tactic.
// pub struct EventTactic { ... }

/// A class to define a set of codes that describes a retail tactic.
///
/// **UBL Dictionary Entry Name:** `Event Tactic Enumeration. Details`
///
/// Generated from XSD type `EventTacticEnumerationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ConsumerIncentiveTacticTypeCode` — A code signifying the type of consumer incentive. Examples include:Free Item, Temporary Price reduction
/// - `DisplayTacticTypeCode` — A code signifying the type of display. Examples Include: ON_COUNTER_DISPLAY, FLOOR_GRAPHICS FLOOR_STACK_DISPLAY
/// - `FeatureTacticTypeCode` — A code signifying a special feature. Examples Include: BILLBOARD DIRECT_MAIL_AD, FLYER
/// - `TradeItemPackingLabelingTypeCode` — A code signifying the type of trade item packing and labeling. Examples Include: BONUS_SIZE CO_BRANDED_TRADE_ITEM
// pub struct EventTacticEnumeration { ... }

/// A class to describe an item of evidentiary support for representations of capabilities or the ability to meet tendering requirements, which an economic operator must provide for acceptance into a tendering process.
///
/// **UBL Dictionary Entry Name:** `Evidence. Details`
///
/// Generated from XSD type `EvidenceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this item of evidentiary support.
/// - `EvidenceTypeCode` — A code signifying the type of evidence.
/// - `Name` — The name of the evidence.
/// - `Description` — The textual description for this Evidence.
/// - `CandidateStatement` — Information about a candidate statement that the contracting authority accepts as a sufficient response.
/// - `ConfidentialityLevelCode` — A code specifying the confidentiality level of this evidence.
/// - `EvidenceIssuingParty` — The Party who issues the evidentiary Document.
/// - `DocumentReference` — A reference to the evidentiary document.
/// - `Language` — Information about a required translation to be part of the response, i.e. the language.
// pub struct Evidence { ... }

/// (Deprecated) A reference to evidence.
///
/// **UBL Dictionary Entry Name:** `Evidence Supplied. Details`
///
/// Generated from XSD type `EvidenceSuppliedType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — The identifier of the referenced evidence.
// pub struct EvidenceSupplied { ... }

/// A class to define a line in an ExceptionCriteria document that specifies a threshold for forecast variance, product activity, or performance history, the exceeding of which will trigger an exception message.
///
/// **UBL Dictionary Entry Name:** `Exception Criteria Line. Details`
///
/// Generated from XSD type `ExceptionCriteriaLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this exception criteria line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `ThresholdValueComparisonCode` — Type of comparison to be carried out in reference to the set threshold." Allowed values are: EXCEEDS_EXCEPTION_VALUE FALLS_BELOW_EXCEPTION_VALUE
/// - `ThresholdQuantity` — A quantity beyond which an exception will be triggered.
/// - `ExceptionStatusCode` — A code signifying status specific to a shipment exception.
/// - `CollaborationPriorityCode` — A collaboratively assigned code signifying priority of the Exception. Possible values are: HIGH, LOW, MEDIUM
/// - `ExceptionResolutionCode` — Coded representation of possible resolution methods". Possible values are: DEFAULT_TO_AVERAGE_OF_COMPARED_VALUES DEFAULT_TO_BUYERS_VALUE DEFAULT_TO_HIGH_VALUE DEFAULT_TO_LOW_VALUE DEFAULT_TO_SELLERS_VALUE MANUAL_RESOLUTION MUTUALLY_DEFINED
/// - `SupplyChainActivityTypeCode` — Establishes the criterion for one of the three types of exceptions. There can be three types of exception criteria: Operational, Metric and Forecast Exceptions. This will be set if this Exception is about an Operational Exception. Description could be: A code used to identify an operational exception. Possible values are: CANCELED_ORDERS EMERGENCY_ORDERS ON_HAND ORDERS RECEIPTS SALES SHIPMENTS
/// - `PerformanceMetricTypeCode` — A code signifying a measure of performance.
/// - `EffectivePeriod` — The period during which this exception criteria line is in effect.
/// - `SupplyItem` — The Trade Item that is the subject of the Exception Criterion.
/// - `ForecastExceptionCriterionLine` — Establishes the criterion for one of the three types of exceptions. This class provides the criterion for the kind of forecast exception, the identification of the purpose of the forecast, the source of data and the time basis criterion for the exception.
// pub struct ExceptionCriteriaLine { ... }

/// A class to define a line in an Exception Notification.
///
/// **UBL Dictionary Entry Name:** `Exception Notification Line. Details`
///
/// Generated from XSD type `ExceptionNotificationLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this exception notification line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `Description` — Text describing the exception.
/// - `ExceptionStatusCode` — A code signifying status specific to a shipment exception.
/// - `CollaborationPriorityCode` — Priority of Exception.
/// - `ResolutionCode` — Coded representation of possible resolution methods". Possible values are: DEFAULT_TO_AVERAGE_OF_COMPARED_VALUES DEFAULT_TO_BUYERS_VALUE DEFAULT_TO_HIGH_VALUE DEFAULT_TO_LOW_VALUE DEFAULT_TO_SELLERS_VALUE MANUAL_RESOLUTION MUTUALLY_DEFINED
/// - `ComparedValueMeasure` — The value that was compared with the source value that resulted in the exception
/// - `SourceValueMeasure` — The value used as the basis of comparison
/// - `VarianceQuantity` — The variance of a data item from an expected value during a particular time interval.
/// - `SupplyChainActivityTypeCode` — Establishes the criterion for one of the three types of exceptions: Operational, performance metric and forecast. This reports an exception notification about an operational exception. Description could be: A code used to identify an operational exception. Possible values are: CANCELED_ORDERS EMERGENCY_ORDERS ON_HAND ORDERS RECEIPTS SALES SHIPMENTS
/// - `PerformanceMetricTypeCode` — A code used to identify a measure of performance. It defines the type of the Performance Metric on which an exception criteria is being defined
/// - `ExceptionObservationPeriod` — The period (start-end date) when this exception is observed
/// - `DocumentReference` — A reference to Exception Criteria document can be provided.
/// - `ForecastException` — A forecast accuracy or comparison exception.
/// - `SupplyItem` — The product associated with this exception notification line.
// pub struct ExceptionNotificationLine { ... }

/// A class to define an exchange rate.
///
/// **UBL Dictionary Entry Name:** `Exchange Rate. Details`
///
/// Generated from XSD type `ExchangeRateType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `SourceCurrencyCode` — The reference currency for this exchange rate; the currency from which the exchange is being made.
/// - `SourceCurrencyBaseRate` — In the case of a source currency with denominations of small value, the unit base.
/// - `TargetCurrencyCode` — The target currency for this exchange rate; the currency to which the exchange is being made.
/// - `TargetCurrencyBaseRate` — In the case of a target currency with denominations of small value, the unit base.
/// - `ExchangeMarketID` — An identifier for the currency exchange market used as the source of this exchange rate.
/// - `CalculationRate` — The factor applied to the source currency to calculate the target currency.
/// - `MathematicOperatorCode` — A code signifying whether the calculation rate is a multiplier or a divisor.
/// - `Date` — The date on which the exchange rate was established.
/// - `ForeignExchangeContract` — A contract for foreign exchange.
// pub struct ExchangeRate { ... }

/// A class to describe an external object, such as a document stored at a remote location.
///
/// **UBL Dictionary Entry Name:** `External Reference. Details`
///
/// Generated from XSD type `ExternalReferenceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `URI` — The Uniform Resource Identifier (URI) that identifies the external object as an Internet resource.
/// - `DocumentHash` — A hash value for the externally stored object.
/// - `HashAlgorithmMethod` — A hash algorithm used to calculate the hash value of the externally stored object.
/// - `ExpiryDate` — The date on which availability of the resource can no longer be relied upon.
/// - `ExpiryTime` — The time after which availability of the resource can no longer be relied upon.
/// - `MimeCode` — A code signifying the mime type of the external object.
/// - `FormatCode` — A code signifying the format of the external object.
/// - `EncodingCode` — A code signifying the encoding/decoding algorithm used with the external object.
/// - `CharacterSetCode` — A code signifying the character set of an external document.
/// - `FileName` — The file name of the external object.
/// - `Description` — Text describing the external object.
// pub struct ExternalReference { ... }

/// A class to describe a revenue.
///
/// **UBL Dictionary Entry Name:** `Fee. Details`
///
/// Generated from XSD type `FeeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `FeeTypeCode` — A code signifying the type of this fee.
/// - `FeeAmount` — The amount of a fee.
/// - `FeeDescription` — Text describing this fee.
// pub struct Fee { ... }

/// A class to describe a financial account.
///
/// **UBL Dictionary Entry Name:** `Financial Account. Details`
///
/// Generated from XSD type `FinancialAccountType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — The identifier for this financial account; the bank account number or wallet identifer.
/// - `Name` — The name of this financial account.
/// - `AliasName` — An alias for the name of this financial account, to be used in place of the actual account name for security reasons.
/// - `AccountTypeCode` — A code signifying the type of this financial account.
/// - `AccountFormatCode` — A code signifying the format of this financial account.
/// - `CurrencyCode` — A code signifying the fiat or crypto currency in which this financial account is held.
/// - `BlockchainID` — An identifier of the Blockchain on which the crypto or stablecoin is being held.
/// - `PaymentNote` — Free-form text applying to the Payment for the owner of this account.
/// - `FinancialInstitutionBranch` — The branch of the financial institution associated with this financial account.
/// - `Country` — The country in which the holder of the financial account is domiciled.
// pub struct FinancialAccount { ... }

/// A class to describe the bond guarantee of a tenderer or bid submitter's actual entry into a contract in the event that it is the successful bidder.
///
/// **UBL Dictionary Entry Name:** `Financial Guarantee. Details`
///
/// Generated from XSD type `FinancialGuaranteeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `GuaranteeTypeCode` — A code signifying the type of financial guarantee. For instance "Provisional Guarantee" or "Final Guarantee"
/// - `Description` — Text describing this financial guarantee.
/// - `LiabilityAmount` — The amount of liability in this financial guarantee.
/// - `AmountRate` — The rate used to calculate the amount of liability in this financial guarantee.
/// - `ConstitutionPeriod` — The period during the tendering process to which this financial guarantee has to be settled.
// pub struct FinancialGuarantee { ... }

/// A class to describe a financial institution.
///
/// **UBL Dictionary Entry Name:** `Financial Institution. Details`
///
/// Generated from XSD type `FinancialInstitutionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this financial institution. It is recommended that the ISO 9362 Bank Identification Code (BIC) be used as the ID.
/// - `Name` — The name of this financial institution.
/// - `Address` — The address of this financial institution.
// pub struct FinancialInstitution { ... }

/// As explained in Exception Criteria Line: Three types of exception criteria can be defined, Operational, Metric or Forecast Exceptions. This class provides criteria for forecast exception type: the identification of the purpose of the forecast, the source of data and the time basis criteria for the exception.
///
/// **UBL Dictionary Entry Name:** `Forecast Exception. Details`
///
/// Generated from XSD type `ForecastExceptionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ForecastPurposeCode` — It is either Sales forecast or Order Forecast. Definition can be changed like: "The purpose of the Forecast (either sales or order), about which an exception criteria is being defined".
/// - `ForecastTypeCode` — A code signifying the type of forecast. Example of values are:BASE PROMOTIONAL SEASONAL TOTAL
/// - `IssueDate` — The date on which the forecast was issued.
/// - `IssueTime` — The time at which the forecast was issued.
/// - `DataSourceCode` — A code signifying the partner who provides this information.
/// - `ComparisonDataCode` — A code signifying the partner providing the information in this forecast exception.
/// - `ComparisonForecastIssueTime` — The time at which this comparison forecast was issued.
/// - `ComparisonForecastIssueDate` — The date on which this comparison forecast was issued.
// pub struct ForecastException { ... }

/// Establishes the criterion for one of the three types of exceptions. This class provides criteria for the kind of forecast exception, the identification of the purpose of the forecast, the source of data and the time basis criterion for the exception.
///
/// **UBL Dictionary Entry Name:** `Forecast Exception Criterion Line. Details`
///
/// Generated from XSD type `ForecastExceptionCriterionLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ForecastPurposeCode` — A description of the purpose for the forecast that is assigned to each forecast data item exception criterion.
/// - `ForecastTypeCode` — A description of a Forecast selected from a list.
/// - `ComparisonDataSourceCode` — If it is a forecast comparison exception, this value indicates the other source of information.
/// - `DataSourceCode` — Indication of the partner who provides the information.
/// - `TimeDeltaDaysQuantity` — Time basis in days for the Exception.
// pub struct ForecastExceptionCriterionLine { ... }

/// Detailed information about a particular Forecast Line within a Forecast Document
///
/// **UBL Dictionary Entry Name:** `Forecast Line. Details`
///
/// Generated from XSD type `ForecastLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this forecast line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `FrozenDocumentIndicator` — An indicator that the status of the forecast is modifiable (true) or not (false).
/// - `ForecastTypeCode` — A code signifying the type of forecast. Examples: BASE PROMOTIONAL SEASONAL TOTAL
/// - `ForecastPeriod` — The period to which the forecast applies.
/// - `SalesItem` — Sales information for the item to which this line applies.
// pub struct ForecastLine { ... }

/// A class to define a line in a Forecast Revision describing a revision to a line in a Forecast.
///
/// **UBL Dictionary Entry Name:** `Forecast Revision Line. Details`
///
/// Generated from XSD type `ForecastRevisionLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this forecast revision line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `Description` — Text describing the revision to this line.
/// - `RevisedForecastLineID` — An identifier for the revised forecast line.
/// - `SourceForecastIssueDate` — The date on which the forecast modified by this revision was generated or created.
/// - `SourceForecastIssueTime` — The time at which the forecast modified by this revision was generated or created.
/// - `AdjustmentReasonCode` — A code signifying the reason for the adjustment specified in this forecast revision line.
/// - `ForecastPeriod` — The period to which this forecast revision line applies.
/// - `SalesItem` — Sales information for the item to which this line applies.
// pub struct ForecastRevisionLine { ... }

/// A class to describe a tendering framework agreement.
///
/// **UBL Dictionary Entry Name:** `Framework Agreement. Details`
///
/// Generated from XSD type `FrameworkAgreementType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ExpectedOperatorQuantity` — The number of economic operators expected to participate in this framework agreement.
/// - `MaximumOperatorQuantity` — The maximum number of economic operators allowed to participate in this framework agreement.
/// - `Justification` — Text describing the justification for this framework agreement.
/// - `Frequency` — Text describing the frequency with which subsequent contracts will be awarded.
/// - `EstimatedMaximumValueAmount` — The estimated value which will be spent within a framework agreement over its whole duration, including options and renewals.
/// - `MaximumValueAmount` — The maximum Value which can be spent within a framework agreement over its whole duration, including options and renewals.
/// - `DurationPeriod` — The period during which this framework agreement applies.
/// - `SubsequentProcessTenderRequirement` — A tender requirement intended for consumption by downstream tendering processes derived from the establishment of this framework agreement.
// pub struct FrameworkAgreement { ... }

/// A class to describe fuel consumption.
///
/// **UBL Dictionary Entry Name:** `Fuel Consumption. Details`
///
/// Generated from XSD type `FuelConsumptionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this fuel consumption.
/// - `FuelTypeCode` — The type of fuel, expressed as a code.
/// - `FuelType` — The type of fuel, expressed as text.
/// - `FuelConsumptionMeasure` — The measure of this fuel consumption.
/// - `AdditionalFuelProperty` — One or more additional properties of the fuel being consumed.
/// - `FuelMetering` — One or more meters of the fuel being consumed.
/// - `EnvironmentalEmission` — One or more environmental emissions of the fuel being measured.
/// - `FuelProviderParty` — The Party who provides the fuel.
// pub struct FuelConsumption { ... }

/// A class to describe fuel metering.
///
/// **UBL Dictionary Entry Name:** `Fuel Metering. Details`
///
/// Generated from XSD type `FuelMeteringType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `TypeID` — An identifier for the type of fuel metering.
/// - `Value` — The value of this fuel metering.
// pub struct FuelMetering { ... }

/// A class to describe a fuel property.
///
/// **UBL Dictionary Entry Name:** `Fuel Property. Details`
///
/// Generated from XSD type `FuelPropertyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `TypeID` — An identifier for the type of the fuel property.
/// - `Value` — The value of this fuel property.
// pub struct FuelProperty { ... }

/// A class to describe a separately identifiable quantity of goods of a single product type.
///
/// **UBL Dictionary Entry Name:** `Goods Item. Details`
///
/// Generated from XSD type `GoodsItemType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this goods item.
/// - `SequenceNumberID` — A sequence number differentiating a specific goods item within a consignment.
/// - `Description` — Text describing this goods item to identify it for customs, statistical, or transport purposes.
/// - `HazardousRiskIndicator` — An indication that the transported goods item is subject to an international regulation concerning the carriage of dangerous goods (true) or not (false).
/// - `DeclaredCustomsValueAmount` — The total declared value for customs purposes of the goods item.
/// - `DeclaredForCarriageValueAmount` — The value of this goods item, declared by the shipper or his agent solely for the purpose of varying the carrier's level of liability from that provided in the contract of carriage, in case of loss or damage to goods or delayed delivery.
/// - `DeclaredStatisticsValueAmount` — The total declared value of all the goods items in the same consignment with this goods item that have the same statistical heading.
/// - `FreeOnBoardValueAmount` — The monetary amount that has to be or has been paid as calculated under the applicable trade delivery.
/// - `InsuranceValueAmount` — The amount covered by insurance for this goods item.
/// - `ValueAmount` — The amount on which a duty, tax, or fee will be assessed.
/// - `GrossWeightMeasure` — The weight of this goods item, including packing and packaging but excluding the carrier's equipment.
/// - `NetWeightMeasure` — The weight of this goods item, excluding packing but including packaging that normally accompanies the goods.
/// - `NetNetWeightMeasure` — The total weight of this goods item, excluding all packing and packaging.
/// - `ChargeableWeightMeasure` — The weight on which a charge is to be based.
/// - `GrossVolumeMeasure` — The volume of this goods item, normally calculated by multiplying its maximum length, width, and height.
/// - `NetVolumeMeasure` — The volume contained by a goods item, excluding the volume of any packaging material.
/// - `Quantity` — The number of units making up this goods item.
/// - `PreferenceCriterionCode` — A code signifying the treatment preference for this goods item according to international trading agreements.
/// - `RequiredCustomsID` — An identifier for a set of tariff codes required to specify a type of goods for customs, transport, statistical, or other regulatory purposes.
/// - `CustomsStatusCode` — A code assigned by customs to signify the status of this goods item.
/// - `CustomsProcedureCode` — A code assigned by customs to signifying the customs procedure applied to this Goods Item.
/// - `CustomsTariffQuantity` — Quantity of the units in this goods item as required by customs for tariff, statistical, or fiscal purposes.
/// - `CustomsImportClassifiedIndicator` — An indicator that this goods item has been classified for import by customs (true) or not (false).
/// - `ChargeableQuantity` — The number of units in the goods item to which charges apply.
/// - `ReturnableQuantity` — The number of units in the goods item that may be returned.
/// - `TraceID` — An identifier for use in tracing this goods item, such as the EPC number used in RFID.
/// - `Item` — Product information relating to a goods item.
/// - `GoodsItemContainer` — The transporting of a goods item in a unit of transport equipment (e.g., container).
/// - `FreightAllowanceCharge` — A cost incurred by the shipper in moving goods, by whatever means, from one place to another under the terms of the contract of carriage. In addition to transport costs, this may include such elements as packing, documentation, loading, unloading, and insurance to the extent that they relate to the freight costs.
/// - `InvoiceLine` — Information about an invoice line relating to this goods item.
/// - `OrderLineReference` — A reference to an order line associated with this goods item.
/// - `DespatchLineReference` — A reference to the despatch line associated with this goods item.
/// - `ReceiptLineReference` — A reference to the receipt line associated with this goods item.
/// - `Temperature` — The temperature of the goods item.
/// - `ContainedGoodsItem` — A goods item contained in this goods item.
/// - `OriginAddress` — The region in which the goods have been produced or manufactured, according to criteria laid down for the purposes of application of the customs tariff, or of quantitative restrictions, or of any other measure related to trade.
/// - `Delivery` — The delivery of this goods item.
/// - `Pickup` — The pickup of this goods item.
/// - `Despatch` — The despatch of this goods item.
/// - `BondedWarehouseLocation` — The location of the bonded warehouse where this goods item is temporarily stored.
/// - `MeasurementDimension` — A measurable dimension (length, mass, weight, or volume) of this goods item.
/// - `ContainingPackage` — A package containing this goods item.
/// - `ShipmentDocumentReference` — A reference to a shipping document associated with this goods item.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this goods item.
/// - `MinimumTemperature` — Information about minimum temperature.
/// - `MaximumTemperature` — Information about maximum temperature.
/// - `InsurancePolicy` — One or more Insurance Policies that apply to this Goods Item.
/// - `EnergyConsumptionAllocation` — An allocation of energy consumption and associated emissions attributable to the transport of this goods item.
// pub struct GoodsItem { ... }

/// A class defining how goods items are split across transport equipment.
///
/// **UBL Dictionary Entry Name:** `Goods Item Container. Details`
///
/// Generated from XSD type `GoodsItemContainerType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this goods item container.
/// - `Quantity` — The number of goods items loaded into or onto one piece of transport equipment as a total consignment or part of a consignment.
/// - `TransportEquipment` — A piece of transport equipment used to contain a single goods item.
// pub struct GoodsItemContainer { ... }

/// A class describing a Goods Item Passport or ATA Carnet Counterfoil
///
/// **UBL Dictionary Entry Name:** `Goods Item Passport Counterfoil. Details`
///
/// Generated from XSD type `GoodsItemPassportCounterfoilType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — This identifier for this Goods Item Passport Counterfoil
/// - `GoodsItemPassportID` — The identifier of the Goods Item Passport or ATA Carnet of this counterfoil, usually the number on the upper part of the orange hazard placard required on the means of transport
/// - `FinalReexportationDate` — Final date of re-exportation, if less than the overall validity period of te Goods Item Passport or ATA Carnet
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `CustomsOfficeLocation` — The location of the customs office to where the counterfoil has been presented
/// - `GoodsItem` — A goods item associated with this counterfoil
/// - `ExportationDocumentReference` — A reference to a document used for the export of the goods related to this counterfoil
/// - `ImportationDocumentReference` — A reference to a document used for the import of the goods related to this counterfoil
/// - `ReexportationDocumentReference` — A reference to a document used for the re-exportation of the goods related to this counterfoil
/// - `ReimportationDocumentReference` — A reference to a document used for re-importation of the goods related to this counterfoil
/// - `VoucherDocumentReference` — A reference to a voucher related to this counterfoil
// pub struct GoodsItemPassportCounterfoil { ... }

/// A class to describe the processing of goods and products
///
/// **UBL Dictionary Entry Name:** `Goods Processing. Details`
///
/// Generated from XSD type `GoodsProcessingType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this goods processing.
/// - `TypeCode` — A type code for this goods processing.
/// - `Description` — A description for this goods processing expressed in one or more languages
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `Period` — The period within this goods processing was performed
/// - `ProcessingParty` — The Party who processes the goods.
/// - `CriterionItem` — A reference to a criterion item that applies to this goods processing
/// - `SubGoodsProcessing` — A subordinate processing to this goods processing
// pub struct GoodsProcessing { ... }

/// A class to describe hazardous goods in transit.
///
/// **UBL Dictionary Entry Name:** `Hazardous Goods Transit. Details`
///
/// Generated from XSD type `HazardousGoodsTransitType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `TransportEmergencyCardCode` — An identifier for a transport emergency card describing the actions to be taken in an emergency in transporting the hazardous goods. It may be the identity number of a hazardous emergency response plan assigned by the appropriate authority.
/// - `PackingCriteriaCode` — A code signifying the packaging requirement for transportation of the hazardous goods as assigned by IATA, IMDB, ADR, RID etc.
/// - `HazardousRegulationCode` — A code signifying the set of legal regulations governing the transportation of the hazardous goods.
/// - `InhalationToxicityZoneCode` — A code signifying the Inhalation Toxicity Hazard Zone for the hazardous goods, as defined by the US Department of Transportation.
/// - `TransportAuthorizationCode` — A code signifying authorization for the transportation of hazardous cargo.
/// - `TransitDescription` — A textual description of this hazardous goods transit.
/// - `MaximumTemperature` — The maximum temperature at which the hazardous goods can safely be transported.
/// - `MinimumTemperature` — The minimum temperature at which the hazardous goods can safely be transported.
// pub struct HazardousGoodsTransit { ... }

/// A class to describe a hazardous item.
///
/// **UBL Dictionary Entry Name:** `Hazardous Item. Details`
///
/// Generated from XSD type `HazardousItemType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this hazardous item.
/// - `PlacardNotation` — Text of the placard notation corresponding to the hazard class of this hazardous item. Can also be the hazard identification number of the orange placard (upper part) required on the means of transport.
/// - `PlacardEndorsement` — Text of the placard endorsement that is to be shown on the shipping papers for this hazardous item. Can also be used for the number of the orange placard (lower part) required on the means of transport.
/// - `AdditionalInformation` — Text providing further information about the hazardous substance.
/// - `UNDGCode` — The UN code for this kind of hazardous item.
/// - `UNPackingGroupCode` — A code signifying the UN Packing Group protective packaging requirements for this hazardous item.
/// - `UNPackingGroup` — A text describing the UN Packing Group protective packaging requirements for this hazardous item.
/// - `EmergencyProceduresCode` — A code signifying the emergency procedures for this hazardous item.
/// - `MedicalFirstAidGuideCode` — A code signifying a medical first aid guide appropriate to this hazardous item.
/// - `TunnelRestrictionCode` — A code signifying the restrictions for this hazardous item for passing through a tunnel.
/// - `MaritimePollutantCode` — A code for specifying the maritime pollutant for this hazardous item.
/// - `TechnicalName` — The full technical name of a specific hazardous substance contained in this goods item.
/// - `CategoryName` — The name of the category of hazard that applies to the Item.
/// - `ProperShippingName` — The proper shipping name supplemented.
/// - `HazardousCategoryCode` — A code signifying a kind of hazard for a material.
/// - `UpperOrangeHazardPlacardID` — The number for the upper part of the orange hazard placard required on the means of transport.
/// - `LowerOrangeHazardPlacardID` — The number for the lower part of the orange hazard placard required on the means of transport.
/// - `MarkingID` — An identifier to the marking of the Hazardous Item
/// - `HazardClassID` — An identifier for the hazard class applicable to this hazardous item as defined by the relevant regulation authority (e.g., the IMDG Class Number of the SOLAS Convention of IMO and the ADR/RID Class Number for the road/rail environment).
/// - `HazardousTypeCode` — The code specifying the type of hazard for this hazardous item.
/// - `PackagingDangerLevelCode` — The code specifying the level of danger that the packaging of these dangerous goods must cover for transport purposes.
/// - `GrossWeightMeasure` — The measure of the gross weight (mass) of these transported hazardous items including packaging but excluding the transport equipment.
/// - `NetWeightMeasure` — The net weight of this hazardous item, excluding packaging.
/// - `NetVolumeMeasure` — The volume of this hazardous item, excluding packaging and transport equipment.
/// - `Quantity` — The quantity of goods items in this hazardous item that are hazardous.
/// - `ContactParty` — The individual, group, or body who is contacted in case of a hazardous incident associated with this item.
/// - `SecondaryHazard` — A secondary hazard associated with this hazardous item.
/// - `HazardousGoodsTransit` — Information related to the transit of this kind of hazardous goods.
/// - `EmergencyTemperature` — The threshold temperature at which emergency procedures apply in the handling of temperature-controlled goods.
/// - `FlashpointTemperature` — The flashpoint temperature of this hazardous item; i.e., the lowest temperature at which vapors above a volatile combustible substance ignite in air when exposed to flame.
/// - `AdditionalTemperature` — Another temperature relevant to the handling of this hazardous item.
/// - `PositionOnBoardStowage` — A stowage indicating where to find this hazardous item.
/// - `RadioactiveMaterial` — The Radioactive Material (Class 7) of this Hazadous Item.
/// - `Package` — The Package details for this Hazardous Item.
// pub struct HazardousItem { ... }

/// A class to describe a set of ISPS Requirements.
///
/// **UBL Dictionary Entry Name:** `ISPS Requirements. Details`
///
/// Generated from XSD type `ISPSRequirementsType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for these ISPS requirements.
/// - `ValidISSCIndicator` — An indicator of whether the International Ship Security Certificate (ISSC) is valid (true) or not (false).
/// - `ISSCAbsenceReason` — A text describing the reason if not having a International Ship Security Certificate (ISSC).
/// - `ISSCExpiryDate` — The expiration date of the International Ship Security Certificate (ISSC).
/// - `SSPOnBoardIndicator` — An indicator of whether the vessel has a Ship Security Plan (SSP) on board (true) or not (false).
/// - `SSPSecurityMeasuresAppliedIndicator` — An indication of whether the Ship Security Plan (SSP) meassures are applied (true) or not (false).
/// - `CurrentOperatingSecurityLevelCode` — A code describing the current operating security level.
/// - `AdditionalMattersDescription` — A textual description of any addidtional matters concerning these ISPS requirements.
/// - `AdditionalSecurityMeasure` — Security measures for these ISPS requirements in addition to those in the ship security plan, such as special measures taken in response to unforeseen events.
/// - `PortCallRecord` — The port call records for these ISPS requirements.
/// - `ShipToShipActivityRecord` — The recordded ship to ship activities for these ISPS requirements.
/// - `ReportLocation` — The location where these ISPC requirements are reported.
/// - `ISSCIssuerParty` — The Party who issues the International Ship Security Certificate (ISSC).
/// - `SecurityOfficerPerson` — The security officer reponsible for these ISPC requirements.
// pub struct ISPSRequirements { ... }

/// A class to describe an immobilized security to be used as a guarantee.
///
/// **UBL Dictionary Entry Name:** `Immobilized Security. Details`
///
/// Generated from XSD type `ImmobilizedSecurityType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ImmobilizationCertificateID` — An identifier for the certificate of this immobilized security.
/// - `SecurityID` — An identifier for the security being immobilized.
/// - `IssueDate` — The date on which this immobilized security was issued.
/// - `FaceValueAmount` — The value of the security on the day it was immobilized.
/// - `MarketValueAmount` — The current market value of the immobilized security.
/// - `SharesNumberQuantity` — The number of shares immobilized.
/// - `IssuerParty` — The Party who issues the Immobilized Security Certificate.
// pub struct ImmobilizedSecurity { ... }

/// A class to define a line in an Instruction for Returns.
///
/// **UBL Dictionary Entry Name:** `Instruction For Returns Line. Details`
///
/// Generated from XSD type `InstructionForReturnsLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this instruction for returns line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `Quantity` — The quantity of goods being returned.
/// - `ManufacturerParty` — The Party who manufactures the Goods being returned.
/// - `Item` — A description of the item being returned.
// pub struct InstructionForReturnsLine { ... }

/// A class to define an insurance policy.
///
/// **UBL Dictionary Entry Name:** `Insurance Policy. Details`
///
/// Generated from XSD type `InsurancePolicyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this insurance policy, such as the policy number.
/// - `InsuranceTypeCode` — A code describing the type of insurance under this policy.
/// - `InsuranceTypeDescription` — A textual description of the type of insurance under this policy.
/// - `InsuredValueAmount` — The amount covered by this Insurance Policy.
/// - `DeductibleAmount` — The deductible amount specified in the policy of this Insurance.
/// - `ExcessAmount` — The excess amount specified in the policy of this Insurance.
/// - `InsurancePremiumAmount` — The amount of the premium payable to an insurance company under this Insurance Policy.
/// - `InsurerParty` — The Party providing the insurance under this Insurance Policy.
/// - `BrokerParty` — Intermediary agent or broker of the insurance under this Insurance Policy.
/// - `PolicyHolderParty` — The Party holding this Insurance Policy.
/// - `BeneficiaryParty` — A Party entitled to benefit from this Insurance Policy.
/// - `PolicyDocumentReference` — A reference to the policy document.
// pub struct InsurancePolicy { ... }

/// A class to describe an interest rate applied to a monetary amount over a defined period.
///
/// **UBL Dictionary Entry Name:** `Interest Rate. Details`
///
/// Generated from XSD type `InterestRateType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `InterestRatePercent` — The numeric value of the interest rate expressed as a percentage.
/// - `TimeBasisCode` — A code specifying the time basis to which the interest rate applies, such as per annum or per day.
/// - `CalculationMethodCode` — A code specifying how the interest is calculated (e.g., simple, compount).
// pub struct InterestRate { ... }

/// A class to define a line in an Inventory Report.
///
/// **UBL Dictionary Entry Name:** `Inventory Report Line. Details`
///
/// Generated from XSD type `InventoryReportLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this inventory report line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `Quantity` — The quantity of the item reported that is currently in stock.
/// - `InventoryValueAmount` — The value of the quantity of the item reported that is currently in stock.
/// - `AvailabilityDate` — The date from which the goods will be available. If not present, the goods are available now.
/// - `AvailabilityStatusCode` — A code signifying the item's level of availability.
/// - `Item` — The item associated with this inventory report line.
/// - `InventoryLocation` — The location of the reported quantity of goods.
// pub struct InventoryReportLine { ... }

/// A class to define a line in an Invoice.
///
/// **UBL Dictionary Entry Name:** `Invoice Line. Details`
///
/// Generated from XSD type `InvoiceLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this invoice line.
/// - `UUID` — A universally unique identifier for this invoice line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `InvoicedQuantity` — The quantity (of items) on this invoice line.
/// - `LineExtensionAmount` — The total amount for this invoice line, including allowance charges but net of taxes.
/// - `TaxInclusiveLineExtensionAmount` — The total amount for this invoice line, including all allowances, charges and taxes.
/// - `TaxPointDate` — The date of this invoice line, used to indicate the point at which tax becomes applicable.
/// - `AccountingCostCode` — The buyer's accounting cost centre for this invoice line, expressed as a code.
/// - `AccountingCost` — The buyer's accounting cost centre for this invoice line, expressed as text.
/// - `PaymentPurposeCode` — A code signifying the business purpose for this payment.
/// - `FreeOfChargeIndicator` — An indicator that this invoice line is free of charge (true) or not (false). The default is false.
/// - `InvoicePeriod` — An invoice period to which this invoice line applies.
/// - `OrderLineReference` — A reference to an order line associated with this invoice line.
/// - `DespatchLineReference` — A reference to a despatch line associated with this invoice line.
/// - `ReceiptLineReference` — A reference to a receipt line associated with this invoice line.
/// - `WorkReportLineReference` — A reference to a work report line associated with this invoice line.
/// - `BillingReference` — A reference to a billing document associated with this invoice line.
/// - `DocumentReference` — A reference to a document associated with this invoice line.
/// - `PricingReference` — A reference to pricing and item location information associated with this invoice line.
/// - `PurchaseReference` — A reference to an object, such as a subscription number, telephone number, meter, vehicle, person, etc., to which this Invoice Line relates.
/// - `OriginatorParty` — The Party who originates the Order to which the Invoice is related.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `CollectedForParty` — The Party on whose behalf this item or amount is collected.
/// - `Delivery` — A delivery associated with this invoice line.
/// - `PaymentTerms` — A specification of payment terms associated with this invoice line.
/// - `AllowanceCharge` — An allowance or charge associated with this invoice line.
/// - `TaxTotal` — A total amount of taxes of a particular kind applicable to this invoice line.
/// - `WithholdingTaxTotal` — A reference to a TaxTotal class describing the amount that has been withhold by the authorities, e.g. if the creditor is in dept because of non paid taxes.
/// - `Item` — The item associated with this invoice line.
/// - `Price` — The price of the item associated with this invoice line.
/// - `DeliveryTerms` — Terms and conditions of the delivery associated with this invoice line.
/// - `SubInvoiceLine` — An invoice line subsidiary to this invoice line.
/// - `ItemPriceExtension` — The price extension, calculated by multiplying the price per unit by the quantity of items on this invoice line.
// pub struct InvoiceLine { ... }

/// A class to describe an item of trade. It includes a generic description applicable to all examples of the item together with optional subsidiary descriptions of any number of actual instances of the type.
///
/// **UBL Dictionary Entry Name:** `Item. Details`
///
/// Generated from XSD type `ItemType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Description` — Text describing this item.
/// - `PackQuantity` — The unit packaging quantity; the number of subunits making up this item.
/// - `PackSizeNumeric` — The number of items in a pack of this item.
/// - `CatalogueIndicator` — An indicator that this item was ordered from a catalogue (true) or not (false).
/// - `Name` — A short name optionally given to this item, such as a name from a catalogue, as distinct from a description.
/// - `ItemTypeCode` — A code indicating the type of the item (eg., service, product, etc.).
/// - `HazardousRiskIndicator` — An indication that the transported item, as delivered, is subject to an international regulation concerning the carriage of dangerous goods (true) or not (false).
/// - `AdditionalInformation` — Further details regarding this item (e.g., the URL of a relevant web page).
/// - `Keyword` — A keyword (search string) for this item, assigned by the seller party. Can also be a synonym for the name of the item.
/// - `BrandName` — A brand name of this item.
/// - `ModelName` — A model name of this item.
/// - `WarrantyInformation` — Text describing the warranty for this item.
/// - `LifecycleStageCode` — A code indicating the product’s lifecycle stage (e.g., sourcing, manufacturing, distribution, usage, end-of-life)
/// - `LifecycleStageDescription` — A text describing the specific environmental impact associated with this product's lifecycle stage.
/// - `BuyersItemIdentification` — Identifying information for this item, assigned by the buyer.
/// - `SellersItemIdentification` — Identifying information for this item, assigned by the seller.
/// - `ManufacturersItemIdentification` — Identifying information for this item, assigned by the manufacturer.
/// - `StandardItemIdentification` — Identifying information for this item, assigned according to a standard system.
/// - `CatalogueItemIdentification` — Identifying information for this item, assigned according to a cataloguing system.
/// - `AdditionalItemIdentification` — An additional identifier for this item.
/// - `CatalogueDocumentReference` — A reference to the catalogue in which this item appears.
/// - `ItemSpecificationDocumentReference` — A reference to a specification document for this item.
/// - `OriginCountry` — The country of origin of this item.
/// - `CommodityClassification` — A classification of this item according to a specific system for classifying commodities.
/// - `TransactionConditions` — A set of sales conditions applying to this item.
/// - `HazardousItem` — Information pertaining to this item as a hazardous item.
/// - `ClassifiedTaxCategory` — A tax category applicable to this item.
/// - `AdditionalItemProperty` — An additional property of this item.
/// - `ManufacturerParty` — The Party who manufacters this Item.
/// - `InformationContentProviderParty` — The Party who specifies this Item.
/// - `OriginAddress` — A region (not country) of origin of this item.
/// - `ItemInstance` — A trackable, unique instantiation of this item.
/// - `Certificate` — A certificate associated with this item.
/// - `EnvironmentalCertificate` — One or more environmental certificatations issued for this item.
/// - `Dimension` — One of the measurable dimensions (length, mass, weight, or volume) of this item.
/// - `EnvironmentalEmission` — One or more environmental emissions of this item.
/// - `CircularityProfile` — The Circularity Profile of this Item
// pub struct Item { ... }

/// A class to provide information about price and quantity of an item for use in price comparisons based on price, quantity, or measurements.
///
/// **UBL Dictionary Entry Name:** `Item Comparison. Details`
///
/// Generated from XSD type `ItemComparisonType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `PriceAmount` — The price for the Item Comparison
/// - `Quantity` — The quantity for which this comparison is valid.
// pub struct ItemComparison { ... }

/// A class for assigning identifying information to an item.
///
/// **UBL Dictionary Entry Name:** `Item Identification. Details`
///
/// Generated from XSD type `ItemIdentificationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for the item.
/// - `ExtendedID` — An extended identifier for the item that identifies the item with specific properties, e.g., Item 123 = Chair / Item 123 Ext 45 = brown chair. Two chairs can have the same item number, but one is brown. The other is white.
/// - `BarcodeSymbologyID` — An identifier for a system of barcodes.
/// - `IssuerScopeID` — A scope within which the issuer has assigned this identifier.
/// - `PhysicalAttribute` — A physical attribute of the item.
/// - `MeasurementDimension` — A measurable dimension (length, mass, weight, or volume) of the item.
/// - `IssuerParty` — The Party who issues this Item Identification.
// pub struct ItemIdentification { ... }

/// A class to define a line in an Item Information Request asking a trading partner for item information.
///
/// **UBL Dictionary Entry Name:** `Item Information Request Line. Details`
///
/// Generated from XSD type `ItemInformationRequestLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `TimeFrequencyCode` — A code signifying the frequency with which item information will be sent to the requester.
/// - `SupplyChainActivityTypeCode` — A code used to identify the type of supply chain activity about which information request is issued. Examples: CANCELED_ORDERS EMERGENCY_ORDERS ON_HAND ORDERS
/// - `ForecastTypeCode` — The information request can be either about supply chain activity or about forecasts or about performance metrics, so it will be optional
/// - `PerformanceMetricTypeCode` — A code signifying a measure of performance.
/// - `Period` — A period for which this information is requested.
/// - `SalesItem` — Sales information for the item to which this line applies.
// pub struct ItemInformationRequestLine { ... }

/// A class to describe a specific, trackable instance of an item.
///
/// **UBL Dictionary Entry Name:** `Item Instance. Details`
///
/// Generated from XSD type `ItemInstanceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ProductTraceID` — An identifier used for tracing this item instance, such as the EPC number used in RFID.
/// - `ManufactureDate` — The date on which this item instance was manufactured.
/// - `ManufactureTime` — The time at which this item instance was manufactured.
/// - `BestBeforeDate` — The date before which it is best to use this item instance.
/// - `RegistrationID` — The registration identifier of this item instance.
/// - `SerialID` — The serial number of this item instance.
/// - `AdditionalItemProperty` — An additional property of this item instance.
/// - `LotIdentification` — The lot identifier of this item instance (the identifier that allows recall of the item if necessary).
// pub struct ItemInstance { ... }

/// A class for information about pricing structure, lead time, delivery, and location associated with an item.
///
/// **UBL Dictionary Entry Name:** `Item Location Quantity. Details`
///
/// Generated from XSD type `ItemLocationQuantityType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `LeadTimeMeasure` — The lead time, i.e., the time taken from the time at which an item is ordered to the time of its delivery.
/// - `MinimumQuantity` — The minimum quantity that can be ordered to qualify for a specific price.
/// - `MaximumQuantity` — The maximum quantity that can be ordered to qualify for a specific price.
/// - `HazardousRiskIndicator` — An indication that the transported item, as delivered, in the stated quantity to the stated location, is subject to an international regulation concerning the carriage of dangerous goods (true) or not (false).
/// - `TradingRestrictions` — Text describing trade restrictions on the quantity of this item or on the item itself.
/// - `ApplicableTerritoryAddress` — The applicable sales territory.
/// - `Price` — The price associated with this item location quantity
/// - `DeliveryUnit` — A delivery unit in which the item is located.
/// - `ApplicableTaxCategory` — A tax category applicable to this item location quantity.
/// - `Package` — The package to which this price applies.
/// - `AllowanceCharge` — An allowance or charge associated with this item location quantity.
/// - `DependentPriceReference` — The price of the item as a percentage of the price of some other item.
/// - `ApplicableDeliveryPeriod` — The period during which item must be delivered for the price to apply
// pub struct ItemLocationQuantity { ... }

/// A class to define a management profile for an item.
///
/// **UBL Dictionary Entry Name:** `Item Management Profile. Details`
///
/// Generated from XSD type `ItemManagementProfileType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `FrozenPeriodDaysNumeric` — The number of days in the future that an order forecast quantity automatically becomes a confirmed order for a product.
/// - `MinimumInventoryQuantity` — The quantity of the item that will trigger a replenishment order to avoid depleting the safety stock.
/// - `MultipleOrderQuantity` — The order quantity multiples in which the product may be ordered.
/// - `OrderIntervalDaysNumeric` — The number of days between regular replenishment orders for the product.
/// - `ReplenishmentOwnerDescription` — The trading partner maintaining this item management profile.
/// - `TargetServicePercent` — The Unit Service Level the trading partners expect to be maintained, expressed as a percentage. Unite Service Level (USL) is a term used in Inventory Management, which is sometimes known as "fill rate", counts the average number of units short expressed as the percentage of the order quantity.
/// - `TargetInventoryQuantity` — The target inventory quantity.
/// - `EffectivePeriod` — The period during which this profile is effective.
/// - `Item` — The item associated with this item management profile.
/// - `ItemLocationQuantity` — A set of location-specific properties (e.g., price and quantity) associated with the item.
// pub struct ItemManagementProfile { ... }

/// A class to describe a specific property of an item.
///
/// **UBL Dictionary Entry Name:** `Item Property. Details`
///
/// Generated from XSD type `ItemPropertyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this property of an item.
/// - `Name` — The name of this item property.
/// - `NameCode` — The name of this item property, expressed as a code.
/// - `TestMethod` — The method of testing the value of this item property.
/// - `Value` — The value of this item property, expressed as text.
/// - `ValueQuantity` — The value of this item property, expressed as a quantity.
/// - `ValueQualifier` — Text qualifying the value of the property.
/// - `ImportanceCode` — A code signifying the importance of this property in using it to describe a related Item.
/// - `ListValue` — The value expressed as a text in case the property is a value in a list. For example, a colour.
/// - `UsabilityPeriod` — The period during which this item property is valid.
/// - `ItemPropertyGroup` — A description of the property group to which this item property belongs.
/// - `RangeDimension` — The range of values for the dimensions of this property.
/// - `ItemPropertyRange` — A range of values for this item property.
/// - `StandardPropertyIdentification` — Identifying information for this property, assigned according to a standard system.
/// - `SubItemProperty` — A property subsidiary to this property.
// pub struct ItemProperty { ... }

/// A class to describe a property group or classification.
///
/// **UBL Dictionary Entry Name:** `Item Property Group. Details`
///
/// Generated from XSD type `ItemPropertyGroupType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this group of item properties.
/// - `Name` — The name of this item property group.
/// - `ImportanceCode` — A code signifying the importance of this property group in using it to describe a required Item.
// pub struct ItemPropertyGroup { ... }

/// A class to describe a range of values for an item property.
///
/// **UBL Dictionary Entry Name:** `Item Property Range. Details`
///
/// Generated from XSD type `ItemPropertyRangeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `MinimumValue` — The minimum value in this range of values.
/// - `MaximumValue` — The maximum value in this range of values.
// pub struct ItemPropertyRange { ... }

/// A class to describe a language.
///
/// **UBL Dictionary Entry Name:** `Language. Details`
///
/// Generated from XSD type `LanguageType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this language.
/// - `Name` — The name of this language.
/// - `LocaleCode` — A code signifying the locale in which this language is used.
// pub struct Language { ... }

/// A class to describe a reference to a piece of legislation.
///
/// **UBL Dictionary Entry Name:** `Legislation. Details`
///
/// Generated from XSD type `LegislationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier to refer to the legislation.
/// - `Title` — The title of the legislation.
/// - `Description` — The textual description of the legislation.
/// - `JurisdictionLevel` — The jurisdiction level for the legislation.
/// - `Article` — The article of the legislation.
/// - `URI` — A URI to the legislation.
/// - `Language` — The language of the legislation.
/// - `JurisdictionRegionAddress` — The geopolitical region in which this legislation applies.
// pub struct Legislation { ... }

/// A class to describe a line item.
///
/// **UBL Dictionary Entry Name:** `Line Item. Details`
///
/// Generated from XSD type `LineItemType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this line item, assigned by the buyer.
/// - `SalesOrderID` — An identifier for this line item, assigned by the seller.
/// - `UUID` — A universally unique identifier for this line item.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `LineStatusCode` — A code signifying the status of this line item with respect to its original state.
/// - `Quantity` — The quantity of items associated with this line item.
/// - `LineExtensionAmount` — The total amount for this line item, including allowance charges but net of taxes.
/// - `TaxInclusiveLineExtensionAmount` — The total amount for this line item, including all allowances, charges and taxes.
/// - `TotalTaxAmount` — The total tax amount for this line item.
/// - `MinimumQuantity` — The minimum quantity of the item associated with this line.
/// - `MaximumQuantity` — The maximum quantity of the item associated with this line.
/// - `MinimumBackorderQuantity` — The minimum back order quantity of the item associated with this line (where back order is allowed).
/// - `MaximumBackorderQuantity` — The maximum back order quantity of the item associated with this line (where back order is allowed).
/// - `InspectionMethodCode` — A code signifying the inspection requirements for the item associated with this line item.
/// - `PartialDeliveryIndicator` — An indicator that a partial delivery is allowed (true) or not (false).
/// - `BackOrderAllowedIndicator` — An indicator that back order is allowed (true) or not (false).
/// - `AccountingCostCode` — The buyer's accounting cost centre for this line item, expressed as a code.
/// - `AccountingCost` — The buyer's accounting cost centre for this line item, expressed as text.
/// - `WarrantyInformation` — Text describing a warranty (provided by WarrantyParty) for the good or service described in this line item.
/// - `Delivery` — A delivery associated with this line item.
/// - `DeliveryTerms` — Terms and conditions of the delivery associated with this line item.
/// - `OriginatorParty` — The Party who originates the Order to which this Line Item is related.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `OrderedShipment` — An ordered shipment associated with this line item.
/// - `PricingReference` — A reference to pricing and item location information associated with this line item.
/// - `AllowanceCharge` — An allowance or charge associated with this line item.
/// - `Price` — The price of the item of trade associated with this line item.
/// - `Item` — The item of trade associated with this line item.
/// - `SubLineItem` — The subsidiary line items that constitute the main line item, such as in a bill of materials.
/// - `WarrantyValidityPeriod` — The period during which the warranty associated with this line item is valid.
/// - `WarrantyParty` — The Party who is responsible for any warranty associated with this Line Item.
/// - `TaxTotal` — A total amount of taxes of a particular kind applicable to this item.
/// - `ItemPriceExtension` — The price extension, calculated by multiplying the price per unit by the quantity of items.
/// - `LineReference` — A reference to a line in a document associated with this line item.
// pub struct LineItem { ... }

/// A class to define a reference to a line in a document.
///
/// **UBL Dictionary Entry Name:** `Line Reference. Details`
///
/// Generated from XSD type `LineReferenceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `LineID` — Identifies the referenced line in the document.
/// - `UUID` — A universally unique identifier for this line reference.
/// - `LineStatusCode` — A code signifying the status of the referenced line with respect to its original state.
/// - `DocumentReference` — A reference to the document containing the referenced line.
// pub struct LineReference { ... }

/// A class to describe responses to a line in a document.
///
/// **UBL Dictionary Entry Name:** `Line Response. Details`
///
/// Generated from XSD type `LineResponseType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `LineReference` — A reference to the line being responded to.
/// - `Response` — A response to the referenced line.
// pub struct LineResponse { ... }

/// A class to describe a location.
///
/// **UBL Dictionary Entry Name:** `Location. Details`
///
/// Generated from XSD type `LocationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this location, e.g., the EAN Location Number, GLN.
/// - `Description` — Text describing this location.
/// - `Conditions` — Free-form text describing the physical conditions of the location.
/// - `CountrySubentity` — A territorial division of a country, such as a county or state, expressed as text.
/// - `CountrySubentityCode` — A territorial division of a country, such as a county or state, expressed as a code.
/// - `LocationTypeCode` — A code signifying the type of location.
/// - `InformationURI` — The Uniform Resource Identifier (URI) of a document providing information about this location.
/// - `Name` — The name of this location.
/// - `ValidityPeriod` — A period during which this location can be used (e.g., for delivery).
/// - `Address` — The address of this location.
/// - `Storage` — The description and requirements of the storage at this location.
/// - `SubsidiaryLocation` — A location subsidiary to this location.
/// - `LocationCoordinate` — The geographical coordinates of this location.
// pub struct Location { ... }

/// A class for defining a set of geographical coordinates (apparently misnamed).
///
/// **UBL Dictionary Entry Name:** `Location Coordinate. Details`
///
/// Generated from XSD type `LocationCoordinateType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `CoordinateSystemCode` — A code signifying the location system used.
/// - `LatitudeDegreesMeasure` — The degree component of a latitude measured in degrees and minutes.
/// - `LatitudeMinutesMeasure` — The minutes component of a latitude measured in degrees and minutes (modulo 60).
/// - `LatitudeDirectionCode` — A code signifying the direction of latitude measurement from the equator (north or south).
/// - `LongitudeDegreesMeasure` — The degree component of a longitude measured in degrees and minutes.
/// - `LongitudeMinutesMeasure` — The minutes component of a longitude measured in degrees and minutes (modulo 60).
/// - `LongitudeDirectionCode` — A code signifying the direction of longitude measurement from the prime meridian (east or west).
/// - `AltitudeMeasure` — The altitude of the location.
// pub struct LocationCoordinate { ... }

/// A class defining how to treat different lots in a single procurement.
///
/// **UBL Dictionary Entry Name:** `Lot Distribution. Details`
///
/// Generated from XSD type `LotDistributionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `MaximumLotsAwardedNumeric` — The maximum number of lots that can be awarded to a single tenderer.
/// - `MaximumLotsSubmittedNumeric` — The maximum number of lots to which a tenderer can submit an offer to.
/// - `GroupingLots` — Description on how to combine lots when submitting a tender.
/// - `LotsGroup` — A combination of lots used when evaluating a tender.
// pub struct LotDistribution { ... }

/// A class for defining a lot identifier (the identifier of a set of item instances that would be used in case of a recall of that item).
///
/// **UBL Dictionary Entry Name:** `Lot Identification. Details`
///
/// Generated from XSD type `LotIdentificationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `LotNumberID` — An identifier for the lot.
/// - `ExpiryDate` — The expiry date of the lot.
/// - `AdditionalItemProperty` — An additional property of the lot.
// pub struct LotIdentification { ... }

/// A class for defining set of lots.
///
/// **UBL Dictionary Entry Name:** `Lots Group. Details`
///
/// Generated from XSD type `LotsGroupType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `LotsGroupID` — An identifier for the lotsgroup.
/// - `ProcurementProjectLotReference` — A Procurement project lot that is included in this LotsGroup.
// pub struct LotsGroup { ... }

/// A class to desccribe a maritime health declaration.
///
/// **UBL Dictionary Entry Name:** `Maritime Health Declaration. Details`
///
/// Generated from XSD type `MaritimeHealthDeclarationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this maritime health declaration.
/// - `InfectiousDiseaseCaseOnBoardIndicator` — Indicates whether there is an infectious diasase case on board (true) or not (false).
/// - `MoreIllThanExpectedIndicator` — Indicates whether a sick person is more ill than extected (true) or not (false).
/// - `MedicalPractitionerConsultedIndicator` — Indicates whether a medical practioner has been consulted (true) or not (false).
/// - `StowawaysFoundOnBoardIndicator` — Indicates whether stowaways have been found on board (true) or not (false).
/// - `SickAnimalOnBoardIndicator` — Indicates whether a sick animal is on board (true) or not (false).
/// - `FumigatedCargoTransportIndicator` — Indicates whether the cargo under transport has been fumigated (true) or not (false).
/// - `SanitaryMeasuresAppliedIndicator` — Indicates whether sanity measures are applied (true) or not (false).
/// - `ValidSanitationCertificateOnBoardIndicator` — Indicates whether a valid sanitary certificate is on board (true) or not (false).
/// - `ReinspectionRequiredIndicator` — Indicates whether a reinspaction is required (true) or not (false).
/// - `TotalDeadPersonQuantity` — Specifies the total number of dead persons on board the vessel.
/// - `TotalIllPersonQuantity` — Specifies the total number of ill persons on board the vessel.
/// - `SickAnimalDescription` — Describes any sick animals on board the vessel.
/// - `StowawayDescription` — Describes any stowaways on board the vessel.
/// - `LastDrinkingWaterAnalysisDate` — The date when the last drinking water analysis was made.
/// - `WHOAffectedAreaVisit` — A WHO Affected Area visit related to this maritime health declaration.
/// - `PersonnelHealthIncident` — A personal health incident related to this maritime health declaration.
/// - `SanitaryMeasure` — A sanitary meassure for this health declaration.
/// - `PlaceOfReportLocation` — The location where this maritime health declaration is reported.
/// - `MedicalCertificate` — The medical certificate for this maritime health declaration.
/// - `ShipSanitationControlCertificate` — A certificate describing the sanitation control of this maritime health certificate.
/// - `ShipSanitationControlExemptionDocumentReference` — A reference to a document evidencing the exemption of a ship sanitation control certificate, when absent.
// pub struct MaritimeHealthDeclaration { ... }

/// A class to describe a vessel used for transport by water (including sea, river, and canal).
///
/// **UBL Dictionary Entry Name:** `Maritime Transport. Details`
///
/// Generated from XSD type `MaritimeTransportType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `VesselID` — An identifier for a specific vessel.
/// - `VesselName` — The name of the vessel.
/// - `RadioCallSignID` — The radio call sign of the vessel.
/// - `MMSIRegistrationID` — A Maritime Mobile Service Identity (MMSI) required for this vessel.
/// - `ShipsRequirements` — Information about what services a vessel will require when it arrives at a port, such as refueling, maintenance, waste disposal etc.
/// - `GrossTonnageMeasure` — Gross tonnage is calculated by measuring a ship's volume (from keel to funnel, to the outside of the hull framing) and applying a mathematical formula and is used to determine things such as a ship's manning regulations, safety rules, registration fees and port dues.
/// - `NetTonnageMeasure` — Net tonnage is calculated by measuring a ship's internal volume and applying a mathematical formula and is used to calculate the port duties.
/// - `SegregatedBallastMeasure` — The measure of the segregated ballast of the vessel.
/// - `ShipConfigurationCode` — A code specifying the ship configuration.
/// - `INFShipClassCode` — A code specifying the irradiated nuclear fuel (INF) ship class.
/// - `AntennaLocus` — The locus or exact location of the antenna on the vessel
/// - `RegistryCertificateDocumentReference` — The certificate issued to the ship by the ships registry in a given flag state.
/// - `RegistryPortLocation` — The port in which a vessel is registered or permanently based.
/// - `VesselDynamics` — The vessel dynamics for this maritime transport.
// pub struct MaritimeTransport { ... }

/// A class to describe a transaction of maritime waste.
///
/// **UBL Dictionary Entry Name:** `Maritime Waste. Details`
///
/// Generated from XSD type `MaritimeWasteType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this maritime waste transaction.
/// - `Description` — A text descriping this maritime waste transaction.
/// - `WasteTypeCode` — A code specifying the type of waste in this maritime waste transaction.
/// - `ToBeDeliveredMeasure` — The messure of waste to be delivered.
/// - `RetainedOnBoardMeasure` — The meassure of waste retained on board.
/// - `MaxDedicatedStorageCapacityMeasure` — The messure for the maximum dedicated storage capacity.
/// - `EstimatedGeneratedUntilNextPortMeasure` — The messure of waste generated until the next port.
/// - `RemainingWasteDeliveryPortLocation` — The location of the port where the remaining waste is delivered.
// pub struct MaritimeWaste { ... }

/// A class to describe how a message is delivered (routed).
///
/// **UBL Dictionary Entry Name:** `Message Delivery. Details`
///
/// Generated from XSD type `MessageDeliveryType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ProtocolID` — An identifier for the protocol to be used within this message delivery.
/// - `EnvelopeTypeCode` — A code signifying the type of envelope to be used within this message delivery (e.g. OASIS BDX Business Document Envelope).
/// - `EndpointURI` — The Uniform Resource Identifier (URI) of the access point (e.g. an HTTP URL including the port).
// pub struct MessageDelivery { ... }

/// A class to describe a meter and its readings.
///
/// **UBL Dictionary Entry Name:** `Meter. Details`
///
/// Generated from XSD type `MeterType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `MeterNumber` — The meter number, expressed as text.
/// - `MeterName` — The name of this meter, which serves as an identifier to distinguish a main meter from a submeter.
/// - `MeterConstant` — The factor by which readings of this meter must be multiplied to calculate consumption, expressed as text.
/// - `MeterConstantCode` — A code signifying the formula to be used in applying the meter constant.
/// - `TotalDeliveredQuantity` — The quantity delivered; the total quantity consumed as calculated from the meter readings.
/// - `MeterReading` — A reading of this meter.
/// - `MeterProperty` — A property of this meter.
// pub struct Meter { ... }

/// The name of this meter property.
///
/// **UBL Dictionary Entry Name:** `Meter Property. Details`
///
/// Generated from XSD type `MeterPropertyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Name` — The name of this meter property, expressed as a code.
/// - `NameCode` — The value of this meter property, expressed as text.
/// - `Value` — The value of this meter property, expressed as a quantity.
/// - `ValueQuantity` — The value of this meter property, expressed as a quantity.
/// - `ValueQualifier` — An additional value to qualify the value of the meter
// pub struct MeterProperty { ... }

/// A class to describe a meter reading.
///
/// **UBL Dictionary Entry Name:** `Meter Reading. Details`
///
/// Generated from XSD type `MeterReadingType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this meter reading.
/// - `MeterReadingType` — The type of this meter reading, expressed as text.
/// - `MeterReadingTypeCode` — The type of this meter reading, expressed as a code.
/// - `PreviousMeterReadingDate` — The date of the previous meter reading.
/// - `PreviousMeterQuantity` — The quantity of the previous meter reading.
/// - `LatestMeterReadingDate` — The date of the latest meter reading.
/// - `LatestMeterQuantity` — The quantity of the latest meter reading.
/// - `PreviousMeterReadingMethod` — The method used for the previous meter reading, expressed as text.
/// - `PreviousMeterReadingMethodCode` — The method used for the previous meter reading, expressed as a code.
/// - `LatestMeterReadingMethod` — The method used for the latest meter reading, expressed as text.
/// - `LatestMeterReadingMethodCode` — The method used for the latest meter reading, expressed as a code.
/// - `MeterReadingComments` — Text containing comments on this meter reading.
/// - `DeliveredQuantity` — Consumption in the period from PreviousMeterReadingDate to LatestMeterReadingDate.
// pub struct MeterReading { ... }

/// A class to describe a miscellaneous event associated with a retail event.
///
/// **UBL Dictionary Entry Name:** `Miscellaneous Event. Details`
///
/// Generated from XSD type `MiscellaneousEventType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `MiscellaneousEventTypeCode` — A code signifying the type of this miscellaneous event. Examples are: ASSORTMENT_CHARGE DISASTER FORECAST_DECREASE FORECAST_INCREASE FREIGHT_FLOW_ALLOCATION INVENTORY_POLICY_CHANGE LOCATION_CLOSING LOCATION_OPENING OTHER OUT_OF_STOCK PACKAGING_LABELING_CHANGE PRICE_DECREASE PRICE_INCREASE STORE_FORMAT_OR_PLANOGRAM_CHANGE TEST_MARKET WEATHER
/// - `EventLineItem` — An event line item for this miscellaneous retail event.
// pub struct MiscellaneousEvent { ... }

/// A class to define a monetary total.
///
/// **UBL Dictionary Entry Name:** `Monetary Total. Details`
///
/// Generated from XSD type `MonetaryTotalType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `LineExtensionAmount` — The monetary amount of an extended transaction line, net of tax and settlement discounts, but inclusive of any applicable rounding amount.
/// - `TaxExclusiveAmount` — The monetary amount of an extended transaction line, exclusive of taxes.
/// - `TaxInclusiveAmount` — The monetary amount including taxes; the sum of payable amount and prepaid amount.
/// - `AllowanceTotalAmount` — The total monetary amount of all allowances.
/// - `AllowanceTotalTaxInclusiveAmount` — The total monetary amount of all allowances, inclusive of all taxes.
/// - `ChargeTotalAmount` — The total monetary amount of all charges.
/// - `ChargeTotalTaxInclusiveAmount` — The total monetary amount of all charges, inclusive of all taxes.
/// - `WithholdingTaxTotalAmount` — The total withholding tax amount.
/// - `PrepaidAmount` — The total prepaid monetary amount.
/// - `PayableRoundingAmount` — The rounding amount (positive or negative) added to produce the line extension amount.
/// - `PayableAmount` — The amount of the monetary total to be paid.
/// - `PayableAlternativeAmount` — The amount of the monetary total to be paid, expressed in an alternative currency.
// pub struct MonetaryTotal { ... }

/// A class to define a Notice Subtype
///
/// **UBL Dictionary Entry Name:** `Notice Sub Type. Details`
///
/// Generated from XSD type `NoticeSubTypeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `SubTypeCode` — A code to specify the Subtype of notice
/// - `SubTypeDescription` — Subtype of notice as text
// pub struct NoticeSubType { ... }

/// A class to describe a notification requirement.
///
/// **UBL Dictionary Entry Name:** `Notification Requirement. Details`
///
/// Generated from XSD type `NotificationRequirementType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `NotificationTypeCode` — A code signifying the type of notification (e.g., pickup status).
/// - `PostEventNotificationDurationMeasure` — The length of time between the occurrence of a given event and the issuance of a notification.
/// - `PreEventNotificationDurationMeasure` — The length of time to elapse between the issuance of a notification and the occurrence of the event it relates to.
/// - `NotifyParty` — The Party who is notified.
/// - `NotificationPeriod` — A period during which a notification will be issued.
/// - `NotificationLocation` — A location at which a notification will be issued.
// pub struct NotificationRequirement { ... }

/// A scheduled prepayment (on-account payment) for a estimated utility consumption
///
/// **UBL Dictionary Entry Name:** `On Account Payment. Details`
///
/// Generated from XSD type `OnAccountPaymentType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `EstimatedConsumedQuantity` — The estimated consumed quantity covered by the payment.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `PaymentTerms` — A specification of payment terms associated with this payment.
// pub struct OnAccountPayment { ... }

/// A class to define the type of operation
///
/// **UBL Dictionary Entry Name:** `Operation Type. Details`
///
/// Generated from XSD type `OperationTypeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Code` — A code to specify the type of operation (e.g. deletion)
/// - `Description` — Operation type description as text
// pub struct OperationType { ... }

/// A class to define a line in an order document (e.g., Order, Order Change, or Order Response) describing an item being ordered.
///
/// **UBL Dictionary Entry Name:** `Order Line. Details`
///
/// Generated from XSD type `OrderLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `SubstitutionStatusCode` — A code signifying the substitution status of the item on this order line. The order line may indicate that the substitute is proposed by the buyer (in Order) or by the seller (in Order Response) or that a substitution has been made by the seller (in Order Response).
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `LineItem` — The line item itself.
/// - `SellerProposedSubstituteLineItem` — In Order Response, a line item proposed by the seller describing a product that might substitute for the product described in this order line.
/// - `SellerSubstitutedLineItem` — In Order Response, a line item that has replaced the original order line item. The specified quantity and pricing may differ from those in the original line item, but when a line item is substituted by the seller, it is assumed that other information, such as shipment details, will remain the same.
/// - `BuyerProposedSubstituteLineItem` — A description of an item proposed by the buyer as a possible alternative to the item associated with this order line.
/// - `CatalogueLineReference` — A reference to a catalogue line associated with this order line.
/// - `QuotationLineReference` — A reference to a quotation line associated with this order line.
/// - `OrderLineReference` — A reference to another order line, such as in a replacement order or another line on the same order that is related.
/// - `DocumentReference` — A reference to a document associated with this order line.
// pub struct OrderLine { ... }

/// A class to define a reference to an order line.
///
/// **UBL Dictionary Entry Name:** `Order Line Reference. Details`
///
/// Generated from XSD type `OrderLineReferenceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `LineID` — An identifier for the referenced order line, assigned by the buyer.
/// - `SalesOrderLineID` — An identifier for the referenced order line, assigned by the seller.
/// - `UUID` — A universally unique identifier for this order line reference.
/// - `LineStatusCode` — A code signifying the status of the referenced order line with respect to its original state.
/// - `OrderReference` — A reference to the Order containing the referenced order line.
// pub struct OrderLineReference { ... }

/// A class to define a reference to an Order.
///
/// **UBL Dictionary Entry Name:** `Order Reference. Details`
///
/// Generated from XSD type `OrderReferenceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this order reference, assigned by the buyer.
/// - `SalesOrderID` — An identifier for this order reference, assigned by the seller.
/// - `CopyIndicator` — (Deprecated) Indicates whether the referenced Order is a copy (true) or the original (false).
/// - `UUID` — A universally unique identifier for this order reference.
/// - `IssueDate` — The date on which the referenced Order was issued.
/// - `IssueTime` — The time at which the referenced Order was issued.
/// - `CustomerReference` — Text used for tagging purchasing card transactions.
/// - `OrderTypeCode` — A code signifying the type of the referenced Order.
/// - `DocumentReference` — A document associated with this reference to an Order.
// pub struct OrderReference { ... }

/// A class to describe an ordered shipment.
///
/// **UBL Dictionary Entry Name:** `Ordered Shipment. Details`
///
/// Generated from XSD type `OrderedShipmentType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Shipment` — The ordered shipment.
/// - `Package` — A package in this ordered shipment.
// pub struct OrderedShipment { ... }

/// A class to describe a package.
///
/// **UBL Dictionary Entry Name:** `Package. Details`
///
/// Generated from XSD type `PackageType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this package.
/// - `Quantity` — The quantity of items contained in this package.
/// - `ReturnableMaterialIndicator` — An indicator that the packaging material is returnable (true) or not (false).
/// - `PackageLevelCode` — A code signifying a level of packaging.
/// - `PackagingTypeCode` — A code signifying a type of packaging.
/// - `PackagingType` — The type of packaging, described as a text.
/// - `PackingMaterial` — Text describing the packaging material.
/// - `TraceID` — An identifier for use in tracing this package, such as the EPC number used in RFID.
/// - `ContainedPackage` — A package contained within this package.
/// - `ContainingTransportEquipment` — The piece of transport equipment containing this package.
/// - `GoodsItem` — A goods item included in this package.
/// - `MeasurementDimension` — A measurable dimension (length, mass, weight, or volume) of this package.
/// - `DeliveryUnit` — A delivery unit within this package.
/// - `Delivery` — The delivery of this package.
/// - `Pickup` — The pickup of this package.
/// - `Despatch` — The despatch of this package.
/// - `Status` — The status of this transport handling unit.
// pub struct Package { ... }

/// A class to describe a participant party.
///
/// **UBL Dictionary Entry Name:** `Participant Party. Details`
///
/// Generated from XSD type `ParticipantPartyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `InitiatingPartyIndicator` — An indicator that this party is playing the role of the initiator within a transaction (true) or not (false).
/// - `PrivatePartyIndicator` — An indicator that this party is a private entity (true) or not (false).
/// - `PublicPartyIndicator` — An indicator that this party is a public (governmental) entity (true) or not (false).
/// - `ServiceProviderPartyIndicator` — An indicator that this party is a service provider (true) or not (false).
/// - `Party` — The Party who participates.
/// - `LegalContact` — A legal contact associated to this participant for sending legal notices.
/// - `TechnicalContact` — A technical contact associated to this participant.
/// - `SupportContact` — A support contact associated to this participant.
/// - `CommercialContact` — A commercial contact associated to this participant.
// pub struct ParticipantParty { ... }

/// A class to describe an organization, sub-organization, or individual fulfilling a role in a business process.
///
/// **UBL Dictionary Entry Name:** `Party. Details`
///
/// Generated from XSD type `PartyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `MarkCareIndicator` — (Deprecated) An indicator that this party is "care of" (c/o) (true) or not (false).
/// - `MarkAttentionIndicator` — (Deprecated) An indicator that this party is "for the attention of" (FAO) (true) or not (false).
/// - `WebsiteURI` — The Uniform Resource Identifier (URI) that identifies this party's web site; i.e., the web site's Uniform Resource Locator (URL).
/// - `LogoReferenceID` — An identifier for this party's logo.
/// - `EndpointID` — (Deprecated) An identifier for the end point of the routing service (e.g., EAN Location Number, GLN).
/// - `IndustryClassificationCode` — This party's Industry Classification Code.
/// - `PartyIdentification` — (Endorsed cardinality: 0..1) A preferred identifier for this Party.
/// - `AdditionalPartyIdentification` — One or more additional identifiers for this Party.
/// - `PartyName` — (Endorsed cardinality: 0..1) A name for this party.
/// - `TradePartyName` — A trade name for this Party.
/// - `Language` — The language associated with this party.
/// - `PostalAddress` — The party's postal address.
/// - `PhysicalLocation` — The physical location of this party.
/// - `PartyTaxScheme` — A tax scheme applying to this party.
/// - `PartyLegalEntity` — A description of this party as a legal entity.
/// - `Contact` — The primary contact for this party.
/// - `Person` — A person associated with this party.
/// - `AgentParty` — The Party who acts as an Agent for this Party.
/// - `ServiceProviderParty` — A party providing a service to this party.
/// - `PowerOfAttorney` — A power of attorney associated with this party.
/// - `PartyAuthorization` — An authorization issued to this party
/// - `FinancialAccount` — The financial account associated with this party.
/// - `AdditionalWebSite` — An additional web site associated with this party (e.g. a satellite web site).
/// - `SocialMediaProfile` — A social media profile associated with this party.
/// - `ElectronicAddress` — An Electronic Address where this Party is registered.
// pub struct Party { ... }

/// A class to define a Group of Parties
///
/// **UBL Dictionary Entry Name:** `Party Group. Details`
///
/// Generated from XSD type `PartyGroupType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `GroupTypeCode` — A code to specify the type of grouping (e.g. EEIG).
/// - `GroupType` — Type of grouping as text.
/// - `Party` — A member of this Group of Parties.
// pub struct PartyGroup { ... }

/// A class to define an identifier for a party.
///
/// **UBL Dictionary Entry Name:** `Party Identification. Details`
///
/// Generated from XSD type `PartyIdentificationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for the party.
// pub struct PartyIdentification { ... }

/// A class to describe a party as a legal entity.
///
/// **UBL Dictionary Entry Name:** `Party Legal Entity. Details`
///
/// Generated from XSD type `PartyLegalEntityType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `RegistrationName` — The name of the party as registered with the relevant legal authority.
/// - `CompanyID` — An identifier for the party as registered within a company registration scheme.
/// - `RegistrationDate` — The registration date of the CompanyID.
/// - `RegistrationExpirationDate` — The date upon which a registration expires (e.g., registration for an import/export license).
/// - `CompanyLegalFormCode` — A code signifying the party's legal status.
/// - `CompanyLegalForm` — The company legal status, expressed as a text.
/// - `SoleProprietorshipIndicator` — An indicator that the company is owned and controlled by one person (true) or not (false).
/// - `EntitySizeCode` — A code signifying the size category of the legal entity (e.g., micro, small, medium, large).
/// - `CompanyLiquidationStatusCode` — A code signifying the party's liquidation status.
/// - `CorporateStockAmount` — The number of shares in the capital stock of a corporation.
/// - `FullyPaidSharesIndicator` — An indicator that all shares of corporate stock have been paid by shareholders (true) or not (false).
/// - `RegistrationAddress` — The registered address of the party within a corporate registration scheme.
/// - `CorporateRegistrationScheme` — The corporate registration scheme used to register the party.
/// - `HeadOfficeParty` — The head office of this Legal Entity.
/// - `ShareholderParty` — A Party that owns shares or equity in this Legal Entity.
/// - `SecurityListing` — One or more securities issued by this Party Legal Entity that are listed on regulated markets.
// pub struct PartyLegalEntity { ... }

/// A class for defining the name of a party.
///
/// **UBL Dictionary Entry Name:** `Party Name. Details`
///
/// Generated from XSD type `PartyNameType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Name` — The name of the party.
// pub struct PartyName { ... }

/// A class to describe a taxation scheme applying to a party.
///
/// **UBL Dictionary Entry Name:** `Party Tax Scheme. Details`
///
/// Generated from XSD type `PartyTaxSchemeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `RegistrationName` — The name of the party as registered with the relevant fiscal authority.
/// - `CompanyID` — An identifier for the party assigned for tax purposes by the taxation authority.
/// - `TaxLevelCode` — A code signifying the tax level applicable to the party within this taxation scheme.
/// - `ExemptionReasonCode` — A reason for the party's exemption from tax, expressed as a code.
/// - `ExemptionReason` — A reason for the party's exemption from tax, expressed as text.
/// - `RegistrationAddress` — The address of the party as registered for tax purposes.
/// - `TaxScheme` — The taxation scheme applicable to the party.
// pub struct PartyTaxScheme { ... }

/// A class to describe a payment.
///
/// **UBL Dictionary Entry Name:** `Payment. Details`
///
/// Generated from XSD type `PaymentType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this payment.
/// - `PaidAmount` — The amount of this payment.
/// - `PaidCashAmount` — The amount given by the customer in cash or cash equivalents, if different from the payable amount. The Paid Amount = Paid Cash Amount - Cash Change Amount.
/// - `CashChangeAmount` — The change returned to the customer when the paid cash amount is more than the payable amount.
/// - `ReceivedDate` — The date on which this payment was received.
/// - `PaidDate` — The date on which this payment was made.
/// - `PaidTime` — The time at which this payment was made.
/// - `InstructionID` — An identifier for the payment instruction.
/// - `MerchantID` — An identifier for the merchant who handled the payment.
/// - `AuthorizationID` — The authorization identifier for this payment.
/// - `TransactionID` — The transaction identifier for this payment.
/// - `PaymentTerminalID` — An identifier for the payment terminal used for this payment.
/// - `StatusCode` — A code signifying the status of the Payment (e.g., planned, in process, executed).
/// - `ExchangeRate` — The exchange rate applicable to this payment, if the payment currency differs from the document currency.
/// - `BillingReference` — A reference to a billing document to which this Payment relates.
/// - `RemittanceDocumentReference` — A reference to a Remittance Advice document associated with this Payment.
// pub struct Payment { ... }

/// A class to describe a payment mandate.
///
/// **UBL Dictionary Entry Name:** `Payment Mandate. Details`
///
/// Generated from XSD type `PaymentMandateType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this payment mandate.
/// - `MandateTypeCode` — A code signifying the type of this payment mandate.
/// - `MaximumPaymentInstructionsNumeric` — The number of maximum payment instructions allowed within the validity period.
/// - `MaximumPaidAmount` — The maximum amount to be paid within a single instruction.
/// - `SignatureID` — An identifier for a signature applied by a signatory party.
/// - `PayerParty` — The Party, if different from the debtor, that makes the Payment.
/// - `PayerFinancialAccount` — The payer's financial account.
/// - `ValidityPeriod` — The period during which this mandate is valid.
/// - `PaymentReversalPeriod` — The period of the reverse payment.
/// - `Clause` — A clause applicable to this payment mandate.
// pub struct PaymentMandate { ... }

/// A class to describe a means of payment.
///
/// **UBL Dictionary Entry Name:** `Payment Means. Details`
///
/// Generated from XSD type `PaymentMeansType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this means of payment.
/// - `PaymentMeansCode` — A code signifying the type of this means of payment.
/// - `PaymentMeansDescription` — A description of this means of payment.
/// - `PaymentDueDate` — The date on which payment is due for this means of payment.
/// - `PaymentChannelCode` — A code signifying the Payment Channel for this Payment Means.
/// - `PaymentRailID` — An identifier of the Payment Rail or network through which the Payment is executed.
/// - `PaymentPlatformID` — An identifier of the Payment Platform on which the Payment is executed or received.
/// - `InstructionID` — An identifier for the payment instruction.
/// - `InstructionNote` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `PaymentID` — An identifier for a payment made using this means of payment.
/// - `ChargeBearerCode` — A code signifying which party or parties will assume the charges and fees associated with the payment using this payment means.
/// - `ServiceLevelCode` — A code signifying an agreed service level for the type of payment associated with this payment means.
/// - `CardAccount` — A credit card, debit card, or charge card account that constitutes this means of payment.
/// - `PayerFinancialAccount` — The payer's financial account.
/// - `PayeeFinancialAccount` — The payee's financial account.
/// - `CreditAccount` — A credit account associated with this means of payment.
/// - `PaymentMandate` — The payment mandate associated with this means of payment.
/// - `TradeFinancing` — A trade finance agreement applicable to this means of payment.
/// - `RemittanceDocumentDistribution` — A person or entity who will receive the remittance advice information about the payment associated with this payment means.
/// - `PaymentInstructionAttachment` — Structured payment instruction information including such intended for rendering as a scannable symbol (e.g., QR-code) or for automated processing by external systems.
// pub struct PaymentMeans { ... }

/// A class to describe a set of payment terms.
///
/// **UBL Dictionary Entry Name:** `Payment Terms. Details`
///
/// Generated from XSD type `PaymentTermsType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this set of payment terms.
/// - `PaymentMeansID` — An identifier for a means of payment associated with these payment terms.
/// - `PrepaidPaymentReferenceID` — An identifier for a reference to a prepaid payment.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `ReferenceEventCode` — A code signifying the event during which these terms are offered.
/// - `SettlementDiscountPercent` — The percentage for the settlement discount that is offered for payment under these payment terms.
/// - `PenaltySurchargePercent` — The penalty for payment after the settlement period, expressed as a percentage of the payment.
/// - `PaymentPercent` — The part of a payment, expressed as a percent, relevant for these payment terms.
/// - `Amount` — The monetary amount covered by these payment terms.
/// - `SettlementDiscountAmount` — The amount of a settlement discount offered for payment under these payment terms.
/// - `PenaltyAmount` — The monetary amount of the penalty for payment after the settlement period.
/// - `PaymentTermsDetailsURI` — The Uniform Resource Identifier (URI) of a document providing additional details regarding these payment terms.
/// - `PaymentDueDate` — The due date for these payment terms.
/// - `InstallmentDueDate` — The due date for an installment payment for these payment terms.
/// - `InvoicingPartyReference` — A reference to the payment terms used by the invoicing party. This may have been requested of the payer by the payee to accompany its remittance.
/// - `SettlementPeriod` — The period during which settlement may occur.
/// - `PenaltyPeriod` — The period during which penalties may apply.
/// - `PenaltyInterestRate` — An interest rate to be applied in case of late payment.
/// - `ExchangeRate` — The currency exchange rate for purposes of these payment terms.
/// - `ValidityPeriod` — The period during which these payment terms are valid.
// pub struct PaymentTerms { ... }

/// A class to define a line in a Performance History.
///
/// **UBL Dictionary Entry Name:** `Performance Data Line. Details`
///
/// Generated from XSD type `PerformanceDataLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this performance data line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `PerformanceValueQuantity` — The value of the reported attribute.
/// - `PerformanceMetricTypeCode` — A code signifying the measure of performance applicable to the reported attribute.
/// - `Period` — The period to which this performance data line applies.
/// - `Item` — The item whose performance is reported in this data line.
// pub struct PerformanceDataLine { ... }

/// A class to describe a period of time.
///
/// **UBL Dictionary Entry Name:** `Period. Details`
///
/// Generated from XSD type `PeriodType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `StartDate` — The date on which this period begins.
/// - `StartTime` — The time at which this period begins.
/// - `EndDate` — The date on which this period ends.
/// - `EndTime` — The time at which this period ends.
/// - `DurationMeasure` — The duration of this period, expressed as an ISO 8601 code.
/// - `DescriptionCode` — (Endorsed cardinality: 0..1) A description of this period, expressed as a code.
/// - `Description` — A description of this period, expressed as text.
// pub struct Period { ... }

/// A class to describe a person.
///
/// **UBL Dictionary Entry Name:** `Person. Details`
///
/// Generated from XSD type `PersonType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this person.
/// - `FirstName` — This person's given name.
/// - `FamilyName` — This person's family name.
/// - `Title` — This person's title of address (e.g., Mr, Ms, Dr, Sir).
/// - `MiddleName` — This person's middle name(s) or initials.
/// - `OtherName` — This person's second family name.
/// - `NameSuffix` — A suffix to this person's name (e.g., PhD, OBE, Jr).
/// - `JobTitle` — This person's job title (for a particular role) within an organization.
/// - `NationalityID` — (Deprecated) An identifier for this person's nationality.
/// - `NationalID` — An identifier issued by a national authority that uniquely identifies the person within that country, such as a social security number or national registration number.
/// - `NationalityCode` — A code signifying the person’s nationality as defined by the applicable legal or administrative framework.
/// - `GenderCode` — A code (e.g., ISO 5218, ICAO Doc 9303, etc.) signifying the gender of this person.
/// - `BirthDate` — This person's date of birth.
/// - `BirthplaceName` — The name of the place where this person was born, expressed as text.
/// - `OrganizationDepartment` — The department or subdivision of an organization that this person belongs to (in a particular role).
/// - `RoleCode` — A code stating the person's role
/// - `BirthplaceLocation` — The location where this person was born.
/// - `CitizenshipCountry` — The country of the person's citizenship.
/// - `Contact` — Contact information for this person.
/// - `FinancialAccount` — The financial account associated with this person.
/// - `IdentityDocumentReference` — A reference to a document that can precisely identify this person (e.g., a driver's license).
/// - `ResidenceAddress` — This person's address of residence.
// pub struct Person { ... }

/// A class to describe a health incident involving crew or other personnel.
///
/// **UBL Dictionary Entry Name:** `Personnel Health Incident. Details`
///
/// Generated from XSD type `PersonnelHealthIncidentType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this personal health incident.
/// - `JoinedShipDate` — The date when the person joined the ship.
/// - `NatureOfIllnessDescription` — A text decribing the nature of the illness.
/// - `OnsetDate` — The first date of the health incident.
/// - `ReportedToMedicalOfficerIndicator` — An indicator of whether this personal health incident has been reported to a medical officer (true) or not (false).
/// - `GivenTreatmentDescription` — A text describing the given treatment.
/// - `StillIllIndicator` — Indicates whether the person is still ill (true) or not (false).
/// - `DiedIndicator` — Indicates whether the person died from this health incident (true) or not (false).
/// - `StillOnBoardIndicator` — Indicates whether the person is still on board (true) or not (false).
/// - `EvacuatedIndicator` — Indicates whether the person has been evacuated (true) or not (false).
/// - `BuriedAtSeaIndicator` — Indicates whether the person has been buired at sea (true) or not (false).
/// - `Note` — Any additional information that is not included elsewhere, expressed as text.
/// - `Person` — The person associated to this health incident.
// pub struct PersonnelHealthIncident { ... }

/// A class to describe a physical attribute.
///
/// **UBL Dictionary Entry Name:** `Physical Attribute. Details`
///
/// Generated from XSD type `PhysicalAttributeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `AttributeID` — An identifier for this physical attribute.
/// - `PositionCode` — A code signifying the position of this physical attribute.
/// - `DescriptionCode` — A description of the physical attribute, expressed as a code.
/// - `Description` — A description of the physical attribute, expressed as text.
// pub struct PhysicalAttribute { ... }

/// A class to describe a pickup for delivery.
///
/// **UBL Dictionary Entry Name:** `Pickup. Details`
///
/// Generated from XSD type `PickupType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this pickup.
/// - `ActualPickupDate` — The actual pickup date.
/// - `ActualPickupTime` — The actual pickup time.
/// - `EarliestPickupDate` — The earliest pickup date.
/// - `EarliestPickupTime` — The earliest pickup time.
/// - `LatestPickupDate` — The latest pickup date.
/// - `LatestPickupTime` — The latest pickup time.
/// - `PickupLocation` — The pickup location.
/// - `PickupParty` — The Party who picks up the Delivery.
// pub struct Pickup { ... }

/// A class to describe a call to a port.
///
/// **UBL Dictionary Entry Name:** `Port Call. Details`
///
/// Generated from XSD type `PortCallType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this Port Call.
/// - `PlannedOperationsDescription` — Description of the planned operations in this Port Call.
/// - `PlannedWorksDescription` — Description of the planned works in this Port Call.
/// - `PlannedInspectionsDescription` — Description of the planned inspections in this Port Call.
/// - `ExpectedAnchorageIndicator` — An indicator of whether the ship is expected to stay at an anchorage upon arrival at the port of call (true) or not (false).
/// - `PositionInPortID` — An identifier for the position in the port for this Port Call
/// - `CargoAndBallastTankConditionDescription` — Description about the condition of the cargo and ballast tank.
/// - `ShipRequirement` — Ship requirements for this port call.
/// - `PrimaryPortCallPurpose` — The primary purpose of this port call.
/// - `AdditionalPortCallPurpose` — Any additional or secondary purposes of this port call.
/// - `RequestedArrivalEvent` — The requested arrival event.
// pub struct PortCall { ... }

/// A class to describe the purpose of a port call.
///
/// **UBL Dictionary Entry Name:** `Port Call Purpose. Details`
///
/// Generated from XSD type `PortCallPurposeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `PurposeTypeCode` — The purpose of this port call, expressed as a code.
/// - `PurposeType` — The purpose of this port call, expressed as a text.
/// - `Description` — A description of the purpose of the port call.
// pub struct PortCallPurpose { ... }

/// A record for a ship call at a port facility.
///
/// **UBL Dictionary Entry Name:** `Port Call Record. Details`
///
/// Generated from XSD type `PortCallRecordType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this port call record.
/// - `SecurityLevelCode` — A code describing the security level of the port facility call record.
/// - `SecurityMeasure` — One or more security measures applied to this port call record.
/// - `PortFacilityLocation` — The location of the port facility.
/// - `Period` — The period when this port call took place.
// pub struct PortCallRecord { ... }

/// A class to describe a post award process. These processes following the agreement on a contract for supply of goods or services ( for example, after the awarding of a tender).
///
/// **UBL Dictionary Entry Name:** `Post Award Process. Details`
///
/// Generated from XSD type `PostAwardProcessType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ElectronicCatalogueUsageIndicator` — An indicator to specify whether an electronic catalogue will be used during the post award phase.
/// - `ElectronicInvoiceAcceptedIndicator` — An indicator on whether the electronic invoice is allowed for this process.
/// - `ElectronicOrderUsageIndicator` — An indicator on whether electronic ordering will be used in the post award process.
/// - `ElectronicPaymentUsageIndicator` — (Endorsed cardinality: 0..1) An indicator on whether electronic payment will be used in the post award process.
// pub struct PostAwardProcess { ... }

/// A class to describe a power of attorney.
///
/// **UBL Dictionary Entry Name:** `Power Of Attorney. Details`
///
/// Generated from XSD type `PowerOfAttorneyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this power of attorney.
/// - `IssueDate` — The date on which this power of attorney was issued.
/// - `IssueTime` — The time at which this power of attorney was issued.
/// - `Description` — Text describing this power of attorney.
/// - `NotaryParty` — The Party who notarises this Power of Attorney.
/// - `AgentParty` — The Party acting as an agent or fiduciary for the principal and holding this Power of Attorney on behalf of the principal.
/// - `WitnessParty` — A Witness to this Power of Attorney.
/// - `MandateDocumentReference` — A reference to a mandate associated with this power of attorney.
// pub struct PowerOfAttorney { ... }

/// A class to describe a price, expressed in a data structure containing multiple properties (compare with UnstructuredPrice).
///
/// **UBL Dictionary Entry Name:** `Price. Details`
///
/// Generated from XSD type `PriceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `PriceAmount` — The amount of the price.
/// - `TaxInclusivePriceAmount` — The amount of the price inclusive of all taxes.
/// - `BaseQuantity` — The quantity at which this price applies.
/// - `PriceChangeReason` — A reason for a price change.
/// - `PriceTypeCode` — The type of price, expressed as a code.
/// - `PriceType` — The type of price, expressed as text.
/// - `OrderableUnitFactorRate` — The factor by which the base price unit can be converted to the orderable unit.
/// - `ValidityPeriod` — A period during which this price is valid.
/// - `PriceList` — Information about a price list applicable to this price.
/// - `AllowanceCharge` — An allowance or charge associated with this price.
/// - `PricingExchangeRate` — The exchange rate applicable to this price, if it differs from the exchange rate applicable to the document as a whole.
/// - `AlternativeCurrencyPrice` — The price expressed in an alternative currency
// pub struct Price { ... }

/// A class to describe a price extension, calculated by multiplying the price per unit by the quantity of items.
///
/// **UBL Dictionary Entry Name:** `Price Extension. Details`
///
/// Generated from XSD type `PriceExtensionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Amount` — The amount of this price extension.
/// - `TaxInclusiveAmount` — The amount of this price extension inclusive of all taxes.
/// - `TaxTotal` — A total amount of taxes of a particular kind applicable to this price extension.
// pub struct PriceExtension { ... }

/// A class to describe a price list.
///
/// **UBL Dictionary Entry Name:** `Price List. Details`
///
/// Generated from XSD type `PriceListType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this price list.
/// - `StatusCode` — A code signifying whether this price list is an original, copy, revision, or cancellation.
/// - `ValidityPeriod` — A period during which this price list is valid.
/// - `PreviousPriceList` — The previous price list.
// pub struct PriceList { ... }

/// A reference to the basis for pricing. This may be based on a catalogue or a quoted amount from a price list and include some alternative pricing conditions.
///
/// **UBL Dictionary Entry Name:** `Pricing Reference. Details`
///
/// Generated from XSD type `PricingReferenceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `OriginalItemLocationQuantity` — An original set of location-specific properties (e.g., price and quantity) associated with this item.
/// - `AlternativeConditionPrice` — The price expressed in terms other than the actual price, e.g., the list price v. the contracted price, or the price in bags v. the price in kilos, or the list price in bags v. the contracted price in kilos.
// pub struct PricingReference { ... }

/// A class to describe something valuable offered or striven for in competition.
///
/// **UBL Dictionary Entry Name:** `Prize. Details`
///
/// Generated from XSD type `PrizeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `RankCode` — The relative position in the competition associated with a prize.
/// - `ValueAmount` — The monetary value amount of a prize.
/// - `Description` — Text providing more information about this prize.
// pub struct Prize { ... }

/// A class to describe a justification for the choice of tendering process.
///
/// **UBL Dictionary Entry Name:** `Process Justification. Details`
///
/// Generated from XSD type `ProcessJustificationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `PreviousCancellationReasonCode` — A code signifying the type of the previous tendering process (which is now being cancelled).
/// - `ProcessReasonCode` — The reason why the contracting authority has followed a particular tendering procedure for the awarding of a contract, expressed as a code.
/// - `ProcessReason` — The reason why the contracting authority has followed a particular tendering procedure for the awarding of a contract, expressed as text.
/// - `Description` — Text providing justification for the selection of this process.
// pub struct ProcessJustification { ... }

/// A class to describe additional types for a procurement project
///
/// **UBL Dictionary Entry Name:** `Procurement Additional Type. Details`
///
/// Generated from XSD type `ProcurementAdditionalTypeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ProcurementTypeCode` — A code signifying the type of procurement project (e.g., goods, works, services).
/// - `ProcurementType` — Tthe type of procurement project (e.g., goods, works, services), expressed as text.
// pub struct ProcurementAdditionalType { ... }

/// A class to describe a project to procure goods, works, or services.
///
/// **UBL Dictionary Entry Name:** `Procurement Project. Details`
///
/// Generated from XSD type `ProcurementProjectType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this procurement project.
/// - `Name` — A name of this procurement project.
/// - `Description` — Text describing this procurement project.
/// - `ProcurementTypeCode` — A code signifying the type of procurement project (e.g., goods, works, services).
/// - `ProcurementSubTypeCode` — A code signifying the subcategory of the type of work for this project (e.g., land surveying, IT consulting).
/// - `QualityControlCode` — The indication of whether or not the control quality is included in the works project.
/// - `RequiredFeeAmount` — The amount of the reimbursement fee for concession procurement projects.
/// - `FeeDescription` — Text describing the reimbursement fee for concession procurement projects.
/// - `RequestedDeliveryDate` — The requested delivery date for this procurement project.
/// - `EstimatedOverallContractQuantity` — The estimated overall quantity for this procurement project.
/// - `Note` — Free-form text applying to the Procurement Project. This element may contain additional information about the lot/contract that is not contained explicitly in another structure.
/// - `SMESuitableIndicator` — Suitable for Small- and Medium-sized Enterprises. This element specifies that the buyer accepts the risks associated of contracting with SMEs.
/// - `ProcurementAdditionalType` — An association to additional procurement type.
/// - `RequestedTenderTotal` — Budget monetary amounts for the project as whole.
/// - `MainCommodityClassification` — An association to the main classification category for the deliverable requested.
/// - `AdditionalCommodityClassification` — An association to additional classification categories for the deliverable requested.
/// - `RealizedLocation` — A place where this procurement project will be physically realized.
/// - `PlannedPeriod` — The period during which this procurement project is planned to take place.
/// - `ContractExtension` — The contract extension for this tendering process.
/// - `RequestForTenderLine` — A good or service this project is intended to procure.
// pub struct ProcurementProject { ... }

/// A class to describe one of the parts of a procurement project that is being subdivided to allow the contracting party to award different lots to different economic operators under different contracts.
///
/// **UBL Dictionary Entry Name:** `Procurement Project Lot. Details`
///
/// Generated from XSD type `ProcurementProjectLotType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this procurement project lot.
/// - `LegalDocumentReference` — A reference to a legal document.
/// - `TechnicalDocumentReference` — A reference to a technical document.
/// - `RequiredDocumentReference` — A reference to a required document.
/// - `ProvidedDocumentReference` — A reference to a provided document.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `TenderingTerms` — Tendering terms for this procurement project lot.
/// - `TenderingProcess` — Tendering process for this procurement project lot.
/// - `ProcurementProject` — A description of the procurement project to be divided.
// pub struct ProcurementProjectLot { ... }

/// A class to reference to a lot identifier.
///
/// **UBL Dictionary Entry Name:** `Procurement Project Lot Reference. Details`
///
/// Generated from XSD type `ProcurementProjectLotReferenceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this procurement project lot.
// pub struct ProcurementProjectLotReference { ... }

/// A class to define a reference to a procurement project.
///
/// **UBL Dictionary Entry Name:** `Project Reference. Details`
///
/// Generated from XSD type `ProjectReferenceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for the referenced project.
/// - `UUID` — A universally unique identifier for the referenced project.
/// - `IssueDate` — The date on which the referenced project was issued.
/// - `WorkPhaseReference` — A specific phase of work in the referenced project.
// pub struct ProjectReference { ... }

/// Agree can be renamed as PromotionalEvents
///
/// **UBL Dictionary Entry Name:** `Promotional Event. Details`
///
/// Generated from XSD type `PromotionalEventType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `PromotionalEventTypeCode` — A code signifying the type of this promotional event. Examples can be: Holiday, Seasonal Event, Store Closing, Trade Item Introduction
/// - `SubmissionDate` — The date on which a proposal for this promotional event was submitted.
/// - `FirstShipmentAvailibilityDate` — (Deprecated) The first day that products will be available to ship from buyer to seller if the proposal for this promotional event is accepted.
/// - `FirstShipmentAvailabilityDate` — The first day that products will be available to ship from buyer to seller if the proposal for this promotional event is accepted.
/// - `LatestProposalAcceptanceDate` — The deadline for acceptance of this promotional event.
/// - `PromotionalSpecification` — A specification for a promotional event.
// pub struct PromotionalEvent { ... }

/// A class to describe a line item associated with a promotional event.
///
/// **UBL Dictionary Entry Name:** `Promotional Event Line Item. Details`
///
/// Generated from XSD type `PromotionalEventLineItemType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Amount` — The amount associated with this promotional event line item.
/// - `EventLineItem` — A line item describing the expected impacts associated with this promotional event for a specific product at a specific location.
// pub struct PromotionalEventLineItem { ... }

/// A class to describe a promotional event as a set of item locations that share a set of promotional tactics.
///
/// **UBL Dictionary Entry Name:** `Promotional Specification. Details`
///
/// Generated from XSD type `PromotionalSpecificationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `SpecificationID` — An identifier for this promotional specification.
/// - `PromotionalEventLineItem` — A line item for a promotional event involving a specific product at a specific location; it describes the expected impacts associated with the event and specifies the promotional price of the item."
/// - `EventTactic` — An event tactic associated with this promotion.
// pub struct PromotionalSpecification { ... }

/// A class for assigning identifying information for a property
///
/// **UBL Dictionary Entry Name:** `Property Identification. Details`
///
/// Generated from XSD type `PropertyIdentificationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An Identifier for the property.
/// - `IssuerScopeID` — A scope within which the issuer has assigned this identifier.
/// - `IssuerParty` — The party that issued this property identifier.
// pub struct PropertyIdentification { ... }

/// A class to define a line item in a purchase receipt.
///
/// **UBL Dictionary Entry Name:** `Purchase Receipt Line. Details`
///
/// Generated from XSD type `PurchaseReceiptLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this purchase receipt line.
/// - `UUID` — A universally unique identifier for this invoice line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `Quantity` — The quantity (of items) on this purchase receipt line.
/// - `LineExtensionAmount` — The total amount for this purchase receipt line, including allowances and charges but net of taxes.
/// - `TaxInclusiveLineExtensionAmount` — The total amount for this purchase receipt line, including all allowances, charges and taxes.
/// - `PurchaseLinePeriod` — A period to which this purchase line applies.
/// - `PurchaseReference` — A reference to an object, such as a subscription number, telephone number, meter, vehicle, person, etc., to which this purchase line relates.
/// - `AllowanceCharge` — An allowance or charge associated with this purchase line.
/// - `TaxTotal` — A total amount of taxes of a particular kind applicable to this invoice line.
/// - `Item` — The item associated with this invoice line.
/// - `Price` — The price of the item associated with this purchase line.
// pub struct PurchaseReceiptLine { ... }

/// A class for referencing an object to which a purchase relates, such as a subscription number, telephone number, meter, vehicle, person, etc.
///
/// **UBL Dictionary Entry Name:** `Purchase Reference. Details`
///
/// Generated from XSD type `PurchaseReferenceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An Identifier for this purchase reference.
/// - `Description` — A description of this purchase reference.
// pub struct PurchaseReference { ... }

/// A class to describe the acceptance or rejection of an economic operator in a tendering process.
///
/// **UBL Dictionary Entry Name:** `Qualification Resolution. Details`
///
/// Generated from XSD type `QualificationResolutionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `AdmissionCode` — An indicator that the economic operator has been accepted into the tendering process (true) or rejected from the tendering process (false).
/// - `ExclusionReason` — Text describing a reason for an exclusion from the tendering process.
/// - `Resolution` — Text describing this qualification resolution.
/// - `ResolutionDate` — The date on which this qualification resolution was formalized.
/// - `ResolutionTime` — The time at which this qualification resolution was formalized.
/// - `ProcurementProjectLot` — The Procurement project lot to which this tenderer is accepted or rejected.
// pub struct QualificationResolution { ... }

/// A class to describe the distinctive features or characteristics qualifying an economic operator to be a party in a tendering process (e.g., number of employees, number of operating units, type of business, technical and financial capabilities, completed projects).
///
/// **UBL Dictionary Entry Name:** `Qualifying Party. Details`
///
/// Generated from XSD type `QualifyingPartyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ParticipationPercent` — The extent to which this party is expected to participate in the tendering process, expressed as a percentage.
/// - `PersonalSituation` — Text describing the personal situation of the qualifying party.
/// - `OperatingYearsQuantity` — The number of years that this qualifying party has been in operation.
/// - `EmployeeQuantity` — The number of people employed by this qualifying party.
/// - `BusinessClassificationEvidenceID` — An identifier for an item of evidence to support the classification of this qualifying party.
/// - `BusinessIdentityEvidenceID` — An identifier for an item of evidence to support the business identity of this qualifying party.
/// - `TendererRoleCode` — A code stating the Tenderer Role.
/// - `BusinessClassificationScheme` — The classification scheme used for the business profile.
/// - `TechnicalCapability` — A technical capability of this qualifying party.
/// - `FinancialCapability` — A financial capability of this qualifying party.
/// - `CompletedTask` — A former task completed by this qualifying party.
/// - `Declaration` — A declaration by this qualifying party. of certain characteristics or capabilities in fulfilment of requirements specified in a call for tenders.
/// - `Party` — The Party who qualifies to participate in the Tender.
/// - `EconomicOperatorRole` — A class to describe the tenderer contracting role.
// pub struct QualifyingParty { ... }

/// A class to define a line in a Quotation.
///
/// **UBL Dictionary Entry Name:** `Quotation Line. Details`
///
/// Generated from XSD type `QuotationLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this quotation line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `Quantity` — The quantity of the item quoted.
/// - `LineExtensionAmount` — The total amount for this quotation line, including allowance charges but net of taxes.
/// - `TaxInclusiveLineExtensionAmount` — The total amount for this quotation line, including all allowances, charges and taxes.
/// - `TotalTaxAmount` — The total tax amount for this quotation line.
/// - `RequestForQuotationLineID` — An identifier for the line in the Request for Quotation to which this line is a response.
/// - `DocumentReference` — A reference to a document associated with this quotation line.
/// - `LineItem` — The item that is the subject of this quotation line.
/// - `SellerProposedSubstituteLineItem` — An item proposed by the seller as a substitute for the item that is the subject of this quotation line.
/// - `AlternativeLineItem` — An item proposed by the seller as an alternative to the item that is the subject of this quotation line.
/// - `RequestLineReference` — A reference to the line in the Request for Quotation to which this line is a response.
// pub struct QuotationLine { ... }

/// A class defining the maximum activity of the radioactive contents capable of sustaining a nuclear fission chain reaction during carriage.
///
/// **UBL Dictionary Entry Name:** `Radioactive Isotope. Details`
///
/// Generated from XSD type `RadioactiveIsotopeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Name` — The name of this Iradioactive Isotope.
/// - `ActivityLevelMeasure` — The measure of the radioactive contents during carriage.
// pub struct RadioactiveIsotope { ... }

/// A class defining a radioactive material.
///
/// **UBL Dictionary Entry Name:** `Radioactive Material. Details`
///
/// Generated from XSD type `RadioactiveMaterialType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Name` — The name of this Iradioactive Material.
/// - `SpecialFormDescription` — The description of the physical and chemical form of the material, including any notation that the material is a special form radioactive material or a low dispersible radioactive material.
/// - `TransportIndexNumeric` — A number specifying the Transport Index for this Radioactive Material.
/// - `FissileCriticalitySafetyIndexNumeric` — The number assigned to and placed on the label of a fissile radioactive material package to designate the degree of control of accumulation of packages, overpacks or freight containers containing fissile material during transportation.
/// - `ApplicableRadioactiveIsotope` — The maximum activity of the radioactive contents capable of sustaining a nuclear fission chain reaction during carriage
// pub struct RadioactiveMaterial { ... }

/// A class defining details about a train wagon used as a means of transport.
///
/// **UBL Dictionary Entry Name:** `Rail Transport. Details`
///
/// Generated from XSD type `RailTransportType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `TrainID` — An identifier for the train used as the means of transport.
/// - `RailCarID` — An identifier for the rail car on the train used as the means of transport.
// pub struct RailTransport { ... }

/// A class to define a line in a Receipt Advice.
///
/// **UBL Dictionary Entry Name:** `Receipt Line. Details`
///
/// Generated from XSD type `ReceiptLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this receipt line.
/// - `UUID` — A universally unique identifier for this receipt line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `ReceivedQuantity` — The quantity received.
/// - `ShortQuantity` — The quantity received short; the difference between the quantity reported despatched and the quantity actually received.
/// - `ShortageActionCode` — A code signifying the action that the delivery party wishes the despatch party to take as the result of a shortage.
/// - `RejectedQuantity` — The quantity rejected.
/// - `RejectReasonCode` — The reason for a rejection, expressed as a code.
/// - `RejectReason` — The reason for a rejection, expressed as text.
/// - `RejectActionCode` — A code signifying the action that the delivery party wishes the despatch party to take as the result of a rejection.
/// - `QuantityDiscrepancyCode` — A code signifying the type of a discrepancy in quantity.
/// - `OversupplyQuantity` — The quantity over-supplied, i.e., the quantity over and above the quantity ordered.
/// - `ReceivedDate` — The date on which the goods or services were received.
/// - `ReceivedTime` — The time at which the goods or services were received.
/// - `TimingComplaintCode` — A complaint about the timing of delivery, expressed as a code.
/// - `TimingComplaint` — A complaint about the timing of delivery, expressed as text.
/// - `OrderLineReference` — A reference to the order line associated with this receipt line.
/// - `DespatchLineReference` — A reference to a despatch line associated with this receipt line.
/// - `DocumentReference` — A reference to a document associated with this receipt line.
/// - `Item` — An item associated with this receipt line.
/// - `Shipment` — A shipment associated with this receipt line.
// pub struct ReceiptLine { ... }

/// A class to describe a regulation.
///
/// **UBL Dictionary Entry Name:** `Regulation. Details`
///
/// Generated from XSD type `RegulationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Name` — A name for this regulation.
/// - `LegalReference` — Text describing a legal reference.
/// - `OntologyURI` — The Uniform Resource Identifier (URI) of an ontology related to this regulation.
// pub struct Regulation { ... }

/// A class to describe the relationship to an item different from the item associated with the item line in which RelatedItem is used.
///
/// **UBL Dictionary Entry Name:** `Related Item. Details`
///
/// Generated from XSD type `RelatedItemType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for the related item.
/// - `Quantity` — The quantity that applies to the relationship.
/// - `Description` — Text describing the relationship.
// pub struct RelatedItem { ... }

/// A class to define a line in a Reminder document.
///
/// **UBL Dictionary Entry Name:** `Reminder Line. Details`
///
/// Generated from XSD type `ReminderLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this reminder line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `UUID` — A universally unique identifier for this reminder line.
/// - `BalanceBroughtForwardIndicator` — An indication that this reminder line contains a balance brought forward (true) or does not (false).
/// - `DebitLineAmount` — The amount debited on this reminder line.
/// - `CreditLineAmount` — The amount credited on this reminder line.
/// - `AccountingCostCode` — The buyer's accounting cost centre for this reminder line, expressed as a code.
/// - `AccountingCost` — The buyer's accounting cost centre for this reminder line, expressed as text.
/// - `PenaltySurchargePercent` — The penalty for late payment, expressed as a percentage.
/// - `Amount` — The amount on this reminder line.
/// - `PaymentPurposeCode` — A code signifying the business purpose for this payment.
/// - `ReminderPeriod` — A period to which this reminder line applies.
/// - `BillingReference` — A reference to a billing document associated with this reminder line.
/// - `ExchangeRate` — The rate of exchange between the currency of the Reminder and the currency of the document described in the BillingReference.
// pub struct ReminderLine { ... }

/// A class to define a line in a Remittance Advice.
///
/// **UBL Dictionary Entry Name:** `Remittance Advice Line. Details`
///
/// Generated from XSD type `RemittanceAdviceLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this remittance advice line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `UUID` — A universally unique identifier for this remittance advice line.
/// - `DebitLineAmount` — The amount debited on this remittance advice line.
/// - `CreditLineAmount` — The amount credited on this remittance advice line.
/// - `BalanceAmount` — The monetary balance associated with this remittance advice line.
/// - `PaymentPurposeCode` — A code signifying the business purpose for this payment.
/// - `InvoicingPartyReference` — A reference to the order for payment used by the invoicing party. This may have been requested of the payer by the payee to accompany its remittance.
/// - `AccountingSupplierParty` — The Accounting Supplier Party related to the remittance information reported on this Remittance Advice Line.
/// - `AccountingCustomerParty` — The Accounting Customer Party related to the remittance information reported on this Remittance Advice Line.
/// - `BuyerCustomerParty` — The buyer associated with this remittance advice line.
/// - `SellerSupplierParty` — The seller/supplier associated with this remittance advice line.
/// - `OriginatorCustomerParty` — The originating party.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `PayeeParty` — The Party who receives the Payment.
/// - `InvoicePeriod` — An invoice period to which this remittance advice line applies.
/// - `BillingReference` — A reference to a billing document associated with this remittance advice line.
/// - `DocumentReference` — A reference to a document associated with this remittance advice line.
/// - `ExchangeRate` — The rate of exchange between the currency of the Remittance Advice and the currency of the document described in the BillingReference.
// pub struct RemittanceAdviceLine { ... }

/// A class to describe the renewal of a commercial arrangement, such as a contract or licence fee.
///
/// **UBL Dictionary Entry Name:** `Renewal. Details`
///
/// Generated from XSD type `RenewalType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Amount` — The monetary amount of this renewal.
/// - `Period` — The period for which the arrangement is now valid
// pub struct Renewal { ... }

/// A class to define a line in a Request for Quotation.
///
/// **UBL Dictionary Entry Name:** `Request For Quotation Line. Details`
///
/// Generated from XSD type `RequestForQuotationLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this line in the request for quotation.
/// - `UUID` — A universally unique identifier for this line in the request for quotation.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `OptionalLineItemIndicator` — An indication whether this line is optional (true) or not (false) for purposes of this request for quotation.
/// - `PrivacyCode` — A code signifying the level of confidentiality of this request for quotation line.
/// - `SecurityClassificationCode` — A code signifying the security classification of this request for quotation line.
/// - `DocumentReference` — A document associated with this request for quotation line.
/// - `LineItem` — A description of the item for which a quotation is requested.
// pub struct RequestForQuotationLine { ... }

/// A class to define a line in a Request for Tender describing an item of goods or a service solicited in the Request for Tender.
///
/// **UBL Dictionary Entry Name:** `Request For Tender Line. Details`
///
/// Generated from XSD type `RequestForTenderLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this request for tender line.
/// - `UUID` — A universally unique identifier for this request for tender line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `Quantity` — The quantity of the item for which a tender is requested in this line.
/// - `MinimumQuantity` — The minimum quantity of the item associated with this request for tender line.
/// - `MaximumQuantity` — The maximum quantity of the item associated with this request for tender line.
/// - `TaxIncludedIndicator` — Indicates whether the amounts are taxes included (true) or not (false).
/// - `MinimumAmount` — The minimum amount allowed for this deliverable.
/// - `MaximumAmount` — The maximum amount allowed for this deliverable.
/// - `EstimatedAmount` — The estimated total amount of the deliverable.
/// - `DocumentReference` — A reference to a document associated with this request for tender line.
/// - `DeliveryPeriod` — An applicable period for the deliverable or set of deliverables in this tendering process.
/// - `RequiredItemLocationQuantity` — Properties of the item specified in this request for tender line that are dependent on location and quantity.
/// - `WarrantyValidityPeriod` — The period during which a warranty to be associated with this request for tender line must apply.
/// - `Item` — An item for which a tender is requested.
/// - `SubRequestForTenderLine` — A subsidiary request for tender line.
// pub struct RequestForTenderLine { ... }

/// A class defining budgeted monetary amounts.
///
/// **UBL Dictionary Entry Name:** `Requested Tender Total. Details`
///
/// Generated from XSD type `RequestedTenderTotalType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `EstimatedOverallContractAmount` — The estimated overall monetary amount of a contract.
/// - `EstimatedOverallFrameworkContractsAmount` — The estimated overall monetary amount of subsequent framework contracts.
/// - `TotalAmount` — The monetary amount of the total budget including net amount, taxes, and material and instalment costs.
/// - `TaxIncludedIndicator` — Indicates whether the amounts are taxes included (true) or not (false).
/// - `MinimumAmount` — The minimum monetary amount of the budget.
/// - `MaximumAmount` — The maximum monetary amount of the budget.
/// - `MonetaryScope` — A description of the monetary scope of the budget.
/// - `AverageSubsequentContractAmount` — The average monetary amount for the subsequent contracts following this budget amount.
/// - `ApplicableTaxCategory` — Describes the categories of taxes that apply to the budget amount.
// pub struct RequestedTenderTotal { ... }

/// A class to describe the type and amount of resources consumed during a product’s lifecycle, including information about source and timing.
///
/// **UBL Dictionary Entry Name:** `Resource Consumption. Details`
///
/// Generated from XSD type `ResourceConsumptionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ResourceTypeCode` — A code identifying the type of resource (e.g., water, electricity, raw material).
/// - `ConsumptionMeasure` — The amount of the resource consumed.
/// - `ResourceOriginDescription` — A text description of the source or origin of the consumed resource.
/// - `MeasurementPeriod` — The period during which this resource consumption was measured.
// pub struct ResourceConsumption { ... }

/// A class to describe an application-level response to a document.
///
/// **UBL Dictionary Entry Name:** `Response. Details`
///
/// Generated from XSD type `ResponseType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ReferenceID` — An identifier for the section (or line) of the document to which this response applies.
/// - `ResponseCode` — A code signifying the type of response.
/// - `Description` — Text describing this response.
/// - `EffectiveDate` — The date upon which this response is valid.
/// - `EffectiveTime` — The time at which this response is valid.
/// - `Status` — A status report associated with this response.
// pub struct Response { ... }

/// A class to describe the criterion requirement response value.
///
/// **UBL Dictionary Entry Name:** `Response Value. Details`
///
/// Generated from XSD type `ResponseValueType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier to refer to the criterion requirement response value.
/// - `Description` — A description of the response value to the criterion requirement.
/// - `Response` — A text or name used as a reply to the criterion requirement.
/// - `ResponseAmount` — An amount used as a reply to the criterion requirement.
/// - `ResponseBinaryObject` — A binary graphic, picture, sound or video object used as a reply to the criterion requirement.
/// - `ResponseCode` — A code used as a reply to the criterion requirement.
/// - `ResponseDate` — A date used as a reply to the criterion requirement.
/// - `ResponseID` — An identifier used as a reply to the criterion requirement.
/// - `ResponseIndicator` — An indicator used as a reply to the criterion requirement.
/// - `ResponseMeasure` — A measure used as a reply to the criterion requirement.
/// - `ResponseNumeric` — A number, rate or percent used as a reply to the criterion requirement.
/// - `ResponseQuantity` — A quantity used as a reply to the criterion requirement.
/// - `ResponseTime` — A time used as a reply to the criterion requirement.
/// - `ResponseURI` — A URI value used as a reply to the criterion requirement.
// pub struct ResponseValue { ... }

/// A class to describe the result of an attempt to verify a signature.
///
/// **UBL Dictionary Entry Name:** `Result Of Verification. Details`
///
/// Generated from XSD type `ResultOfVerificationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ValidatorID` — An identifier for the organization, person, service, or server that verified the signature.
/// - `ValidationResultCode` — A code signifying the result of the verification.
/// - `ValidationDate` — The date upon which verification took place.
/// - `ValidationTime` — The time at which verification took place.
/// - `ValidateProcess` — The verification process.
/// - `ValidateTool` — The tool used to verify the signature.
/// - `ValidateToolVersion` — The version of the tool used to verify the signature.
/// - `SignatoryParty` — The signing party.
// pub struct ResultOfVerification { ... }

/// A class to describe a planned effect of a retail event (e.g., a promotion or a change in inventory policy) upon supply or demand.
///
/// **UBL Dictionary Entry Name:** `Retail Planned Impact. Details`
///
/// Generated from XSD type `RetailPlannedImpactType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Amount` — Estimated monetary value of the planned event as an impact
/// - `ForecastPurposeCode` — It will have impact on either Sales forecast or Order Forecast
/// - `ForecastTypeCode` — A code signifying the type of forecast. Examples of values are: BASE PROMOTIONAL SEASONAL TOTAL
/// - `Period` — The period to which this impact applies.
// pub struct RetailPlannedImpact { ... }

/// A class for identifying a vehicle used for road transport.
///
/// **UBL Dictionary Entry Name:** `Road Transport. Details`
///
/// Generated from XSD type `RoadTransportType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `LicensePlateID` — The license plate identifier of this vehicle.
/// - `TrailerLicensePlateID` — The license plate identifier of a trailer pulled by this vehicle.
// pub struct RoadTransport { ... }

/// A class to describe information related to an item in a sales context
///
/// **UBL Dictionary Entry Name:** `Sales Item. Details`
///
/// Generated from XSD type `SalesItemType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Quantity` — The quantity the given information are related to
/// - `ActivityProperty` — A class to describe the activity (for example "sales", "movement", ...) related to the item.
/// - `TaxExclusivePrice` — A price for this sales item, exclusive of tax.
/// - `TaxInclusivePrice` — A price for this sales item, including tax.
/// - `Item` — The sales item itself.
// pub struct SalesItem { ... }

/// A class describing a plan, action or meassure that has been implemented for sanitary reasons.
///
/// **UBL Dictionary Entry Name:** `Sanitary Measure. Details`
///
/// Generated from XSD type `SanitaryMeasureType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `SanitaryMeasureTypeCode` — A code decribing the type of sanitary meassure.
/// - `ApplicationDate` — The date this sanitary meassure was applied.
// pub struct SanitaryMeasure { ... }

/// A class to describe an assigned numeric or qualitative score using a recognized scoring system.
///
/// **UBL Dictionary Entry Name:** `Score. Details`
///
/// Generated from XSD type `ScoreType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ScoreNumeric` — A numeric value representing the score assigned to this item by a recognized scoring system.
/// - `ScoringSystemCode` — A code identifying the scoring system used to determine this Score.
// pub struct Score { ... }

/// A class to describe a secondary hazard associated with a hazardous item.
///
/// **UBL Dictionary Entry Name:** `Secondary Hazard. Details`
///
/// Generated from XSD type `SecondaryHazardType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this secondary hazard.
/// - `PlacardNotation` — Text of the placard notation corresponding to the hazard class of this secondary hazard. Can also be the hazard identification number of the orange placard (upper part) required on the means of transport.
/// - `PlacardEndorsement` — Text of the placard endorsement for this secondary hazard that is to be shown on the shipping papers for a hazardous item. Can also be used for the number of the orange placard (lower part) required on the means of transport.
/// - `EmergencyProceduresCode` — A code signifying the emergency procedures for this secondary hazard.
/// - `Extension` — Additional information about the hazardous substance, which can be used (for example) to specify the type of regulatory requirements that apply to this secondary hazard.
// pub struct SecondaryHazard { ... }

/// A class to specify security clearance terms.
///
/// **UBL Dictionary Entry Name:** `Security Clearance Term. Details`
///
/// Generated from XSD type `SecurityClearanceTermType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Code` — A code signifying the security clearance requirement.
/// - `Description` — A description of the security clearance requirement.
// pub struct SecurityClearanceTerm { ... }

/// A class to describe a financial security listed on a regulated market.
///
/// **UBL Dictionary Entry Name:** `Security Listing. Details`
///
/// Generated from XSD type `SecurityListingType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for the listed security, such as an ISIN.
/// - `Description` — A description of the listed security, such as the name or type of the instrument.
/// - `MarketName` — The name of the regulated market on which this security is listed.
/// - `MarketCode` — A code identifying the regulated market (e.g., MIC code as per ISO 10383).
// pub struct SecurityListing { ... }

/// A class to describe a security measure
///
/// **UBL Dictionary Entry Name:** `Security Measure. Details`
///
/// Generated from XSD type `SecurityMeasureType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this security measure.
/// - `Description` — The description of this security measure
// pub struct SecurityMeasure { ... }

/// A class to specify which day of the week a transport service is operational.
///
/// **UBL Dictionary Entry Name:** `Service Frequency. Details`
///
/// Generated from XSD type `ServiceFrequencyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `WeekDayCode` — A day of the week, expressed as code.
// pub struct ServiceFrequency { ... }

/// A class to describe a service level agreement which regulates the quality, availability and responsibilities of digital services.
///
/// **UBL Dictionary Entry Name:** `Service Level Agreement. Details`
///
/// Generated from XSD type `ServiceLevelAgreementType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this service level agreement.
/// - `ServiceTypeCode` — A specific type of service subject to this service level agreement.
/// - `ServiceType` — A specific type of service subject to this service level agreement, expressed as text.
/// - `AvailabilityTimePercent` — The availability percentage (e.g. 98.5% of the time).
/// - `MondayAvailabilityIndicator` — Indicates whether this service is available on monday (true) or not (false).
/// - `TuesdayAvailabilityIndicator` — Indicates whether this service is available on tuesday (true) or not (false).
/// - `WednesdayAvailabilityIndicator` — Indicates whether this service is available on wednesday (true) or not (false).
/// - `ThursdayAvailabilityIndicator` — Indicates whether this service is available on thursday (true) or not (false).
/// - `FridayAvailabilityIndicator` — Indicates whether this service is available on friday (true) or not (false).
/// - `SaturdayAvailabilityIndicator` — Indicates whether this service is available on saturday (true) or not (false).
/// - `SundayAvailabilityIndicator` — Indicates whether this service is available on sunday (true) or not (false).
/// - `MinimumResponseTimeDurationMeasure` — The response time for aknowledgment (e.g. to send a receipt to a sending Access Point within 300 seconds).
/// - `MinimumDownTimeScheduleDurationMeasure` — The minimum down time schedule for programmed maintenance (e.g. scheduled 3 days before).
/// - `MaximumIncidentNotificationDurationMeasure` — The maximum length of time between the occurrence of an incident and the issuance of a notification (e.g. within 4 hours).
/// - `MaximumDataLossDurationMeasure` — The maximum data loss permitted (e.g. last 24 hours).
/// - `MeanTimeToRecoverDurationMeasure` — The time taken to recover after an outage of service (e.g. 3 hours).
/// - `ServiceAvailabilityPeriod` — The period for which the service is available.
/// - `ServiceMaintenancePeriod` — The period of time designated in advance by the technical staff, during which preventive maintenance that could cause disruption of service may be performed.
// pub struct ServiceLevelAgreement { ... }

/// A class to describe a party contracting to provide services, such as transportation, finance, etc.
///
/// **UBL Dictionary Entry Name:** `Service Provider Party. Details`
///
/// Generated from XSD type `ServiceProviderPartyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this Service Provider.
/// - `ServiceTypeCode` — The type of service provided, expressed as a code.
/// - `ServiceType` — The type of service provided, expressed as text.
/// - `Party` — The Party who provides this service.
/// - `SellerContact` — The contact for the Service Provider.
// pub struct ServiceProviderParty { ... }

/// A class to describe a Party that owns shares or equity.
///
/// **UBL Dictionary Entry Name:** `Shareholder Party. Details`
///
/// Generated from XSD type `ShareholderPartyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `PartecipationPercent` — (Deprecated) The shareholder participation, expressed as a percentage.
/// - `ParticipationPercent` — The percentage of shares or equity owned by this Party.
/// - `Party` — The Party that owns shares or equity.
// pub struct ShareholderParty { ... }

/// A class to describe a requirement for a ship
///
/// **UBL Dictionary Entry Name:** `Ship Requirement. Details`
///
/// Generated from XSD type `ShipRequirementType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this ship requirement.
/// - `Description` — The description of this ship requirement
// pub struct ShipRequirement { ... }

/// An article in the ship's stores during a shipment stage.
///
/// **UBL Dictionary Entry Name:** `Ship Store Article. Details`
///
/// Generated from XSD type `ShipStoreArticleType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An indentifier for this ship store article.
/// - `Name` — A name for this ship store article.
/// - `Quantity` — The quantity of this ship store article.
/// - `OfficialUse` — A text describing the official use of this ship store article.
/// - `Stowage` — The stowage or location on board of this ship store article.
// pub struct ShipStoreArticle { ... }

/// A class to describe a ship to ship activity record.
///
/// **UBL Dictionary Entry Name:** `Ship To Ship Activity Record. Details`
///
/// Generated from XSD type `ShipToShipActivityRecordType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An indentifier for this ship to ship activity.
/// - `Description` — A text describing the ship to ship activity.
/// - `AppliedSecurityMeasure` — Any security measures applied to this ship to ship activity in lieu of those specified in the approved Ship Security Plan (SSP).
/// - `Period` — The duration of this ship to ship activity.
/// - `Location` — The location where this ship to ship activity took place.
// pub struct ShipToShipActivityRecord { ... }

/// A class defining an identifiable collection of one or more goods items to be transported between the seller party and the buyer party. This information may be defined within a commercial contract. A shipment can be transported in different consignments (e.g., split for logistical purposes).
///
/// **UBL Dictionary Entry Name:** `Shipment. Details`
///
/// Generated from XSD type `ShipmentType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this shipment.
/// - `ShippingPriorityLevelCode` — A code signifying the priority or level of service required for this shipment.
/// - `HandlingCode` — The handling required for this shipment, expressed as a code.
/// - `HandlingInstructions` — The handling required for this shipment, expressed as text.
/// - `Information` — Free-form text pertinent to this shipment, conveying information that is not contained explicitly in other structures.
/// - `GrossWeightMeasure` — The total gross weight of a shipment; the weight of the goods plus packaging plus transport equipment.
/// - `NetWeightMeasure` — The net weight of this shipment, excluding packaging.
/// - `NetNetWeightMeasure` — The total net weight of this shipment, excluding packaging and transport equipment.
/// - `GrossVolumeMeasure` — The total volume of the goods in this shipment, including packaging.
/// - `NetVolumeMeasure` — The total volume of the goods in this shipment, excluding packaging and transport equipment.
/// - `TotalGoodsItemQuantity` — The total number of goods items in this shipment.
/// - `TotalTransportHandlingUnitQuantity` — The number of pieces of transport handling equipment (pallets, boxes, cases, etc.) in this shipment.
/// - `InsuranceValueAmount` — The amount covered by insurance for this shipment.
/// - `DeclaredCustomsValueAmount` — The total declared value for customs purposes of those goods in this shipment that are subject to the same customs procedure and have the same tariff/statistical heading, country information, and duty regime.
/// - `DeclaredForCarriageValueAmount` — The value of this shipment, declared by the shipper or his agent solely for the purpose of varying the carrier's level of liability from that provided in the contract of carriage, in case of loss or damage to goods or delayed delivery.
/// - `DeclaredStatisticsValueAmount` — The value, declared for statistical purposes, of those goods in this shipment that have the same statistical heading.
/// - `FreeOnBoardValueAmount` — The monetary amount that has to be or has been paid as calculated under the applicable trade delivery.
/// - `SpecialInstructions` — Special instructions relating to this shipment.
/// - `DeliveryInstructions` — Delivery instructions relating to this shipment.
/// - `SplitConsignmentIndicator` — An indicator that the consignment has been split in transit (true) or not (false).
/// - `ConsignmentQuantity` — The total number of consignments within this shipment.
/// - `Consignment` — A consignment covering this shipment.
/// - `GoodsItem` — A goods item included in this shipment.
/// - `ShipmentStage` — A stage in the transport movement of this shipment.
/// - `Delivery` — The delivery of this shipment.
/// - `TransportHandlingUnit` — A transport handling unit associated with this shipment.
/// - `ReturnAddress` — The address to which a shipment ought to be returned.
/// - `OriginAddress` — The region in which the goods have been produced or manufactured, according to criteria laid down for the purposes of application of the customs tariff, or of quantitative restrictions, or of any other measure related to trade.
/// - `FirstArrivalPortLocation` — The first arrival location of a shipment. This would be a port for sea, an airport for air, a terminal for rail, or a border post for land crossing.
/// - `LastExitPortLocation` — The final exporting location for a shipment. This would be a port for sea, an airport for air, a terminal for rail, or a border post for land crossing.
/// - `ExportCountry` — The country from which the goods were originally exported, without any commercial transaction taking place in intermediate countries.
/// - `FreightAllowanceCharge` — A cost incurred by the shipper in moving goods, by whatever means, from one place to another under the terms of the contract of carriage. In addition to transport costs, this may include such elements as packing, documentation, loading, unloading, and insurance to the extent that they relate to the freight costs.
/// - `InsurancePolicy` — One or more Insurance Policies that apply to this Shipment.
// pub struct Shipment { ... }

/// A class to describe one stage of movement in a transport of goods.
///
/// **UBL Dictionary Entry Name:** `Shipment Stage. Details`
///
/// Generated from XSD type `ShipmentStageType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this shipment stage.
/// - `ShipmentStageTypeCode` — The type of shipment stage, expressed as a code.
/// - `ShipmentStageType` — The type of shipment stage, expressed as text.
/// - `TransportModeCode` — A code signifying the method of transport used for this shipment stage.
/// - `TransportMeansTypeCode` — A code signifying the kind of transport means (truck, vessel, etc.) used for this shipment stage.
/// - `TransitDirectionCode` — A code signifying the direction of transit in this shipment stage.
/// - `PreCarriageIndicator` — An indicator that this stage takes place before the main carriage of the shipment (true) or not (false).
/// - `OnCarriageIndicator` — An indicator that this stage takes place after the main carriage of the shipment (true) or not (false).
/// - `CabotageIndicator` — An indicator that cabotage applies for this shipment stage (true) or not (false).
/// - `HazardousRiskIndicator` — An indicator that the transported goods in this shipment stage are subject to an international regulation concerning the carriage of dangerous goods (true) or not (false).
/// - `EstimatedDeliveryDate` — The estimated date of delivery in this shipment stage.
/// - `EstimatedDeliveryTime` — The estimated time of delivery in this shipment stage.
/// - `RequiredDeliveryDate` — The delivery date required by the buyer in this shipment stage.
/// - `RequiredDeliveryTime` — The delivery time required by the buyer in this shipment stage.
/// - `LoadingSequenceID` — An identifier for the loading sequence (of consignments) associated with this shipment stage.
/// - `SuccessiveSequenceID` — Identifies the successive loading sequence (of consignments) associated with a shipment stage.
/// - `Instructions` — Text of instructions applicable to a shipment stage.
/// - `DemurrageInstructions` — Text of instructions relating to demurrage (the case in which a vessel is prevented from loading or discharging cargo within the stipulated laytime).
/// - `CrewQuantity` — The total number of crew aboard a transport means.
/// - `PassengerQuantity` — The total number of passengers aboard a transport means.
/// - `TransitPeriod` — The period during which this shipment stage actually took place.
/// - `CarrierParty` — The Party who provides the transport of goods between named points.
/// - `TransportMeans` — The means of transport used in this shipment stage.
/// - `LoadingPortLocation` — The location of loading for a shipment stage.
/// - `UnloadingPortLocation` — The location of unloading for a shipment stage.
/// - `TransshipPortLocation` — The location of transshipment relating to a shipment stage.
/// - `LoadingTransportEvent` — The loading of goods in this shipment stage.
/// - `ExaminationTransportEvent` — The examination of shipments in this shipment stage.
/// - `AvailabilityTransportEvent` — The making available of shipments in this shipment stage.
/// - `ExportationTransportEvent` — The export event associated with this shipment stage.
/// - `DischargeTransportEvent` — The discharge event associated with this shipment stage.
/// - `WarehousingTransportEvent` — The warehousing event associated with this shipment stage.
/// - `TakeoverTransportEvent` — The receiver's takeover of the goods in this shipment stage.
/// - `OptionalTakeoverTransportEvent` — The optional takeover of the goods in this shipment stage.
/// - `DropoffTransportEvent` — The dropping off of goods in this shipment stage.
/// - `ActualPickupTransportEvent` — The pickup of goods in this shipment stage.
/// - `DeliveryTransportEvent` — The delivery of goods in this shipment stage.
/// - `ReceiptTransportEvent` — The receipt of goods in this shipment stage.
/// - `StorageTransportEvent` — The storage of goods in this shipment stage.
/// - `AcceptanceTransportEvent` — The acceptance of goods in this shipment stage.
/// - `TerminalOperatorParty` — A terminal operator associated with this shipment stage.
/// - `CustomsAgentParty` — The Customs Agent who is associated with this Shipment Stage.
/// - `EstimatedTransitPeriod` — The estimated transit period of this shipment stage.
/// - `FreightAllowanceCharge` — A freight allowance charge for this shipment stage.
/// - `FreightChargeLocation` — The location associated with a freight charge related to this shipment stage.
/// - `DetentionTransportEvent` — The detention of a transport means during loading and unloading operations.
/// - `RequestedDepartureTransportEvent` — The departure requested by the party requesting a transportation service.
/// - `RequestedArrivalTransportEvent` — The arrival requested by the party requesting a transportation service.
/// - `RequestedWaypointTransportEvent` — A waypoint requested by the party requesting a transportation service.
/// - `PlannedDepartureTransportEvent` — The departure planned by the party providing a transportation service.
/// - `PlannedArrivalTransportEvent` — The arrival planned by the party providing a transportation service.
/// - `PlannedWaypointTransportEvent` — A waypoint planned by the party providing a transportation service.
/// - `ActualDepartureTransportEvent` — The actual departure from a specific location during a transportation service.
/// - `ActualWaypointTransportEvent` — The location of an actual waypoint during a transportation service.
/// - `ActualArrivalTransportEvent` — The actual arrival at a specific location during a transportation service.
/// - `TransportEvent` — A additional significant occurrence in the course of this shipment of goods that is not defined elsewhere in this Shipment Stage.
/// - `EstimatedDepartureTransportEvent` — Describes an estimated departure at a location during a transport service.
/// - `EstimatedArrivalTransportEvent` — Describes an estimated arrival at a location during a transport service.
/// - `PassengerPerson` — A person who travels in a conveyance without participating in its operation.
/// - `DriverPerson` — Describes a person responsible for driving the transport means.
/// - `ReportingPerson` — Describes a person being responsible for providing the required administrative reporting relating to a transport.
/// - `CrewMemberPerson` — A person operating or serving aboard a transport means.
/// - `SecurityOfficerPerson` — The person on board the vessel, accountable to the master, designated by the company as responsible for the security of the ship, including implementation and maintenance of the ship security plan and for the liaison with the company security officer and the port facility security officers.
/// - `MasterPerson` — The person responsible for the ship's safe and efficient operation, including cargo operations, navigation, crew management and for ensuring that the vessel complies with local and international laws, as well as company and flag state policies.
/// - `ShipsSurgeonPerson` — The person responsible for the health of the people aboard a ship at sea.
/// - `DestinationPortCall` — A destination port call for this shipment stage.
/// - `ShipStoreArticle` — The ship store articles for this shipment stage.
/// - `CrewPersonEffect` — The crew person effects for this shipment stage.
/// - `MaritimeWaste` — The maritime waste for this shipment stage.
/// - `BallastWaterSummary` — A ballast water summary for this shipment stage.
/// - `ISPSRequirements` — The ISPS (International Ship and Port Facility Security Code) requirements for this shipment stage.
/// - `MaritimeHealthDeclaration` — A maritime declaration of health for this shipment stage.
/// - `FuelConsumption` — One or more fuel consumptions of this shipment stage.
// pub struct ShipmentStage { ... }

/// A class to define a signature.
///
/// **UBL Dictionary Entry Name:** `Signature. Details`
///
/// Generated from XSD type `SignatureType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this signature.
/// - `ReasonCode` — A code defining the reason or purpose of this signature
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures; in particular, information regarding the circumstances in which the signature is being used.
/// - `ValidationDate` — The date upon which this signature was verified.
/// - `ValidationTime` — The time at which this signature was verified.
/// - `ValidatorID` — An identifier for the organization, person, service, or server that verified this signature.
/// - `CanonicalizationMethod` — The method used to perform XML canonicalization of this signature.
/// - `SignatureMethod` — Text describing the method of signature.
/// - `SignatoryParty` — The Party that provides the signature.
/// - `DigitalSignatureAttachment` — The actual encoded signature (e.g., in XMLDsig format).
/// - `OriginalDocumentReference` — A reference to the document that the signature applies to. For evidentiary purposes, this may be the document image that the signatory party saw when applying their signature.
// pub struct Signature { ... }

/// A class to describe a social media profile.
///
/// **UBL Dictionary Entry Name:** `Social Media Profile. Details`
///
/// Generated from XSD type `SocialMediaProfileType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for a specific social media.
/// - `Name` — The common name of the social media.
/// - `SocialMediaTypeCode` — A code that specifies the type of social media.
/// - `URI` — The Uniform Resource Identifier (URI) of a party profile in the social media; i.e., its Uniform Resource Locator (URL).
// pub struct SocialMediaProfile { ... }

/// A class to define a line in a Statement of account.
///
/// **UBL Dictionary Entry Name:** `Statement Line. Details`
///
/// Generated from XSD type `StatementLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this statement line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `UUID` — A universally unique identifier for this statement line.
/// - `BalanceBroughtForwardIndicator` — An indication that this statement line contains an outstanding balance from the previous bill(s) (true) or does not (false).
/// - `DebitLineAmount` — The amount debited on this statement line.
/// - `CreditLineAmount` — The amount credited on this statement line.
/// - `BalanceAmount` — The balance amount on this statement line.
/// - `PaymentPurposeCode` — A code signifying the business purpose for this payment.
/// - `PaymentMeans` — A means of payment associated with this statement line.
/// - `PaymentTerms` — A specification of payment terms associated with this statement line.
/// - `BuyerCustomerParty` — The buyer associated with this statement line.
/// - `SellerSupplierParty` — The seller/supplier associated with this statement line.
/// - `OriginatorCustomerParty` — The originating party.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `AccountingCustomerParty` — The Accounting Customer Party related to the statement information reported on this Statement Line.
/// - `AccountingSupplierParty` — The Accounting Supplier Party related to the statement information reported on this Statement Line.
/// - `PayeeParty` — The Party who receives the Payment.
/// - `InvoicePeriod` — An invoice period to which this statement line applies.
/// - `BillingReference` — A reference to a billing document associated with this statement line.
/// - `DocumentReference` — A reference to a document associated with this statement line.
/// - `ExchangeRate` — The rate of exchange between the currency of the Statement and the currency of the document described in the BillingReference.
/// - `AllowanceCharge` — A charge or discount price component associated with this statement line.
/// - `CollectedPayment` — A collected payment.
// pub struct StatementLine { ... }

/// A class to describe the condition or position of an object.
///
/// **UBL Dictionary Entry Name:** `Status. Details`
///
/// Generated from XSD type `StatusType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ConditionCode` — Specifies the status condition of the related object.
/// - `ReferenceDate` — The reference date for this status.
/// - `ReferenceTime` — The reference time for this status.
/// - `Description` — Text describing this status.
/// - `StatusReasonCode` — The reason for this status condition or position, expressed as a code.
/// - `StatusReason` — The reason for this status condition or position, expressed as text.
/// - `SequenceID` — A sequence identifier for this status.
/// - `Text` — Provides any textual information related to this status.
/// - `IndicationIndicator` — Specifies an indicator relevant to a specific status.
/// - `Percent` — A percentage meaningful in the context of this status.
/// - `ReliabilityPercent` — The reliability of this status, expressed as a percentage.
/// - `DocumentationAttachment` — One or more attachments (such as photos) used to document the status of the object.
/// - `SubStatus` — An additional sub status to clarify or ellaborate on the status
/// - `Condition` — Measurements that quantify the condition of the objects covered by the status.
// pub struct Status { ... }

/// A class to define a line in a Stock Availability Report describing the availability of an item of sale.
///
/// **UBL Dictionary Entry Name:** `Stock Availability Report Line. Details`
///
/// Generated from XSD type `StockAvailabilityReportLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this stock availability line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `Quantity` — The quantity of the item currently in stock.
/// - `ValueAmount` — The monetary value of the quantity of the item currently in stock.
/// - `AvailabilityDate` — The date from which the item will be available. A date identical to or earlier than the IssueDate of the Stock Availability Report means that the item is available now
/// - `AvailabilityStatusCode` — A code signifying the level of availability of the item.
/// - `Item` — The item associated with this stock availability report line.
// pub struct StockAvailabilityReportLine { ... }

/// A class to describe a storage and storage requirements
///
/// **UBL Dictionary Entry Name:** `Storage. Details`
///
/// Generated from XSD type `StorageType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this storage.
/// - `Name` — The common name this storage
/// - `GateID` — An identifier for the agreed gate to enter, deliver or pick up at this storage
/// - `AirFlowPercent` — The percent of the airflow within this storage.
/// - `HumidityPercent` — The percent humidity within this storage.
/// - `AnimalFoodApprovedIndicator` — An indicator that this storage is approved for animal food (true) or not (false).
/// - `HumanFoodApprovedIndicator` — An indicator that this storage is approved for human food (true) or not (false).
/// - `DangerousGoodsApprovedIndicator` — An indicator that this stroage is approved for dangerous goods (true) or not (false).
/// - `RefrigeratedIndicator` — An indicator that storage is refrigerated (true) or not (false).
/// - `PowerIndicator` — An indicator that this storage can supply power (true) or not (false).
/// - `MinimumTemperature` — The minimum allowable operating temperature for this refriguated storage.
/// - `MaximumTemperature` — The maximum allowable operating temperature for this refriguated storage.
/// - `Certificate` — A certificate associated with this storage
// pub struct Storage { ... }

/// A class to describe a location on board a means of transport where specified goods or transport equipment have been stowed or are to be stowed.
///
/// **UBL Dictionary Entry Name:** `Stowage. Details`
///
/// Generated from XSD type `StowageType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `LocationID` — An identifier for the location.
/// - `Location` — Text describing the location.
/// - `MeasurementDimension` — A measurable dimension (length, mass, weight, or volume) of this stowage.
// pub struct Stowage { ... }

/// A class to describe subcontract terms for a tendering process.
///
/// **UBL Dictionary Entry Name:** `Subcontract Terms. Details`
///
/// Generated from XSD type `SubcontractTermsType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Rate` — The precise percentage allowed to be subcontracted.
/// - `UnknownPriceIndicator` — (Updated definition) An indicator that the subcontract price is unknown (true) or not (false).
/// - `Description` — Text describing the subcontract terms.
/// - `Amount` — The monetary amount assigned to the subcontracted task.
/// - `SubcontractingConditionsCode` — A code specifying the conditions for subcontracting.
/// - `MaximumPercent` — The maximum percentage allowed to be subcontracted.
/// - `MinimumPercent` — The minimum percentage allowed to be subcontracted.
// pub struct SubcontractTerms { ... }

/// The consumption for a specific party for given consumption point provided by a numbers of suppliers. An enterprise can have one utility statement for several parties (e.g. a ministry of defence receiving a telephone bill). In this way each subscriber consumption represent a sub utility statement.
///
/// **UBL Dictionary Entry Name:** `Subscriber Consumption. Details`
///
/// Generated from XSD type `SubscriberConsumptionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ConsumptionID` — The identifier tor this specification.
/// - `SpecificationTypeCode` — The code which specifies the type of this specification, e.g. an on account specification or the yearly specification.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `TotalMeteredQuantity` — The total quantity consumed, as calculated from meter readings.
/// - `SubscriberParty` — The Party who is subscribed to the utility.
/// - `UtilityConsumptionPoint` — The point at which the utility is consumed.
/// - `OnAccountPayment` — The planned prepayments (on account) regarding this subscription.
/// - `Consumption` — The consumption in case the consumption is from one and only one supplier.
/// - `SupplierConsumption` — The consumption in case the consumption is from more than one supplier.
// pub struct SubscriberConsumption { ... }

/// The consumption in case the consumption is for one and only one supplier.
///
/// **UBL Dictionary Entry Name:** `Supplier Consumption. Details`
///
/// Generated from XSD type `SupplierConsumptionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Description` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `UtilitySupplierParty` — The Party who supplies the utility.
/// - `UtilityCustomerParty` — The Customer Party for this utility.
/// - `Consumption` — The consumption regarding this supplier
/// - `Contract` — A contract setting forth conditions regulating the consumption.
/// - `ConsumptionLine` — The consumption of a utility product.
// pub struct SupplierConsumption { ... }

/// A class to describe a supplier party.
///
/// **UBL Dictionary Entry Name:** `Supplier Party. Details`
///
/// Generated from XSD type `SupplierPartyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `CustomerAssignedAccountID` — An identifier for this supplier party, assigned by the customer.
/// - `AdditionalAccountID` — An additional identifier for this supplier party.
/// - `DataSendingCapability` — Text describing the supplier's ability to send invoice data via a purchase card provider (e.g., VISA, MasterCard, American Express).
/// - `Party` — The Supplier Party itself.
/// - `DespatchContact` — A contact at this supplier party for despatches (pickups).
/// - `AccountingContact` — A contact at this supplier party for accounting.
/// - `SellerContact` — The primary contact for this supplier party.
// pub struct SupplierParty { ... }

/// A class to describe one of the tax categories within a taxation scheme (e.g., High Rate VAT, Low Rate VAT).
///
/// **UBL Dictionary Entry Name:** `Tax Category. Details`
///
/// Generated from XSD type `TaxCategoryType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this tax category.
/// - `Name` — The name of this tax category.
/// - `Percent` — The tax rate for this category, expressed as a percentage.
/// - `BaseUnitMeasure` — A Unit of Measures used as the basic for the tax calculation applied at a certain rate per unit.
/// - `PerUnitAmount` — Where a tax is applied at a certain rate per unit, the rate per unit applied.
/// - `TaxExemptionReasonCode` — The reason for tax being exempted, expressed as a code.
/// - `TaxExemptionReason` — The reason for tax being exempted, expressed as text.
/// - `TierRange` — Where a tax is tiered, the range of taxable amounts that determines the rate of tax applicable to this tax category.
/// - `TierRatePercent` — Where a tax is tiered, the tax rate that applies within the specified range of taxable amounts for this tax category.
/// - `SupplyTypeCode` — A code signifying the type of supply to which this tax category applies, such as goods, services, or a mixture.
/// - `TaxScheme` — The taxation scheme within which this tax category is defined.
// pub struct TaxCategory { ... }

/// A class to describe a taxation scheme (e.g., VAT, State tax, County tax).
///
/// **UBL Dictionary Entry Name:** `Tax Scheme. Details`
///
/// Generated from XSD type `TaxSchemeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this taxation scheme.
/// - `Name` — The name of this taxation scheme.
/// - `TaxTypeCode` — A code signifying the type of tax.
/// - `CurrencyCode` — A code signifying the currency in which the tax is collected and reported.
/// - `JurisdictionRegionAddress` — A geographic area in which this taxation scheme applies.
// pub struct TaxScheme { ... }

/// A class to define the subtotal for a particular tax category within a particular taxation scheme, such as standard rate within VAT.
///
/// **UBL Dictionary Entry Name:** `Tax Subtotal. Details`
///
/// Generated from XSD type `TaxSubtotalType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `TaxableAmount` — The net amount to which the tax percent (rate) is applied to calculate the tax amount.
/// - `TaxAmount` — The amount of this tax subtotal.
/// - `TaxInclusiveAmount` — The total amount after the tax amount has been added to the taxable amount.
/// - `CalculationSequenceNumeric` — The number of this tax subtotal in the sequence of subtotals corresponding to the order in which multiple taxes are applied. If all taxes are applied to the same taxable amount (i.e., their order of application is inconsequential), then CalculationSequenceNumeric is 1 for all tax subtotals applied to a given amount.
/// - `TransactionCurrencyTaxAmount` — The amount of this tax subtotal, expressed in the currency used for invoicing.
/// - `Percent` — The tax rate of the tax category applied to this tax subtotal, expressed as a percentage.
/// - `BaseUnitMeasure` — The unit of measure on which the tax calculation is based
/// - `PerUnitAmount` — Where a tax is applied at a certain rate per unit, the rate per unit applied.
/// - `TierRange` — Where a tax is tiered, the range of taxable amounts that determines the rate of tax applicable to this tax subtotal.
/// - `TierRatePercent` — Where a tax is tiered, the tax rate that applies within a specified range of taxable amounts for this tax subtotal.
/// - `TaxCategory` — The tax category applicable to this subtotal.
/// - `TaxDueCountry` — The country where this tax is due.
// pub struct TaxSubtotal { ... }

/// A class to describe the total tax for a particular taxation scheme.
///
/// **UBL Dictionary Entry Name:** `Tax Total. Details`
///
/// Generated from XSD type `TaxTotalType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `TaxAmount` — The total tax amount for a particular taxation scheme, e.g., VAT; the sum of the tax subtotals for each tax category within the taxation scheme.
/// - `CalculationSequenceNumeric` — The number of this tax total in the sequence of tax totals corresponding to the order in which multiple taxes are applied. If all taxes are applied to the same taxable amount (i.e., their order of application is inconsequential), then CalculationSequenceNumeric is 1 for all tax totals applied to a given amount.
/// - `RoundingAmount` — The rounding amount (positive or negative) added to the calculated tax total to produce the rounded TaxAmount.
/// - `TaxEvidenceIndicator` — An indicator that this total is recognized as legal evidence for taxation purposes (true) or not (false).
/// - `TaxIncludedIndicator` — An indicator that tax is included in the calculation (true) or not (false).
/// - `TaxSubtotal` — One of the subtotals the sum of which equals the total tax amount for a particular taxation scheme.
// pub struct TaxTotal { ... }

/// A class to describe a telecommunications service (e.g., a telephone call or a video on demand service).
///
/// **UBL Dictionary Entry Name:** `Telecommunications Service. Details`
///
/// Generated from XSD type `TelecommunicationsServiceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this telecommunications service.
/// - `CallDate` — In the case of a telephone call, the date of the call.
/// - `CallTime` — In the case of a telephone call, the time of the call.
/// - `ServiceNumberCalled` — In the case of a telephone call, the phone number called.
/// - `TelecommunicationsServiceCategory` — The telecommunications category, expressed as text.
/// - `TelecommunicationsServiceCategoryCode` — The telecommunications category, expressed as a code.
/// - `MovieTitle` — The title of a movie delivered via this telecommunications service.
/// - `RoamingPartnerName` — Statement of the roaming partner name.
/// - `PayPerView` — A pay-per-view delivered via this telecommunications service.
/// - `Quantity` — The number of calls.
/// - `TelecommunicationsServiceCall` — The telecommunications call described as a text
/// - `TelecommunicationsServiceCallCode` — The telecommunications call described as a code
/// - `CallBaseAmount` — The amount to be payed as the base for one call
/// - `CallExtensionAmount` — The amount to be payed for the call
/// - `Price` — The price for using the telecommunication service
/// - `Country` — The country to which the service is provided. In case of a telephone call it is the country where the receiver is located.
/// - `ExchangeRate` — A exchanges rates used in the pricing e.g.. when phone calls has crossed border lines.
/// - `AllowanceCharge` — An allowance or charge that applies to the UtilityStatement as a whole.
/// - `TaxTotal` — A total amount of taxes of a particular kind applicable to this telecommunications service.
/// - `CallDuty` — In the case of a telephone call, a duty on this call.
/// - `TimeDuty` — A duty on a consumption of time.
// pub struct TelecommunicationsService { ... }

/// A class describing the supply of a telecommunication service, e.g., providing telephone calls.
///
/// **UBL Dictionary Entry Name:** `Telecommunications Supply. Details`
///
/// Generated from XSD type `TelecommunicationsSupplyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `TelecommunicationsSupplyType` — The type of telecommunications supply, expressed as text.
/// - `TelecommunicationsSupplyTypeCode` — The type of telecommunications supply, expressed as a code.
/// - `PrivacyCode` — A code signifying the level of confidentiality of this information for this telecommunication supply.
/// - `Description` — Text describing the telecommunications supply.
/// - `TotalAmount` — The total amount associated with this telecommunications supply.
/// - `TelecommunicationsSupplyLine` — Outlines the provided telecommunication supply
// pub struct TelecommunicationsSupply { ... }

/// A class that outlines the telecommunication supply in details
///
/// **UBL Dictionary Entry Name:** `Telecommunications Supply Line. Details`
///
/// Generated from XSD type `TelecommunicationsSupplyLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this telecommunications supply line.
/// - `PhoneNumber` — The phone number used for this telecommunication supply line
/// - `Description` — The description of the telecommunication supply line
/// - `LineExtensionAmount` — An amount specifying the cost of this telecommunication line
/// - `TaxInclusiveLineExtensionAmount` — The total amount for this telecommunications supply line, including all allowances, charges and taxes.
/// - `ExchangeRate` — Exchanges rates used to calculate the amount for this line.
/// - `AllowanceCharge` — An allowance or charge that applies to this telecommunication supply line.
/// - `TaxTotal` — A total amount of taxes of a particular kind applicable to this telecommunications supply line
/// - `TelecommunicationsService` — A telecommunications service (e.g., a telephone call).
// pub struct TelecommunicationsSupplyLine { ... }

/// A class to describe a measurement of temperature.
///
/// **UBL Dictionary Entry Name:** `Temperature. Details`
///
/// Generated from XSD type `TemperatureType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `AttributeID` — An identifier for this temperature measurement.
/// - `Measure` — The value of this temperature measurement.
/// - `MeasureCode` — A code describing the temperature, when not expressed as a measure.
/// - `Description` — Text describing this temperature measurement.
// pub struct Temperature { ... }

/// A class to define a line in a Tender.
///
/// **UBL Dictionary Entry Name:** `Tender Line. Details`
///
/// Generated from XSD type `TenderLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this tender line.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `Quantity` — The quantity of the item quoted in this tender line.
/// - `LineExtensionAmount` — The total amount for this tender line, including allowance charges but net of taxes.
/// - `TaxInclusiveLineExtensionAmount` — The total amount for this tender line, including all allowances, charges and taxes.
/// - `TotalTaxAmount` — The total tax amount for this tender line.
/// - `OrderableUnit` — Text describing a unit in which the item described in this tender line can be ordered.
/// - `ContentUnitQuantity` — The unit of measure and quantity of the orderable unit.
/// - `OrderQuantityIncrementNumeric` — The number of items that can set the order quantity increments.
/// - `MinimumOrderQuantity` — The minimum number of items described in this tender line that can be ordered.
/// - `MaximumOrderQuantity` — The maximum number of items described in this tender line that can be ordered.
/// - `WarrantyInformation` — Text about a warranty (provided by WarrantyParty) for the good or service described in this tender line.
/// - `PackLevelCode` — A mutually agreed code signifying the level of packaging associated with the item described in this tender line.
/// - `DocumentReference` — A reference to a document associated with this tender line.
/// - `Item` — The item associated with this tender line.
/// - `OfferedItemLocationQuantity` — A set of location-specific properties (e.g., price, quantity, lead time) associated with the item described in this tender line.
/// - `ReplacementRelatedItem` — A catalogue item that may be a replacement for the item described in this tender line.
/// - `WarrantyParty` — The Party who is responsible for any warranty described with this Tender Line.
/// - `WarrantyValidityPeriod` — The period for which a warranty associated with the item described in this tender line is valid.
/// - `SubTenderLine` — An association to a Sub Tender Line
/// - `CallForTendersLineReference` — Reference to a Line on a Call For Tenders document.
/// - `CallForTendersDocumentReference` — One or more references to Call For Tenders documents.
// pub struct TenderLine { ... }

/// A class to describe directions for preparing a tender.
///
/// **UBL Dictionary Entry Name:** `Tender Preparation. Details`
///
/// Generated from XSD type `TenderPreparationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `TenderEnvelopeID` — An identifier for the tender envelope to be used with the tender.
/// - `TenderEnvelopeTypeCode` — A code signifying the type of tender envelope (economical or objective criteria versus technical or subjective criteria).
/// - `Description` — Text describing the tender envelope.
/// - `OpenTenderID` — An identifier for the open tender associated with this tender preparation.
/// - `ProcurementProjectLot` — The procurement project lot associated with a particular tenderer.
/// - `DocumentTenderRequirement` — A reference to the template for a required document in a tendering process.
/// - `TenderEncryptionData` — A reference to the details of the encryption process used for the tender.
// pub struct TenderPreparation { ... }

/// A template for a required document in a tendering process.
///
/// **UBL Dictionary Entry Name:** `Tender Requirement. Details`
///
/// Generated from XSD type `TenderRequirementType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Name` — A name of this tender requirement.
/// - `Description` — Text describing this tender requirement.
/// - `TemplateDocumentReference` — A reference to the template for a required document.
// pub struct TenderRequirement { ... }

/// A class to describe the awarding of a tender in a tendering process.
///
/// **UBL Dictionary Entry Name:** `Tender Result. Details`
///
/// Generated from XSD type `TenderResultType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `AwardID` — An identifier for this tender result.
/// - `TenderResultCode` — A code signifying the result of the tendering process.
/// - `Description` — Text describing the result of the tendering process.
/// - `AdvertisementAmount` — The monetary value of the advertisement for this tendering process.
/// - `AwardDate` — The date on which this result was formalized.
/// - `AwardTime` — The time at which this result was formalized.
/// - `ReceivedTenderQuantity` — The total number of tenders received in this tendering process.
/// - `LowerTenderAmount` — The least expensive tender received in the tendering process.
/// - `HigherTenderAmount` — The most expensive tender received in this tendering process.
/// - `StartDate` — The date on which the awarded contract begins.
/// - `ReceivedElectronicTenderQuantity` — The number of electronic tenders received.
/// - `ReceivedForeignTenderQuantity` — The number of foreing tenders received.
/// - `Contract` — A contract governing this tender result.
/// - `AwardedTenderedProject` — The awarded tendered project associated with this tender result.
/// - `ContractFormalizationPeriod` — The period during which a contract associated with the awarded project is to be formalized.
/// - `SubcontractTerms` — Subcontract terms for this tender result.
/// - `WinningParty` — A party that is identified as the awarded by a tender result.
// pub struct TenderResult { ... }

/// A class to describe a tendered project or project lot.
///
/// **UBL Dictionary Entry Name:** `Tendered Project. Details`
///
/// Generated from XSD type `TenderedProjectType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `VariantID` — An identifier for this variant of a tendered project.
/// - `FeeAmount` — The fee amount for tendered projects.
/// - `FeeDescription` — Text describing the fee amount for tendered projects.
/// - `TenderEnvelopeID` — An identifier for the tender envelope this tendered project belongs to.
/// - `TenderEnvelopeTypeCode` — A code signifying the type of tender envelope this tendered project belongs to.
/// - `AdditionalFee` — An additional fee for this tendered project.
/// - `ProcurementProjectLot` — The procurement project lot to which this Tender Line refers to. If there are no lots, this ought not be defined.
/// - `EvidenceDocumentReference` — A reference to a non-structured evidentiary document supporting this tendered project.
/// - `TaxTotal` — A total amount of taxes of a particular kind applicable to the monetary total for this tendered project.
/// - `LegalMonetaryTotal` — The total amount for this tendered project.
/// - `TenderLine` — A line in the tender for this tendered project.
/// - `AwardingCriterionResponse` — An association to an Awarding Criterion Response.
// pub struct TenderedProject { ... }

/// A class to describe the qualifications of a tenderer party.
///
/// **UBL Dictionary Entry Name:** `Tenderer Party Qualification. Details`
///
/// Generated from XSD type `TendererPartyQualificationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `InterestedProcurementProjectLot` — The procurement project lot the party is interested in.
/// - `MainQualifyingParty` — The qualifications of the main tenderer party.
/// - `AdditionalQualifyingParty` — The qualifications of a tenderer party other than the main tenderer party when bidding as a consortium.
// pub struct TendererPartyQualification { ... }

/// The evaluation that the Contracting Authority party requests to fulfill to the tenderers.
///
/// **UBL Dictionary Entry Name:** `Tenderer Qualification Request. Details`
///
/// Generated from XSD type `TendererQualificationRequestType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `CompanyLegalFormCode` — The legal status requested for potential tenderers, expressed as a code.
/// - `CompanyLegalForm` — The legal status requested for potential tenderers, expressed as text
/// - `PersonalSituation` — Text describing the personal situation of the economic operators in this tendering process.
/// - `OperatingYearsQuantity` — Textual description of the legal form required for potential tenderers.
/// - `EmployeeQuantity` — Textual description of the legal form required for potential tenderers.
/// - `Description` — Text describing the evaluation requirements for this tenderer.
/// - `RequiredBusinessClassificationScheme` — A classification scheme for the business profile.
/// - `TechnicalEvaluationCriterion` — A technical evaluation criterion required for an economic operator in a tendering process.
/// - `FinancialEvaluationCriterion` — A financial evaluation criterion required for an economic operator in a tendering process.
/// - `SpecificTendererRequirement` — A requirement to be met by a tenderer.
/// - `EconomicOperatorRole` — A class to describe the tenderer contracting role.
// pub struct TendererQualificationRequest { ... }

/// A class to describe an action or statement required of an economic operator participating in a tendering process.
///
/// **UBL Dictionary Entry Name:** `Tenderer Requirement. Details`
///
/// Generated from XSD type `TendererRequirementType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Name` — A name of this tenderer requirement.
/// - `TendererRequirementTypeCode` — A code signifying this requirement.
/// - `Description` — Text describing this requirement.
/// - `LegalReference` — The legal reference of the exclusion criterion.
/// - `SuggestedEvidence` — An item of evidence that ought to be submitted to satisfy this requirement.
// pub struct TendererRequirement { ... }

/// A class to describe an item of criterion support for representations of capabilities or the ability to meet tendering requirements, which an economic operator must provide for acceptance into a tendering process.
///
/// **UBL Dictionary Entry Name:** `Tendering Criterion. Details`
///
/// Generated from XSD type `TenderingCriterionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this item of criterion support.
/// - `CriterionTypeCode` — A code signifying the type of criterion.
/// - `Name` — The name of the criterion.
/// - `Description` — The textual description for this criterion.
/// - `WeightNumeric` — A weighting to provide for automatic scoring of the criterion.
/// - `FulfilmentIndicator` — An indication that this criterion has been fulfilled.
/// - `FulfilmentIndicatorTypeCode` — A code signifying how this criterion has been fulfilled.
/// - `EvaluationMethodTypeCode` — A code signifying the type of Evaluation.
/// - `WeightingConsiderationDescription` — The textual description of the Weighting Description
/// - `ProcurementProjectLotReference` — One or more lots to which the tendering criterion applies
/// - `CommodityClassification` — One or more classification to which this criterion applies
/// - `SubTenderingCriterion` — One or more tendering subcriteria.
/// - `Legislation` — The legislation reference for the criterion.
/// - `TenderingCriterionPropertyGroup` — The sets of properties that can be used to fulfil the tendering criterion.
// pub struct TenderingCriterion { ... }

/// A class to describe the criterion properties.
///
/// **UBL Dictionary Entry Name:** `Tendering Criterion Property. Details`
///
/// Generated from XSD type `TenderingCriterionPropertyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier to refer to the criterion property.
/// - `Name` — The name of the criterion property.
/// - `Description` — A description of the criterion property.
/// - `TypeCode` — A mutually agreed code signifying the type of the property.
/// - `ValueDataTypeCode` — The data type of the numeric value and any constraints on the data type metadata.
/// - `ValueUnitCode` — The unit of measure of the numeric value as a quantity or measure.
/// - `ValueCurrencyCode` — The currency of the numeric value as an amount.
/// - `ExpectedAmount` — The expected amount that the responder has to provide in the criterion response.
/// - `ExpectedID` — The expected identifier that the responder has to provide in the criterion response.
/// - `ExpectedIndicator` — The expected indicator (true or false) that the responder has to provide in the criterion response.
/// - `ExpectedCode` — The expected code that the responder has to provide in the criterion response.
/// - `ExpectedValueNumeric` — The expected value that the responder has to provide in the criterion response.
/// - `ExpectedDescription` — The description of the of the expected
/// - `ExpectedURI` — The expected URL that the responder has to provide in the criterion response.
/// - `MaximumAmount` — The maximum amount the response must have.
/// - `MinimumAmount` — The minimum amount the response must have.
/// - `MaximumValueNumeric` — The maximum value the response must have.
/// - `MinimumValueNumeric` — The minimum value the response must have.
/// - `MaximumQuantity` — The maximum quantity value the response must have.
/// - `MinimumQuantity` — The minimum quantity value the response must have.
/// - `TranslationTypeCode` — The type of Transation that the requirement will be translated for example certified translation
/// - `CertificationLevelDescription` — The description of the level of the expected certification
/// - `CopyQualityTypeCode` — The type of Copy quality, expressed as a code.
/// - `ApplicablePeriod` — The period to which this criterion property will apply.
/// - `TemplateEvidence` — An evidence that can be used to meet this criterion property.
// pub struct TenderingCriterionProperty { ... }

/// A class to describe a group of tendering criteria
///
/// **UBL Dictionary Entry Name:** `Tendering Criterion Property Group. Details`
///
/// Generated from XSD type `TenderingCriterionPropertyGroupType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for the group of criteria.
/// - `Name` — The name of the group.
/// - `Description` — The textual description for this group.
/// - `PropertyGroupTypeCode` — A code signifying the type of the property group
/// - `FulfilmentIndicator` — An indication that this group of criteria have been fulfilled.
/// - `FulfilmentIndicatorTypeCode` — A code signifying how this group of criteria have been fulfilled.
/// - `TenderingCriterionProperty` — All the criteria properties comprising the tendering criterion.
/// - `SubsidiaryTenderingCriterionPropertyGroup` — Subsidiary tendering criteria groups comprising this tendering criterion.
// pub struct TenderingCriterionPropertyGroup { ... }

/// A class to describe a response to a criterion property.
///
/// **UBL Dictionary Entry Name:** `Tendering Criterion Response. Details`
///
/// Generated from XSD type `TenderingCriterionResponseType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this criterion property response.
/// - `Name` — The name of the criterion property response
/// - `Description` — A description of the criterion response
/// - `ValidatedCriterionPropertyID` — An identifier for this item of criterion support.
/// - `ConfidentialityLevelCode` — A code specifying the confidentiality level of the response to this criterion.
/// - `ResponseValue` — The criterion requirement property values.
/// - `ApplicablePeriod` — The period to which this criterion property response applies.
/// - `EvidenceSupplied` — (Deprecated) A reference to the evidence supporting this criterion property response.
/// - `SuppliedEvidence` — A reference to the Evidence supporting this criterion property response.
/// - `ProcurementProjectLotReference` — One or more lots to which the criterion response applies
/// - `CommodityClassification` — One or more classification to which this criterion response applies
// pub struct TenderingCriterionResponse { ... }

/// A class to describe the process of a formal offer and response to execute work or supply goods at a stated price.
///
/// **UBL Dictionary Entry Name:** `Tendering Process. Details`
///
/// Generated from XSD type `TenderingProcessType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this tendering process.
/// - `OriginalContractingSystemID` — When reopening a tendering process, the identifier of the original framework agreement or dynamic purchasing system.
/// - `Description` — Text describing the tendering process.
/// - `NegotiationDescription` — Text describing the negotiation to be followed during the tendering process.
/// - `ProcedureCode` — A code signifying the type of this tendering procedure.
/// - `UrgencyCode` — A code signifying the urgency of this tendering process.
/// - `ExpenseCode` — A code signifying the type of expense for this tendering process.
/// - `PartPresentationCode` — A code signifying the type of presentation of tenders required (e.g., one lot, multiple lots, or all the lots).
/// - `ContractingSystemCode` — A code signifying the type of contracting system (e.g., framework agreement, dynamic purchasing system). If the procedure is individual (nonrepetitive), this code ought to be omitted.
/// - `SubmissionMethodCode` — A code signifying the method to be followed in submitting tenders.
/// - `CandidateReductionConstraintIndicator` — An indicator that the number of candidates participating in this process has been reduced (true) or not (false).
/// - `GovernmentAgreementConstraintIndicator` — An indicator that the project associated with this tendering process is constrained by a government procurement agreement (true) or not (false).
/// - `AccessToolsURI` — The URI where the tools for electronic communication related with the tendering process can be found.
/// - `TerminatedIndicator` — An indicator that the competition launched is terminated.
/// - `DocumentAvailabilityPeriod` — The period during which documents relating to this tendering process must be completed.
/// - `TenderSubmissionDeadlinePeriod` — The period during which tenders must be delivered.
/// - `InvitationSubmissionPeriod` — The period during which invitations to tender must be completed and delivered.
/// - `ParticipationInvitationPeriod` — The period during which the invitation to participate must be sent.
/// - `ParticipationRequestReceptionPeriod` — The period during which requests for participation must be completed and delivered.
/// - `AdditionalInformationRequestPeriod` — The period during which additional information about the procurement can be requested.
/// - `NoticeDocumentReference` — A reference to a notice pertaining to this tendering process.
/// - `AdditionalDocumentReference` — A reference to an additional document.
/// - `ProcessJustification` — A justification for the selection of this tendering process.
/// - `EconomicOperatorShortList` — A set of criteria used to create a short list of candidates.
/// - `OpenTenderEvent` — An Event specifying the location and time of the public opening of tenders.
/// - `AuctionTerms` — The terms to be fulfilled by tenderers if an auction is to be executed before the awarding of a tender.
/// - `FrameworkAgreement` — A tendering framework agreement.
/// - `ContractingSystem` — A reference to a contracting system. Only when the procedure is repetitive.
// pub struct TenderingProcess { ... }

/// A class to describe tendering terms for a tendering process.
///
/// **UBL Dictionary Entry Name:** `Tendering Terms. Details`
///
/// Generated from XSD type `TenderingTermsType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `AwardingMethodTypeCode` — A code signifying the awarding method in a tendering process (e.g., a method favoring the tender with the lowest price or the tender that is most economically advantageous).
/// - `PriceEvaluationCode` — Textual description of the legal form required for potential tenderers.
/// - `MaximumVariantQuantity` — Maximum number of variants the tenderer is allowed to present for this tendering project.
/// - `VariantConstraintIndicator` — An indicator that variants are allowed and unconstrained in number (true) or not allowed (false).
/// - `AcceptedVariantsDescription` — Text specifying the things for which variants are accepted.
/// - `VariantConstraintCode` — A code signifying the modalities for a tenderer to submit variants of tenders.
/// - `PriceRevisionFormulaDescription` — Text describing the formula for price revision.
/// - `FundingProgramCode` — The program that funds the tendering process (e.g., "National", "European"), expressed as a code.
/// - `FundingProgram` — The program that funds the tendering process (e.g., EU 6th Framework Program) expressed as text.
/// - `MaximumAdvertisementAmount` — The maximum advertised monetary value of the tendering process.
/// - `Note` — Free-form text conveying information that is not contained explicitly in other structures.
/// - `PaymentFrequencyCode` — A code signifying the frequency of payment in the contract associated with the tendering process.
/// - `EconomicOperatorRegistryURI` — The Uniform Resource Identifier (URI) of an electronic registry of economic operators.
/// - `RequiredCurriculaIndicator` — An indicator that tenderers are required to provide a curriculum vitae for each participant in the project (true) or are not so required (false).
/// - `RequiredCurriculaCode` — A code signifying the conditions applying for tenderers to provide a curriculum vitae.
/// - `OtherConditionsIndicator` — Indicates whether other conditions exist (true) or not (false). If the indicator is true, the description may be provided.
/// - `RecurringProcurementIndicator` — Indicates whether the procurement is recurring (true) or not (false).
/// - `RecurringProcurementDescription` — Any additional information about recurrence (e.g. estimated timing).
/// - `EstimatedTimingFurtherPublication` — The description of the estimated timing for further notices to be published.
/// - `AdditionalConditions` — Other existing conditions.
/// - `LatestSecurityClearanceDate` — The end date until which the candidates can obtain the necessary level of security clearance.
/// - `DocumentationFeeAmount` — The amount to be paid to obtain the contract documents and additional documentation.
/// - `MultipleTendersCode` — A code signifying whether a tenderer is allowed to submit multiple tenders.
/// - `PenaltyClause` — The penalty clauses
/// - `RequiredFinancialGuarantee` — A financial guarantee of a tenderer or bid submitter's actual entry into a contract in the event that it is the successful bidder.
/// - `ProcurementLegislationDocumentReference` — A reference to a document providing references to procurement legislation applicable to the tendering process.
/// - `FiscalLegislationDocumentReference` — A reference to a document providing references to fiscal legislation applicable to the tendering process.
/// - `EnvironmentalLegislationDocumentReference` — A reference to a document providing references to environmental legislation applicable to the tendering process.
/// - `EmploymentLegislationDocumentReference` — A reference to a document providing references to employment legislation applicable to the tendering process.
/// - `ContractualDocumentReference` — A reference to a document that will become part of the awarded contract.
/// - `CallForTendersDocumentReference` — A reference to a Call for Tender associated with these tendering terms.
/// - `WarrantyValidityPeriod` — The period during which a warranty for work, service, or goods associated with these tendering terms is valid.
/// - `PaymentTerms` — A specification of payment terms associated with the tendering process.
/// - `TendererQualificationRequest` — Required set of qualifications for a tenderer in this tendering process.
/// - `AllowedSubcontractTerms` — Subcontract terms for the tendering process.
/// - `TenderPreparation` — Directions for preparing a tender for the+D2057 tendering process.
/// - `ContractExecutionRequirement` — A requirement relating to execution of the contract that will be awarded as a result of the tendering process.
/// - `AwardingTerms` — The terms in the tendering process for awarding the contract for a project.
/// - `AdditionalInformationParty` — The Party who has additional information about the tendering process.
/// - `DocumentProviderParty` — The Party who has the Contract Documents for the tendering process.
/// - `TenderRecipientParty` — The Party who receives the Tenders.
/// - `ContractResponsibleParty` — The Party who executes the Contract.
/// - `TenderEvaluationParty` — The Buyer Party who evaluates the Tenders received.
/// - `QualificationRequestRecipientParty` — The Buyer Party who receives the Qualification Request.
/// - `TenderValidityPeriod` — The period during which tenders submitted for this tendering process must remain valid.
/// - `ContractAcceptancePeriod` — The period of time during which the contracting authority may accept a contract.
/// - `AppealTerms` — Information about the terms to present for an appeal against a tender award.
/// - `Language` — One of the default languages specified for the tendering process.
/// - `BudgetAccountLine` — A budget account line associated with the tendering process.
/// - `ReplacedNoticeDocumentReference` — A class defining a reference to the notice that is being replaced.
/// - `LotDistribution` — List of specific ways to tender to the lots of the procurement project.
/// - `PostAwardProcess` — Information about the post-award process.
/// - `EconomicOperatorShortList` — A set of criteria used to create a short list of candidates.
/// - `SecurityClearanceTerm` — Information about the terms to present for a security clearance.
// pub struct TenderingTerms { ... }

/// A class to describe a trade financing instrument.
///
/// **UBL Dictionary Entry Name:** `Trade Financing. Details`
///
/// Generated from XSD type `TradeFinancingType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this trade financing instrument.
/// - `FinancingInstrumentCode` — A code signifying the type of this financing instrument.
/// - `ContractDocumentReference` — A reference to a contract document.
/// - `DocumentReference` — A reference to a document associated with this trade financing instrument.
/// - `FinancingParty` — A Party that provides funds or credit to support the trade transaction described in this Trade Financing instrument.
/// - `FinancingFinancialAccount` — An internal bank account used by the bank or its first agent to manage the line of credit granted to the financing requester.
/// - `Clause` — A clause applicable to this trade financing instrument.
// pub struct TradeFinancing { ... }

/// A class for describing the terms of a trade agreement.
///
/// **UBL Dictionary Entry Name:** `Trading Terms. Details`
///
/// Generated from XSD type `TradingTermsType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Information` — Text describing the terms of a trade agreement.
/// - `Reference` — A reference quoting the basis of the terms
/// - `ApplicableAddress` — The address at which these trading terms apply.
// pub struct TradingTerms { ... }

/// A class to describe purchasing, sales, or payment conditions.
///
/// **UBL Dictionary Entry Name:** `Transaction Conditions. Details`
///
/// Generated from XSD type `TransactionConditionsType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for conditions of the transaction, typically purchase/sales conditions.
/// - `ActionCode` — A code signifying a type of action relating to sales or payment conditions.
/// - `Description` — Text describing the transaction conditions.
/// - `DocumentReference` — A document associated with these transaction conditions.
// pub struct TransactionConditions { ... }

/// A class to describe a piece of equipment used to transport goods.
///
/// **UBL Dictionary Entry Name:** `Transport Equipment. Details`
///
/// Generated from XSD type `TransportEquipmentType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this piece of transport equipment.
/// - `ReferencedConsignmentID` — An identifier for the consignment contained by this piece of transport equipment.
/// - `TransportEquipmentTypeCode` — A code signifying the type of this piece of transport equipment.
/// - `ProviderTypeCode` — A code signifying the type of provider of this piece of transport equipment.
/// - `OwnerTypeCode` — A code signifying the type of owner of this piece of transport equipment.
/// - `SizeTypeCode` — A code signifying the size and type of this piece of piece of transport equipment. When the piece of transport equipment is a shipping container, it is recommended to use ContainerSizeTypeCode for validation.
/// - `DispositionCode` — A code signifying the current disposition of this piece of transport equipment.
/// - `FullnessIndicationCode` — A code signifying whether this piece of transport equipment is full, partially full, or empty.
/// - `RefrigerationOnIndicator` — An indicator that this piece of transport equipment's refrigeration is on (true) or off (false).
/// - `Information` — Additional information about this piece of transport equipment.
/// - `ReturnabilityIndicator` — An indicator that this piece of transport equipment is returnable (true) or not (false).
/// - `LegalStatusIndicator` — An indication of the legal status of this piece of transport equipment with respect to the Container Convention Code.
/// - `AirFlowPercent` — The percent of the airflow within this piece of transport equipment.
/// - `HumidityPercent` — The percent humidity within this piece of transport equipment.
/// - `AnimalFoodApprovedIndicator` — An indicator that this piece of transport equipment is approved for animal food (true) or not (false).
/// - `HumanFoodApprovedIndicator` — An indicator that this piece of transport equipment is approved for human food (true) or not (false).
/// - `DangerousGoodsApprovedIndicator` — An indicator that this piece of transport equipment is approved for dangerous goods (true) or not (false).
/// - `RefrigeratedIndicator` — An indicator that this piece of transport equipment is refrigerated (true) or not (false).
/// - `Characteristics` — Characteristics of this piece of transport equipment.
/// - `DamageRemarks` — Damage associated with this piece of transport equipment.
/// - `Description` — Text describing this piece of transport equipment.
/// - `SpecialTransportRequirements` — Special transport requirements expressed as text.
/// - `GrossWeightMeasure` — The gross weight of this piece of transport equipment.
/// - `GrossVolumeMeasure` — The gross volume of this piece of transport equipment.
/// - `TareWeightMeasure` — The weight of this piece of transport equipment when empty.
/// - `TrackingDeviceCode` — A code signifying the tracking device for this piece of transport equipment.
/// - `PowerIndicator` — An indicator that this piece of transport equipment can supply power (true) or not (false).
/// - `TraceID` — An identifier for use in tracing this piece of transport equipment, such as the EPC number used in RFID.
/// - `StowagePositionID` — The Stowage Position identifier for this piece of carried logistics Transport Equipment.
/// - `MeasurementDimension` — A measurable dimension (length, mass, weight, or volume) of this piece of transport equipment.
/// - `TransportEquipmentSeal` — A seal securing the door of a piece of transport equipment.
/// - `MinimumTemperature` — In the case of a refrigeration unit, the minimum allowable operating temperature for this container.
/// - `MaximumTemperature` — In the case of a refrigeration unit, the maximum allowable operating temperature for this container.
/// - `ProviderParty` — The Party who provides this piece of Transport Equipment.
/// - `LoadingProofParty` — The authorised Party who certifies the load of the Goods into this piece of Transport Equipment.
/// - `SupplierParty` — The party that supplies this piece of transport equipment.
/// - `OwnerParty` — The Party who owns this Piece of Transport Equipment.
/// - `OperatingParty` — The Party who operates this piece of Transport Equipment.
/// - `LoadingLocation` — The location where this piece of transport equipment is loaded.
/// - `UnloadingLocation` — The location where this piece of transport equipment is unloaded.
/// - `StorageLocation` — The location where this piece of transport equipment is being stored.
/// - `PositioningTransportEvent` — A positioning of this piece of transport equipment.
/// - `QuarantineTransportEvent` — A quarantine of this piece of transport equipment.
/// - `DeliveryTransportEvent` — A delivery of this piece of transport equipment.
/// - `PickupTransportEvent` — A pickup of this piece of transport equipment.
/// - `HandlingTransportEvent` — A handling of this piece of transport equipment.
/// - `LoadingTransportEvent` — A loading of this piece of transport equipment.
/// - `TransportEvent` — An additional transport event not specified elsewhere in this Transport Equipment.
/// - `ApplicableTransportMeans` — The applicable transport means associated with this piece of transport equipment.
/// - `HaulageTradingTerms` — A set of haulage trading terms associated with this piece of transport equipment.
/// - `HazardousGoodsTransit` — Transit-related information regarding a type of hazardous goods contained in this piece of transport equipment.
/// - `PackagedTransportHandlingUnit` — A packaged transport handling unit associated with this piece of transport equipment.
/// - `ServiceAllowanceCharge` — A service allowance charge associated with this piece of transport equipment.
/// - `FreightAllowanceCharge` — A freight allowance charge associated with this piece of transport equipment.
/// - `AttachedTransportEquipment` — A piece of transport equipment attached to this piece of transport equipment.
/// - `Delivery` — The delivery of this piece of transport equipment.
/// - `Pickup` — The pickup of this piece of transport equipment.
/// - `Despatch` — The despatch of this piece of transport equipment.
/// - `ShipmentDocumentReference` — A reference to a shipping document associated with this piece of transport equipment.
/// - `ContainedInTransportEquipment` — A piece of transport equipment contained in this piece of transport equipment.
/// - `Package` — A package contained in this piece of transport equipment.
/// - `GoodsItem` — A goods item contained in this piece of transport equipment.
/// - `VerifiedGrossMass` — The verified gross mass of this piece of transport equipment.
/// - `LoadedHazardousItem` — Hazardous items loaded into this transport equipment
// pub struct TransportEquipment { ... }

/// A class to describe a device (a transport equipment seal) for securing the doors of a shipping container.
///
/// **UBL Dictionary Entry Name:** `Transport Equipment Seal. Details`
///
/// Generated from XSD type `TransportEquipmentSealType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this transport equipment seal.
/// - `SealIssuerTypeCode` — A code signifying the type of party that issues and is responsible for this transport equipment seal.
/// - `Condition` — The condition of this transport equipment seal.
/// - `SealStatusCode` — A code signifying the condition of this transport equipment seal.
/// - `SealingPartyType` — The role of the sealing party.
// pub struct TransportEquipmentSeal { ... }

/// A class to describe a significant occurrence or happening related to the transportation of goods.
///
/// **UBL Dictionary Entry Name:** `Transport Event. Details`
///
/// Generated from XSD type `TransportEventType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `IdentificationID` — An identifier for this transport event within an agreed event identification scheme.
/// - `OccurrenceDate` — The date of this transport event.
/// - `OccurrenceTime` — The time of this transport event.
/// - `TransportEventTypeCode` — A code signifying the type of this transport event.
/// - `Description` — Text describing this transport event.
/// - `CompletionIndicator` — An indicator that this transport event has been completed (true) or not (false).
/// - `ReportedShipment` — The shipment involved in this transport event.
/// - `CurrentStatus` — The current status of this transport event.
/// - `ResponsibleParty` — The Party reponsible for this Transport Event.
/// - `Contact` — A contact associated with this transport event.
/// - `Location` — The location associated with this transport event.
/// - `Signature` — A signature that can be used to sign for an entry or an exit at a transport location (e.g., port terminal).
/// - `Period` — A period of time associated with this transport event.
// pub struct TransportEvent { ... }

/// A class to describe terms applying to a transport execution plan.
///
/// **UBL Dictionary Entry Name:** `Transport Execution Terms. Details`
///
/// Generated from XSD type `TransportExecutionTermsType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `TransportUserSpecialTerms` — Text describing special terms specified by the transport user.
/// - `TransportServiceProviderSpecialTerms` — Text describing special terms specified by the transport service provider.
/// - `ChangeConditions` — Text describing conditions applying to a change of these transport execution terms.
/// - `PaymentTerms` — Payment terms associated with the transportation service.
/// - `DeliveryTerms` — Delivery terms (e.g., Incoterms) associated with the transportation service.
/// - `BonusPaymentTerms` — Terms relating to payment of applicable bonuses associated with the transport service.
/// - `CommissionPaymentTerms` — Terms of payment applying to a commission specified in the transport execution plan.
/// - `PenaltyPaymentTerms` — Terms of payment applying to a penalty specified in the transport execution plan.
/// - `EnvironmentalEmission` — An environmental emission resulting from the transportation service.
/// - `NotificationRequirement` — A notification requirement related to the transportation service; e.g., a requirement that the transport user will be notified when goods are ready for pickup.
/// - `ServiceChargePaymentTerms` — Payment terms for the service charge associated with the transport service.
// pub struct TransportExecutionTerms { ... }

/// A class to describe a uniquely identifiable unit consisting of one or more packages, goods items, or pieces of transport equipment.
///
/// **UBL Dictionary Entry Name:** `Transport Handling Unit. Details`
///
/// Generated from XSD type `TransportHandlingUnitType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this transport handling unit.
/// - `TransportHandlingUnitTypeCode` — A code signifying the type of this transport handling unit.
/// - `HandlingCode` — The handling required for this transport handling unit, expressed as a code.
/// - `HandlingInstructions` — The handling required for this transport handling unit, expressed as text.
/// - `HazardousRiskIndicator` — An indicator that the materials contained in this transport handling unit are subject to an international regulation concerning the carriage of dangerous goods (true) or not (false).
/// - `TotalGoodsItemQuantity` — The total number of goods items in this transport handling unit.
/// - `TotalPackageQuantity` — The total number of packages in this transport handling unit.
/// - `DamageRemarks` — Text describing damage associated with this transport handling unit.
/// - `ShippingMarks` — Text describing the marks and numbers on this transport handling unit.
/// - `TraceID` — An identifier for use in tracing this transport handling unit, such as the EPC number used in RFID.
/// - `HandlingUnitDespatchLine` — A despatch line associated with this transport handling unit.
/// - `ActualPackage` — A package contained in this transport handling unit.
/// - `ReceivedHandlingUnitReceiptLine` — A receipt line associated with this transport handling unit.
/// - `TransportEquipment` — A piece of transport equipment associated with this transport handling unit.
/// - `TransportMeans` — A means of transport associated with this transport handling unit.
/// - `HazardousGoodsTransit` — Transit-related information regarding a type of hazardous goods contained in this transport handling unit.
/// - `MeasurementDimension` — A measurable dimension (length, mass, weight, or volume) of this transport handling unit.
/// - `MinimumTemperature` — The minimum required operating temperature of this transport handling unit.
/// - `MaximumTemperature` — The maximum allowable operating temperature of this transport handling unit.
/// - `GoodsItem` — A goods item contained in this transport handling unit.
/// - `FloorSpaceMeasurementDimension` — The floor space measurement dimension associated with this transport handling unit.
/// - `PalletSpaceMeasurementDimension` — The pallet space measurement dimension associated to this transport handling unit.
/// - `ShipmentDocumentReference` — A reference to a shipping document associated with this transport handling unit.
/// - `Status` — The status of this transport handling unit.
/// - `CustomsDeclaration` — Describes identifiers or references relating to customs procedures.
/// - `ReferencedShipment` — A shipment associated with this transport handling unit.
/// - `Package` — A package contained in this transport handling unit.
/// - `DamageDocumentationAttachment` — An attachment, such as a photo, documenting damage associated with this transport handling unit.
/// - `EnergyConsumptionAllocation` — An allocation of energy consumption and associated emissions attributable to the handling or transport of this unit.
// pub struct TransportHandlingUnit { ... }

/// A class to describe a particular vehicle or vessel used for the conveyance of goods or persons.
///
/// **UBL Dictionary Entry Name:** `Transport Means. Details`
///
/// Generated from XSD type `TransportMeansType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `JourneyID` — An identifier for the regular service schedule of this means of transport.
/// - `RegistrationNationalityID` — An identifier for the country in which this means of transport is registered.
/// - `RegistrationNationality` — Text describing the country in which this means of transport is registered.
/// - `DirectionCode` — A code signifying the direction of this means of transport.
/// - `TransportMeansTypeCode` — A code signifying the type of this means of transport (truck, vessel, etc.).
/// - `TradeServiceCode` — A code signifying the service regularly provided by the carrier operating this means of transport.
/// - `Stowage` — The location within the means of transport where goods are to be or have been stowed.
/// - `AirTransport` — An aircraft used for transport.
/// - `RoadTransport` — A vehicle used for road transport.
/// - `RailTransport` — Equipment used for rail transport.
/// - `MaritimeTransport` — A vessel used for transport by water (not only by sea).
/// - `OwnerParty` — The Party who owns these Means of Transport.
/// - `MeasurementDimension` — A measurable dimension (length, mass, weight, or volume) of this means of transport.
// pub struct TransportMeans { ... }

/// Describes the location and schedule relating to a transport means.
///
/// **UBL Dictionary Entry Name:** `Transport Schedule. Details`
///
/// Generated from XSD type `TransportScheduleType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `SequenceNumeric` — A number indicating the order of this status in the sequence in which statuses are to be presented.
/// - `ReferenceDate` — The reference date for the transport schedule status.
/// - `ReferenceTime` — The reference time for the transport schedule status.
/// - `ReliabilityPercent` — The reliability of the transport schedule status, expressed as a percentage.
/// - `Remarks` — Remarks related to the transport schedule status.
/// - `StatusLocation` — The location for which status is reported.
/// - `ActualArrivalTransportEvent` — The actual arrival at a location.
/// - `ActualDepartureTransportEvent` — The actual departure from a location.
/// - `EstimatedDepartureTransportEvent` — An estimated departure from a specified location.
/// - `EstimatedArrivalTransportEvent` — An estimated arrival at a specified location.
/// - `PlannedDepartureTransportEvent` — The planned departure from a specified location.
/// - `PlannedArrivalTransportEvent` — The planned arrival at a specified location.
// pub struct TransportSchedule { ... }

/// A class to describe one segment or leg in a transportation service.
///
/// **UBL Dictionary Entry Name:** `Transportation Segment. Details`
///
/// Generated from XSD type `TransportationSegmentType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `SequenceNumeric` — A number indicating the order of this segment in the sequence of transportation segments making up a transportation service.
/// - `TransportExecutionPlanReferenceID` — An identifier for the transport execution plan governing this transportation segment.
/// - `TransportationService` — The transportation service used in this transportation segment.
/// - `TransportServiceProviderParty` — The Transport Service Provider who is reponsible for the Transportation Service in this Transportation Segment.
/// - `ReferencedConsignment` — A consignment referenced in this transportation segment. Such a consignment may have different identifiers than the consignment identifiers being used in the transportation service agreed between the transport user and the transport service provider.
/// - `ShipmentStage` — The shipment stage associated with this transportation segment.
// pub struct TransportationSegment { ... }

/// A class to describe a transportation service.
///
/// **UBL Dictionary Entry Name:** `Transportation Service. Details`
///
/// Generated from XSD type `TransportationServiceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `TransportServiceCode` — A code signifying the extent of this transportation service (e.g., door-to-door, port-to-port).
/// - `TariffClassCode` — A code signifying the tariff class applicable to this transportation service.
/// - `Priority` — The priority of this transportation service.
/// - `FreightRateClassCode` — A code signifying the rate class for freight in this transportation service.
/// - `TransportationServiceDescription` — Text describing this transportation service.
/// - `TransportationServiceDetailsURI` — The Uniform Resource Identifier (URI) of a document providing additional details regarding this transportation service.
/// - `NominationDate` — In a transport contract, the deadline date by which this transportation service has to be booked. For example, if this service is scheduled for Wednesday 16 February 2011 at 10 a.m. CET, the nomination date might be Tuesday15 February 2011.
/// - `NominationTime` — In a transport contract, the deadline time by which this transportation service has to be booked. For example, if this service is scheduled for Wednesday 16 February 2011 at 10 a.m. CET, the nomination date might be Tuesday15 February 2011 and the nomination time 4 p.m. at the latest.
/// - `Name` — The name of this transportation service.
/// - `SequenceNumeric` — A number indicating the order of this transportation service in a sequence of transportation services.
/// - `TransportEquipment` — A piece of transport equipment used in this transportation service.
/// - `SupportedTransportEquipment` — A piece of transport equipment supported in this transportation service.
/// - `UnsupportedTransportEquipment` — A piece of transport equipment that is not supported in this transportation service.
/// - `CommodityClassification` — A classification of this transportation service.
/// - `SupportedCommodityClassification` — A classification (e.g., general cargo) for commodities that can be handled in this transportation service.
/// - `UnsupportedCommodityClassification` — A classification for commodities that cannot be handled in this transportation service.
/// - `TotalCapacityDimension` — The total capacity or volume available in this transportation service.
/// - `ShipmentStage` — One or more stages of shipment in this transportation service.
/// - `TransportEvent` — One or more transport events taking place in this transportation service.
/// - `ResponsibleTransportServiceProviderParty` — The Transport Service Provider who is reponsible for this Transportation Service.
/// - `EnvironmentalEmission` — An environmental emission resulting from this transportation service.
/// - `EstimatedDurationPeriod` — The estimated duration of this transportation service.
/// - `ScheduledServiceFrequency` — A class to specify which day of the week a transport service is operational.
// pub struct TransportationService { ... }

/// A simplified version of the Price class intended for applications such as telephone billing.
///
/// **UBL Dictionary Entry Name:** `Unstructured Price. Details`
///
/// Generated from XSD type `UnstructuredPriceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `PriceAmount` — The price amount.
/// - `TaxInclusivePriceAmount` — The price amount inclusive of all taxes.
/// - `TimeAmount` — The usage time upon which the price is based.
// pub struct UnstructuredPrice { ... }

/// A class to describe the consumption of a utility product.
///
/// **UBL Dictionary Entry Name:** `Utility Item. Details`
///
/// Generated from XSD type `UtilityItemType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this utility item.
/// - `SubscriberID` — An identifier for the subscriber to the utility.
/// - `SubscriberType` — Identification of the subscriber type, expressed as text..
/// - `SubscriberTypeCode` — The code identifying for the service type.
/// - `Description` — Text describing the consumption product.
/// - `PackQuantity` — The unit packaging quantity.
/// - `PackSizeNumeric` — The number of items in a pack.
/// - `ConsumptionType` — The type of product consumed, expressed as text.
/// - `ConsumptionTypeCode` — The type of product consumed, expressed as a code.
/// - `CurrentChargeType` — Information of the actual payments type for the utility Item
/// - `CurrentChargeTypeCode` — Information of the actual payments type code expressed as a code
/// - `OneTimeChargeType` — Information about the one-time payment type in case everything is paid One time
/// - `OneTimeChargeTypeCode` — Information about the one-time payment type code
/// - `TaxCategory` — The tax category applicable to this utility item.
/// - `Contract` — A contract setting forth conditions applicable to this utility item.
// pub struct UtilityItem { ... }

/// A class to describe a verified gross mass (VGM) measure and its documentation.
///
/// **UBL Dictionary Entry Name:** `Verified Gross Mass. Details`
///
/// Generated from XSD type `VerifiedGrossMassType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this mass measure.
/// - `WeighingDate` — The weighing date.
/// - `WeighingTime` — The weighing time.
/// - `WeighingMethodCode` — A code signifying the weighing method used (e.g. according the SOLAS Convention).
/// - `WeighingDeviceID` — An identifier for the weighing device used for executing the weight measurement.
/// - `WeighingDeviceType` — Text describing the weighing device type used for executing the weight measurement.
/// - `GrossMassMeasure` — The total verified gross mass of a packed container which includes the cargo weight, block and bracing materials and container tare.
/// - `WeighingParty` — The Party who executes the weight measure.
/// - `ShipperParty` — The Party who is reponsible of the Verified Gross Mass (VGM) according to the SOLAS Convention. This Party plays the role of the Shipper (BCO, FF or NVOCC).
/// - `ResponsibleParty` — The Party who signs the Verified Gross Mass (VGM) on behalf of the Shipper.
/// - `DocumentReference` — A reference to the VGM documentary evidence.
// pub struct VerifiedGrossMass { ... }

/// A class to describe the dynamics of a vesssel.
///
/// **UBL Dictionary Entry Name:** `Vessel Dynamics. Details`
///
/// Generated from XSD type `VesselDynamicsType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `NavigationStatusCode` — A code specifying the navigation status for the vessel.
/// - `AtAnchorageIndicator` — Indicates whether the vessel is at anchor.
/// - `CourseOverGroundDirection` — Text describing the actual direction of progress of a vessel, between two points, in relation to the surface of the earth.
/// - `SpeedOverGroundMeasure` — Text describing the speed of the vessel relative to the surface of the earth.
/// - `RateOfTurnMeasure` — Text describing the rate at which the vessel is turning.
// pub struct VesselDynamics { ... }

/// A class to describe a visit to a port located in a geographical area considered an “affected area” by the World Health Organization (WHO).
///
/// **UBL Dictionary Entry Name:** `WHO Affected Area Visit. Details`
///
/// Generated from XSD type `WHOAffectedAreaVisitType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `VisitDate` — The date that the WHO Affected Area was visited.
/// - `WHOAffectedAreaPortLocation` — The location of the port of the visited WHO Affected Area.
// pub struct WHOAffectedAreaVisit { ... }

/// A class to describe a quantity of waste generated by an item, process, or activity, including type, quantity, and optional lifecycle context.
///
/// **UBL Dictionary Entry Name:** `Waste Generated. Details`
///
/// Generated from XSD type `WasteGeneratedType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `WasteTypeCode` — A code identifying the type of waste generated.
/// - `WasteTypeDescription` — A text describing the type of waste generated.
/// - `WasteMeasure` — A measurement of the amount of waste generated.
/// - `MeasurementPeriod` — The period during which this waste was generated or measured.
// pub struct WasteGenerated { ... }

/// A class to describe a web site.
///
/// **UBL Dictionary Entry Name:** `Web Site. Details`
///
/// Generated from XSD type `WebSiteType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for a specific web site.
/// - `Name` — The common name of the web site.
/// - `Description` — Text describing the web site.
/// - `WebSiteTypeCode` — A code that specifies the type web site.
/// - `URI` — The Uniform Resource Identifier (URI) of the web site; i.e., its Uniform Resource Locator (URL).
/// - `WebSiteAccess` — Access information for the website (e.g. guest credentials).
// pub struct WebSite { ... }

/// A class to describe access to a web site.
///
/// **UBL Dictionary Entry Name:** `Web Site Access. Details`
///
/// Generated from XSD type `WebSiteAccessType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `URI` — The Uniform Resource Identifier (URI) for this web site; i.e., its Uniform Resource Locator (URL).
/// - `Password` — A password to the web site.
/// - `Login` — Text describing login details.
// pub struct WebSiteAccess { ... }

/// A party that is identified as the awarded by a tender result.
///
/// **UBL Dictionary Entry Name:** `Winning Party. Details`
///
/// Generated from XSD type `WinningPartyType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Rank` — Indicates the rank obtained in the award.
/// - `Party` — Information about an organization, sub-organization, or individual fulfilling a role in a business process.
// pub struct WinningParty { ... }

/// A class that refers to a phase of work. Used for instance to specify what part of the contract the billing is referring to.
///
/// **UBL Dictionary Entry Name:** `Work Phase Reference. Details`
///
/// Generated from XSD type `WorkPhaseReferenceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this phase of work.
/// - `WorkPhaseCode` — A code signifying this phase of work.
/// - `WorkPhase` — Text describing this phase of work.
/// - `ProgressPercent` — The progress percentage of the work phase.
/// - `StartDate` — The date on which this phase of work begins.
/// - `EndDate` — The date on which this phase of work ends.
/// - `WorkOrderDocumentReference` — A reference to a document regarding the work order for the project in which this phase of work takes place.
// pub struct WorkPhaseReference { ... }

/// A class to define a document-level total of reported work expressed as a quantity.
///
/// **UBL Dictionary Entry Name:** `Work Quantity Total. Details`
///
/// Generated from XSD type `WorkQuantityTotalType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `Quantity` — The total quantity for this entry.
/// - `WorkTypeCode` — A code specifying the kind of work quantity being totaled.
/// - `WorkTypeDescription` — A description of what is totaled.
// pub struct WorkQuantityTotal { ... }

/// A class to describe a line in a Work Report, specifying the work performed.
///
/// **UBL Dictionary Entry Name:** `Work Report Line. Details`
///
/// Generated from XSD type `WorkReportLineType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `ID` — An identifier for this Work Report Line.
/// - `WorkItemID` — An identifier for the specific work item to which this line relates.
/// - `WorkItemDescription` — A description of the specific work item to which this line relates.
/// - `Quantity` — The quantity of work reported in this line.
/// - `LineExtensionAmount` — The total amount for this work report line, excluding taxes.
/// - `TaxInclusiveLineExtensionAmount` — The total amount for this work report line, including taxes.
/// - `CompletionPercent` — The overall percentage of completion for the work item represented by this line.
/// - `ActivityOriginLocation` — The Location from which the work in this Work Report Line originated or was performed.
/// - `Period` — The Period during which the work described in this Work Report Line was performed.
/// - `PerformingParty` — The Party performing the work in this Work Report Line.
/// - `WorkPhaseReference` — A reference to the Work Phase to which this Work Report Line relates.
/// - `DocumentReference` — A reference to an external document relevant to this Work Report Line.
/// - `Price` — The price applicable to this Work Report Line.
/// - `TaxTotal` — A total amount of taxes of a particular kind applicable to this Work Report Line.
/// - `SubWorkReportLine` — A subsidiary Work Report Line related to this Work Report Line.
// pub struct WorkReportLine { ... }
