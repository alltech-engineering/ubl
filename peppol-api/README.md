# Peppol API

Axum REST server exposing all Peppol BIS validation rules as HTTP endpoints.

## Overview

The Peppol API provides a REST interface for validating UBL documents against Peppol
Business Interoperability Specifications (BIS). It wraps the `peppol-billing`,
`peppol-ordering`, `peppol-despatch`, `peppol-mlr`, `peppol-imr`, and
`peppol-catalogue` rule crates behind a unified HTTP API.

- **Framework:** [Axum](https://github.com/tokio-rs/axum) (Tokio-based)
- **Default port:** `3000`
- **Storage backend:** `peppol-storage` (in-memory default, optional Postgres via SQLx)

## Endpoints

### Validation — `POST /api/validate/{type}`

Eight validation endpoints, one per Peppol BIS document type:

| Endpoint | BIS Document |
|---|---|
| `POST /api/validate/invoice` | BIS Billing 3.0 Invoice |
| `POST /api/validate/creditnote` | BIS Billing 3.0 Credit Note |
| `POST /api/validate/order` | BIS Ordering 3.0 Order |
| `POST /api/validate/order_response` | BIS Ordering 3.0 Order Response |
| `POST /api/validate/despatch_advice` | BIS Despatch 3.0 Despatch Advice |
| `POST /api/validate/mlr` | BIS MLR 3.0 Message Level Response |
| `POST /api/validate/imr` | BIS IMR 3.0 Invoice Message Response |
| `POST /api/validate/catalogue` | BIS Catalogue 3.0 Catalogue |

All validation endpoints accept a JSON-encoded UBL document in the request body and
return a JSON response with validation results (pass/fail, rule violations, warnings).

### Documents — `GET /api/documents`

Two document endpoints for retrieving previously persisted documents:

| Endpoint | Description |
|---|---|
| `GET /api/documents` | List all stored documents |
| `GET /api/documents/{id}` | Retrieve a specific document by ID |

### Health — `GET /api/health`

Returns the server health status.

## Storage

Document persistence is provided by the `peppol-storage` crate, which implements a
trait-based storage abstraction:

- **In-memory** — default, no configuration needed. Documents live for the lifetime
  of the server process.
- **Postgres** — opt-in via feature flag. Uses SQLx for async, compile-time-checked
  queries. Set the `DATABASE_URL` environment variable to activate.

## Architecture

Handlers are generated via Rust macros to keep the codebase clean and DRY.
Each BIS rule set is an `Arc`-based rules engine that the API crate composes
together. Validation results flow through a common response type regardless of
which BIS document is being validated.

## Quick Start

```bash
# From the workspace root
cargo run -p peppol-api
```

The server starts on `http://localhost:3000`.

## Example

```bash
curl -X POST http://localhost:3000/api/validate/invoice \
  -H 'Content-Type: application/json' \
  -d '{
    "id": "INV-001",
    "issue_date": "2025-01-15",
    "supplier": { "name": "Acme Corp", "vat": "BE0123456789" },
    "customer": { "name": "Buyer Ltd", "vat": "NL0987654321" },
    "line_items": [
      { "description": "Widget", "quantity": 10, "unit_price": "25.00", "total": "250.00" }
    ],
    "total": "250.00",
    "currency": "EUR"
  }'
```

## Workspace Context

`peppol-api` sits at Layer 5 of the UBL/Peppol workspace:

```
Layer 0:  ubl-common          — CBC + CAC core types
Layer 1:  ubl-documents        — 101 UBL document types
Layer 2a: ubl-xml              — UBL XML serialization
Layer 2b: peppol-common        — Shared Peppol infrastructure
Layer 3:  peppol-xml           — Peppol XML serialization
Layer 4:  peppol-billing, peppol-ordering, peppol-despatch,
          peppol-mlr, peppol-imr, peppol-catalogue    — BIS rule crates
Layer 5:  peppol-api (here) + peppol-storage          — REST server + persistence
```

See the workspace root `ARCHITECTURE.md` for the full architecture overview.
