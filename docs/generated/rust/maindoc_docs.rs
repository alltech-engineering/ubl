//! Auto-generated documentation from UBL 2.5 XSD annotations.
//! 101 types from maindoc.

/// A document to indicate the application's response to a transaction. This may be a business response initiated by a user or a technical response sent automatically by an application.
///
/// **UBL Dictionary Entry Name:** `Application Response. Details`
///
/// Generated from XSD type `ApplicationResponseType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `ResponseDate` — The date on which the information in the response was created.
/// - `ResponseTime` — The time at which the information in the response was created.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Identifies the current version of this document.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Document.
/// - `ReceiverParty` — The Party who receives this Document.
/// - `DocumentResponse` — A response to a document.
// pub struct ApplicationResponse { ... }

/// (Deprecated) A wrapper that allows a document of any kind to be packaged with the UBL document that references it.
///
/// **UBL Dictionary Entry Name:** `Attached Document. Details`
///
/// Generated from XSD type `AttachedDocumentType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `DocumentTypeCode` — A code signifying the type of document.
/// - `DocumentType` — Text specifying the type of document.
/// - `ParentDocumentID` — The Identifier of the parent document.
/// - `ParentDocumentTypeCode` — A code signifying the type of parent document.
/// - `ParentDocumentVersionID` — Indicates the current version of the referred document.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Document.
/// - `ReceiverParty` — The Party who receives this Document.
/// - `Attachment` — An attachment containing the document content.
/// - `ParentDocumentLineReference` — A reference to a line in the attached document.
// pub struct AttachedDocument { ... }

/// The document used to communicate a contract award to the winner.
///
/// **UBL Dictionary Entry Name:** `Awarded Notification. Details`
///
/// Generated from XSD type `AwardedNotificationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `ContractName` — The name, expressed as text, of this procurement project.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `SenderParty` — The Party who sends this Document.
/// - `ReceiverParty` — The Party who receives this Document.
/// - `MinutesDocumentReference` — A reference to a set of minutes associated with this award.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `TenderResult` — The result of the tendering process reported in this notification.
/// - `FinalFinancialGuarantee` — A bond guarantee by the submitter of a tender or bid, required of the tender winner.
/// - `Signature` — A signature applied to this document.
// pub struct AwardedNotification { ... }

/// A document issued by the party who acts as an agent for a transportation carrier or other agents to the party who gives instructions for the transportation services (shipper, consignor, etc.) stating the details of the transportation, charges, and terms and conditions under which the transportation service is provided. The party issuing this document does not necessarily provide the physical transportation service. The information in the Bill of Lading corresponds to the information on the Forwarding Instructions. It is used for any mode of transport. A Bill of Lading can serve as a contractual document between the parties for the transportation service. The document evidences a contract of carriage by sea and the acceptance of responsibility for the goods by the carrier, by which the carrier undertakes to deliver the goods against surrender of the document. A provision in the document that the goods are to be delivered to the order of a named person, or to order, or to bearer, constitutes such an undertaking.
///
/// **UBL Dictionary Entry Name:** `Bill Of Lading. Details`
///
/// Generated from XSD type `BillOfLadingType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CarrierAssignedID` — Reference number (such as a booking reference number) assigned by a carrier or its agent to identify a specific shipment when cargo space is reserved prior to loading.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Name` — Text, assigned by the sender, that identifies this document to business users.
/// - `Description` — Textual description of the document instance.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `DocumentStatusCode` — A code signifying the status of the Bill Of Lading (revision, replacement, etc.).
/// - `ShippingOrderID` — Reference number to identify a Shipping Order or Forwarding Instruction.
/// - `ToOrderIndicator` — Indicates whether the transport document is consigned to order.
/// - `AdValoremIndicator` — A term used in commerce in reference to certain duties, called ad valorem duties, which are levied on commodities at certain rates per centum on their value.
/// - `DeclaredCarriageValueAmount` — Value declared by the shipper or his agent solely for the purpose of varying the carrier's level of liability from that provided in the contract of carriage in case of loss or damage to goods or delayed delivery.
/// - `OtherInstruction` — Other free-text instructions to the forwarders or carriers related to the shipment. This element ought to be used only where such information cannot be represented in other structured information entities within the document.
/// - `ConsignorParty` — The Party who is reponsible for sending the goods.
/// - `CarrierParty` — The Party who provides the transport of goods between named points.
/// - `FreightForwarderParty` — The Party who combines individual smaller consignments into a single larger shipment (a so-called consolidated consignment or shipment) which is sent to a counterpart who mirrors the consolidator's activity by dividing the consolidated consignment into its original components.
/// - `Shipment` — An identifiable collection of one or more goods items to be transported between the seller party and the buyer party.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `ExchangeRate` — Information that directly relates to the rate of exchange (conversion) between two currencies.
/// - `DocumentDistribution` — A list of interested parties to whom this document is distributed.
/// - `Signature` — A signature applied to this document.
// pub struct BillOfLading { ... }

/// A document used to provide information about a business party and its business capabilities.
///
/// **UBL Dictionary Entry Name:** `Business Card. Details`
///
/// Generated from XSD type `BusinessCardType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `VersionID` — Identifies the current version of this business card.
/// - `PreviousVersionID` — Identifies the previous version of this business card.
/// - `BriefDescription` — Textual description of the document instance.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Business Card. This Party may be the owner of this Business Card or a third-Party who acts on behalf of the owner (e.g. business network).
/// - `ReceiverParty` — The Party who receives this Business Card.
/// - `BusinessParty` — The Party who owns this Business Card.
/// - `BrochureDocumentReference` — A reference to a company brochure document.
/// - `AdditionalDocumentReference` — A reference to an additional document (e.g. presentations).
/// - `BusinessCapability` — The business capabilities of the party.
// pub struct BusinessCard { ... }

/// A document used to provide information about a business.
///
/// **UBL Dictionary Entry Name:** `Business Information. Details`
///
/// Generated from XSD type `BusinessInformationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Identifies the current version of this Business Registration Information.
/// - `PreviousVersionID` — Identifies the previous version of this Business Registration Information.
/// - `BriefDescription` — Textual description of the document instance.
/// - `RequestedPublicationDate` — The requested publication date for this Business Registration Information Notice.
/// - `RegulatoryDomain` — Information about the law that defines the regulatory domain.
/// - `NoticeTypeCode` — The type of notice.
/// - `NoticeLanguageCode` — The language used for this notice.
/// - `AdditionalNoticeLanguage` — An additional official language used in this document.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Business Registration Information. This Party may be the owner of this Business Registration Information or a third-Party who acts on behalf of the owner (e.g. business network).
/// - `ReceiverParty` — The Party who receives this Business Registration Information.
/// - `BusinessParty` — The Party who owns this Business Registration Information.
/// - `BrochureDocumentReference` — A reference to a company brochure document.
/// - `AdditionalDocumentReference` — A reference to an additional document (e.g. presentations).
/// - `BusinessCapability` — The business capabilities of the party.
/// - `BusinessPartyGroup` — A group of Business Parties.
/// - `OperationType` — The type of operation for which this document is created.
/// - `NoticeSubType` — The subtype of this notice.
// pub struct BusinessInformation { ... }

/// A document used by a Contracting Party to define a procurement project to buy goods, services, or works during a specified period.
///
/// **UBL Dictionary Entry Name:** `Call For Tenders. Details`
///
/// Generated from XSD type `CallForTendersType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `ApprovalDate` — The date, assigned by the contracting party, on which the Call For Tenders was approved.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Indicates the current version of the Call for Tenders.
/// - `PreviousVersionID` — Identifies the previous version of the Call for Tenders which is superceded by this version.
/// - `LegalDocumentReference` — A reference to a legal document.
/// - `TechnicalDocumentReference` — A reference to a technical document.
/// - `RequiredDocumentReference` — A reference to a required document.
/// - `ProvidedDocumentReference` — A reference to a provided document.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `ContractingParty` — The contracting party or parties in case of joint procurement.
/// - `OriginatorCustomerParty` — The party who originated Order.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `ReceiverParty` — The Party who receives this Document.
/// - `TenderingTerms` — The tendering terms associated with this tendering process.
/// - `TenderingProcess` — A description of the tendering process itself.
/// - `ProcurementProject` — An overall definition of this procurement project.
/// - `ProcurementProjectLot` — One of the procurement project lots into which this contract can be split.
// pub struct CallForTenders { ... }

/// A document that describes items, prices, and price validity.
///
/// **UBL Dictionary Entry Name:** `Catalogue. Details`
///
/// Generated from XSD type `CatalogueType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ActionCode` — A code signifying whether the transaction is a replacement or an update.
/// - `Name` — Text, assigned by the sender, that identifies this document to business users.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `RevisionDate` — The date, assigned by the seller party, on which the information in the Catalogue was last revised.
/// - `RevisionTime` — The time, assigned by the Seller party, at which the information in the Catalogue was last revised.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `Description` — Textual description of the document instance.
/// - `VersionID` — An identifier for the current version of the Catalogue.
/// - `PreviousVersionID` — An identifier for the previous version of the Catalogue that is superseded by this version.
/// - `LineCountNumeric` — The number of Catalogue Lines in the document.
/// - `ValidityPeriod` — A period, assigned by the seller, during which the information in the Catalogue is effective. This may be given as start and end dates or as a duration.
/// - `ReferencedContract` — A contract or framework agreement with which this Catalogue is associated.
/// - `SourceCatalogueReference` — A reference to the source catalogue.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `ApplicableTerritoryAddress` — A geographic or territorial area to which the Catalogue as a whole applies.
/// - `Signature` — A signature applied to this document.
/// - `ProviderParty` — The Party who provides this Catalogue.
/// - `ReceiverParty` — The Party who receives this Catalogue.
/// - `SellerSupplierParty` — The seller.
/// - `ContractorCustomerParty` — The customer party responsible for the contracts with which the Catalogue is associated.
/// - `TradingTerms` — The trading terms associated with this Catalogue.
/// - `CatalogueLine` — A line in a Catalogue describing an item of sale.
// pub struct Catalogue { ... }

/// A document used to cancel an entire Catalogue.
///
/// **UBL Dictionary Entry Name:** `Catalogue Deletion. Details`
///
/// Generated from XSD type `CatalogueDeletionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `Name` — Text, assigned by the sender, that identifies this document to business users.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `EffectiveDate` — The effective date, assigned by the seller, on which the Catalogue expires.
/// - `EffectiveTime` — The effective time, assigned by the seller, at which the Catalogue expires.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Identifies the current version of the Catalogue.
/// - `Description` — Textual description of the document instance.
/// - `ValidityPeriod` — The period during which the Deletion of the catalogue becomes effective. This may be given as start (after date) and end dates (before date).
/// - `DeletedCatalogueReference` — A reference to the Catalogue being deleted.
/// - `ReferencedContract` — A contract or framework agreement with which the Catalogue was associated.
/// - `Signature` — A signature applied to this document.
/// - `ReceiverParty` — The Party who receives the Catalogue Deletion.
/// - `ProviderParty` — The Party who sends the Catalogue Deletion.
/// - `SellerSupplierParty` — The seller.
/// - `ContractorCustomerParty` — The customer party responsible for the contracts with which the Catalogue was associated.
// pub struct CatalogueDeletion { ... }

/// A document used to update information (e.g., technical descriptions and properties) about Items in an existing Catalogue.
///
/// **UBL Dictionary Entry Name:** `Catalogue Item Specification Update. Details`
///
/// Generated from XSD type `CatalogueItemSpecificationUpdateType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the subset of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `Name` — Text, assigned by the sender, that identifies this document to business users.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `RevisionDate` — The date, assigned by the seller, on which the Catalogue was revised.
/// - `RevisionTime` — The time, assigned by the seller, at which the Catalogue was revised.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `Description` — Textual description of the document instance.
/// - `VersionID` — Identifies the current version of the Catalogue.
/// - `LineCountNumeric` — The number of Catalogue Item Specification Update Lines in this document.
/// - `ValidityPeriod` — A period, assigned by the seller, during which the information in the Catalogue Revision is effective. This may be given as start and end dates or as a duration.
/// - `RelatedCatalogueReference` — A reference to the Catalogue being updated.
/// - `ReferencedContract` — A contract or framework agreement with which the Catalogue is associated.
/// - `Signature` — A signature applied to this document.
/// - `ProviderParty` — The Party who sends the Catalogue Item Specification Update.
/// - `ReceiverParty` — The Party who receives the Catalogue Item Specification Update.
/// - `SellerSupplierParty` — The seller.
/// - `ContractorCustomerParty` — The customer party responsible for the contracts with which the Catalogue is associated.
/// - `TradingTerms` — The trading terms associated with the Catalogue.
/// - `DefaultLanguage` — The default language for the item specifications.
/// - `CatalogueItemSpecificationUpdateLine` — One or more lines in the Catalogue Item Specification Update, each line updating a specific catalogue item.
// pub struct CatalogueItemSpecificationUpdate { ... }

/// A document used to update information about prices in an existing Catalogue.
///
/// **UBL Dictionary Entry Name:** `Catalogue Pricing Update. Details`
///
/// Generated from XSD type `CataloguePricingUpdateType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the subset of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `Name` — Text, assigned by the sender, that identifies this document to business users.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `RevisionDate` — The date, assigned by the seller, on which the Catalogue was revised.
/// - `RevisionTime` — The time, assigned by the seller, at which the Catalogue was revised.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `Description` — Describes the Catalogue Revision.
/// - `VersionID` — Indicates the current version of the catalogue.
/// - `LineCountNumeric` — The number of lines in the document.
/// - `ValidityPeriod` — A period, assigned by the seller, during which the information in the Catalogue Revision is effective. This may be given as start and end dates or as a duration.
/// - `RelatedCatalogueReference` — A reference to the Catalogue being updated.
/// - `ReferencedContract` — A contract or framework agreement with which the Catalogue is associated.
/// - `Signature` — A signature applied to this document.
/// - `ProviderParty` — The Party who sends the Catalogue Pricing Update.
/// - `ReceiverParty` — The Party who receives the Catalogue Pricing Update.
/// - `SellerSupplierParty` — The seller.
/// - `ContractorCustomerParty` — The customer party responsible for the contracts with which the Catalogue is associated.
/// - `TradingTerms` — The trading terms associated with the Catalogue.
/// - `DefaultLanguage` — The default language for the catalogue pricing update.
/// - `CataloguePricingUpdateLine` — One or more lines in the Catalogue Pricing Update, each line updating a specific catalogue item.
// pub struct CataloguePricingUpdate { ... }

/// A document used to request a Catalogue.
///
/// **UBL Dictionary Entry Name:** `Catalogue Request. Details`
///
/// Generated from XSD type `CatalogueRequestType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `Name` — Text, assigned by the sender, that identifies this document to business users.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `Description` — Textual description of the document instance.
/// - `PricingUpdateRequestIndicator` — Indicates a request for a pricing update.
/// - `ItemUpdateRequestIndicator` — Indicates a request for an update of the item specifications.
/// - `LineCountNumeric` — The number of Catalogue Lines in this document.
/// - `ValidityPeriod` — The period, assigned by the Catalogue Managing party, during which the information in the Catalogue requested is to be effective. This may be given as start and end dates or a duration.
/// - `Signature` — A signature applied to this document.
/// - `ReceiverParty` — The Party who receives the Catalogue Request.
/// - `ProviderParty` — The Party who sends the Catalogue Request.
/// - `SellerSupplierParty` — The seller.
/// - `ContractorCustomerParty` — The customer party responsible for the contracts with which the Catalogue is associated.
/// - `RequestedCatalogueReference` — A reference to a specific Catalogue; used if the Catalogue Request is for an update.
/// - `ReferencedContract` — A contract or framework agreement with which the Catalogue being requested is associated.
/// - `TradingTerms` — The trading terms associated with the requested Catalogue.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `ApplicableTerritoryAddress` — A reference to a territory (region, country, city, etc.) to which the requested Catalogue will apply, expressed as an Address.
/// - `RequestedLanguage` — The language in which the Catalogue is requested to be provided.
/// - `RequestedClassificationScheme` — A requested classification scheme for the requested Catalogue.
/// - `CatalogueRequestLine` — An association to specific Catalogue Lines for the catalogue requested.
// pub struct CatalogueRequest { ... }

/// A document that describes the Certificate of Origin.
///
/// **UBL Dictionary Entry Name:** `Certificate Of Origin. Details`
///
/// Generated from XSD type `CertificateOfOriginType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Description` — Textual description of the document instance.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Identifies the version of this Certificate of Origin.
/// - `Signature` — A signature applied to this document.
/// - `ExporterParty` — The Party who makes the export declaration, or on whose behalf the export declaration is made, and who is the owner of the goods or has similar right of disposal over them at the time when the declaration is accepted.
/// - `ImporterParty` — The Party who imports the goods, or on whose behalf the goods are being imported.
/// - `EndorserParty` — The Party providing the endorsement.
/// - `CertificateOfOriginApplication` — Details of the application for a Certificate of Origin.
/// - `IssuerEndorsement` — Issuer Endorsement details.
/// - `EmbassyEndorsement` — Embassy Endorsement details.
/// - `InsuranceEndorsement` — Insurance Endorsement details.
// pub struct CertificateOfOrigin { ... }

/// A common document used for reporting transport related issues to authorities or regulators.
///
/// **UBL Dictionary Entry Name:** `Common Transportation Report. Details`
///
/// Generated from XSD type `CommonTransportationReportType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `DocumentStatusCode` — A code signifying the status of this Common Transportation Report with respect to its original state.
/// - `ReportTypeCode` — A code signifying the type of report being provided
/// - `ReportType` — A text that identifies the type of report to business users.
/// - `Description` — Textual description of this document instance.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Identifies a version of a common transportation report in order to distinguish updates.
/// - `ReporterParty` — The Party who provides this Common Transportation Report.
/// - `AuthorityParty` — The Party who receives the Common Transportation Report. This Party is normally an Authority or regulator.
/// - `SenderParty` — The Party who sends this Report.
/// - `ReceiverParty` — The Party who receives this Report.
/// - `ReportingLocation` — A location to which this common transportation report applies.
/// - `Shipment` — A shipment to which this common transportation report applies.
/// - `TransportMeans` — A means of transport used in relation to this common transportation report.
/// - `DocumentReference` — A reference to a document relevant for or associated with this common transportation report.
/// - `Signature` — A signature applied to this document.
// pub struct CommonTransportationReport { ... }

