# peppol-common

Peppol e-procurement shared infrastructure — the foundation for all Peppol BIS
implementations (Billing 3.0, Ordering 3.0, Despatch 3.0, etc.).

Provides participant identifiers, document identity, validated code lists, and
a Schematron-style business rules engine.

## Directory Structure

```
src/
├── participant.rs   # ISO 6523 ICD codes, Peppol EAS codes, ParticipantId
├── identity.rs      # CustomizationID/ProfileID, BisDocument trait
├── codes.rs         # Code lists: UNCL 1001/4461/5305, ISO 4217, ISO 3166-1
├── rules.rs         # Schematron-style rule engine (Rule, RuleEngine, Severity)
└── lib.rs           # Public re-exports
```

## Key Types

### Participant Identifiers

```rust
IcdCode { code: String, agency: String, country: Option<String> }
EASCode { code: String, scheme: String }
ParticipantId { icd: IcdCode, id: String }
// Parse: ParticipantId::parse("9933:za1234567890").unwrap()
```

35 registries pre-loaded: 9933 (South Africa), 0009 (DUNS), 0088 (GLN),
0060 (EU VAT), 0192 (Kvk NL), etc.

### Document Identity

```rust
DocumentIdentity {
    customization_id: "urn:fdc:peppol.eu:2017:poacc:billing:3.0::2.1",
    profile_id:       "urn:fdc:peppol.eu:2017:poacc:billing:01:1.0",
}
```

Factory functions: `identities::billing_3_0("Invoice")`,
`identities::ordering_3_0("Order")`, etc.

### Code Lists

Pre-validated code lists for Peppol's restricted subsets:
- UNCL 1001 — Document type codes (380=Invoice, 381=CreditNote)
- UNCL 5305 — Tax category codes (S, Z, E, etc.)
- UNCL 4461 — Payment means codes
- ISO 4217 — Currency codes
- ISO 3166-1 — Country codes (alpha-2)

### Rules Engine

Schematron-style validation with three severity levels:

| Severity | Meaning |
|----------|---------|
| `Fatal` | Mandatory rule violated — reject document |
| `Error` | Conditional rule violated — should reject |
| `Warning` | Best practice not followed |

```rust
let mut engine = RuleEngine::new();
engine.add_rule(Rule {
    id: "PEPPOL-EN16931-R001".into(),
    description: "Invoice total must match line totals + tax".into(),
    severity: Severity::Fatal,
    check: Box::new(|| { /* validation logic */ Ok(()) }),
});
let failures = engine.evaluate_failures();
if !failures.is_empty() {
    eprintln!("Validation failed: {failures:?}");
}
```

## Usage

```rust
use peppol_common::{ParticipantId, DocumentIdentity, RuleEngine};
use peppol_common::identity::identities;

let party_id = ParticipantId::parse("9933:za1234567890").unwrap();
let identity = identities::billing_3_0("Invoice");
```
