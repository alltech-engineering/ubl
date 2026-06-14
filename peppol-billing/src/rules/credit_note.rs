// Peppol BIS Billing 3.0 — CreditNote Business Rules
//
// Validates credit note documents per EN16931/Peppol BIS Billing 3.0.
// A credit note reduces a previously issued invoice and has additional
// requirements beyond those for invoices.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::billing::CreditNote;

pub fn add_rules(engine: &mut RuleEngine, cn: &Arc<CreditNote>) {
    // ── CN-R001: CreditNote ID must be present and non-empty ──────────────
    engine.add_rule(Rule {
        id: "PEPPOL-CN-R001".into(),
        description: "CreditNote ID must be present and non-empty".into(),
        severity: Severity::Fatal,
        check: {
            let cn = Arc::clone(cn);
            Box::new(move || {
                let v = cn.id.value();
                if v.is_empty() {
                    Err("CreditNote ID is empty — a non-empty credit note number is required"
                        .into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── CN-R002: Issue date must be present ───────────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-CN-R002".into(),
        description: "CreditNote issue date must be present".into(),
        severity: Severity::Fatal,
        check: {
            let cn = Arc::clone(cn);
            Box::new(move || {
                let _date = &cn.issue_date;
                Ok(())
            })
        },
    });

    // ── CN-R003: CreditNoteTypeCode must be present ───────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-CN-R003".into(),
        description: "CreditNoteTypeCode must be present".into(),
        severity: Severity::Fatal,
        check: {
            let cn = Arc::clone(cn);
            Box::new(move || match &cn.credit_note_type_code {
                None => Err(
                    "CreditNoteTypeCode is missing — required for credit note documents"
                        .into(),
                ),
                Some(tc) if tc.value().is_empty() => {
                    Err("CreditNoteTypeCode is present but empty".into())
                }
                Some(_) => Ok(()),
            })
        },
    });

    // ── CN-R004: Document currency code must be present ──────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-CN-R004".into(),
        description: "Document currency code must be present".into(),
        severity: Severity::Fatal,
        check: {
            let cn = Arc::clone(cn);
            Box::new(move || match &cn.document_currency_code {
                None => Err(
                    "Document currency code is missing — required for credit note documents"
                        .into(),
                ),
                Some(cc) if cc.value().is_empty() => {
                    Err("Document currency code is present but empty".into())
                }
                Some(_) => Ok(()),
            })
        },
    });

    // ── CN-R005: Must reference original invoice ─────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-CN-R005".into(),
        description:
            "CreditNote must reference the original invoice via BillingReference.InvoiceDocumentReference.ID"
                .into(),
        severity: Severity::Fatal,
        check: {
            let cn = Arc::clone(cn);
            Box::new(move || {
                let has_ref = cn.billing_reference.iter().any(|br| {
                    br.invoice_document_reference
                        .as_ref()
                        .and_then(|doc| doc.id.as_ref())
                        .map(|id| !id.value().is_empty())
                        .unwrap_or(false)
                });
                if has_ref {
                    Ok(())
                } else {
                    Err(
                        "CreditNote must reference the original invoice via BillingReference/InvoiceDocumentReference/ID"
                            .into(),
                    )
                }
            })
        },
    });

    // ── CN-R006: Reason for credit must be documented in notes ──────────
    engine.add_rule(Rule {
        id: "PEPPOL-CN-R006".into(),
        description: "Reason for credit must be documented in notes".into(),
        severity: Severity::Fatal,
        check: {
            let cn = Arc::clone(cn);
            Box::new(move || {
                if cn.note.is_empty() {
                    Err("No credit note notes provided — the reason for the credit must be documented".into())
                } else if cn.note.iter().all(|n| n.value().trim().is_empty()) {
                    Err(
                        "Credit note notes are present but all are empty — provide a meaningful reason"
                            .into(),
                    )
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── CN-R007: Line total must be negative or zero ─────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-CN-R007".into(),
        description: "Line total must be negative or zero (credit notes reduce amounts)".into(),
        severity: Severity::Error,
        check: {
            let cn = Arc::clone(cn);
            Box::new(move || {
                let total = cn.legal_monetary_total.line_extension_amount.value();
                if *total <= rust_decimal::Decimal::ZERO {
                    Ok(())
                } else {
                    Err(format!(
                        "Line extension amount is positive ({}) — credit notes must have negative or zero totals",
                        total
                    ))
                }
            })
        },
    });

    // ── CN-R008: Supplier must have party identification ─────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-CN-R008".into(),
        description: "Supplier must have party identification".into(),
        severity: Severity::Fatal,
        check: {
            let cn = Arc::clone(cn);
            Box::new(move || {
                let has_id = cn
                    .accounting_supplier_party
                    .party
                    .as_ref()
                    .map(|p| !p.party_identification.is_empty())
                    .unwrap_or(false);
                if has_id {
                    Ok(())
                } else {
                    Err("Supplier party identification is missing — required for credit note documents".into())
                }
            })
        },
    });

    // ── CN-R009: Customer must have party identification (if present) ────
    engine.add_rule(Rule {
        id: "PEPPOL-CN-R009".into(),
        description: "Customer must have party identification if present".into(),
        severity: Severity::Fatal,
        check: {
            let cn = Arc::clone(cn);
            Box::new(move || match &cn.accounting_customer_party {
                None => Ok(()),
                Some(cp) => {
                    let has_id = cp
                        .party
                        .as_ref()
                        .map(|p| !p.party_identification.is_empty())
                        .unwrap_or(false);
                    if has_id {
                        Ok(())
                    } else {
                        Err("Accounting customer party is present but lacks party identification — required for credit note documents".into())
                    }
                }
            })
        },
    });

    // ── CN-R010: At least one credit note line must be present ───────────
    engine.add_rule(Rule {
        id: "PEPPOL-CN-R010".into(),
        description: "At least one credit note line must be present".into(),
        severity: Severity::Fatal,
        check: {
            let cn = Arc::clone(cn);
            Box::new(move || {
                if cn.credit_note_line.is_empty() {
                    Err("No credit note lines present — at least one line is required".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── CN-R011: OrderReference should be present ────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-CN-R011".into(),
        description: "OrderReference should be present to link to original order".into(),
        severity: Severity::Warning,
        check: {
            let cn = Arc::clone(cn);
            Box::new(move || {
                if cn.order_reference.is_some() {
                    Ok(())
                } else {
                    Err("OrderReference is missing — should be provided to link credit note to the original order".into())
                }
            })
        },
    });

    // ── CN-R012: Tax total must be present ───────────────────────────────
    engine.add_rule(Rule {
        id: "PEPPOL-CN-R012".into(),
        description: "Tax total must be present".into(),
        severity: Severity::Fatal,
        check: {
            let cn = Arc::clone(cn);
            Box::new(move || {
                if cn.tax_total.is_empty() {
                    Err("Tax total is missing — at least one tax total entry is required".into())
                } else {
                    Ok(())
                }
            })
        },
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::NaiveDate;
    use peppol_common::rules::RuleEngine;
    use rust_decimal::Decimal;
    use std::sync::Arc;
    use ubl_common::cac::line::CreditNoteLine;
    use ubl_common::cac::order_reference::BillingReference;
    use ubl_common::cac::party::{Party, PartyIdentification, PartyName};
    use ubl_common::cac::supplier::SupplierParty;
    use ubl_common::cac::tax::{TaxCategory, TaxScheme, TaxSubtotal, TaxTotal};
    use ubl_common::cac::totals::LegalTotal;
    use ubl_common::cbc::*;

    fn empty_item() -> ubl_common::cac::item::Item {
        ubl_common::cac::item::Item {
            description: None,
            pack_quantity: None,
            pack_size_numeric: None,
            catalogue_indicator: None,
            name: None,
            hazardous_risk_indicator: None,
            additional_information: None,
            item_type_code: None,
            warranty_information: None,
            lifecycle_stage_code: None,
            lifecycle_stage_description: None,
            keyword: vec![],
            brand_name: vec![],
            model_name: vec![],
            buyers_item_identification: None,
            sellers_item_identification: None,
            manufacturers_item_identification: None,
            standard_item_identification: None,
            catalogue_item_identification: None,
            additional_item_identification: vec![],
            commodity_classification: vec![],
            item_instance: vec![],
            item_property: vec![],
            classified_tax_category: vec![],
        }
    }

    fn minimal_credit_note() -> CreditNote {
        CreditNote {
            id: ID::new("CN-001"),
            issue_date: IssueDate::new(NaiveDate::from_ymd_opt(2026, 6, 12).unwrap()),
            credit_note_type_code: Some(CreditNoteTypeCode::new("381")),
            document_currency_code: Some(DocumentCurrencyCode::new("ZAR")),
            note: vec![Note::new("Customer return — damaged goods")],
            billing_reference: vec![BillingReference {
                invoice_document_reference: Some(Box::new(
                    ubl_common::cac::document::DocumentReference {
                        id: Some(ID::new("INV-001")),
                        copy_indicator: None,
                        uuid: None,
                        issue_date: None,
                        issue_time: None,
                        document_type_code: None,
                        document_type: None,
                        xpath: vec![],
                        referenced_document_internal_address: None,
                        language_id: None,
                        locale_code: None,
                        version_id: None,
                        document_status_code: None,
                        document_description: vec![],
                        attachment: None,
                        validity_period: None,
                        issuer_party: None,
                        result_of_verification: None,
                    },
                )),
                self_billed_invoice_document_reference: None,
                credit_note_document_reference: None,
                self_billed_credit_note_document_reference: None,
                debit_note_document_reference: None,
                reminder_document_reference: None,
                additional_document_reference: None,
                billing_reference_line: vec![],
            }],
            legal_monetary_total: LegalTotal {
                line_extension_amount: LineExtensionAmount::new(Decimal::new(-5000, 2), "ZAR"),
                tax_exclusive_amount: None,
                tax_inclusive_amount: None,
                allowance_total_amount: None,
                charge_total_amount: None,
                prepaid_amount: None,
                payable_rounding_amount: None,
                payable_amount: PayableAmount::new(Decimal::new(-5750, 2), "ZAR"),
            },
            accounting_supplier_party: SupplierParty {
                customer_assigned_account_id: None,
                additional_account_id: vec![],
                data_sending_capability: None,
                party: Some(Party {
                    mark_care_indicator: None,
                    mark_attention_indicator: None,
                    website_uri: None,
                    logo_reference_id: None,
                    endpoint_id: None,
                    industry_classification_code: None,
                    party_identification: vec![PartyIdentification {
                        id: ID::new("SUPP-001"),
                    }],
                    party_name: vec![PartyName {
                        name: Name::new("Acme Supplier"),
                    }],
                    language: None,
                    postal_address: None,
                    physical_location: None,
                    party_tax_scheme: vec![],
                    party_legal_entity: vec![],
                    contact: None,
                    person: None,
                    agent_party: None,
                }),
                despatch_contact: None,
                accounting_contact: None,
                seller_contact: None,
            },
            accounting_customer_party: Some(ubl_common::cac::customer::CustomerParty {
                customer_assigned_account_id: None,
                supplier_assigned_account_id: None,
                additional_account_id: vec![],
                party: Some(Party {
                    mark_care_indicator: None,
                    mark_attention_indicator: None,
                    website_uri: None,
                    logo_reference_id: None,
                    endpoint_id: None,
                    industry_classification_code: None,
                    party_identification: vec![PartyIdentification {
                        id: ID::new("CUST-001"),
                    }],
                    party_name: vec![PartyName {
                        name: Name::new("Beta Customer"),
                    }],
                    language: None,
                    postal_address: None,
                    physical_location: None,
                    party_tax_scheme: vec![],
                    party_legal_entity: vec![],
                    contact: None,
                    person: None,
                    agent_party: None,
                }),
                delivery_contact: None,
                accounting_contact: None,
                buyer_contact: None,
            }),
            tax_total: vec![TaxTotal {
                tax_amount: TaxAmount::new(Decimal::new(-750, 2), "ZAR"),
                rounding_amount: None,
                tax_evidence_indicator: None,
                tax_included_indicator: None,
                calculation_sequence_numeric: None,
                tax_subtotal: vec![TaxSubtotal {
                    taxable_amount: Some(TaxableAmount::new(Decimal::new(-5000, 2), "ZAR")),
                    tax_amount: TaxAmount::new(Decimal::new(-750, 2), "ZAR"),
                    calculation_sequence_numeric: None,
                    transaction_currency_tax_amount: None,
                    percent: Some(Percent::new(Decimal::new(15, 0))),
                    base_unit_measure: None,
                    per_unit_amount: None,
                    tier_range: None,
                    tier_rate_percent: None,
                    tax_inclusive_amount: None,
                    tax_category: TaxCategory {
                        id: None,
                        supply_type_code: None,
                        name: None,
                        percent: Some(Percent::new(Decimal::new(15, 0))),
                        base_unit_measure: None,
                        per_unit_amount: None,
                        tax_exemption_reason_code: None,
                        tax_exemption_reason: vec![],
                        tier_range: None,
                        tier_rate_percent: None,
                        tax_scheme: TaxScheme {
                            id: Some(ID::new("VAT")),
                            tax_type_code: None,
                            currency_code: None,
                            name: Some(Name::new("VAT")),
                            jurisdiction_region_address: vec![],
                        },
                    },
                }],
            }],
            credit_note_line: vec![CreditNoteLine {
                id: ID::new("1"),
                uuid: None,
                note: vec![],
                credited_quantity: Some(CreditedQuantity::new(Decimal::new(1, 0))),
                line_extension_amount: Some(LineExtensionAmount::new(Decimal::new(-5000, 2), "ZAR")),
                tax_inclusive_line_extension_amount: None,
                tax_point_date: None,
                accounting_cost_code: None,
                accounting_cost: None,
                payment_purpose_code: None,
                free_of_charge_indicator: None,
                invoice_period: vec![],
                order_line_reference: vec![],
                despatch_line_reference: vec![],
                receipt_line_reference: vec![],
                billing_reference: vec![],
                document_reference: vec![],
                allowance_charge: vec![],
                item: empty_item(),
                price: None,
                pricing_reference: None,
                originator_party: None,
                delivery: vec![],
                payment_terms: vec![],
                tax_total: vec![],
            }],
            order_reference: None,
            ubl_version_id: None,
            customization_id: None,
            profile_id: None,
            profile_execution_id: None,
            copy_indicator: None,
            uuid: None,
            issue_time: None,
            due_date: None,
            tax_point_date: None,
            tax_currency_code: None,
            pricing_currency_code: None,
            payment_currency_code: None,
            payment_alternative_currency_code: None,
            accounting_cost_code: None,
            accounting_cost: None,
            line_count_numeric: None,
            buyer_reference: None,
            default_language_code: None,
            invoice_period: vec![],
            discrepancy_response: vec![],
            despatch_document_reference: vec![],
            delivery_note_document_reference: vec![],
            work_report_document_reference: vec![],
            receipt_document_reference: vec![],
            statement_document_reference: vec![],
            originator_document_reference: vec![],
            contract_document_reference: vec![],
            additional_document_reference: vec![],
            payee_party: None,
            buyer_customer_party: None,
            seller_supplier_party: None,
            tax_representative_party: None,
            delivery: vec![],
            delivery_terms: None,
            payment_means: vec![],
            payment_terms: vec![],
            allowance_charge: vec![],
            tax_exchange_rate: None,
            pricing_exchange_rate: None,
            payment_exchange_rate: None,
            payment_alternative_exchange_rate: None,
        }
    }

    #[test]
    fn test_valid_credit_note_passes_all() {
        let cn = minimal_credit_note();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(cn));
        let failures = engine.evaluate_failures();

        // Only R011 (OrderReference — Warning) should fail, since we didn't set it.
        // Everything else should pass.
        let non_warning_failures: Vec<_> = failures
            .iter()
            .filter(|f| f.severity != Some(Severity::Warning))
            .collect();
        assert!(
            non_warning_failures.is_empty(),
            "Expected no non-warning failures, got: {:?}",
            non_warning_failures
                .iter()
                .map(|f| &f.rule_id)
                .collect::<Vec<_>>()
        );

        // R011 should be a warning (OrderReference missing)
        let r011_failure = failures.iter().find(|f| f.rule_id == "PEPPOL-CN-R011");
        assert!(
            r011_failure.is_some(),
            "Expected CN-R011 to warn about missing OrderReference"
        );
    }

    #[test]
    fn test_missing_original_reference_fails_r005() {
        let mut cn = minimal_credit_note();
        cn.billing_reference = vec![]; // No billing reference at all
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(cn));
        let failures = engine.evaluate_failures();
        assert!(
            failures.iter().any(|f| f.rule_id == "PEPPOL-CN-R005"),
            "Expected CN-R005 to fail when billing_reference is empty"
        );
    }

    #[test]
    fn test_missing_reason_fails_r006() {
        let mut cn = minimal_credit_note();
        cn.note = vec![]; // No reason notes
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(cn));
        let failures = engine.evaluate_failures();
        assert!(
            failures.iter().any(|f| f.rule_id == "PEPPOL-CN-R006"),
            "Expected CN-R006 to fail when notes are empty"
        );
    }
}