/// A document published by a Contracting Party to announce the awarding of a procurement project.
///
/// **UBL Dictionary Entry Name:** `Contract Award Notice. Details`
///
/// Generated from XSD type `ContractAwardNoticeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — An identifier of the current version of the Contract Award Notice.
/// - `PreviousVersionID` — An identifier of the previous version of the Contract Award Notice which is superceded by this version.
/// - `RequestedPublicationDate` — The requested publication date for this Contract Award Notice.
/// - `RegulatoryDomain` — Information about the law that defines the regulatory domain.
/// - `NoticeTypeCode` — The type of notice (CAN general, CAN social, Design)
/// - `PublishAwardIndicator` — An indicator specifying if the notice is published for service contracts within certain service categories (true) or not (false).
/// - `NoticeLanguageCode` — The language used for this contract award notice.
/// - `AdditionalNoticeLanguage` — An additional official language used in this contract award notice.
/// - `PreviousDocumentReference` — A reference to a previously sent document.
/// - `MinutesDocumentReference` — A reference to a set of minutes.
/// - `Signature` — A signature applied to this document.
/// - `ContractingParty` — The contracting party or parties in case of joint procurement.
/// - `OriginatorCustomerParty` — The party who originated Order.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `ReceiverParty` — The Party who receives this Document.
/// - `TenderingTerms` — The tendering terms associated with this tendering process.
/// - `TenderingProcess` — A description of the tendering process itself.
/// - `ProcurementProject` — An overall definition of this procurement project.
/// - `ProcurementProjectLot` — One of the procurement project lots into which this contract can be split.
/// - `TenderResult` — A result of the bid opening in the tendering process.
// pub struct ContractAwardNotice { ... }

/// A document used by a Contracting party to announce a project to buy goods, services, or works.
///
/// **UBL Dictionary Entry Name:** `Contract Notice. Details`
///
/// Generated from XSD type `ContractNoticeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — An identifier of the current version of the Contract Notice.
/// - `PreviousVersionID` — An identifier of the previous version of the Contract Notice which is superceded by this version.
/// - `RequestedPublicationDate` — The requested publication date for this Contract Notice.
/// - `RegulatoryDomain` — Information about the law that defines the regulatory domain.
/// - `NoticeTypeCode` — The type of notice (PIN, Qualification, Reduce time...)
/// - `NoticeLanguageCode` — The language used for this contract notice.
/// - `AdditionalNoticeLanguage` — An additional official language used in this contract notice.
/// - `FrequencyPeriod` — The estimated frequency of future notices.
/// - `Signature` — A signature applied to this document.
/// - `ContractingParty` — The contracting party or parties in case of joint procurement.
/// - `OriginatorCustomerParty` — A party who originally requested the tender.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `ReceiverParty` — The Party who receives this Document.
/// - `TenderingTerms` — The tendering terms associated with this tendering process.
/// - `TenderingProcess` — A description of the tendering process itself.
/// - `ProcurementProject` — An overall definition of this procurement project.
/// - `ProcurementProjectLot` — One of the procurement project lots into which this contract can be split.
// pub struct ContractNotice { ... }

/// A document used to specify credits due to the Debtor from the Creditor.
///
/// **UBL Dictionary Entry Name:** `Credit Note. Details`
///
/// Generated from XSD type `CreditNoteType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `DueDate` — The date on which this Credit Note is due.
/// - `TaxPointDate` — The date of the Credit Note, used to indicate the point at which tax becomes applicable.
/// - `CreditNoteTypeCode` — A code signifying the type of the Credit Note.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `DocumentCurrencyCode` — A code signifying the default currency for this document.
/// - `TaxCurrencyCode` — A code signifying the currency used for tax amounts in the Credit Note.
/// - `PricingCurrencyCode` — A code signifying the currency used for prices in the Credit Note.
/// - `PaymentCurrencyCode` — A code signifying the currency used for payment in the Credit Note.
/// - `PaymentAlternativeCurrencyCode` — A code signifying the alternative currency used for payment in the Credit Note.
/// - `AccountingCostCode` — The buyer's accounting code, applied to the Credit Note as a whole.
/// - `AccountingCost` — The buyer's accounting code, applied to the Credit Note as a whole, expressed as text.
/// - `LineCountNumeric` — The number of Credit Note Lines in the document.
/// - `BuyerReference` — (Deprecated) A reference provided by the buyer used for internal routing of the document.
/// - `DefaultLanguageCode` — A code signifying the default natural language used by the sender for human-readable textual content that does not include a languageID.
/// - `InvoicePeriod` — Associates the Credit Note with Invoicing Periods rather than with a specific Invoice.
/// - `DiscrepancyResponse` — A reason for the Credit Note as a whole.
/// - `OrderReference` — The Order associated with this Credit Note.
/// - `BillingReference` — A reference to a billing document associated with this document.
/// - `DespatchDocumentReference` — A reference to a Despatch Advice associated with this document.
/// - `DeliveryNoteDocumentReference` — A reference to a Delivery Note associated with this document.
/// - `WorkReportDocumentReference` — A reference to a Work Report associated with this document.
/// - `ReceiptDocumentReference` — A reference to a Receipt Advice associated with this document.
/// - `ContractDocumentReference` — A reference to a contract associated with this document.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `StatementDocumentReference` — A reference to a Statement associated with this document.
/// - `OriginatorDocumentReference` — A reference to an originator document associated with this document.
/// - `ProjectReference` — A reference to a project associated with this document.
/// - `BuyerAssignedReference` — A reference provided by the buyer used for internal routing of the document.
/// - `PurchaseReference` — A reference to an object, such as a subscription number, telephone number, meter, vehicle, person, etc., to which this Credit Note relates.
/// - `Annotation` — A structured annotation providing contextual or explanatory information related to this Credit Note.
/// - `Signature` — A signature applied to this document.
/// - `AccountingSupplierParty` — The accounting supplier party.
/// - `AccountingCustomerParty` — The accounting customer party.
/// - `PayeeParty` — The Party who receives the Payment.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `TaxRepresentativeParty` — The Party authorized to act as the Tax Representative for this Credit Note.
/// - `Delivery` — A delivery associated with this document.
/// - `DeliveryTerms` — A set of delivery terms associated with this document.
/// - `PaymentMeans` — Expected means of payment.
/// - `PaymentTerms` — A set of payment terms associated with this document.
/// - `TaxExchangeRate` — The exchange rate between the document currency and the tax currency.
/// - `PricingExchangeRate` — The exchange rate between the document currency and the pricing currency.
/// - `PaymentExchangeRate` — The exchange rate between the document currency and the payment currency.
/// - `PaymentAlternativeExchangeRate` — The exchange rate between the document currency and the payment alternative currency.
/// - `AllowanceCharge` — A discount or charge that applies to a price component.
/// - `TaxTotal` — The total amount of a specific type of tax.
/// - `WithholdingTaxTotal` — The total withholding tax.
/// - `LegalMonetaryTotal` — The total amount payable on the Credit Note, including allowances, charges, and taxes.
/// - `CollectionCreditNoteLine` — A line describing an item or amount collected on behalf of a third party.
/// - `CreditNoteLine` — A Credit Note line.
// pub struct CreditNote { ... }

/// A document used to specify debits due to the Creditor from the Debtor.
///
/// **UBL Dictionary Entry Name:** `Debit Note. Details`
///
/// Generated from XSD type `DebitNoteType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `DueDate` — The date on which this Debit Note is due.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `DebitNoteTypeCode` — A code signifying the type of the Debit Note.
/// - `TaxPointDate` — The date of the Debit Note, used to indicate the point at which tax becomes applicable.
/// - `DocumentCurrencyCode` — A code signifying the default currency for this document.
/// - `TaxCurrencyCode` — A code signifying the currency used for tax amounts in the Debit Note.
/// - `PricingCurrencyCode` — A code signifying the currency used for prices in the Debit Note.
/// - `PaymentCurrencyCode` — A code signifying the currency used for payment in the Debit Note.
/// - `PaymentAlternativeCurrencyCode` — A code signifying the alternative currency used for payment in the Debit Note.
/// - `AccountingCostCode` — The Buyer's accounting code, applied to the Credit Note as a whole.
/// - `AccountingCost` — The Buyer's accounting code, applied to the Credit Note as a whole, expressed as text.
/// - `LineCountNumeric` — The number of Debit Note Lines in this document.
/// - `DefaultLanguageCode` — A code signifying the default natural language used by the sender for human-readable textual content that does not include a languageID.
/// - `InvoicePeriod` — A period (rather than a specific invoice) associated with this document.
/// - `DiscrepancyResponse` — A reason for the Debit Note as a whole.
/// - `OrderReference` — A reference to an Order with which this Debit Note is associated.
/// - `BillingReference` — A reference to a billing document associated with this document.
/// - `DespatchDocumentReference` — A reference to a Despatch Advice associated with this document.
/// - `DeliveryNoteDocumentReference` — A reference to a Delivery Note associated with this document.
/// - `WorkReportDocumentReference` — A reference to a Work Report associated with this document.
/// - `ReceiptDocumentReference` — A reference to a Receipt Advice associated with this document.
/// - `StatementDocumentReference` — A reference to a Statement associated with this document.
/// - `OriginatorDocumentReference` — A reference to an originator document associated with this document.
/// - `ContractDocumentReference` — A reference to a contract associated with this document.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `ProjectReference` — A reference to a project associated with this document.
/// - `BuyerAssignedReference` — A reference provided by the buyer used for internal routing of the document.
/// - `PurchaseReference` — A reference to an object, such as a subscription number, telephone number, meter, vehicle, person, etc., to which this Debit Note relates.
/// - `Annotation` — A structured annotation providing contextual or explanatory information related to this Debit Note.
/// - `Signature` — A signature applied to this document.
/// - `AccountingSupplierParty` — The accounting supplier party.
/// - `AccountingCustomerParty` — The accounting customer party.
/// - `PayeeParty` — The Party who receives the Payment.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `TaxRepresentativeParty` — The Party authorized to act as the Tax Representative for this Debit Note.
/// - `PrepaidPayment` — A prepaid payment.
/// - `AllowanceCharge` — A discount or charge that applies to a price component.
/// - `Delivery` — A delivery associated with this document.
/// - `DeliveryTerms` — A set of delivery terms associated with this document.
/// - `PaymentMeans` — Expected means of payment.
/// - `PaymentTerms` — A set of payment terms associated with this document.
/// - `TaxExchangeRate` — The exchange rate between the document currency and the tax currency.
/// - `PricingExchangeRate` — The exchange rate between the document currency and the pricing currency.
/// - `PaymentExchangeRate` — The exchange rate between the document currency and the payment currency.
/// - `PaymentAlternativeExchangeRate` — The exchange rate between the document currency and the payment alternative currency.
/// - `TaxTotal` — The total amount of a specific type of tax.
/// - `WithholdingTaxTotal` — The total withholding tax.
/// - `RequestedMonetaryTotal` — The total amount payable on the Debit Note, including allowances, charges, and taxes.
/// - `LegalMonetaryTotal` — The total amount payable on the Debit Note, including allowances, charges, and taxes.
/// - `CollectionDebitNoteLine` — A line describing an item or amount collected on behalf of a third party.
/// - `DebitNoteLine` — A Debit Note line.
// pub struct DebitNote { ... }

/// A document confirming the actual delivery of goods or services, detailing delivered items, quantities, and shipment information.
///
/// **UBL Dictionary Entry Name:** `Delivery Note. Details`
///
/// Generated from XSD type `DeliveryNoteType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `DocumentStatusCode` — A code signifying the status of the Delivery Note with respect to its original state, e.g., original, revised, or cancelled.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `AccountingCostCode` — The accounting cost centre, applied to the Delivery Note as a whole, expressed as a code.
/// - `AccountingCost` — The accounting cost centre, applied to the Delivery Note as a whole, expressed as text.
/// - `LineCountNumeric` — The number of lines in this Delivery Note.
/// - `OrderReference` — A reference to an Order with which this Delivery Note is associated.
/// - `ProjectReference` — A reference to a project with which this Delivery Note is associated.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `DespatchSupplierParty` — The Party that executed the delivery.
/// - `DeliveryCustomerParty` — The Party to whom the goods or services were delivered.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `OriginatorCustomerParty` — A customer party as originator.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `Shipment` — The shipment.
/// - `DespatchLine` — A Despatch Line associated with a kind of item delivered.
// pub struct DeliveryNote { ... }

/// A document used to describe the despatch or delivery of goods and services.
///
/// **UBL Dictionary Entry Name:** `Despatch Advice. Details`
///
/// Generated from XSD type `DespatchAdviceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `DocumentStatusCode` — A code signifying the status of the Despatch Advice with respect to its original state. This code may be used if the document precedes the event and is subsequently found to be incorrect and in need of cancellation or revision.
/// - `DespatchAdviceTypeCode` — A code signifying the type of the Despatch Advice.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `AccountingCostCode` — The accounting cost centre, applied to the Despatch Advice as a whole, expressed as a code.
/// - `AccountingCost` — The accounting cost centre, applied to the Despatch Advice as a whole, expressed as text.
/// - `LineCountNumeric` — The number of Despatch Lines in this document.
/// - `OrderReference` — A reference to an Order with which this Despatch Advice is associated.
/// - `ProjectReference` — A reference to a project with which this Despatch Advice is associated.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `DespatchSupplierParty` — The despatch party.
/// - `DeliveryCustomerParty` — The delivery recipient.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `OriginatorCustomerParty` — A customer party as originator.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `Shipment` — The shipment.
/// - `DespatchLine` — A Despatch Line associated with a kind of item delivered.
// pub struct DespatchAdvice { ... }

/// A document used to support business parties agreeing on a set of digital processes, terms and conditions to ensure interoperability.
///
/// **UBL Dictionary Entry Name:** `Digital Agreement. Details`
///
/// Generated from XSD type `DigitalAgreementType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `AgreementTypeCode` — A code signifying the type of digital agreement (e.g. bi-lateral, multi-lateral).
/// - `VersionID` — Identifies the current version of this digital agreement.
/// - `PreviousVersionID` — Identifies the previous version of this digital agreement.
/// - `RequiredResponseMessageLevelCode` — A code signifying the minimum response message level the parties are required to provide (e.g. EESPA response message level).
/// - `Signature` — A signature applied to this document.
/// - `GovernorParty` — The Party who governs the Agreement (e.g. a multi-lateral Digital Agreement).
/// - `ParticipantParty` — The business parties agreeing on a set of digital processes, terms and conditions to ensure interoperability.
/// - `AgreementCountry` — The country to which this digital agreement applies.
/// - `RequiredCertificationDocumentReference` — A reference to a certification document required by this digital agreement.
/// - `DigitalAgreementTerms` — A reference to digital agreement terms and conditions.
/// - `DigitalProcess` — The digital processes in scope of this digital agreement.
// pub struct DigitalAgreement { ... }

/// A document used to provide information about a business party and its digital trade capabilities.
///
/// **UBL Dictionary Entry Name:** `Digital Capability. Details`
///
/// Generated from XSD type `DigitalCapabilityType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `VersionID` — Identifies the current version of party's digital capabilities.
/// - `PreviousVersionID` — Identifies the previous version of party's digital capabilities.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends these Digital Capabilities. This Party may be the owner of these Digital Capabilities or a third-Party who acts on behalf of the owner (e.g. service provider).
/// - `ReceiverParty` — The Party who receives these Digital Capabilities.
/// - `BusinessParty` — The Party who owns these Digital Capabilities.
/// - `DigitalProcess` — The digital trade processes supported by the party.
// pub struct DigitalCapability { ... }

/// A document used to provide information about the status of a collaboration/process associated with a document.
///
/// **UBL Dictionary Entry Name:** `Document Status. Details`
///
/// Generated from XSD type `DocumentStatusType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Document.
/// - `ReceiverParty` — The Party who receives this Document.
/// - `DocumentResponse` — A response to the document.
/// - `AdditionalDocumentResponse` — A document linked or related to the document for which the status was requested.
// pub struct DocumentStatus { ... }

/// A document used to request the status of a collaboration/process associated with a document.
///
/// **UBL Dictionary Entry Name:** `Document Status Request. Details`
///
/// Generated from XSD type `DocumentStatusRequestType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `TrackingID` — An identifier for tracking status of the business process .
/// - `RequestedDocumentReference` — The document about which status is requested.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Document.
/// - `ReceiverParty` — The Party who receives this Document.
// pub struct DocumentStatusRequest { ... }

/// A document sent by a requestor to a responder resquesting information about a particular business process.
///
/// **UBL Dictionary Entry Name:** `Enquiry. Details`
///
/// Generated from XSD type `EnquiryType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the requestor.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the requestor, at which this enquiry was issued.
/// - `IssueTime` — The time, assigned by the requestor, at which this enquiry was issued.
/// - `LatestReplyDate` — The date, assigned by the requestor, by which this enquiry will be replied.
/// - `LatestReplyTime` — The time, assigned by the requestor, by which this enquiry will be replied.
/// - `Description` — Free-form text-only description pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `Signature` — A signature applied to this document.
/// - `RequestorParty` — The Party who issues this Enquiry.
/// - `ResponderParty` — The Party who responds to this Enquiry.
/// - `AdditionalDocumentReference` — References to relevant documents for the enquiry such as the Contract folder or the lot in the eTendering.
/// - `Attachment` — Attachment that includes file-based enquiry.
// pub struct Enquiry { ... }

