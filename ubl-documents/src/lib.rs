// UBL 2.5 Document Types
//
// Reference: https://docs.oasis-open.org/ubl/cs01-UBL-2.5/UBL-2.5.html
//
// Complete inventory of 101 UBL 2.5 document types, organized by business process:
//
// BILLING (10): Invoice, CreditNote, DebitNote, SelfBilledInvoice,
//               SelfBilledCreditNote, Reminder, Statement, UtilityStatement,
//               RemittanceAdvice, FreightInvoice
//
// ORDERING (5): Order, OrderResponse, OrderResponseSimple, OrderChange, OrderCancellation
//
// DESPATCH (6): DespatchAdvice, ReceiptAdvice, FulfilmentCancellation,
//               DeliveryNote, PackingList, InstructionForReturns
//
// CATALOGUE (5): Catalogue, CatalogueRequest, CatalogueItemSpecificationUpdate,
//                CataloguePricingUpdate, CatalogueDeletion
//
// QUOTATION (2): RequestForQuotation, Quotation
//
// TENDERING (22): Tender, TenderReceipt, TenderStatus, TenderStatusRequest,
//                 TenderWithdrawal, TendererQualification, TendererQualificationResponse,
//                 TenderContract, AwardedNotification, UnawardedNotification,
//                 CallForTenders, ContractNotice, ContractAwardNotice,
//                 PriorInformationNotice, ExpressionOfInterestRequest,
//                 ExpressionOfInterestResponse, QualificationApplicationRequest,
//                 QualificationApplicationResponse, UnsubscribeFromProcedureRequest,
//                 UnsubscribeFromProcedureResponse, Enquiry, EnquiryResponse
//
// TRANSPORTATION (16): BillOfLading, Waybill, CertificateOfOrigin,
//                      ForwardingInstructions, TransportationStatus,
//                      TransportationStatusRequest, TransportExecutionPlan,
//                      TransportExecutionPlanRequest, TransportServiceDescription,
//                      TransportServiceDescriptionRequest, TransportProgressStatus,
//                      TransportProgressStatusRequest, GoodsItemItinerary,
//                      GoodsItemPassport, Manifest, CommonTransportationReport
//
// INVENTORY (5): InventoryReport, StockAvailabilityReport, ProductActivity,
//                ItemInformationRequest, TradeItemLocationProfile
//
// FORECAST (2): Forecast, ForecastRevision
//
// STATUS (8): ApplicationResponse, DocumentStatus, DocumentStatusRequest,
//             InvoiceStatusRequest, InvoiceStatusResponse, ExceptionCriteria,
//             ExceptionNotification, ProcurementStatus, ProcurementStatusRequest
//
// CUSTOMS (9): ExportCustomsDeclaration, ImportCustomsDeclaration,
//              TransitCustomsDeclaration, ProofOfReexportation,
//              ProofOfReexportationReminder, ProofOfReexportationRequest,
//              GoodsCertificate, GuaranteeCertificate, PurchaseReceipt
//
// DIRECTORY (5): BusinessCard, BusinessInformation, DigitalAgreement,
//                DigitalCapability, AttachedDocument (Deprecated)
//
// WASTE (2): WasteMovement, WasteNotification
//
// OTHER (4): RetailEvent, WeightStatement, WorkReport
//
// Implementation status: PLANNING ONLY — no code yet.
