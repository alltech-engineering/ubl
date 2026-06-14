// Peppol API — handler functions
//
// Each handler deserializes JSON, runs Peppol rules, stores the document.

use axum::Json;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use chrono::Utc;
use peppol_common::rules::Severity;
use peppol_storage::StoredDocument;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::state::AppState;

// ── Domain types ──
use ubl_documents::billing::{CreditNote, Invoice};
use ubl_documents::catalogue::Catalogue;
use ubl_documents::despatch::DespatchAdvice;
use ubl_documents::ordering::{Order, OrderResponse};
use ubl_documents::status::ApplicationResponse;

// ── Peppol rule engines ──
use peppol_billing::rules::{billing_rules, credit_note_rules};
use peppol_catalogue::rules::catalogue_rules;
use peppol_despatch::rules::despatch_rules;
use peppol_imr::rules::imr_rules;
use peppol_mlr::rules::mlr_rules;
use peppol_ordering::rules::{ordering_response_rules, ordering_rules};

// ── Response types ──

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: &'static str,
    pub version: &'static str,
    pub uptime_seconds: u64,
}

#[derive(Serialize)]
struct ValidationResponse {
    valid: bool,
    stored_id: String,
    errors: Vec<RuleInfo>,
    warnings: Vec<RuleInfo>,
}

#[derive(Serialize, Deserialize)]
struct RuleInfo {
    rule_id: String,
    message: String,
    severity: String,
}

#[derive(Serialize)]
struct DocumentListResponse {
    documents: Vec<StoredDocument>,
}

// ── Health ─────────────────────────────────────────────────────────────

pub async fn health() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        version: env!("CARGO_PKG_VERSION"),
        uptime_seconds: 0,
    })
}

// ── Helper ─────────────────────────────────────────────────────────────

fn build_response(
    outcomes: Vec<peppol_common::rules::RuleOutcome>,
    stored_id: Uuid,
) -> ValidationResponse {
    let mut errors = vec![];
    let mut warnings = vec![];
    for o in outcomes {
        if let Some(ref sev) = o.severity {
            let info = RuleInfo {
                rule_id: o.rule_id,
                message: o.message,
                severity: format!("{:?}", sev),
            };
            match sev {
                Severity::Warning => warnings.push(info),
                _ => errors.push(info),
            }
        }
    }
    ValidationResponse {
        valid: errors.is_empty(),
        stored_id: stored_id.to_string(),
        errors,
        warnings,
    }
}

fn status_from_response(response: &ValidationResponse) -> StatusCode {
    if response.valid {
        StatusCode::OK
    } else {
        StatusCode::UNPROCESSABLE_ENTITY
    }
}

async fn store_and_respond(
    state: &AppState,
    doc_type: &str,
    doc_id: &str,
    payload: serde_json::Value,
    outcomes: Vec<peppol_common::rules::RuleOutcome>,
) -> impl IntoResponse {
    let failures: Vec<_> = outcomes
        .iter()
        .filter(|o| o.severity.is_some())
        .collect();
    let validated = failures.iter().all(|f| {
        matches!(f.severity, Some(Severity::Warning))
    });

    let doc = StoredDocument {
        id: Uuid::new_v4(),
        document_type: doc_type.to_string(),
        document_id: doc_id.to_string(),
        payload,
        created_at: Utc::now(),
        validated,
        validation_errors: serde_json::to_value(&failures).unwrap_or_default(),
    };

    let stored_id = doc.id;
    let _ = state.store.store(doc).await;
    let response = build_response(outcomes, stored_id);
    (status_from_response(&response), Json(response))
}

// ── Validate handlers ──────────────────────────────────────────────────

pub async fn validate_invoice(
    State(state): State<AppState>,
    Json(invoice): Json<Invoice>,
) -> impl IntoResponse {
    let doc_id = invoice.id.value().to_string();
    let payload = serde_json::to_value(&invoice).unwrap_or_default();
    let outcomes = billing_rules(&invoice).evaluate_all();
    store_and_respond(&state, "Invoice", &doc_id, payload, outcomes).await
}

pub async fn validate_credit_note(
    State(state): State<AppState>,
    Json(cn): Json<CreditNote>,
) -> impl IntoResponse {
    let doc_id = cn.id.value().to_string();
    let payload = serde_json::to_value(&cn).unwrap_or_default();
    let outcomes = credit_note_rules(&cn).evaluate_all();
    store_and_respond(&state, "CreditNote", &doc_id, payload, outcomes).await
}

pub async fn validate_order(
    State(state): State<AppState>,
    Json(order): Json<Order>,
) -> impl IntoResponse {
    let doc_id = order.id.value().to_string();
    let payload = serde_json::to_value(&order).unwrap_or_default();
    let outcomes = ordering_rules(&order).evaluate_all();
    store_and_respond(&state, "Order", &doc_id, payload, outcomes).await
}

pub async fn validate_order_response(
    State(state): State<AppState>,
    Json(response): Json<OrderResponse>,
) -> impl IntoResponse {
    let doc_id = response.id.value().to_string();
    let payload = serde_json::to_value(&response).unwrap_or_default();
    let outcomes = ordering_response_rules(&response).evaluate_all();
    store_and_respond(&state, "OrderResponse", &doc_id, payload, outcomes).await
}

pub async fn validate_despatch(
    State(state): State<AppState>,
    Json(despatch): Json<DespatchAdvice>,
) -> impl IntoResponse {
    let doc_id = despatch.id.value().to_string();
    let payload = serde_json::to_value(&despatch).unwrap_or_default();
    let outcomes = despatch_rules(&despatch).evaluate_all();
    store_and_respond(&state, "DespatchAdvice", &doc_id, payload, outcomes).await
}

pub async fn validate_mlr(
    State(state): State<AppState>,
    Json(app_response): Json<ApplicationResponse>,
) -> impl IntoResponse {
    let doc_id = app_response.id.value().to_string();
    let payload = serde_json::to_value(&app_response).unwrap_or_default();
    let outcomes = mlr_rules(&app_response).evaluate_all();
    store_and_respond(&state, "MLR", &doc_id, payload, outcomes).await
}

pub async fn validate_imr(
    State(state): State<AppState>,
    Json(app_response): Json<ApplicationResponse>,
) -> impl IntoResponse {
    let doc_id = app_response.id.value().to_string();
    let payload = serde_json::to_value(&app_response).unwrap_or_default();
    let outcomes = imr_rules(&app_response).evaluate_all();
    store_and_respond(&state, "IMR", &doc_id, payload, outcomes).await
}

pub async fn validate_catalogue(
    State(state): State<AppState>,
    Json(catalogue): Json<Catalogue>,
) -> impl IntoResponse {
    let doc_id = catalogue.id.value().to_string();
    let payload = serde_json::to_value(&catalogue).unwrap_or_default();
    let outcomes = catalogue_rules(&catalogue).evaluate_all();
    store_and_respond(&state, "Catalogue", &doc_id, payload, outcomes).await
}

// ── Document retrieval ─────────────────────────────────────────────────

pub async fn list_documents(
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.store.list(None, 100).await {
        Ok(docs) => (StatusCode::OK, Json(serde_json::json!({ "documents": docs }))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        ),
    }
}

pub async fn get_document(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> impl IntoResponse {
    match state.store.get(id).await {
        Ok(Some(doc)) => (StatusCode::OK, Json(doc)),
        Ok(None) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "document not found" })),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": e.to_string() })),
        ),
    }
}