/// A document sent by a responder to a requester answering a particular enqury.
///
/// **UBL Dictionary Entry Name:** `Enquiry Response. Details`
///
/// Generated from XSD type `EnquiryResponseType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the responder.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the responder, at which this enquiry response was issued.
/// - `IssueTime` — The time, assigned by the responder, at which this enquiry response was issued.
/// - `Description` — Free-form text-only enquiry response description pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `Signature` — A signature applied to this document.
/// - `RequestorParty` — The Party who issued the Enquiry.
/// - `ResponderParty` — The Party who responds to the Enquiry.
/// - `ParentDocumentReference` — Reference to the enquiry that this response refers to.
/// - `AdditionalDocumentReference` — References to relevant documents for the response such as the Contract folder or the lot in the eTendering.
/// - `Attachment` — Attachment that includes file-based response.
// pub struct EnquiryResponse { ... }

/// A document used to specify the thresholds for forecast variance, product activity, and performance history beyond which exceptions will be triggered.
///
/// **UBL Dictionary Entry Name:** `Exception Criteria. Details`
///
/// Generated from XSD type `ExceptionCriteriaType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Identifies the current version of this document.
/// - `ValidityPeriod` — The period of time during which the Exception Criteria is valid.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Document.
/// - `ReceiverParty` — The Party who receives this Document.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `ExceptionCriteriaLine` — A document used to specify the thresholds for forecast variance, product activity, and performance history beyond which exceptions will be triggered.
// pub struct ExceptionCriteria { ... }

/// A document used to notify an exception in forecast variance, product activity, or performance history.
///
/// **UBL Dictionary Entry Name:** `Exception Notification. Details`
///
/// Generated from XSD type `ExceptionNotificationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `ExceptionObservationPeriod` — The period of time during which the exceptions are observed.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Document.
/// - `ReceiverParty` — The Party who receives this Document.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `ExceptionNotificationLine` — A line in the Exception Notification.
// pub struct ExceptionNotification { ... }

/// A customs declaration document for exporting goods
///
/// **UBL Dictionary Entry Name:** `Export Customs Declaration. Details`
///
/// Generated from XSD type `ExportCustomsDeclarationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `ExportTypeCode` — A code specifying the type of export
/// - `ExportReasonCode` — A code specifying the reason for the goods being exported
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Identifies the current version of this export customs declaration
/// - `ExporterParty` — The Party who exports the goods or has similar right of disposal over them at the time of export.
/// - `CustomsDeclaration` — The reference to the customs declaration of the goods being exported.
/// - `Signature` — A signature applied to this document.
// pub struct ExportCustomsDeclaration { ... }

/// An expression of interest to a tendering process. An Economic Operator can demonstrate interest in a tendering process issuing an Expression Of Interest document to the contracting party. Upon reception, the Contracting Party registers the interest of the Economic Operator sending the relevant information for the tendering process.
///
/// **UBL Dictionary Entry Name:** `Expression Of Interest Request. Details`
///
/// Generated from XSD type `ExpressionOfInterestRequestType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — The earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `ContractName` — Short title of a contract associated with this Expression of Interest.
/// - `PreferredLanguageLocaleCode` — A code signifying the locale in which the language in the required documents is preferred.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `ValidityPeriod` — The period for which the expression of interest is valid.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `EconomicOperatorParty` — The Economic Operator issuing the expression of interest.
/// - `ContractingParty` — The Contracting Party or the contracting parties in case of joint procurement.
/// - `ProcurementProject` — An overall definition of this procurement project.
/// - `ProcurementProjectLotReference` — One of the procurement project lots into which this contract can be split.
// pub struct ExpressionOfInterestRequest { ... }

/// An expression of interest confirmation issued by a Contracting Party in reply to an expression of interest. The purpose of this document is to inform the Economic Operator he has been registered as an interested party.
///
/// **UBL Dictionary Entry Name:** `Expression Of Interest Response. Details`
///
/// Generated from XSD type `ExpressionOfInterestResponseType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — The earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `ContractName` — Short title of a contract associated with this Expression of Interest.
/// - `TenderLanguageLocaleCode` — A code signifying the language required for the tender.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `ExpressionOfInterestDocumentReference` — A reference to the expression of interest document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `EconomicOperatorParty` — The economic operator that issued the expression of interest and is receiving the confirmation.
/// - `ContractingParty` — The contracting party or parties in case of joint procurement.
/// - `ProcurementProject` — An overall definition of this procurement project.
/// - `ProcurementProjectLotReference` — One of the procurement project lots into which this contract can be split.
// pub struct ExpressionOfInterestResponse { ... }

/// A document used to forecast sales or orders.
///
/// **UBL Dictionary Entry Name:** `Forecast. Details`
///
/// Generated from XSD type `ForecastType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Identifies the current version of this document.
/// - `BasedOnConsensusIndicator` — Indicates whether the Forecast is based on consensus (true) or not (false).
/// - `ForecastPurposeCode` — A code signifying the purpose of the Forecast document.
/// - `ForecastPeriod` — The period to which the Forecast applies.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Forecast.
/// - `ReceiverParty` — The Party who receives this Forecast.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `ForecastLine` — A Forecast Line.
// pub struct Forecast { ... }

/// A document used to revise a Forecast.
///
/// **UBL Dictionary Entry Name:** `Forecast Revision. Details`
///
/// Generated from XSD type `ForecastRevisionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `SequenceNumberID` — A sequence number, to ensure the proper sequencing of revisions.
/// - `RevisionStatusCode` — Indicates the revision status of this Forecast Revision.
/// - `PurposeCode` — Indicates the purpose of the revision.
/// - `ForecastPeriod` — The period to which the Forecast applies.
/// - `OriginalDocumentReference` — The Forecast document being revised.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Forecast Revision.
/// - `ReceiverParty` — The Party who receives this Forecast Revision.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `ForecastRevisionLine` — A line that revises a line in the Forecast.
// pub struct ForecastRevision { ... }

/// A document issued to a forwarder, giving instructions regarding the action to be taken for the forwarding of goods described therein. Forwarding Instructions is used by any party who gives instructions for the transportation services required for a consignment of goods to any party who is contracted to provide the transportation services. The parties who issue this document are commonly referred to as the shipper or consignor, while the parties who receive this document are forwarders, carriers, shipping agents, etc. This document may also be issued by a forwarder or shipping agent in its capacity as a shipper. This document can be used to arrange for the transportation (1) of different types of goods or cargoes; (2) whether containerized or non-containerized; (3) through different modes of transport including multi-modal; and (4) from any origin to any destination.
///
/// **UBL Dictionary Entry Name:** `Forwarding Instructions. Details`
///
/// Generated from XSD type `ForwardingInstructionsType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CarrierAssignedID` — Reference number assigned by a carrier or its agent to identify a specific shipment, such as a booking reference number when cargo space is reserved prior to loading.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Name` — Text, assigned by the sender, that identifies this document to business users.
/// - `Description` — Textual description of the document instance.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `DocumentStatusCode` — A code signifying the status of the Forwarding Instructions with respect to its original state. This code may be used if the document precedes the event and is subsequently found to be incorrect and in need of cancellation or revision.
/// - `ShippingOrderID` — Reference number to identify a Shipping Order.
/// - `ToOrderIndicator` — Indicates whether the transport document is consigned to order.
/// - `AdValoremIndicator` — A term used in commerce in reference to certain duties, called ad valorem duties, which are levied on commodities at certain rates per centum on their value.
/// - `DeclaredCarriageValueAmount` — Value declared by the shipper or his agent solely for the purpose of varying the carrier's level of liability from that provided in the contract of carriage in case of loss or damage to goods or delayed delivery.
/// - `OtherInstruction` — Contains other free-text instructions to the forwarders or carriers related to the shipment. This ought to be used only where such information cannot be represented in other structured information entities within the document.
/// - `ConsignorParty` — The Party who is reponsible for sending the goods.
/// - `CarrierParty` — The Party who provides the transport of goods between named points.
/// - `FreightForwarderParty` — The Party who combines individual smaller consignments into a single larger shipment (a so-called consolidated consignment or shipment) which is sent to a counterpart who mirrors the consolidator's activity by dividing the consolidated consignment into its original components.
/// - `Shipment` — An identifiable collection of one or more goods items to be transported between the seller party and the buyer party.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `ExchangeRate` — Information about the rate of exchange (conversion) between two currencies.
/// - `DocumentDistribution` — A list of interested parties to whom this document is distributed.
/// - `Signature` — A signature applied to this document.
// pub struct ForwardingInstructions { ... }

/// (Deprecated) A document stating the charges incurred for a logistics service.
///
/// **UBL Dictionary Entry Name:** `Freight Invoice. Details`
///
/// Generated from XSD type `FreightInvoiceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `DueDate` — The date on which Invoice is due.
/// - `InvoiceTypeCode` — A code signifying the type of the Freight Invoice.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `TaxPointDate` — The date of the Freight Invoice, used to indicate the point at which tax becomes applicable.
/// - `DocumentCurrencyCode` — A code signifying the default currency for this document.
/// - `TaxCurrencyCode` — A code signifying the currency used for tax amounts in the Freight Invoice.
/// - `PricingCurrencyCode` — A code signifying the currency used for prices in the Freight Invoice.
/// - `PaymentCurrencyCode` — A code signifying the currency used for payment in the Freight Invoice.
/// - `PaymentAlternativeCurrencyCode` — A code signifying the alternative currency used for payment in the Freight Invoice.
/// - `AccountingCostCode` — The buyer's accounting code, applied to the Freight Invoice as a whole.
/// - `AccountingCost` — The buyer's accounting cost centre, applied to the Freight Invoice as a whole, expressed as text.
/// - `LineCountNumeric` — The number of Invoice Lines in the document.
/// - `InvoicePeriod` — The time periods to which the Freight Invoice applies.
/// - `Shipment` — Details about one or more shipments covered by this Freight Invoice.
/// - `OrderReference` — Reference to an Order associated with this Freight Invoice.
/// - `BillingReference` — A reference to a billing document associated with this document.
/// - `DespatchDocumentReference` — A reference to a Despatch Advice associated with this document.
/// - `ReceiptDocumentReference` — A reference to a Receipt Advice associated with this document.
/// - `OriginatorDocumentReference` — A reference to an originator document associated with this document.
/// - `ContractDocumentReference` — A reference to a contract associated with this document.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `ProjectReference` — A reference to a project associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `AccountingSupplierParty` — The accounting supplier party.
/// - `AccountingCustomerParty` — The accounting customer party.
/// - `PayeeParty` — The Party who receives the Payment.
/// - `TaxRepresentativeParty` — The Tax Representative.
/// - `PaymentMeans` — Expected means of payment.
/// - `PaymentTerms` — A set of payment terms associated with this document.
/// - `PrepaidPayment` — A prepaid payment.
/// - `AllowanceCharge` — A discount or charge that applies to a price component.
/// - `TaxExchangeRate` — The exchange rate between the document currency and the tax currency.
/// - `PricingExchangeRate` — The exchange rate between the document currency and the pricing currency.
/// - `PaymentExchangeRate` — The exchange rate between the document currency and the payment currency.
/// - `PaymentAlternativeExchangeRate` — The exchange rate between the document currency and the payment alternative currency.
/// - `TaxTotal` — The total amount of a specific type of tax.
/// - `WithholdingTaxTotal` — The total withholding tax.
/// - `LegalMonetaryTotal` — The total amount payable on the Freight Invoice, including Allowances, Charges, and Taxes.
/// - `InvoiceLine` — An Invoice Line.
// pub struct FreightInvoice { ... }

/// A document used to cancel an entire fulfilment document (Despatch Advice or Receipt Advice).
///
/// **UBL Dictionary Entry Name:** `Fulfilment Cancellation. Details`
///
/// Generated from XSD type `FulfilmentCancellationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `CancellationNote` — The reason for cancellation of the referenced document.
/// - `DespatchDocumentReference` — A reference to a Despatch Advice associated with this document.
/// - `ReceiptDocumentReference` — A reference to a Receipt Advice associated with this document.
/// - `OrderReference` — A reference to an Order document associated with the referenced Despatch or Receipt Advice(s).
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Contract` — The contracts or framework agreements with which the referenced fulfilment document is associated.
/// - `Signature` — A signature applied to this document.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `DeliveryCustomerParty` — The delivery party.
/// - `DespatchSupplierParty` — The despatch party.
/// - `OriginatorCustomerParty` — The originator party
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
// pub struct FulfilmentCancellation { ... }

/// A document that describes a certificate of goods for importation and exportation
///
/// **UBL Dictionary Entry Name:** `Goods Certificate. Details`
///
/// Generated from XSD type `GoodsCertificateType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `TypeCode` — A code specifying the type of goods certificate
/// - `Description` — Textual description of this goods certificate
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Identifies the current version of this goods certificate
/// - `ValidityPeriod` — The period of time for which this goods certificate is considered valid
/// - `ApplicableTerritoryAddress` — A geographic area where this goods certificate is valid
/// - `ExporterParty` — The Party who exports the goods or has similar right of disposal over them at the time of export.
/// - `ImporterParty` — The Party who imports the goods, or on whose behalf the goods are being imported.
/// - `WarehouseParty` — The Party who is responsible for storing the Goods.
/// - `ConsignorParty` — The Party who is reponsible for sending the goods.
/// - `ConsigneeParty` — The Party who receives the goods.
/// - `FreightForwarderParty` — The Party who combines individual smaller consignments into a single larger shipment (a so-called consolidated consignment or shipment) which is sent to a counterpart who mirrors the consolidator's activity by dividing the consolidated consignment into its original components.
/// - `IssuerParty` — The Party who issues this Goods Certificate.
/// - `LegalAuthorityParty` — The legal Authority, when different from the issuer, who sanctions this Goods Certificate.
/// - `ApplicantParty` — The Party who applies for this Goods Certificate.
/// - `Shipment` — The shipment for which this goods certificate is issued
/// - `Attestation` — Any attestations made for the goods related to this goods certificate
/// - `GoodsProcessing` — Any processing that the goods have been undergoing
/// - `OriginalDocumentReference` — A reference to the original version of the goods certificate
/// - `PreviousDocumentReference` — A reference to the previous version of the goods certificate
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this goods certificate
/// - `Signature` — A signature applied to this document.
// pub struct GoodsCertificate { ... }

/// A document providing details relating to a transport service, such as transport movement, identification of equipment and goods, subcontracted service providers, etc.
///
/// **UBL Dictionary Entry Name:** `Goods Item Itinerary. Details`
///
/// Generated from XSD type `GoodsItemItineraryType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Identifies a version of a Goods Item Itinerary in order to distinguish updates.
/// - `TransportExecutionPlanReferenceID` — The Transport Execution Plan associated with this Goods Item Itinerary.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Goods Item Itinerary.
/// - `ReceiverParty` — The Party who receives this Goods Item Itinerary.
/// - `ReferencedConsignment` — A consignment being transported in the transport service associated with this Goods Item Itinerary.
/// - `ReferencedTransportEquipment` — Transport equipment being transported in the transport service associated with this Goods Item Itinerary.
/// - `ReferencedPackage` — A package being transported in the transport service associated with this Goods Item Itinerary.
/// - `ReferencedGoodsItem` — An item of goods being transported in the transport service associated with this Goods Item Itinerary.
/// - `TransportationSegment` — A part of a transport service that has its own Transport Execution Plan. A Transportation Segment may cover services other than transport, such as terminal handling, document management, customs procedures, etc.
// pub struct GoodsItemItinerary { ... }

/// A document providing a temporary export license, also knowned as an ATA Carnet
///
/// **UBL Dictionary Entry Name:** `Goods Item Passport. Details`
///
/// Generated from XSD type `GoodsItemPassportType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `StatusCode` — The reason for importing the goods, expressed as a code.
/// - `Status` — The reason for importing the goods, expressed as text in one or more languages.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Identifies the current version of this request for proof
/// - `ExportReasonCode` — The reason for importing the goods, expressed as a code
/// - `ExportReason` — The reason for importing the goods, expressed as text in one or more languages
/// - `ValidityPeriod` — The period within which this Goods Item Passport is valid
/// - `IssuerParty` — The Party who issues this Goods Item Passport.
/// - `HolderParty` — The Party who holds the Goods Item Passport. This Party is normally the temporary exporter of the Goods.
/// - `RepresentativeParty` — The Party who accompanies the Goods while temporarily exported.
/// - `ExportingGuarantorParty` — The Party who provides a guarantee for the Goods while being temporarily exported. This Party is normally a chamber of commerce.
/// - `ImportingGuarantorParty` — The Party who provides a guarantee for the Goods while being temporarily imported. This Party is normally a chamber of commerce.
/// - `ExportingCustomsParty` — The Party who is competent of Customs in the exporting country.
/// - `ImportingCustomsParty` — The Party who is competent of Customs in the importing country.
/// - `Shipment` — The reference to the shipment of the goods included under this Goods Item Passport
/// - `GoodsItemPassportCounterfoil` — One or more counterfoils associated with this Goods Item Passport
/// - `IssuerEndorsement` — A reference to the issuer's endorsement of this Goods Item Passport
/// - `AdditionalDocumentReference` — One or more references to additional documents related to this Goods Item Passport
/// - `DocumentDistribution` — One or more parties to whom this document is distributed
/// - `Signature` — A signature applied to this document.
// pub struct GoodsItemPassport { ... }

/// A document to notify the deposit of a Guarantee, such as a bid bond.
///
/// **UBL Dictionary Entry Name:** `Guarantee Certificate. Details`
///
/// Generated from XSD type `GuaranteeCertificateType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `GuaranteeTypeCode` — A code signifying the type of the Guarantee.
/// - `Purpose` — A textual description of the purpose of the Guarantee.
/// - `LiabilityAmount` — The liability amount (a monetary value) in the Guarantee.
/// - `ConstitutionCode` — The code stating the constitution means of the Guarantee.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `ApplicablePeriod` — The specified period in the tendering process for which this Guarantee is effective
/// - `ApplicableRegulation` — A reference to an applicable regulation.
/// - `GuaranteeDocumentReference` — A reference to a legal document.
/// - `ImmobilizedSecurity` — Details of an immobilized security.
/// - `Signature` — A signature applied to this document.
/// - `GuarantorParty` — The guarantee creditor organisation who has the authority to charge bid bond guarantee credit.
/// - `InterestedParty` — The Party who deposits the bid bond guarantee.
/// - `BeneficiaryParty` — The recipient who benefits from the bid bond guarantee.
// pub struct GuaranteeCertificate { ... }

