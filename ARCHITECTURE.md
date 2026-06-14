# Architecture

UBL + Peppol workspace — a layered Rust monorepo implementing the OASIS Universal
Business Language (UBL) 2.1 standard and Peppol BIS 3.0 validation rules.

## Layer Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                                                                 │
│  Layer 5   │ peppol-api (REST server) + peppol-storage (DB)    │
│            │ Axum endpoints, macro handlers, trait storage      │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Layer 4   │ peppol-billing   │ peppol-ordering                 │
│            │ peppol-despatch  │ peppol-mlr                      │
│            │ peppol-imr       │ peppol-catalogue                │
│            │ Peppol BIS 3.0 rule sets (Arc-based engines)      │
│                                                                 │
├──────────────────────────────┬──────────────────────────────────┤
│                              │                                  │
│  Layer 3   │ peppol-xml      │                                  │
│            │ Peppol XML      │                                  │
│            │ serialization   │                                  │
├──────────────────────────────┤                                  │
│                              │                                  │
│  Layer 2a  │ ubl-xml         │  Layer 2b  │ peppol-common      │
│            │ UBL XML output  │            │ Shared Peppol infra │
│                              │            │ (profiles, ids)     │
├──────────────────────────────┴──────────────────────────────────┤
│                                                                 │
│  Layer 1   │ ubl-documents                                      │
│            │ All 101 UBL 2.1 document types                     │
│            │ (Invoice, CreditNote, Order, DespatchAdvice, etc.) │
│                                                                 │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  Layer 0   │ ubl-common                                         │
│            │ CBC (Core Basic Components) + CAC (Core Aggregate  │
│            │ Components) — the atomic building blocks           │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Layer Descriptions

### Layer 0: `ubl-common`

Foundation crate. Defines all Core Basic Component (CBC) types — `Identifier`,
`Code`, `Text`, `Amount`, `Quantity`, `Date`, `Time`, `Indicator`, etc. — and all
Core Aggregate Component (CAC) types — `Party`, `Address`, `Contact`,
`TaxTotal`, `LegalMonetaryTotal`, `LineItem`, etc. These are the atomic building
blocks from which all UBL documents are assembled. Every other crate in the
workspace depends on `ubl-common`.

### Layer 1: `ubl-documents`

Assembles CBC and CAC types into the 101 full UBL 2.1 document types (`Invoice`,
`CreditNote`, `Order`, `OrderResponse`, `DespatchAdvice`, `ReceiptAdvice`,
`Catalogue`, `CatalogueItemSpecificationUpdate`, etc.). Each document type is a
struct with `#[serde(default)]` on `Vec` fields to gracefully handle partial input
during deserialization.

### Layer 2a: `ubl-xml`

UBL XML output. Implements `Serialize`/`Deserialize` with proper UBL XML namespace
handling so documents can be round-tripped between the in-memory Rust types and
standards-compliant UBL XML.

### Layer 2b: `peppol-common`

Shared Peppol infrastructure. Peppol profile identifiers, customization IDs,
business process types, and common utilities shared across all BIS rule crates.

### Layer 3: `peppol-xml`

Peppol-specific XML serialization layer. Adds Peppol namespace bindings and
extensions on top of `ubl-xml`.

### Layer 4: BIS Rule Crates

Six standalone crates, each implementing the full Peppol BIS 3.0 rule set for one
business document family:

| Crate | BIS Specification | Documents Covered |
|---|---|---|
| `peppol-billing` | BIS Billing 3.0 | Invoice, Credit Note |
| `peppol-ordering` | BIS Ordering 3.0 | Order, Order Response |
| `peppol-despatch` | BIS Despatch 3.0 | Despatch Advice |
| `peppol-mlr` | BIS MLR 3.0 | Message Level Response |
| `peppol-imr` | BIS IMR 3.0 | Invoice Message Response |
| `peppol-catalogue` | BIS Catalogue 3.0 | Catalogue |

Each crate exposes an `Arc`-based rules engine. Rules are loaded once and shared
across requests, enabling zero-allocation hot-path validation.

### Layer 5: `peppol-api` + `peppol-storage`

- **`peppol-api`** — Axum REST server on port `3000`. Exposes 8 `POST` validation
  endpoints (`/api/validate/{type}`), 2 `GET` document endpoints (`/api/documents`),
  and a health check (`GET /api/health`). Handlers are generated via declarative
  macros for clean, DRY code.

- **`peppol-storage`** — Trait-based document persistence. Default implementation
  is an in-memory store. Optional Postgres backend via SQLx (activate with feature
  flag, set `DATABASE_URL`).

## Architecture Principles

### Clean Architecture

Dependencies flow strictly downward: higher layers depend on lower layers, never
the reverse. `ubl-common` has zero workspace-internal dependencies. Each BIS rule
crate depends on `peppol-common` and `peppol-xml` (and transitively on
`ubl-documents` and `ubl-common`). The API crate composes everything together but
owns no business logic itself.

### Arc-Based Rules Engine

BIS rule crates load and precompile rules into `Arc`-wrapped structures at startup.
Validation requests borrow these rules through `Arc` clones, which are cheap
(atomic reference count bump, no heap allocation). This means the hot validation
path contains zero expensive initialization.

### `#[serde(default)]` on Vec Fields

All `Vec` fields in UBL document structs carry `#[serde(default)]`. This prevents
deserialization failures when upstream systems omit optional repeating elements.
Empty collections default to `Vec::new()` rather than causing a parse error.

### Trait-Based Storage

`peppol-storage` defines a `DocumentStore` trait. Consumers (the API crate) code
against the trait, not a concrete type. Swapping the in-memory store for Postgres
is a one-line change at server startup.

## Test Stats

- **Tests:** 129+ across the workspace
- **Failures:** 0
- **Coverage:** unit tests in every crate, integration tests in BIS rule crates
  validating real Peppol example documents

## Quick Start

```bash
# Build the entire workspace
cargo build --workspace

# Run all tests
cargo test --workspace

# Start the API server
cargo run -p peppol-api
```

## Workspace Layout

```
ubl/
├── ubl-common/              # Layer 0 — CBC + CAC core types
├── ubl-documents/           # Layer 1 — 101 UBL document types
├── ubl-xml/                 # Layer 2a — UBL XML serialization
├── peppol-common/           # Layer 2b — Shared Peppol infrastructure
├── peppol-xml/              # Layer 3 — Peppol XML serialization
├── peppol-billing/          # Layer 4 — BIS Billing 3.0 rules
├── peppol-ordering/         # Layer 4 — BIS Ordering 3.0 rules
├── peppol-despatch/         # Layer 4 — BIS Despatch 3.0 rules
├── peppol-mlr/              # Layer 4 — BIS MLR 3.0 rules
├── peppol-imr/              # Layer 4 — BIS IMR 3.0 rules
├── peppol-catalogue/        # Layer 4 — BIS Catalogue 3.0 rules
├── peppol-storage/          # Layer 5 — Document persistence
├── peppol-api/              # Layer 5 — Axum REST server
├── Cargo.toml               # Workspace manifest
└── ARCHITECTURE.md          # This file
```
