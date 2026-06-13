// Peppol Common — shared infrastructure for all Peppol BIS implementations.
//
// Provides:
//   - Participant identifiers (ISO 6523 ICD + Peppol EAS)
//   - Document identity (CustomizationID, ProfileID per BIS)
//   - Code lists (UNCL, ISO, Peppol-specific)
//   - Schematron-style rules engine

pub mod codes;
pub mod identity;
pub mod participant;
pub mod rules;

pub use codes::CodeList;
pub use identity::{BisDocument, DocumentIdentity};
pub use participant::{EASCode, IcdCode, ParticipantId};
pub use rules::{Rule, RuleEngine, RuleOutcome};