/// A customs declaration document for importing goods.
///
/// **UBL Dictionary Entry Name:** `Import Customs Declaration. Details`
///
/// Generated from XSD type `ImportCustomsDeclarationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `TypeCode` — Code specifying the type of import.
/// - `SubTypeCode` — Code specifying the subtype of import.
/// - `NatureOfTransactionCode` — Code specifying the type of transactions for this import
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Identifies a version of an Import Customs Declaration in order to distinguish updates.
/// - `ValidityPeriod` — A period, assigned by the issuer, during which the information in the declaration is valid.
/// - `CustomsExitOfficeLocation` — Customs exit office of the goods being declared.
/// - `JurisdictionRegionAddress` — A geographic area in which this declaration applies.
/// - `ImporterParty` — The Party who makes the import declaration, or on whose behalf the import declaration is made, and who is the owner of the goods or has similar right of disposal over them at the time when the declaration is accepted.
/// - `ConsignorParty` — The Party who is reponsible for sending the goods.
/// - `ConsigneeParty` — The Party who receives the goods.
/// - `FreightForwarderParty` — The Party who combines individual smaller consignments into a single larger shipment (a so-called consolidated consignment or shipment) which is sent to a counterpart who mirrors the consolidator's activity by dividing the consolidated consignment into its original components.
/// - `CustomsParty` — The Authority who is legally responsible for processing the Declaration.
/// - `NotifierParty` — The Party who is responsible for contact on master level.
/// - `Shipment` — The shipment related to this trade certificate
/// - `PreviousCustomsDeclaration` — A reference to a previously sent customs declaration.
/// - `AdditionalDocumentReference` — A reference to additional documents related to or relevant for this customs declaration.
/// - `Signature` — A signature applied to this document.
// pub struct ImportCustomsDeclaration { ... }

/// A document used to initiate a return of goods. The producer is requesting the return of products that are not selling well, either to use in other places or to free up rack or shelf space.
///
/// **UBL Dictionary Entry Name:** `Instruction For Returns. Details`
///
/// Generated from XSD type `InstructionForReturnsType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `SellerSupplierParty` — The seller.
/// - `RetailerCustomerParty` — The retailer.
/// - `ManufacturerParty` — The Party that manufactures the goods to which this Instruction for Return applies.
/// - `Shipment` — The shipment.
/// - `InstructionForReturnsLine` — A line providing details about one type of article to be returned.
// pub struct InstructionForReturns { ... }

/// A report on the quantities of each item that are, or will be, in stock. This document is sent by a Buyer (for example a retailer) to a Seller (for example a producer).
///
/// **UBL Dictionary Entry Name:** `Inventory Report. Details`
///
/// Generated from XSD type `InventoryReportType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for the Inventory Report, assigned by the Issuer.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time at which the Inventory Report was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `DocumentCurrencyCode` — A code signifying the currency in which the Document is presented. This may be the same currency as the pricing or as the tax.
/// - `InventoryPeriod` — The period covered by this report.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `RetailerCustomerParty` — The retailer, who sends this message.
/// - `InventoryReportingParty` — The Party that is reporting the inventory details.
/// - `SellerSupplierParty` — The seller.
/// - `InventoryReportLine` — A line representing a particular item of sale and associated with a line in the Catalogue.
// pub struct InventoryReport { ... }

/// A document used to request payment.
///
/// **UBL Dictionary Entry Name:** `Invoice. Details`
///
/// Generated from XSD type `InvoiceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `DueDate` — The date on which Invoice is due.
/// - `InvoiceTypeCode` — A code signifying the type of the Invoice.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `TaxPointDate` — The date of the Invoice, used to indicate the point at which tax becomes applicable.
/// - `DocumentCurrencyCode` — A code signifying the default currency for this document.
/// - `TaxCurrencyCode` — A code signifying the currency used for tax amounts in the Invoice.
/// - `PricingCurrencyCode` — A code signifying the currency used for prices in the Invoice.
/// - `PaymentCurrencyCode` — A code signifying the currency used for payment in the Invoice.
/// - `PaymentAlternativeCurrencyCode` — A code signifying the alternative currency used for payment in the Invoice.
/// - `AccountingCostCode` — The buyer's accounting code, applied to the Invoice as a whole.
/// - `AccountingCost` — The buyer's accounting code, applied to the Invoice as a whole, expressed as text.
/// - `LineCountNumeric` — The number of lines in the document.
/// - `BuyerReference` — (Deprecated) A reference provided by the buyer used for internal routing of the document.
/// - `DefaultLanguageCode` — A code signifying the default natural language used by the sender for human-readable textual content that does not include a languageID.
/// - `InvoicePeriod` — A period to which the Invoice applies.
/// - `OrderReference` — A reference to the Order with which this Invoice is associated.
/// - `BillingReference` — A reference to a billing document associated with this document.
/// - `DespatchDocumentReference` — A reference to a Despatch Advice associated with this document.
/// - `DeliveryNoteDocumentReference` — A reference to a Delivery Note associated with this document.
/// - `WorkReportDocumentReference` — A reference to a Work Report associated with this document.
/// - `ReceiptDocumentReference` — A reference to a Receipt Advice associated with this document.
/// - `StatementDocumentReference` — A reference to a Statement associated with this document.
/// - `OriginatorDocumentReference` — A reference to an originator document associated with this document.
/// - `ContractDocumentReference` — A reference to a contract associated with this document.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `ProjectReference` — Information about a project.
/// - `BuyerAssignedReference` — A reference provided by the buyer used for internal routing of the document.
/// - `PurchaseReference` — A reference to an object, such as a subscription number, telephone number, meter, vehicle, person, etc., to which this Invoice relates.
/// - `Annotation` — A structured annotation providing contextual or explanatory information related to this Invoice.
/// - `Signature` — A signature applied to this document.
/// - `AccountingSupplierParty` — The accounting supplier party.
/// - `AccountingCustomerParty` — The accounting customer party.
/// - `PayeeParty` — The Party who receives the Payment.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `OriginatorCustomerParty` — The Party who originated the Order to which this Invoice is related.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `TaxRepresentativeParty` — The Party authorized to act as the Tax Representative for this Invoice.
/// - `Delivery` — A delivery associated with this document.
/// - `DeliveryTerms` — A set of delivery terms associated with this document.
/// - `PaymentMeans` — Expected means of payment.
/// - `PaymentTerms` — A set of payment terms associated with this document.
/// - `PrepaidPayment` — A prepaid payment.
/// - `AllowanceCharge` — A discount or charge that applies to a price component.
/// - `TaxExchangeRate` — The exchange rate between the document currency and the tax currency.
/// - `PricingExchangeRate` — The exchange rate between the document currency and the pricing currency.
/// - `PaymentExchangeRate` — The exchange rate between the document currency and the payment currency.
/// - `PaymentAlternativeExchangeRate` — The exchange rate between the document currency and the payment alternative currency.
/// - `TaxTotal` — The total amount of a specific type of tax.
/// - `WithholdingTaxTotal` — The total withholding tax.
/// - `LegalMonetaryTotal` — The total amount payable on the Invoice, including Allowances, Charges, and Taxes.
/// - `CollectionInvoiceLine` — A line describing an item or amount collected on behalf of a third party.
/// - `InvoiceLine` — A line describing an invoice item.
// pub struct Invoice { ... }

/// A document used to request the status of a previously issued Invoice.
///
/// **UBL Dictionary Entry Name:** `Invoice Status Request. Details`
///
/// Generated from XSD type `InvoiceStatusRequestType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `RequestDate` — The date on which the sender of the Invoice Status Request requested a status update for the referenced invoice(s).
/// - `RequestTime` — The time at which the sender of the Invoice Status Request requested a status update for the referenced invoice(s).
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The party sending this document.
/// - `ReceiverParty` — The party receiving this document.
/// - `BillingReference` — A reference to the Invoice for which a status update is requested.
// pub struct InvoiceStatusRequest { ... }

/// A document used to provide information about the status of a collaboration/process associated with a document.
///
/// **UBL Dictionary Entry Name:** `Invoice Status Response. Details`
///
/// Generated from XSD type `InvoiceStatusResponseType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date on which this Invoice Status Response was issued.
/// - `IssueTime` — The time at which this Invoice Status Response was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — An identifier for the current version of this Invoice Status Response.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The party sending this document.
/// - `ReceiverParty` — The party receiving this document.
/// - `Payment` — A Payment associated with one or more Invoices referenced in this Invoice Status Response.
/// - `DocumentResponse` — A response indicating the status of an Invoice referenced in this Invoice Status Response.
// pub struct InvoiceStatusResponse { ... }

/// A document used to request product activity, forecast, or performance data.
///
/// **UBL Dictionary Entry Name:** `Item Information Request. Details`
///
/// Generated from XSD type `ItemInformationRequestType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `Period` — The period of time to which the Item Information Request applies.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party sending this Item Information Request.
/// - `ReceiverParty` — The Party receiving this Item Information Request.
/// - `BuyerCustomerParty` — The Buyer from whom the Item Information is requested.
/// - `SellerSupplierParty` — The Seller requesting the Item Information.
/// - `ItemInformationRequestLine` — A line requesting information regarding an item of sale.
// pub struct ItemInformationRequest { ... }

/// A document listing the contents, cargo, passengers and crew of an airplane, a ship, a truck or a wagon.
///
/// **UBL Dictionary Entry Name:** `Manifest. Details`
///
/// Generated from XSD type `ManifestType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `ManifestTypeCode` — The type of Manifest, expressed as a code.
/// - `ManifestType` — The type of Manifest, expressed as text.
/// - `Description` — Textual description of this document instance.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Identifies a version of a common transportation report in order to distinguish updates.
/// - `AdValoremIndicator` — An indicator of whether ad valorem duties are levied on commodities described in this manifest (true) or not (false).
/// - `DeclaredCarriageValueAmount` — Value declared by the shipper or his agent for the purpose of varying the carrier's level of liability from that provided in the contract of carriage in case of loss or damage to goods or delayed delivery.
/// - `SendingLogisticsOperatorParty` — The Party who issues this Manifest.This Party is normally the Logistics Operator.
/// - `AuthorityParty` — The Authority or regulator who receives this Manifest.
/// - `ConsignorParty` — The Party who is reponsible for sending the goods.
/// - `ConsigneeParty` — The Party who receives the goods.
/// - `CrewPerson` — A person registred as crew in this manifest.
/// - `PassengerPerson` — A person registred as passenger in this manifest
/// - `Shipment` — A shipment associated with this manifest.
/// - `DocumentReference` — A reference to a document relevant for or associated with this common transportation report.
/// - `DocumentDistribution` — An interested party to whom this document is distributed.
/// - `Signature` — A signature applied to this document.
// pub struct Manifest { ... }

/// A document used to order goods and services.
///
/// **UBL Dictionary Entry Name:** `Order. Details`
///
/// Generated from XSD type `OrderType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `SalesOrderID` — An identifier for the Order, assigned by the seller.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `OrderTypeCode` — A code signifying the type of Order.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `RequestedInvoiceCurrencyCode` — A code signifying the currency requested for amount totals in Invoices related to this Order.
/// - `DocumentCurrencyCode` — A code signifying the default currency for this document.
/// - `PricingCurrencyCode` — A code signifying the currency used for all prices in the Order.
/// - `TaxCurrencyCode` — A code signifying the currency requested for tax amounts in Invoices related to this Order.
/// - `CustomerReference` — A supplementary reference for the Order.
/// - `AccountingCostCode` — The buyer's accounting code, applied to the Order as a whole.
/// - `AccountingCost` — The buyer's accounting cost centre, applied to the Order as a whole, expressed as text.
/// - `LineCountNumeric` — The number of Order Lines in the document.
/// - `ValidityPeriod` — The period for which the Order is valid.
/// - `QuotationDocumentReference` — A reference to a Quotation.
/// - `OrderDocumentReference` — A reference to another Order.
/// - `OriginatorDocumentReference` — A reference to an originator document associated with this document.
/// - `CatalogueReference` — A reference to the Catalogue on which this Order is based.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Contract` — A contracts associated with this Order.
/// - `ProjectReference` — A project with which this Order is associated.
/// - `Signature` — A signature applied to this document.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `OriginatorCustomerParty` — The originator.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `FreightForwarderParty` — A freight forwarder or carrier.
/// - `AccountingCustomerParty` — The accounting customer party.
/// - `Delivery` — A delivery associated with this document.
/// - `DeliveryTerms` — A set of delivery terms associated with this document.
/// - `PaymentMeans` — Expected means of payment.
/// - `PaymentTerms` — A set of payment terms associated with this document.
/// - `TransactionConditions` — A specification of purchasing or sales conditions applying to the whole Order.
/// - `AllowanceCharge` — A discount or charge that applies to a price component.
/// - `TaxExchangeRate` — The exchange rate between the document currency and the tax currency.
/// - `PricingExchangeRate` — The exchange rate between the document currency and the pricing currency.
/// - `PaymentExchangeRate` — The exchange rate between the document currency and the payment currency.
/// - `DestinationCountry` — The country of destination (for customs purposes).
/// - `TaxTotal` — The total amount of a specific type of tax.
/// - `AnticipatedMonetaryTotal` — The total amount for the Order anticipated by the buyer.
/// - `OrderLine` — A line associated with a line in the Catalogue and specifying a kind of item being ordered.
// pub struct Order { ... }

/// A document used to cancel an entire Order.
///
/// **UBL Dictionary Entry Name:** `Order Cancellation. Details`
///
/// Generated from XSD type `OrderCancellationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `CancellationNote` — The general reason for cancellation of the referenced order.
/// - `OrderReference` — A reference to the Order being cancelled. While multiple references are allowed, it is considered better practice to cancel only one Order in each Order Cancellation document.
/// - `OriginatorDocumentReference` — A reference to an originator document associated with this document.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Contract` — A contract associated with the original Order(s).
/// - `Signature` — A signature applied to this document.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `OriginatorCustomerParty` — The originator.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
// pub struct OrderCancellation { ... }

/// A document used to specify changes to an existing Order.
///
/// **UBL Dictionary Entry Name:** `Order Change. Details`
///
/// Generated from XSD type `OrderChangeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `SalesOrderID` — An identifier for the Order Change, assigned by the seller.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `SequenceNumberID` — The Order Change Sequence Number assigned by the Buyer to ensure the proper sequencing of changes.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `RequestedInvoiceCurrencyCode` — A code signifying he currency requested for amount totals in Invoices related to this Order Change.
/// - `DocumentCurrencyCode` — A code signifying the default currency for this document.
/// - `PricingCurrencyCode` — A code signifying the currency that is used for all prices in the Order Change.
/// - `TaxCurrencyCode` — A code signifying the currency requested for tax amounts in Invoices related to this Order Change.
/// - `CustomerReference` — A supplementary reference for the transaction (e.g., CRI when using purchasing card).
/// - `AccountingCostCode` — The buyer's accounting code, applied to the Order Change as a whole.
/// - `AccountingCost` — The buyer's accounting code, applied to the Order Change as a whole, expressed as text.
/// - `LineCountNumeric` — The number of Order Change lines in the document.
/// - `ValidityPeriod` — A period during which the Order Change is valid.
/// - `OrderReference` — A reference to the Order being changed.
/// - `QuotationDocumentReference` — A reference to a Quotation.
/// - `OriginatorDocumentReference` — A reference to an originator document associated with this document.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Contract` — A contract associated with the Order being changed.
/// - `Signature` — A signature applied to this document.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `OriginatorCustomerParty` — The originator.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `FreightForwarderParty` — A freight forwarder or carrier.
/// - `AccountingCustomerParty` — The accounting customer party.
/// - `AccountingSupplierParty` — The accounting supplier party.
/// - `Delivery` — A delivery associated with this document.
/// - `DeliveryTerms` — A set of delivery terms associated with this document.
/// - `PaymentMeans` — Expected means of payment.
/// - `PaymentTerms` — A set of payment terms associated with this document.
/// - `TransactionConditions` — Purchasing, sales, or payment conditions applying to the whole Order being changed.
/// - `AllowanceCharge` — A discount or charge that applies to a price component.
/// - `TaxExchangeRate` — The exchange rate between the document currency and the tax currency.
/// - `PricingExchangeRate` — The exchange rate between the document currency and the pricing currency.
/// - `PaymentExchangeRate` — The exchange rate between the document currency and the payment currency.
/// - `DestinationCountry` — The country of destination (for customs purposes).
/// - `TaxTotal` — The total amount of a specific type of tax.
/// - `AnticipatedMonetaryTotal` — The amount of change to the total cost of the order anticipated by the buyer.
/// - `OrderLine` — An association to one or more (changed) Order Lines.
// pub struct OrderChange { ... }

