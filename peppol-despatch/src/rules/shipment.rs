// Peppol BIS Despatch Advice 3.0 — Shipment Business Rules
//
// Validates shipment information for Despatch Advice.

use peppol_common::rules::{Rule, RuleEngine, Severity};
use std::sync::Arc;
use ubl_documents::despatch::DespatchAdvice;

pub fn add_rules(engine: &mut RuleEngine, inv: &Arc<DespatchAdvice>) {
    // ── DESP-R034 (Warning): Shipment information should be present ───────
    engine.add_rule(Rule {
        id: "DESP-R034".into(),
        description: "Shipment information should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if inv.shipment.is_none() {
                    Err("Shipment information is missing — shipment details should be provided".into())
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── DESP-R035 (Warning): Shipment ID should be present ───────────────
    engine.add_rule(Rule {
        id: "DESP-R035".into(),
        description: "Shipment ID should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(ref shipment) = inv.shipment {
                    match &shipment.id {
                        None => Err("Shipment has no ID — a shipment identifier should be provided".into()),
                        Some(id) if id.value().is_empty() => {
                            Err("Shipment ID is present but empty".into())
                        }
                        Some(_) => Ok(()),
                    }
                } else {
                    Ok(())
                }
            })
        },
    });

    // ── DESP-R036 (Warning): Gross weight measure should be present ──────
    engine.add_rule(Rule {
        id: "DESP-R036".into(),
        description: "Shipment gross weight measure should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(ref shipment) = inv.shipment {
                    if shipment.gross_weight_measure.is_none() {
                        return Err(
                            "Shipment has no gross weight measure — gross weight should be provided"
                                .into(),
                        );
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R037 (Warning): At least one GoodsItem should exist ─────────
    engine.add_rule(Rule {
        id: "DESP-R037".into(),
        description: "Shipment should contain at least one GoodsItem".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(ref shipment) = inv.shipment {
                    if shipment.goods_item.is_empty() {
                        return Err(
                            "Shipment has no GoodsItem — at least one goods item should be specified"
                                .into(),
                        );
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R038 (Warning): Net weight measure should be present ────────
    engine.add_rule(Rule {
        id: "DESP-R038".into(),
        description: "Shipment net weight measure should be present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(ref shipment) = inv.shipment {
                    if shipment.net_weight_measure.is_none() {
                        return Err(
                            "Shipment has no net weight measure — net weight should be provided"
                                .into(),
                        );
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R039 (Warning): Total goods item quantity should match ──────
    engine.add_rule(Rule {
        id: "DESP-R039".into(),
        description: "Total goods item quantity should be consistent with line quantities".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(ref shipment) = inv.shipment {
                    if let Some(ref total_qty) = shipment.total_goods_item_quantity {
                        let sum: rust_decimal::Decimal = inv
                            .despatch_line
                            .iter()
                            .filter_map(|l| l.delivered_quantity.as_ref())
                            .map(|q| *q.value())
                            .sum();
                        if total_qty.value() != &sum {
                            return Err(format!(
                                "TotalGoodsItemQuantity ({}) does not match sum of line delivered quantities ({})",
                                total_qty.value(),
                                sum
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R040 (Warning): Handling code should be valid if present ────
    engine.add_rule(Rule {
        id: "DESP-R040".into(),
        description: "Shipment handling code should be valid if present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(ref shipment) = inv.shipment {
                    if let Some(ref handling) = shipment.handling_code {
                        if handling.value().is_empty() {
                            return Err("Shipment handling code is present but empty".into());
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R041 (Warning): ShipmentStage transport mode should be valid ─
    engine.add_rule(Rule {
        id: "DESP-R041".into(),
        description: "ShipmentStage transport mode code should be valid if present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(ref shipment) = inv.shipment {
                    for (i, stage) in shipment.shipment_stage.iter().enumerate() {
                        if let Some(ref tmc) = stage.transport_mode_code {
                            if tmc.value().is_empty() {
                                return Err(format!(
                                    "ShipmentStage {} has an empty transport mode code",
                                    i + 1
                                ));
                            }
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R042 (Fatal): Consignment quantity must be positive ─────────
    engine.add_rule(Rule {
        id: "DESP-R042".into(),
        description: "Consignment quantity must be positive if present".into(),
        severity: Severity::Fatal,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(ref shipment) = inv.shipment {
                    if let Some(ref consignment) = shipment.consignment_quantity {
                        let val = consignment.value();
                        if val.is_zero() || val.is_sign_negative() {
                            return Err(format!(
                                "Consignment quantity is not positive ({})",
                                val
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });

    // ── DESP-R043 (Warning): Declared customs value should be valid ──────
    engine.add_rule(Rule {
        id: "DESP-R043".into(),
        description: "Declared customs value amount should be valid if present".into(),
        severity: Severity::Warning,
        check: {
            let inv = Arc::clone(inv);
            Box::new(move || {
                if let Some(ref shipment) = inv.shipment {
                    if let Some(ref customs) = shipment.declared_customs_value_amount {
                        let val = customs.value();
                        if val.is_sign_negative() {
                            return Err(format!(
                                "Declared customs value amount is negative ({})",
                                val
                            ));
                        }
                    }
                }
                Ok(())
            })
        },
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use peppol_common::rules::RuleEngine;
    use std::sync::Arc;
    use ubl_documents::despatch::DespatchAdvice;

    fn minimal_despatch() -> DespatchAdvice {
        serde_json::from_str(
            r#"{
            "id": {"value": "DES-001"},
            "issue_date": "2026-06-13",
            "despatch_advice_type_code": {"value": "delivery"},
            "order_reference": [{"id": {"value": "ORD-001"}}],
            "despatch_supplier_party": {
                "party": {
                    "party_name": [{"name": "Shipper Ltd"}],
                    "party_identification": [{"id": {"value": "9933:shipper"}}],
                    "postal_address": {
                        "street_name": "1 Dock Rd",
                        "city_name": "Durban",
                        "country": {"identification_code": {"value": "ZA"}}
                    }
                }
            },
            "delivery_customer_party": {
                "party": {
                    "party_name": [{"name": "Receiver Ltd"}],
                    "party_identification": [{"id": {"value": "9933:receiver"}}],
                    "postal_address": {
                        "street_name": "2 Warehouse St",
                        "city_name": "Cape Town",
                        "country": {"identification_code": {"value": "ZA"}}
                    }
                }
            },
            "despatch_line": [{
                "id": {"value": "1"},
                "delivered_quantity": {"value": "10", "unit_code": "EA"},
                "item": {"name": "Widget"},
                "order_line_reference": [{"line_id": {"value": "1"}}]
            }]
        }"#,
        )
        .unwrap()
    }

    #[test]
    fn test_no_shipment_warns() {
        let despatch = minimal_despatch();
        let mut engine = RuleEngine::new();
        add_rules(&mut engine, &Arc::new(despatch));
        let failures = engine.evaluate_failures();
        assert!(failures.iter().any(|f| f.rule_id == "DESP-R034"));
    }
}