/// A document used to indicate detailed acceptance or rejection of an Order or to make a counter-offer.
///
/// **UBL Dictionary Entry Name:** `Order Response. Details`
///
/// Generated from XSD type `OrderResponseType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `SalesOrderID` — An identifier for the Order, issued by the Seller.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `OrderResponseCode` — A code signifying the type of response for this Order.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `DocumentCurrencyCode` — A code signifying the default currency for this document.
/// - `PricingCurrencyCode` — A code signifying the currency that is used for all prices in the Order Response.
/// - `TaxCurrencyCode` — A code signifying the currency that is used for all tax amounts in the Order Response.
/// - `TotalPackagesQuantity` — The total number of packages contained in the Order Response.
/// - `GrossWeightMeasure` — The total gross weight for the Order Response (goods + packaging + transport equipment).
/// - `NetWeightMeasure` — The total net weight for the Order Response (goods + packaging).
/// - `NetNetWeightMeasure` — The total net weight of the goods in the Order Response excluding packaging.
/// - `GrossVolumeMeasure` — The total volume of the goods in the Order Response including packaging.
/// - `NetVolumeMeasure` — The total volume of the goods in the Order Response excluding packaging.
/// - `CustomerReference` — A supplementary reference assigned by the buyer, e.g., the CRI in a purchasing card transaction.
/// - `AccountingCostCode` — An accounting cost code applied to the order as a whole.
/// - `AccountingCost` — An accounting cost code applied to the order as a whole, expressed as text.
/// - `LineCountNumeric` — The number of Order Lines in this document.
/// - `ValidityPeriod` — The period for which the Order Response is valid.
/// - `OrderReference` — A reference to the Order being responded to.
/// - `OrderDocumentReference` — A reference to an Order other than the one being responded to.
/// - `OrderChangeDocumentReference` — A reference to an Order Change being responded to.
/// - `OriginatorDocumentReference` — A reference to an originator document associated with this document.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Contract` — A contract associated with the Order being responded to.
/// - `Signature` — A signature applied to this document.
/// - `SellerSupplierParty` — The seller.
/// - `BuyerCustomerParty` — The buyer.
/// - `OriginatorCustomerParty` — The originator.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `FreightForwarderParty` — A freight forwarder or carrier.
/// - `AccountingSupplierParty` — The accounting supplier party.
/// - `AccountingCustomerParty` — The accounting customer party.
/// - `Delivery` — A delivery associated with this document.
/// - `DeliveryTerms` — A set of delivery terms associated with this document.
/// - `PaymentMeans` — Expected means of payment.
/// - `PaymentTerms` — A set of payment terms associated with this document.
/// - `AllowanceCharge` — A discount or charge that applies to a price component.
/// - `TransactionConditions` — A specification of purchasing or sales conditions applying to the whole Order.
/// - `TaxExchangeRate` — The exchange rate between the document currency and the tax currency.
/// - `PricingExchangeRate` — The exchange rate between the document currency and the pricing currency.
/// - `PaymentExchangeRate` — The exchange rate between the document currency and the payment currency.
/// - `DestinationCountry` — The country of destination (for customs purposes).
/// - `TaxTotal` — The total amount of a specific type of tax, as calculated by the seller.
/// - `LegalMonetaryTotal` — The total amount of the Order (or counter-offer).
/// - `OrderLine` — A line associated with a line in the Catalogue and specifying a kind of item being ordered.
// pub struct OrderResponse { ... }

/// (Deprecated) A document used to indicate simple acceptance or rejection of an entire Order.
///
/// **UBL Dictionary Entry Name:** `Order Response Simple. Details`
///
/// Generated from XSD type `OrderResponseSimpleType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `AcceptedIndicator` — Indicates whether the Order is accepted (true) or rejected (false).
/// - `RejectionNote` — The reason for rejection if the order was not accepted.
/// - `CustomerReference` — A supplementary reference for the transaction (e.g., when using a purchasing card).
/// - `AccountingCostCode` — An accounting cost code applied to the order as a whole.
/// - `AccountingCost` — An accounting cost code applied to the order as a whole, expressed as text.
/// - `OrderReference` — A reference to the Order being responded to.
/// - `OrderChangeDocumentReference` — A reference to an Order Change being responded to.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `SellerSupplierParty` — The seller.
/// - `BuyerCustomerParty` — The buyer.
/// - `OriginatorCustomerParty` — The originator.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `AccountingSupplierParty` — The accounting supplier party.
/// - `AccountingCustomerParty` — The accounting customer party.
// pub struct OrderResponseSimple { ... }

/// A document describing how goods are packed.
///
/// **UBL Dictionary Entry Name:** `Packing List. Details`
///
/// Generated from XSD type `PackingListType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the subset of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document..
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Name` — Text, assigned by the sender, that identifies this document to business users.
/// - `Description` — Textual description of the document instance.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Version identifier of a Packing List.
/// - `OtherInstruction` — Contains other free-text-based instructions related to the shipment to the forwarders or carriers. This ought to be used only where such information cannot be represented in other structured information entities within the document.
/// - `ConsignorParty` — The Party who is reponsible for sending the goods.
/// - `CarrierParty` — The Party who provides the transport of goods between named points.
/// - `FreightForwarderParty` — The Party who combines individual smaller consignments into a single larger shipment (a so-called consolidated consignment or shipment) which is sent to a counterpart who mirrors the consolidator's activity by dividing the consolidated consignment into its original components.
/// - `Shipment` — A description of the shipment.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `DocumentDistribution` — A list of interested parties to whom this document is distributed.
/// - `Signature` — A signature applied to this document.
// pub struct PackingList { ... }

/// A document used by a contracting party to declare the intention to buy goods, services, or works during a specified period.
///
/// **UBL Dictionary Entry Name:** `Prior Information Notice. Details`
///
/// Generated from XSD type `PriorInformationNoticeType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — An identifier of the current version of the Prior Information Notice.
/// - `PreviousVersionID` — An identifier of the previous version of the Prior Information Notice which is superceded by this version.
/// - `RequestedPublicationDate` — The requested publication date for this Prior Information Notice.
/// - `PlannedDate` — The date planned by the Contracting Party for publication of the contract notice.
/// - `RegulatoryDomain` — Information about the law that defines the regulatory domain.
/// - `NoticeTypeCode` — The type of notice (PIN, Qualification, Reduce time...)
/// - `NoticeLanguageCode` — The language used for this prior information notice.
/// - `AdditionalNoticeLanguage` — An additional official language used for this prior information notice.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `ContractingParty` — The contracting party or parties in case of joint procurement.
/// - `OriginatorCustomerParty` — A party who originated the tendering process.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `ReceiverParty` — The Party receiving this Document.
/// - `TenderingTerms` — The tendering terms associated with this tendering process.
/// - `TenderingProcess` — A description of the tendering process itself.
/// - `ProcurementProject` — An overall definition of this procurement project.
/// - `ProcurementProjectLot` — One of the procurement project lots into which this contract can be split.
// pub struct PriorInformationNotice { ... }

/// A document sent by a Contracting Party to an Economic Operator describing the status of a procurement procedure, Project, or Lot.
///
/// **UBL Dictionary Entry Name:** `Procurement Status. Details`
///
/// Generated from XSD type `ProcurementStatusType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `ContractName` — Short title of a contract associated with this Tender.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `ProcedureCode` — A code signifying the type of this tendering procedure.
/// - `TenderSubmissionDeadlinePeriod` — The period during which tenders must be delivered.
/// - `InvitationSubmissionPeriod` — The period during which invitations to tender must be completed and delivered.
/// - `ParticipationRequestReceptionPeriod` — The period during which requests for participation must be completed and delivered.
/// - `ProcurementLegislationDocumentReference` — A reference to a document providing references to procurement legislation applicable to the tendering process.
/// - `FiscalLegislationDocumentReference` — A reference to a document providing references to fiscal legislation applicable to the tendering process.
/// - `EnvironmentalLegislationDocumentReference` — A reference to a document providing references to environmental legislation applicable to the tendering process.
/// - `EmploymentLegislationDocumentReference` — A reference to a document providing references to employment legislation applicable to the tendering process.
/// - `ProcedureStatusRequestDocumentReference` — A reference to a Procedure Status Request.
/// - `Signature` — A signature applied to this document.
/// - `ContractingParty` — The Contracting Party issuing the information about the tender status.
/// - `EconomicOperatorParty` — The Economic Operator receiving the tender status information.
/// - `DocumentProviderParty` — The Party that provides the procurement documents to the Economic Operator.
/// - `TenderRecipientParty` — The Party to which tenders will be submitted.
/// - `ProcurementProject` — An overall definition of this Procurement Project.
/// - `ProcurementProjectLot` — One of the Procurement Project lots into which this contract can be split.
// pub struct ProcurementStatus { ... }

/// A document sent by an Economic Operator to a Contracting Party asking about the details and status of a procurement procedure, Project, or Lot.
///
/// **UBL Dictionary Entry Name:** `Procurement Status Request. Details`
///
/// Generated from XSD type `ProcurementStatusRequestType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `ContractName` — Short title of a contract associated with this Procedure.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `Signature` — A signature applied to this document.
/// - `ContractingParty` — The Contracting Party receiving the tender status inquiry.
/// - `EconomicOperatorParty` — The Economic Operator issuing the inquiry on the status of a tendering process.
/// - `TenderingProcess` — The Tendering Process of this Procedure Status Request.
/// - `ProcurementProject` — An overall definition of this Procurement Project.
/// - `ProcurementProjectLot` — One of the Procurement Project Lots into which this contract can be split.
// pub struct ProcurementStatusRequest { ... }

/// A document reporting the movement of goods at specified retail locations for inventory tracking purposes.
///
/// **UBL Dictionary Entry Name:** `Product Activity. Details`
///
/// Generated from XSD type `ProductActivityType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `DocumentCurrencyCode` — A code signifying the default currency for this document.
/// - `ActivityPeriod` — The period covered by this Product Activity report.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Product Activity.
/// - `ReceiverParty` — The Party who receives this Product Activity.
/// - `SupplyChainActivityDataLine` — A line describing the movement of goods to a specific location.
// pub struct ProductActivity { ... }

/// A document providing a status or a proof that goods have been re-exported
///
/// **UBL Dictionary Entry Name:** `Proof Of Reexportation. Details`
///
/// Generated from XSD type `ProofOfReexportationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Identifies the current version of this request for proof
/// - `ExportingCustomsParty` — The Party who is competent of Customs in the exporting country.
/// - `ImportingGuarantorParty` — The Party who provides the guarantee for the Goods while being temporarily imported. This Party is normally a chamber of commerce.
/// - `ExportingGuarantorParty` — The Party who provides the guarantee for the Goods while being temporarily exported. This Party is normally a chamber of commerce.
/// - `GoodsItemPassportCounterfoil` — One or more goods item passport or ATA Carnet counterfoils associated with this proof of re-exportation
/// - `ReexportationEvidence` — One or more references to evidence supporting that goods have been re-exported
/// - `GoodsItemPassportAttachment` — Attachment of the goods item passport or ATA Carnet related to this proof of re-exportation
/// - `AdditionalDocumentReference` — One or more references to additional documents related to this proof of re-exportation
/// - `Signature` — A signature applied to this document.
// pub struct ProofOfReexportation { ... }

/// A reminder that a requested Proof of Reexportation is pending.
///
/// **UBL Dictionary Entry Name:** `Proof Of Reexportation Reminder. Details`
///
/// Generated from XSD type `ProofOfReexportationReminderType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `ProcedureCode` — The procedure under which this reminder was sent, expressed as a code
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Identifies a version of a Proof of Reexportation Reminder in order to distinguish updates.
/// - `GoodsItemPassportID` — An identifier for the associated Goods Item Passport, used when all counterfoils refer to the same.
/// - `ProofOfReexportationRequestDocumentReference` — The Document Reference related to this Proof of Reexportation Request
/// - `ImportingGuarantorParty` — The Party who on behalf of their Customs Authority issues this Document. This Party is normally a chamber of commerce.
/// - `ExportingGuarantorParty` — The Party who is fiscally responsible for the Goods Item Passport counterfoils which the Customs Party is requesting. This Party is normally a chamber of commerce.
/// - `ImportingCustomsParty` — The Party who originally requests the Proof of Reexportation.
/// - `IssuerEndorsement` — An Issuers endorsment of this Request for Proof of Reexportation.
/// - `PaymentTerms` — A set of payment terms associated with this Request for Proof of Reexportation, used for generating a subsequent invoice in case no proof of re-exportation can be provided.
/// - `GoodsItemPassportCounterfoil` — The related Goods Item Passport Counterfoils of an associated Goods Item Passport.
/// - `AdditionalDocumentReference` — One or more references to additional documents related to this Request for Proof of Reexportation
/// - `Signature` — A signature applied to this document.
// pub struct ProofOfReexportationReminder { ... }

/// A document requesting the status or proof that goods have been re-exported
///
/// **UBL Dictionary Entry Name:** `Proof Of Reexportation Request. Details`
///
/// Generated from XSD type `ProofOfReexportationRequestType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Identifies the current version of this request for proof
/// - `GoodsItemPassportID` — The identifier of the goods item passport or ATA Carnet of the goods
/// - `GoodsItemPassportCounterfoilID` — A reference to a counterfoil of the goods item passport or ATA Carnet
/// - `ImportingGuarantorParty` — The Party who on behalf of their Customs Authority issues the Proof of Reexportation. This Party is normally a chamber of commerce.
/// - `ExportingGuarantorParty` — The Party who is fiscally responsible for the Goods Item Passport counterfoils which the Customs Party is requesting. This Party is normally a chamber of commerce.
/// - `ImportingCustomsParty` — The Party who originally requests the Proof of Reexportation.
/// - `AdditionalDocumentReference` — One or more references to additional documents related to this request
/// - `Signature` — A signature applied to this document.
// pub struct ProofOfReexportationRequest { ... }

/// A receipt for a purchase made with cash or cash equivalents.
///
/// **UBL Dictionary Entry Name:** `Purchase Receipt. Details`
///
/// Generated from XSD type `PurchaseReceiptType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this purchase receipt, assigned by the seller.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date when the purchase receipt was issued.
/// - `IssueTime` — The time of day when the purchase receipt was issued.
/// - `TransactionDate` — The date when the purchase transaction was initiated.
/// - `TransactionTime` — The time of day when the purchase transaction was initiated.
/// - `PurchaseDate` — The date when the purchase took place.
/// - `PurchaseTime` — The time of day when the purchase took place.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `DocumentCurrencyCode` — A code signifying the default currency for this document.
/// - `PurchaseReference` — A reference to an object, such as a subscription number, telephone number, meter, vehicle, person, etc., to which this purchase relates.
/// - `SalesDocumentReference` — A reference to the sales document to which this purchase receipt is related.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this purchase receipt.
/// - `Signature` — A signature applied to this document.
/// - `AccountingSupplierParty` — The accounting supplier party.
/// - `AccountingCustomerParty` — The accounting customer party.
/// - `CashierContact` — The cashier who handled the purchase at the point of sales.
/// - `CashRegister` — The cash register that was used for this purchase.
/// - `PointOfSaleLocation` — The location of the point of sale where this purchase took place.
/// - `PointOfSaleContact` — The contact person at the point of sale where this purchase took place.
/// - `Delivery` — The delivery associated with this purchase.
/// - `Payment` — One or more payments for this purchase.
/// - `PaymentMeans` — One or more payment means used to pay for this purchase, with their associated payments.
/// - `AllowanceCharge` — A discount or charge that applies to a price component.
/// - `TaxExchangeRate` — The exchange rate between the document currency and the tax currency.
/// - `PricingExchangeRate` — The exchange rate between the document currency and the pricing currency.
/// - `TaxTotal` — The total amount of a specific type of tax.
/// - `LegalMonetaryTotal` — The total amount payable on the Invoice, including Allowances, Charges, and Taxes.
/// - `PurchaseReceiptLine` — One or more line items that describe this purchase.
// pub struct PurchaseReceipt { ... }

/// A document issued by a buyer defining how the Exclusion Grounds and the Selection Criteria should be addressed in a Single Procurement Document for a specific procurement.
///
/// **UBL Dictionary Entry Name:** `Qualification Application Request. Details`
///
/// Generated from XSD type `QualificationApplicationRequestType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `ContractName` — Short title of a contract associated with this Tender.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `VersionID` — Indicates the current version of the Qualification Application Request.
/// - `PreviousVersionID` — Identifies the previous version of the Qualification Application Request which is superceded by this version.
/// - `ProcedureCode` — A code signifying the type of this tendering procedure.
/// - `QualificationApplicationTypeCode` — A code specifying the type of the Qualification Application.
/// - `WeightScoringMethodologyNote` — Free-form text to describing information about Weight Scoring Methodology.
/// - `WeightingTypeCode` — A code specifying the type of the Weighting.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `ContractingParty` — The contracting party.
/// - `EconomicOperatorParty` — The Economic Operator receiving the Qualification Application Resquest.
/// - `ProcurementProject` — An overall definition of this procurement project.
/// - `ProcurementProjectLot` — One of the procurement project lots into which this contract can be split.
/// - `TenderingCriterion` — A criterion supporting Tenderer qualifications.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Signature` — A signature applied to this document.
// pub struct QualificationApplicationRequest { ... }

/// A self-declaration of an economic operator, stating that it does not fall under the Exclusion Grounds and that it meets the Selection Criteria for a specific procurement.
///
/// **UBL Dictionary Entry Name:** `Qualification Application Response. Details`
///
/// Generated from XSD type `QualificationApplicationResponseType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `ContractName` — Short title of a contract associated with this Tender.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `EconomicOperatorGroupName` — Economic Operator Group Name associated with this Qualification.
/// - `VersionID` — Indicates the current version of the Qualification Application Response.
/// - `PreviousVersionID` — Identifies the previous version of the Qualification Application Response which is superceded by this version.
/// - `ProcedureCode` — A code signifying the type of this tendering procedure.
/// - `QualificationApplicationTypeCode` — A code specifying the type of the Qualification Application.
/// - `WeightScoringMethodologyNote` — Free-form text to describe Weight Scoring Methodology.
/// - `WeightingTypeCode` — A code specifying the Weighting type
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `ContractingParty` — The contracting party.
/// - `EconomicOperatorParty` — The Economic Operator issuing the Qualification Application Response.
/// - `ProcurementProject` — An overall definition of this procurement project.
/// - `ProcurementProjectLot` — One of the procurement project lots into which this contract can be split.
/// - `TenderingCriterion` — The criterion as described in the Qualification Application Request.
/// - `TenderingCriterionResponse` — Each criterion requirement response.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Evidence` — The evidence supporting this criterion requirement response.
/// - `Signature` — A signature applied to this document.
// pub struct QualificationApplicationResponse { ... }

/// A document used to quote for the provision of goods and services.
///
/// **UBL Dictionary Entry Name:** `Quotation. Details`
///
/// Generated from XSD type `QuotationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the subset of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `PricingCurrencyCode` — A code signifying the currency used for all prices in the Quotation.
/// - `LineCountNumeric` — The number of Quotation Lines in this document.
/// - `ValidityPeriod` — The period for which the Quotation is valid.
/// - `RequestForQuotationDocumentReference` — A reference to the Request for Quotation associated with this Quotation.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Contract` — A contract associated with this Quotation.
/// - `Signature` — A signature applied to this document.
/// - `SellerSupplierParty` — The seller.
/// - `BuyerCustomerParty` — Association to the Buyer.
/// - `OriginatorCustomerParty` — The originator.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `Delivery` — A delivery associated with this document.
/// - `DeliveryTerms` — A set of delivery terms associated with this document.
/// - `PaymentMeans` — Expected means of payment.
/// - `TransactionConditions` — A specification of purchasing, sales, or payment conditions applying to Orders related to this Quotation.
/// - `AllowanceCharge` — A discount or charge that applies to a price component.
/// - `DestinationCountry` — The country of destination of potential orders (for customs purposes).
/// - `TaxTotal` — The total amount of a specific type of tax.
/// - `QuotedMonetaryTotal` — The total amount of the Quotation.
/// - `QuotationLine` — A line quoting a cost for one kind of item.
// pub struct Quotation { ... }

/// A document used to describe the receipt of goods and services or as a reply to a despatch advice.
///
/// **UBL Dictionary Entry Name:** `Receipt Advice. Details`
///
/// Generated from XSD type `ReceiptAdviceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the subset of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `DocumentStatusCode` — A code signifying the status of the Receipt Advice with respect to its original state. This code may be used if the document precedes the event and is subsequently found to be incorrect and in need of cancellation or revision.
/// - `ReceiptAdviceTypeCode` — A code signifying the type of the Receipt Advice.
/// - `DeliveryAcceptanceCode` — A code to specify the acceptance or rejection of the delivery.
/// - `RejectReasonCode` — The reason for a rejection, expressed as a code.
/// - `RejectReason` — The reason for a rejection, expressed as text.
/// - `RejectActionCode` — A code signifying the action that the delivery party wishes the despatch party to take in the case of a rejection.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `LineCountNumeric` — The number of Receipt Lines in this document.
/// - `OrderReference` — A reference to an Order associated with this Receipt Advice.
/// - `DespatchDocumentReference` — A reference to a Despatch Advice associated with this document.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `DeliveryCustomerParty` — The customer party.
/// - `DespatchSupplierParty` — The supplier party.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `Shipment` — Details about the Shipment.
/// - `ReceiptLine` — A line detailing a kind of item received.
// pub struct ReceiptAdvice { ... }

/// A document used to remind a customer of payments past due.
///
/// **UBL Dictionary Entry Name:** `Reminder. Details`
///
/// Generated from XSD type `ReminderType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `ReminderTypeCode` — A code signifying the type of the Reminder.
/// - `ReminderSequenceNumeric` — The number of the current Reminder in the sequence of reminders relating to the specified payments; the number of reminders previously sent plus one.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `TaxPointDate` — The date of the Reminder, used to indicate the point at which tax becomes applicable.
/// - `DocumentCurrencyCode` — A code signifying the default currency for this document.
/// - `TaxCurrencyCode` — A code signifying the currency used for tax amounts in the Reminder.
/// - `PricingCurrencyCode` — A code signifying the currency used for prices in the Reminder.
/// - `PaymentCurrencyCode` — A code signifying the currency used for payment in the Reminder.
/// - `PaymentAlternativeCurrencyCode` — A code signifying the alternative currency used for payment in the Reminder.
/// - `AccountingCostCode` — The buyer's accounting code, applied to the Reminder as a whole.
/// - `AccountingCost` — The buyer's accounting code, applied to the Reminder as a whole, expressed as text.
/// - `LineCountNumeric` — The number of Reminder Lines in this document.
/// - `ReminderPeriod` — The periods to which the Reminder applies.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `AccountingSupplierParty` — The accounting supplier party.
/// - `AccountingCustomerParty` — The accounting customer party.
/// - `PayeeParty` — The Party who receives the Payment.
/// - `TaxRepresentativeParty` — The Party authorized to act as the Tax Representative for the taxpayer.
/// - `PaymentMeans` — Expected means of payment.
/// - `PaymentTerms` — A set of payment terms associated with this document.
/// - `PrepaidPayment` — A prepaid payment.
/// - `AllowanceCharge` — A discount or charge that applies to a price component.
/// - `TaxExchangeRate` — The exchange rate between the document currency and the tax currency.
/// - `PricingExchangeRate` — The exchange rate between the document currency and the pricing currency.
/// - `PaymentExchangeRate` — The exchange rate between the document currency and the payment currency.
/// - `PaymentAlternativeExchangeRate` — The exchange rate between the document currency and the payment alternative currency.
/// - `TaxTotal` — The total amount of a specific type of tax.
/// - `LegalMonetaryTotal` — The total amount payable on the Invoice, including Allowances, Charges, and Taxes.
/// - `ReminderLine` — A line describing a payment past due.
// pub struct Reminder { ... }

/// A document that specifies details of an actual payment.
///
/// **UBL Dictionary Entry Name:** `Remittance Advice. Details`
///
/// Generated from XSD type `RemittanceAdviceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `DocumentCurrencyCode` — A code signifying the default currency for this document.
/// - `TotalDebitAmount` — The totals of all debit amounts for the Remittance Advice.
/// - `TotalCreditAmount` — The totals of all credit amounts for the Remittance Advice.
/// - `TotalPaymentAmount` — The total payable amount for the Remittance Advice (must be positive).
/// - `PaymentOrderReference` — An internal reference to the order for payment from the payer to the payer's bank.
/// - `PayerReference` — An internal reference to the payer's order for payment.
/// - `InvoicingPartyReference` — An internal reference to the order for payment by the invoicing party. This may have been requested of the payer by the payee to accompany the payer's remittance.
/// - `LineCountNumeric` — The number of Remittance Advice Lines in the document.
/// - `InvoicePeriod` — A period (rather than a specific invoice) associated with this document.
/// - `BillingReference` — A reference to a billing document associated with this document.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `AccountingCustomerParty` — The accounting customer party.
/// - `AccountingSupplierParty` — The accounting supplier party.
/// - `PayeeParty` — The Party who receives the Payment.
/// - `PaymentMeans` — Expected means of payment.
/// - `TaxTotal` — The total amount of a specific type of tax.
/// - `RemittanceAdviceLine` — A line specifying a balance.
// pub struct RemittanceAdvice { ... }

/// A document used to request a Quotation for goods and services from a Seller.
///
/// **UBL Dictionary Entry Name:** `Request For Quotation. Details`
///
/// Generated from XSD type `RequestForQuotationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `SubmissionDueDate` — The due date for submission of the Quotation.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `PricingCurrencyCode` — The currency that the Seller will use to price the Quotation.
/// - `LineCountNumeric` — The number of Request For Quotation Lines in this document.
/// - `RequestedValidityPeriod` — The validity period requested for this Quotation.
/// - `CatalogueDocumentReference` — The Catalogue on which this Request for Quotation is based.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `OriginatorCustomerParty` — The originator.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `SellerSupplierParty` — The seller.
/// - `BuyerCustomerParty` — The buyer.
/// - `Delivery` — A delivery associated with this document.
/// - `DeliveryTerms` — A set of delivery terms associated with this document.
/// - `DestinationCountry` — The country of destination of potential orders (for customs purposes).
/// - `Contract` — A contract associated with this Request for Quotation..
/// - `RequestForQuotationLine` — A line specifying a kind of item of sale.
// pub struct RequestForQuotation { ... }

/// A document used to specify basic information about retail events (such as promotions, product introductions, and community or environmental events) that affect supply or demand.
///
/// **UBL Dictionary Entry Name:** `Retail Event. Details`
///
/// Generated from XSD type `RetailEventType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `RetailEventName` — A title, theme, slogan, or other identifier for the event for use by trading partners.
/// - `RetailEventStatusCode` — Describes the logical state of the discrete activity affecting supply or demand in the supply chain
/// - `SellerEventID` — An event tracking identifier assigned by the seller.
/// - `BuyerEventID` — An event tracking identifier assigned by the buyer.
/// - `Description` — Definition of the discrete activity affecting supply or demand in the supply chain
/// - `Period` — The period during which the event takes place.
/// - `OriginalDocumentReference` — A reference to a Forecast document associated with this event.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Retail Event.
/// - `ReceiverParty` — The Party who receives this Retail Event.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `EventComment` — A comment regarding the event.
/// - `PromotionalEvent` — The description of a promotional event associated with this event.
/// - `MiscellaneousEvent` — A miscellaneous event associated with this event.
// pub struct RetailEvent { ... }

/// A credit note created by the debtor in a self billing arrangement with a creditor.
///
/// **UBL Dictionary Entry Name:** `Self Billed Credit Note. Details`
///
/// Generated from XSD type `SelfBilledCreditNoteType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `DueDate` — The date on which SelfBilledCreditNote is due.
/// - `TaxPointDate` — The date of the Self Billed Credit Note, used to indicate the point at which tax becomes applicable.
/// - `CreditNoteTypeCode` — A code signifying the type of Selfbilled CreditNote
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `DocumentCurrencyCode` — A code signifying the default currency for this document.
/// - `TaxCurrencyCode` — A code signifying the currency used for tax amounts in the Self Billed Credit Note.
/// - `PricingCurrencyCode` — A code signifying the currency used for prices in the Self Billed Credit Note.
/// - `PaymentCurrencyCode` — A code signifying the currency used for payment in the Self Billed Credit Note.
/// - `PaymentAlternativeCurrencyCode` — A code signifying the alternative currency used for payment in the Self Billed Credit Note.
/// - `AccountingCostCode` — The buyer's accounting code, applied to the Self Billed Credit Note as a whole.
/// - `AccountingCost` — The buyer's accounting code, applied to the Self Billed Credit Note as a whole, expressed as text.
/// - `LineCountNumeric` — The number of Self Billed Credit Note Lines in this document.
/// - `BuyerReference` — (Deprecated) A reference provided by the buyer used for internal routing of the document.
/// - `DefaultLanguageCode` — A code signifying the default natural language used by the sender for human-readable textual content that does not include a languageID.
/// - `InvoicePeriod` — A period (rather than a specific Invoice) associated with the Self Billed Credit Note.
/// - `DiscrepancyResponse` — A reason for the Self Billed Credit Note as a whole.
/// - `OrderReference` — The Order associated with this Self Billed Credit Note.
/// - `BillingReference` — A reference to a billing document associated with this document.
/// - `DespatchDocumentReference` — A reference to a Despatch Advice associated with this document.
/// - `DeliveryNoteDocumentReference` — A reference to a Delivery Note associated with this document.
/// - `WorkReportDocumentReference` — A reference to a Work Report associated with this document.
/// - `ReceiptDocumentReference` — A reference to a Receipt Advice associated with this document.
/// - `ContractDocumentReference` — A reference to a contract associated with this document.
/// - `StatementDocumentReference` — A reference to a Statement associated with this document.
/// - `OriginatorDocumentReference` — A reference to an originator document associated with this document.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `ProjectReference` — A reference to a project associated with this document.
/// - `BuyerAssignedReference` — A reference provided by the buyer used for internal routing of the document.
/// - `PurchaseReference` — A reference to an object, such as a subscription number, telephone number, meter, vehicle, person, etc., to which this Credit Note relates.
/// - `Annotation` — A structured annotation providing contextual or explanatory information related to this Credit Note.
/// - `Signature` — A signature applied to this document.
/// - `AccountingCustomerParty` — The accounting customer party.
/// - `AccountingSupplierParty` — The accounting supplier party.
/// - `PayeeParty` — The Party who receives the Payment.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `TaxRepresentativeParty` — The Party authorized to act as the Tax Representative for this Self Billed Credit Note.
/// - `Delivery` — A delivery associated with this document.
/// - `DeliveryTerms` — A set of delivery terms associated with this document.
/// - `PaymentMeans` — Expected means of payment.
/// - `PaymentTerms` — A set of payment terms associated with this document.
/// - `AllowanceCharge` — A discount or charge that applies to a price component.
/// - `TaxExchangeRate` — The exchange rate between the document currency and the tax currency.
/// - `PricingExchangeRate` — The exchange rate between the document currency and the pricing currency.
/// - `PaymentExchangeRate` — The exchange rate between the document currency and the payment currency.
/// - `PaymentAlternativeExchangeRate` — The exchange rate between the document currency and the payment alternative currency.
/// - `TaxTotal` — The total amount of a specific type of tax.
/// - `WithholdingTaxTotal` — The total withholding tax.
/// - `LegalMonetaryTotal` — The total amount payable on the Self Billed Credit Note, including Allowances, Charges, and Taxes.
/// - `CollectionCreditNoteLine` — A line describing an item or amount collected on behalf of a third party.
/// - `CreditNoteLine` — A Self Billed Credit Note Line.
// pub struct SelfBilledCreditNote { ... }

/// An Invoice document created by the Customer (rather than the Supplier) in a Self Billing relationship.
///
/// **UBL Dictionary Entry Name:** `Self Billed Invoice. Details`
///
/// Generated from XSD type `SelfBilledInvoiceType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `DueDate` — The date on which Invoice is due.
/// - `TaxPointDate` — The date of the invoice for tax purposes, in accordance with the applicable tax regulation.
/// - `InvoiceTypeCode` — A code signifying the type of invoice.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `DocumentCurrencyCode` — A code signifying the default currency for this document.
/// - `TaxCurrencyCode` — A code signifying the currency used for tax amounts in the Invoice.
/// - `PricingCurrencyCode` — A code signifying the currency used for prices in the Invoice.
/// - `PaymentCurrencyCode` — A code signifying the currency used for payment in the Invoice.
/// - `PaymentAlternativeCurrencyCode` — A code signifying the alternative currency used for payment in the Invoice.
/// - `AccountingCostCode` — An accounting cost code, applied to the Invoice as a whole.
/// - `AccountingCost` — An accounting cost code, applied to the Invoice as a whole, expressed as text.
/// - `LineCountNumeric` — The number of Invoice Lines in this document.
/// - `BuyerReference` — (Deprecated) A reference provided by the buyer used for internal routing of the document.
/// - `DefaultLanguageCode` — A code signifying the default natural language used by the sender for human-readable textual content that does not include a languageID.
/// - `InvoicePeriod` — A period to which the Self Billed Invoice applies.
/// - `OrderReference` — A reference to the Order with which this Invoice is associated.
/// - `BillingReference` — A reference to a billing document associated with this document.
/// - `ContractDocumentReference` — A reference to a contract associated with this document.
/// - `DespatchDocumentReference` — A reference to a Despatch Advice associated with this document.
/// - `WorkReportDocumentReference` — A reference to a Work Report associated with this document.
/// - `DeliveryNoteDocumentReference` — A reference to a Delivery Note associated with this document.
/// - `ReceiptDocumentReference` — A reference to a Receipt Advice associated with this document.
/// - `StatementDocumentReference` — A reference to a Statement associated with this document.
/// - `OriginatorDocumentReference` — A reference to an originator document associated with this document.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `ProjectReference` — A reference to a project associated with this document.
/// - `BuyerAssignedReference` — A reference provided by the buyer used for internal routing of the document.
/// - `PurchaseReference` — A reference to an object, such as a subscription number, telephone number, meter, vehicle, person, etc., to which this Invoice relates.
/// - `Annotation` — A structured annotation providing contextual or explanatory information related to this Invoice.
/// - `Signature` — A signature applied to this document.
/// - `AccountingCustomerParty` — The accounting customer party.
/// - `AccountingSupplierParty` — The accounting supplier party.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `OriginatorCustomerParty` — The Party who originated the Order to which this Invoice is related.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `PayeeParty` — The Party who receives the Payment.
/// - `TaxRepresentativeParty` — The Party authorized to act as the Tax Representative for this Self Billed Invoice.
/// - `Delivery` — A delivery associated with this document.
/// - `DeliveryTerms` — A set of delivery terms associated with this document.
/// - `PaymentMeans` — Expected means of payment.
/// - `PaymentTerms` — A set of payment terms associated with this document.
/// - `PrepaidPayment` — A prepaid payment.
/// - `AllowanceCharge` — A discount or charge that applies to a price component.
/// - `TaxExchangeRate` — The exchange rate between the document currency and the tax currency.
/// - `PricingExchangeRate` — The exchange rate between the document currency and the pricing currency.
/// - `PaymentExchangeRate` — The exchange rate between the document currency and the payment currency.
/// - `PaymentAlternativeExchangeRate` — The exchange rate between the document currency and the payment alternative currency.
/// - `TaxTotal` — The total amount of a specific type of tax.
/// - `WithholdingTaxTotal` — The total withholding tax.
/// - `LegalMonetaryTotal` — A set of totals associated with this Invoice that are required for the Invoice to be a legal document.
/// - `CollectionInvoiceLine` — A line describing an item or amount collected on behalf of a third party.
/// - `InvoiceLine` — A line describing an Invoice Item.
// pub struct SelfBilledInvoice { ... }

/// A document used to report the status of orders, billing, and payment. This document is a statement of account, not a summary invoice.
///
/// **UBL Dictionary Entry Name:** `Statement. Details`
///
/// Generated from XSD type `StatementType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `DocumentCurrencyCode` — The default currency for the Statement.
/// - `TotalDebitAmount` — The total of all debit amounts for the Statement.
/// - `TotalCreditAmount` — The total of all credit amounts for the Statement.
/// - `TotalBalanceAmount` — The total amount for the Statement.
/// - `LineCountNumeric` — The number of Statement Lines in the Statement.
/// - `StatementTypeCode` — A code signifying the type of the Statement.
/// - `StatementPeriod` — A period to which the Statement applies.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `AccountingSupplierParty` — The accounting supplier party.
/// - `AccountingCustomerParty` — The accounting customer party.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `OriginatorCustomerParty` — The originator.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `PayeeParty` — The Party who receives the Payment.
/// - `PaymentMeans` — Expected means of payment.
/// - `PaymentTerms` — A set of payment terms associated with this document.
/// - `AllowanceCharge` — A discount or charge that applies to a price component.
/// - `TaxTotal` — The total amount of a specific type of tax.
/// - `StatementLine` — A Statement Line.
// pub struct Statement { ... }

/// (Deprecated) A report on the quantities of each item that are, or will be, in stock. This document is sent by a Seller (for example a producer) to a Buyer (for example a retailer).
///
/// **UBL Dictionary Entry Name:** `Stock Availability Report. Details`
///
/// Generated from XSD type `StockAvailabilityReportType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `DocumentCurrencyCode` — A code signifying the default currency for this document.
/// - `InventoryPeriod` — The inventory period covered by the Report.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `SellerSupplierParty` — The seller.
/// - `RetailerCustomerParty` — The retailer.
/// - `InventoryReportingParty` — The party that will receive and use the Stock Availability Report (normally the branch for which the stock is reported).
/// - `StockAvailabilityReportLine` — A line representing a particular item of sale and associated with a line in the Catalogue.
// pub struct StockAvailabilityReport { ... }

/// A document whereby an economic operator (the tenderer) makes a formal offer (the tender) to a contracting authority to execute an order for the supply or purchase of goods, or for the execution of work, according to the terms of a proposed contract.
///
/// **UBL Dictionary Entry Name:** `Tender. Details`
///
/// Generated from XSD type `TenderType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `TenderTypeCode` — A code to specify the type of tender (economical or objective criteria versus technical or subjective criteria)
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `ContractName` — Short title of a contract associated with this Tender.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `ValidityPeriod` — The period for which the Tender is valid.
/// - `CallForTenderDocumentReference` — A reference to the call for tender document of which this tender result of.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `TendererParty` — The Party who submits this Tender.
/// - `TendererQualificationDocumentReference` — A reference to the tenderer qualification document that has been used to qualify the tenderer.
/// - `SubcontractorParty` — The Subcontractor or other Tenderer who participates in the same Tender.
/// - `ContractingParty` — The contracting party.
/// - `OriginatorCustomerParty` — The party originating the Tender.
/// - `BeneficiaryParty` — A Party for whom the associated transaction is ultimately intended or who derives benefit from it.
/// - `TenderedProject` — A project with which this Tender is associated. A single Tender can be used to bid for one project, multiple projects, or the global project.
// pub struct Tender { ... }

/// A document published by a Contracting Party to announce the awarding of a procurement project.
///
/// **UBL Dictionary Entry Name:** `Tender Contract. Details`
///
/// Generated from XSD type `TenderContractType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `ContractName` — Short title of a contract associated with this Tender.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `RegulatoryDomain` — Information about the law that defines the regulatory domain.
/// - `PublishAwardIndicator` — An indicator specifying if the notice is published for service contracts within certain service categories (true) or not (false).
/// - `PreviousDocumentReference` — A reference to a previously sent document.
/// - `ContractDocumentReference` — A reference to a set of minutes.
/// - `Signature` — A signature applied to this document.
/// - `ContractingParty` — The contracting party.
/// - `EconomicOperatorParty` — The Economic Operator issuing the inquiry on the status of a tendering process.
/// - `ReceiverParty` — The Party who receives this Document.
/// - `TenderingTerms` — The tendering terms associated with this tendering process.
/// - `TenderingProcess` — A description of the tendering process itself.
/// - `ProcurementProject` — An overall definition of this procurement project.
/// - `ProcurementProjectLot` — Lots that were awarded to the economic operator that can be grouped in the same contract.
/// - `TenderResult` — A result of the bid opening in the tendering process.
// pub struct TenderContract { ... }

/// A document sent by a contracting party to an economic operator acknowledging receipt of a Tender.
///
/// **UBL Dictionary Entry Name:** `Tender Receipt. Details`
///
/// Generated from XSD type `TenderReceiptType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `ContractName` — Short title of a contract associated with this Tender.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `RegisteredDate` — The date, assigned by the sender, on which the Tender Receipt was created.
/// - `RegisteredTime` — The time, assigned by the sender, at which the Tender Receipt was created.
/// - `TenderDocumentReference` — A reference to a received Tender.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Document.
/// - `ReceiverParty` — The Party who receives this Document.
// pub struct TenderReceipt { ... }

/// (Deprecated) A document sent by the Contracting Party to an Economic Operator describing the status of a tendering procedure.
///
/// **UBL Dictionary Entry Name:** `Tender Status. Details`
///
/// Generated from XSD type `TenderStatusType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `ContractName` — Short title of a contract associated with this Tender.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `ProcedureCode` — A code signifying the type of this tendering procedure.
/// - `TenderSubmissionDeadlinePeriod` — The period during which tenders must be delivered.
/// - `InvitationSubmissionPeriod` — The period during which invitations to tender must be completed and delivered.
/// - `ParticipationRequestReceptionPeriod` — The period during which requests for participation must be completed and delivered.
/// - `ProcurementLegislationDocumentReference` — A reference to a document providing references to procurement legislation applicable to the tendering process.
/// - `FiscalLegislationDocumentReference` — A reference to a document providing references to fiscal legislation applicable to the tendering process.
/// - `EnvironmentalLegislationDocumentReference` — A reference to a document providing references to environmental legislation applicable to the tendering process.
/// - `EmploymentLegislationDocumentReference` — A reference to a document providing references to employment legislation applicable to the tendering process.
/// - `TenderStatusInquiryDocumentReference` — A reference to a received Tender status inquiry.
/// - `Signature` — A signature applied to this document.
/// - `ContractingParty` — The Contracting Party issuing the information about the tender status.
/// - `EconomicOperatorParty` — The Economic Operator receiving the tender status information.
/// - `DocumentProviderParty` — The Party who has the Contract Documents for the tendering process.
/// - `TenderRecipientParty` — The Party who receives Tenders.
/// - `ProcurementProject` — An overall definition of this procurement project.
/// - `ProcurementProjectLot` — One of the procurement project lots into which this contract can be split.
// pub struct TenderStatus { ... }

/// (Deprecated) A document sent by an Economic Operator asking about the details and status of a tendering procedure.
///
/// **UBL Dictionary Entry Name:** `Tender Status Request. Details`
///
/// Generated from XSD type `TenderStatusRequestType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `ContractName` — Short title of a contract associated with this Tender.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `Signature` — A signature applied to this document.
/// - `ContractingParty` — The Contracting Party eceiving the tender status inquiry.
/// - `EconomicOperatorParty` — The Economic Operator issuing the inquiry on the status of a tendering process.
/// - `ProcurementProject` — An overall definition of this procurement project.
/// - `ProcurementProjectLot` — One of the procurement project lots into which this contract can be split.
// pub struct TenderStatusRequest { ... }

/// A document sent by an Economic Operator to a Contracting Party with the intention of withdrawing a previously sent Tender document.
///
/// **UBL Dictionary Entry Name:** `Tender Withdrawal. Details`
///
/// Generated from XSD type `TenderWithdrawalType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `ContractName` — Short title of a contract associated with this Tender.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `WithdrawOfferIndicator` — Indicates whether the referred tender has to be withdrawn (true) or not (false).
/// - `TenderDocumentReference` — A reference to a received Tender.
/// - `TenderNotificationDocumentReference` — A reference to the Tender Receipt Notification.
/// - `Signature` — A signature applied to this document.
/// - `ContractingParty` — The Contracting Party or parties in case of joint procurement.
/// - `TendererParty` — The economic operator or the main Tenderer in case of a consortium who withdraws the Tender.
// pub struct TenderWithdrawal { ... }

/// A document declaring the qualifications of a tenderer.
///
/// **UBL Dictionary Entry Name:** `Tenderer Qualification. Details`
///
/// Generated from XSD type `TendererQualificationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Indicates the current version of the Tenderer Qualification.
/// - `PreviousVersionID` — Identifies the previous version of the Tenderer Qualification which is superceded by this version.
/// - `Signature` — A signature applied to this document.
/// - `TendererPartyQualification` — A specific qualification of the Tenderer.
/// - `ContractingParty` — The contracting party.
/// - `Evidence` — An evidentiary document supporting Tenderer qualifications.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
// pub struct TendererQualification { ... }

/// A document issued by a procurement organization to notify an economic operator whether it has been admitted to or excluded from the tendering process.
///
/// **UBL Dictionary Entry Name:** `Tenderer Qualification Response. Details`
///
/// Generated from XSD type `TendererQualificationResponseType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `ContractName` — Short title of a contract associated with this Tender.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `SenderParty` — The Party who sends this message.
/// - `ReceiverParty` — The Party who receives this message.
/// - `ResolutionDocumentReference` — A document (e.g., meeting minutes) relating to consideration of tenderer qualifications.
/// - `QualificationResolution` — An association to the resolution that is being notified
/// - `AppealTerms` — Terms of appeal for this tendering process.
/// - `Signature` — A signature applied to this document.
// pub struct TendererQualificationResponse { ... }

/// A document specifying trade item attributes relating to replenishment policies.
///
/// **UBL Dictionary Entry Name:** `Trade Item Location Profile. Details`
///
/// Generated from XSD type `TradeItemLocationProfileType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `ProfileStatusCode` — A code signifying the status of this Trade Item Location Profile.
/// - `Period` — An association to Period.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Trade Item Location Profile
/// - `ReceiverParty` — The Party who receives this Trade Item Location Profile.
/// - `BuyerCustomerParty` — The buyer.
/// - `SellerSupplierParty` — The seller.
/// - `ItemManagementProfile` — A profile specifying replenishment policies for a particular trade item.
// pub struct TradeItemLocationProfile { ... }

/// A document that describes an import customs declaration.
///
/// **UBL Dictionary Entry Name:** `Transit Customs Declaration. Details`
///
/// Generated from XSD type `TransitCustomsDeclarationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `TypeCode` — Code specifying the type of transit customs declaration.
/// - `SubTypeCode` — Code specifying the subtype of transit customs declaration.
/// - `NatureOfTransactionCode` — Code specifying the type of transaction for this export.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `VersionID` — Identifies a version of a transit customs declaration in order to distinguish updates.
/// - `ValidityPeriod` — A period, assigned by the issuer, during which the information in this declaration is effective.
/// - `ExportCustomsExitOfficeLocation` — Customs office of exit of the goods being declared for export.
/// - `TransitCustomsExitOfficeLocation` — Customs office of exit of the goods being declared for transit.
/// - `ImportCustomsExitOfficeLocation` — Customs office of exit of the goods being declared for import.
/// - `JurisdictionRegionAddress` — A geographic area in which this declaration is relevant.
/// - `TransitExporterParty` — The Party who makes the Transit Customs Declaration, or on whose behalf the Transit Customs Declaration is made. This Party is the owner of the Goods or has similar right of disposal over them at the time when the Declaration is accepted.
/// - `ConsignorParty` — The Party who is reponsible for sending the goods.
/// - `ConsigneeParty` — The Party who receives the goods.
/// - `FreightForwarderParty` — The Party who combines individual smaller consignments into a single larger shipment (a so-called consolidated consignment or shipment) which is sent to a counterpart who mirrors the consolidator's activity by dividing the consolidated consignment into its original components.
/// - `CustomsParty` — The Authority who is legally responsible for processing the Declaration.
/// - `NotifierParty` — The Party who is responsible for contact on master level.
/// - `Shipment` — The shipment related to this trade certificate.
/// - `PreviousCustomsDeclaration` — A reference to a previously sent transit customs declaration.
/// - `AdditionalDocumentReference` — A reference to a additional documents relevant for this transit customs declaration.
/// - `Signature` — A signature applied to this document.
// pub struct TransitCustomsDeclaration { ... }

/// A document used in the negotiation of a transport service between a transport user and a transport service provider.
///
/// **UBL Dictionary Entry Name:** `Transport Execution Plan. Details`
///
/// Generated from XSD type `TransportExecutionPlanType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `VersionID` — Indicates the current version of the Transport Execution Plan.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `DocumentStatusCode` — A code signifying the status of the Transport Execution Plan (updated, cancelled, confirmed, etc.)
/// - `DocumentStatusReasonCode` — A code signifying a reason associated with the status of a Transport Execution Plan.
/// - `DocumentStatusReasonDescription` — A reason for the status assigned to the Transport Execution Plan, expressed in text.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `TransportUserRemarks` — Remarks from the transport user regarding the transport operations referred to in the Transport Execution Plan.
/// - `TransportServiceProviderRemarks` — Remarks from the transport service provider regarding the transport operations referred to in the Transport Execution Plan.
/// - `SenderParty` — The Party who sends this Document. This Party is normally the transport user or the Transport Service Provider.
/// - `ReceiverParty` — The Party who receives this Document. This Party is normally the transport user or the Transport Service Provider.
/// - `TransportUserParty` — The Party who requests the transport service from a Transport Service Provider.
/// - `TransportServiceProviderParty` — The Party who offers the transport service based upon a request from a transport user.
/// - `BillToParty` — The Party who executes the Payment for the transport service provided in the Transport Execution Plan.
/// - `Signature` — A signature applied to this document.
/// - `TransportExecutionPlanRequestDocumentReference` — A reference to a Transport Execution Plan Request.
/// - `TransportExecutionPlanDocumentReference` — A reference to an original Transport Execution Plan.
/// - `TransportServiceDescriptionDocumentReference` — A reference to the Transport Service Description, which is used by a transport service provider to announce transport services to transport users (buyers).
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `TransportContract` — A contract related to the Transport Execution Plan.
/// - `TransportServiceProviderResponseRequiredPeriod` — Describes the deadline for when the Transport Service Provider will have to respond to a Transport Execution Plan .
/// - `TransportUserResponseRequiredPeriod` — Describes the deadline for when the Transport User will have to respond to a Transport Execution Plan suggested by a Transport Service Provider.
/// - `ValidityPeriod` — A period during which the Transport Execution Plan is valid.
/// - `MainTransportationService` — Description of the main transportation service referenced in the Transport Execution Plan.
/// - `AdditionalTransportationService` — A description of an additional transportation service referenced in the Transport Execution Plan.
/// - `ServiceStartTimePeriod` — The period within which the service must begin.
/// - `ServiceEndTimePeriod` — The period during which the service must be completed.
/// - `FromLocation` — The location of origin of the transport service referenced in the Transport Execution Plan.
/// - `ToLocation` — The destination location for the transport service referenced in the Transport Execution Plan.
/// - `AtLocation` — The location of a transport service (e.g., terminal handling service) that does not require transport movement.
/// - `TransportExecutionTerms` — A description of terms and conditions related to the Transport Execution Plan.
/// - `Consignment` — A description of an identifiable collection of goods items to be transported between the consignor and the consignee. This information may be defined within a transport contract. A consignment may comprise more than one shipment (e.g., when consolidated by a freight forwarder).
// pub struct TransportExecutionPlan { ... }

/// A document sent by a transport user to request a transport service from a transport service provider.
///
/// **UBL Dictionary Entry Name:** `Transport Execution Plan Request. Details`
///
/// Generated from XSD type `TransportExecutionPlanRequestType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `VersionID` — An identifier for the current version of the Transport Execution Plan Request.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `DocumentStatusCode` — A code signifying the status of the Transport Execution Plan Request.
/// - `DocumentStatusReasonCode` — A code signifying a reason associated with the status of the Transport Execution Plan Request.
/// - `DocumentStatusReasonDescription` — A reason associated with the status of the Transport Execution Plan Request.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `TransportUserRemarks` — Remarks from the transport user regarding the transport operations referenced in the Transport Execution Plan Request.
/// - `SenderParty` — The Party who sends the Transport Execution Plan Request.
/// - `ReceiverParty` — The Party who receives the Transport Execution Plan Request.
/// - `TransportUserParty` — The Party who requests the transport services referenced in the Transport Execution Plan Request.
/// - `TransportServiceProviderParty` — The Party who provides the transport services referenced in the Transport Execution Plan Request.
/// - `PayeeParty` — (Deprecated) The party that will pay for the transport service(s) referred to in a Transport Execution Plan.
/// - `BillToParty` — The Party that will receive the invoice for the transport service(s) referred to in the Transport Execution Plan.
/// - `Signature` — A signature applied to this document.
/// - `TransportExecutionPlanDocumentReference` — A reference to an original Transport Execution Plan Document.
/// - `TransportServiceDescriptionDocumentReference` — A reference to the Transport Service Description, which is used by a transport service provider to announce transport services to transport users (buyers).
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `TransportContract` — A potential contract related to the Transport Execution Plan Request.
/// - `TransportServiceProviderResponseDeadlinePeriod` — A deadline for a response from the Transport Service Provider to this Transport Execution Plan Request.
/// - `MainTransportationService` — A description of the main transportation service referenced in the Transport Execution Plan Request.
/// - `AdditionalTransportationService` — A description of an additional transportation service referenced in the Transport Execution Plan Request.
/// - `ServiceStartTimePeriod` — The period within which the services referred to in the Transport Execution Plan Request must begin.
/// - `ServiceEndTimePeriod` — The period during which the services referred to in the Transport Execution Plan Request must be completed.
/// - `FromLocation` — The location of origin of the transport service referenced in the Transport Execution Plan Request.
/// - `ToLocation` — The destination location for the transport service referenced in the Transport Execution Plan Request.
/// - `AtLocation` — The location of a transport service (e.g., terminal handling service) that does not require transport movement.
/// - `TransportExecutionTerms` — A description of terms and conditions related to the Transport Execution Plan Request.
/// - `Consignment` — A description of an identifiable collection of goods items to be transported between the consignor and the consignee. This information may be defined within a transport contract. A consignment may comprise more than one shipment (e.g., when consolidated by a freight forwarder).
// pub struct TransportExecutionPlanRequest { ... }

/// A document sent from a Transportation Network Manager to a Transport Service Provider giving the status of the whereabouts and schedule of the transport means involved in a transport service.
///
/// **UBL Dictionary Entry Name:** `Transport Progress Status. Details`
///
/// Generated from XSD type `TransportProgressStatusType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `StatusAvailableIndicator` — Indicates whether transport progress information is available.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends the Transport Progress Status.
/// - `ReceiverParty` — The Party who receives the Transport Progress Status.
/// - `SourceIssuerParty` — The Party who issues the Transport Progress Status.
/// - `TransportProgressStatusRequestDocumentReference` — A reference to the Transport Progress Status Request document to which this status report is a response.
/// - `TransportMeans` — The transport means by which the current transport service is effectuated.
/// - `TransportSchedule` — Describes the status and schedule of the transport means operating the transport service as well as the current location of the transport means.
// pub struct TransportProgressStatus { ... }

/// A document sent from a transport service provider to a transportation network manager requesting a Transport Progress Status.
///
/// **UBL Dictionary Entry Name:** `Transport Progress Status Request. Details`
///
/// Generated from XSD type `TransportProgressStatusRequestType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends the Transport Progress Status Request.
/// - `ReceiverParty` — The Party who receives the Transport Progress Status Request.
/// - `TransportMeans` — The transport means by which the current transport service is effectuated and for which status is requested.
/// - `StatusLocation` — A location for which status is requested.
// pub struct TransportProgressStatusRequest { ... }

/// A document sent by a transport service provider to announce the availability of a transport service.
///
/// **UBL Dictionary Entry Name:** `Transport Service Description. Details`
///
/// Generated from XSD type `TransportServiceDescriptionType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `ServiceName` — A name, assigned by the Transport Service Provider, for the service being announced.
/// - `ResponseCode` — A code signifying a response related to the Transport Service Description.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends the Transport Service Description.
/// - `ReceiverParty` — The Party who receives the Transport Service Description.
/// - `TransportServiceDescriptionRequestDocumentReference` — A Transport Service Description Request to which this Transport Service Description is a response.
/// - `TransportServiceProviderParty` — The Transport Service Provider.
/// - `ServiceChargePaymentTerms` — The terms of payment under which the transport service would be provided.
/// - `ValidityPeriod` — A period during which this Transport Service Description is valid.
/// - `TransportationService` — A transportation service announced in this Transport Service Description.
// pub struct TransportServiceDescription { ... }

/// A document requesting a Transport Service Description, sent from a party with a transport demand (transport user) to a party providing transport services (transport service provider).
///
/// **UBL Dictionary Entry Name:** `Transport Service Description Request. Details`
///
/// Generated from XSD type `TransportServiceDescriptionRequestType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `ServiceInformationPreferenceCode` — A code signifying the category of service information requested to be provided in the Transport Service Description.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends the Transport Service Description Request.
/// - `ReceiverParty` — The Party who receives the Transport Service Description Request.
/// - `TransportServiceProviderParty` — The Transport Service Provider.
/// - `TransportationService` — A transportation service about which information is requested.
// pub struct TransportServiceDescriptionRequest { ... }

/// A document to circulate reports of transportation status or changes in status (events) among a group of participants.
///
/// **UBL Dictionary Entry Name:** `Transportation Status. Details`
///
/// Generated from XSD type `TransportationStatusType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CarrierAssignedID` — A reference number assigned by a carrier or its agent to identify a specific shipment, such as a booking reference number when cargo space is reserved prior to loading.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Name` — Text, assigned by the sender, that identifies this document to business users.
/// - `Description` — A textual description of transportation status.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `ShippingOrderID` — A reference number for a shipping order.
/// - `OtherInstruction` — An instruction regarding this message.
/// - `TransportationStatusTypeCode` — A code signifying the type of status provided in a Transportation Status document.
/// - `TransportExecutionStatusCode` — A code signifying the overall status of transport service execution.
/// - `Consignment` — A consignment associated with this Transportation Status report.
/// - `TransportEvent` — Any additional events associated with this Transportation Status report that are not defined elsewhere in this document.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Transportation Status Report.
/// - `ReceiverParty` — The Party who receives this Transportation Status Report.
/// - `TransportationStatusRequestDocumentReference` — A reference to the Transportation Status Request to which this report is a response.
/// - `TransportExecutionPlanDocumentReference` — A reference to the Transport Execution Plan associated with the transport service whose status is being reported.
/// - `UpdatedPickupTransportEvent` — Update of the original plan regarding a pickup of goods.
/// - `UpdatedDeliveryTransportEvent` — Update of the original plan regarding a delivery.
/// - `StatusLocation` — Locations associated with this Transportation Status report.
/// - `StatusPeriod` — A period for which status is provided.
// pub struct TransportationStatus { ... }

/// A document requesting a Transportation Status report.
///
/// **UBL Dictionary Entry Name:** `Transportation Status Request. Details`
///
/// Generated from XSD type `TransportationStatusRequestType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CarrierAssignedID` — A reference number assigned by a carrier or its agent to identify a specific shipment, such as a booking reference number when cargo space is reserved prior to loading.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Name` — Text, assigned by the sender, that identifies this document to business users.
/// - `Description` — A textual description of the document instance.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `ShippingOrderID` — A reference number for a shipping order.
/// - `OtherInstruction` — An instruction regarding this message.
/// - `TransportationStatusTypeCode` — A code signifying the type of status requested in a Transportation Status document.
/// - `SenderParty` — The Party who sends this Document.
/// - `ReceiverParty` — The Party who receives this Document.
/// - `TransportExecutionPlanDocumentReference` — A reference to the Transport Execution Plan associated with the transport service for which status is requested.
/// - `Consignment` — A consignment regarding which status is requested.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `RequestedStatusLocation` — A location for which status is requested.
/// - `RequestedStatusPeriod` — A period for which status is requested.
// pub struct TransportationStatusRequest { ... }

/// A document communicating to a tenderer that the contract has been awarded to different tenderer.
///
/// **UBL Dictionary Entry Name:** `Unawarded Notification. Details`
///
/// Generated from XSD type `UnawardedNotificationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `ContractName` — The name, expressed as text, of this procurement project.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Document.
/// - `ReceiverParty` — The Party who receives this Document.
/// - `MinutesDocumentReference` — A reference to a set of minutes associated with this award.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document. It can be used to include annex documents such as the minutes of the awarding process meetings.
/// - `TenderResult` — An association to the Tender Result being notified
/// - `AppealTerms` — Terms of appeal for this tendering process.
// pub struct UnawardedNotification { ... }

/// A request to unsubscribe from a tendering procedure. Economic Operators can subscribe to a tendering procedure using the Expression Of Interest. Upon subscription, the Economic Operator keeps receiving relevant documentation for the tendering process. The unsubscribe to procedure document allows the Economic Operator to be removed from the list of interested parties.
///
/// **UBL Dictionary Entry Name:** `Unsubscribe From Procedure Request. Details`
///
/// Generated from XSD type `UnsubscribeFromProcedureRequestType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — The earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `Signature` — A signature applied to this document.
/// - `EconomicOperatorParty` — The Economic Operator issuing this unsubscribe request.
/// - `ContractingParty` — The Contracting Party.
/// - `ProcurementProject` — An overall definition of this procurement project.
/// - `ProcurementProjectLotReference` — One of the procurement project lots into which this contract can be split.
// pub struct UnsubscribeFromProcedureRequest { ... }

/// A document sent from a Contracting Party to the Economic Operator confirming that the latter has been unsubscribed from a tendering procedure.
///
/// **UBL Dictionary Entry Name:** `Unsubscribe From Procedure Response. Details`
///
/// Generated from XSD type `UnsubscribeFromProcedureResponseType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — The earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `ContractFolderID` — An identifier, assigned by the sender, for the process file (i.e., record) to which this document belongs.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `UnsubscribeToProcedureDocumentReference` — A reference to the unsubscribe to procedure document associated with this confirmation.
/// - `Signature` — A signature applied to this document.
/// - `EconomicOperatorParty` — The Economic Operator receiving this unsubscribe to procedure confirmation.
/// - `ContractingParty` — The Contracting Party.
/// - `ProcurementProject` — An overall definition of this procurement project.
/// - `ProcurementProjectLotReference` — One of the procurement project lots into which this contract can be split.
// pub struct UnsubscribeFromProcedureResponse { ... }

/// A supplement to an Invoice or Credit Note, containing information on the consumption of services provided by utility suppliers to private and public customers, including electricity, gas, water, and telephone services.
///
/// **UBL Dictionary Entry Name:** `Utility Statement. Details`
///
/// Generated from XSD type `UtilityStatementType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CopyIndicator` — (Deprecated) Indicates whether this document is a copy (true) or not (false).
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `UtilityStatementTypeCode` — A code signifying the type of Utility Statement.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `DocumentCurrencyCode` — A code signifying the default currency for this document.
/// - `AccountingCostCode` — The buyer's accounting cost code, applied to the UtilityStatement.
/// - `AccountingCost` — The buyer's accounting cost code, applied to the UtilityStatement, expressed as text.
/// - `ParentDocumentReference` — A reference to the parent Invoice or Credit Note.
/// - `AdditionalDocumentReference` — A reference to an additional document associated with this document.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Utility Statement.
/// - `ReceiverParty` — The Party who receives this Utility Statement.
/// - `CustomerParty` — The buyer, if different from the receiver of the document.
/// - `SubscriberParty` — The Party that is the subscriber of the utility.
/// - `MainOnAccountPayment` — A payment on an account.
/// - `SubscriberConsumption` — A utility statement for a particular consumption point.
// pub struct UtilityStatement { ... }

/// A document used to report the transport of waste.
///
/// **UBL Dictionary Entry Name:** `Waste Movement. Details`
///
/// Generated from XSD type `WasteMovementType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `WasteMovementTypeCode` — A code signifying the type of this Waste Movement.
/// - `SequenceNumberID` — Sequence number of this Waste Movement Document referring to the Waste Notification Document.
/// - `ConsignmentQuantity` — Estimated total number of shipments for the Waste Notification document.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party sending this document.
/// - `ReceiverParty` — The Party receiving this document.
/// - `NotifierParty` — The Party that acts as the notifier in relation to the Waste Movements covered by this document.
/// - `DisposalFacilityParty` — The Party disposing of the waste material.
/// - `RecoveryFacilityParty` — The Party recovering the waste material.
/// - `WasteProducerParty` — The Party producing the waste material.
/// - `Shipment` — The relevant shipment information describing the planned transport and the waste material.
/// - `WasteNotificationDocumentReference` — A reference to the Waste Notification document.
/// - `WeightStatementDocumentReference` — A reference to a Weight Statement document.
/// - `DocumentReference` — A reference to a relevant document associated with this Waste Movement.
/// - `DocumentDistribution` — A distribution of this document to an interested Party.
// pub struct WasteMovement { ... }

/// A document used to notify the competent authorities of planned movements of waste.
///
/// **UBL Dictionary Entry Name:** `Waste Notification. Details`
///
/// Generated from XSD type `WasteNotificationType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `WasteNotificationTypeCode` — A code signifying the type of this Waste Notification.
/// - `ConsignmentQuantity` — Estimated total number of shipments for this Notification Document.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party sending this document.
/// - `ReceiverParty` — The Party receiving this document.
/// - `NotifierParty` — The Party responsible for providing the Waste Notification information and acting in the role of notifier in the waste movement process.
/// - `CustomsParty` — The competent authority granting a permit to export or import the waste.
/// - `DisposalFacilityParty` — The Party disposing of the waste material.
/// - `RecoveryFacilityParty` — The Party recovering the waste material.
/// - `WasteProducerParty` — The Party producing the waste material.
/// - `Shipment` — The relevant shipment information describing the planned transport and the waste material.
/// - `DocumentReference` — A reference to a relevant document associated with this Waste Movement.
// pub struct WasteNotification { ... }

/// A transport document describing a shipment It is issued by the party who undertakes to provide transportation services, or undertakes to arrange for their provision, to the party who gives instructions for the transportation services (shipper, consignor, etc.). It states the instructions for the beneficiary and may contain the details of the transportation, charges, and terms and conditions under which the transportation service is provided.
///
/// **UBL Dictionary Entry Name:** `Waybill. Details`
///
/// Generated from XSD type `WaybillType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `CarrierAssignedID` — An identifier (in the form of a reference number) assigned by a carrier or its agent to identify a specific shipment.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `VersionID` — The version of this waybill.
/// - `StatusCode` — The status of this waybill (draft, signed, approved, etc.), expressed as a code.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `Name` — Text, assigned by the sender, that identifies this document to business users.
/// - `Description` — Text describing the contents of the Waybill.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `ShippingOrderID` — An identifier (in the form of a reference number) of the Shipping Order or Forwarding Instruction associated with this shipment.
/// - `WaybillTypeCode` — The type of waybill (Bill of Laden, Airwaybill, CMR, House waybill, etc.) expressed as a code.
/// - `ConsolidatedIndicator` — An indicator of whether this waybill is consolidated from other waybills (true) or not (false).
/// - `AdValoremIndicator` — A term used in commerce in reference to certain duties, called ad valorem duties, which are levied on commodities at certain rates per centum on their value.
/// - `DeclaredCarriageValueAmount` — Value declared by the shipper or his agent solely for the purpose of varying the carrier's level of liability from that provided in the contract of carriage in case of loss or damage to goods or delayed delivery.
/// - `OtherInstruction` — Other free-text instructions related to the shipment to the forwarders or carriers. This ought to be used only where such information cannot be represented in other structured information entities within the document.
/// - `IssueLocation` — The location where this waybill was issued.
/// - `SenderParty` — The Party who sends this Waybill.
/// - `ReceiverParty` — The Party who receives this Waybill.
/// - `ConsignorParty` — The Party who is reponsible for sending the goods.
/// - `CarrierParty` — The Party who provides the transport of goods between named points.
/// - `FreightForwarderParty` — The Party who combines individual smaller consignments into a single larger shipment (a so-called consolidated consignment or shipment) which is sent to a counterpart who mirrors the consolidator's activity by dividing the consolidated consignment into its original components.
/// - `Shipment` — A description of the shipment.
/// - `DocumentReference` — A reference to another document associated with this document.
/// - `ExchangeRate` — Information about the rate of exchange (conversion) between two currencies.
/// - `DocumentDistribution` — A list of interested parties to whom this document is distributed.
/// - `Signature` — A signature applied to this document.
// pub struct Waybill { ... }

/// A document used to report weight or verified mass measurements in the transport chain.
///
/// **UBL Dictionary Entry Name:** `Weight Statement. Details`
///
/// Generated from XSD type `WeightStatementType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this document, assigned by the sender.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `IssueDate` — The date, assigned by the sender, on which this document was issued.
/// - `IssueTime` — The time, assigned by the sender, at which this document was issued.
/// - `WeightStatementTypeCode` — A code signifying the type of Weight Statement.
/// - `Signature` — A signature applied to this document.
/// - `SenderParty` — The Party who sends this Weight Statement (e.g. Weighing Station, Shipper, Freight Forwarder, Carrier, ...).
/// - `ReceiverParty` — The Party who receives this Weight Statement (e.g. carrier, terminal operator, ...).
/// - `WeighingParty` — The Party who executes the weight measure (e.g. weighing station).
/// - `ShipperParty` — The Party who plays the role of the Shipper (BCO, FF or NVOCC) who is responsible for the VGM (e.g. according the SOLAS Convention).
/// - `ResponsibleParty` — The Party who signs the Verified Gross Mass (VGM) on behalf of the Shipper.
/// - `Shipment` — The relevant shipment information with details on the transport equipment weight or mass measurements, including verified gross mass (VGM) information.
// pub struct WeightStatement { ... }

/// A document used to report work performed.
///
/// **UBL Dictionary Entry Name:** `Work Report. Details`
///
/// Generated from XSD type `WorkReportType`.
/// - `UBLExtensions` — A container for extensions foreign to the document.
/// - `UBLVersionID` — Identifies the earliest version of the UBL 2 schema for this document type that defines all of the elements that might be encountered in the current instance.
/// - `CustomizationID` — Identifies a user-defined customization of UBL for a specific use.
/// - `ProfileID` — Identifies a user-defined profile of the customization of UBL being used.
/// - `ProfileExecutionID` — Identifies an instance of executing a profile, to associate all transactions in a collaboration.
/// - `ID` — An identifier for this Work Report.
/// - `UUID` — A universally unique identifier for an instance of this document.
/// - `VersionID` — Identifies a version of this work report.
/// - `IssueDate` — The date on which this work report was issued.
/// - `Note` — Free-form text pertinent to this document, conveying information that is not contained explicitly in other structures.
/// - `AccountingCostCode` — An accounting cost code applied to this Work Report.
/// - `AccountingCost` — An accounting cost centre or account to which this Work Report is charged.
/// - `Signature` — A signature applied to this document.
/// - `WorkQuantityTotal` — A total quantity of work reported in this Work Report.
/// - `ReportedPeriod` — The period during which the reported work was performed.
/// - `OrderReference` — A reference to the related Order.
/// - `ProjectReference` — A reference to the related project.
/// - `BillingReference` — A reference to a related billing document.
/// - `AdditionalDocumentReference` — A reference to an additional supporting document.
/// - `SellerSupplierParty` — The party providing the work.
/// - `BuyerCustomerParty` — The party receiving the work.
/// - `ApproverParty` — The party approving this work report.
/// - `AllowanceCharge` — A discount or charge applied at the document level.
/// - `TaxTotal` — A total amount of taxes of a particular kind applicable to this Work Report.
/// - `StatementMonetaryTotal` — The total amounts for this Work Report.
/// - `WorkReportLine` — A line describing reported work.
// pub struct WorkReport { ... }
