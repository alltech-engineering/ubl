// UBL 2.5 Common Basic Components (CBC) and Common Aggregate Components (CAC)
//
// Reference: https://docs.oasis-open.org/ubl/cs01-UBL-2.5/UBL-2.5.html
//
// This crate provides pure Rust domain types for UBL 2.5:
//   - cbc: ~200 primitive reusable types (Amount, Code, Date, Identifier, etc.)
//   - cac: ~80 complex reusable aggregates (Address, Party, Item, TaxTotal, etc.)
//
// Design principles:
//   - Newtype wrappers for type safety
//   - serde Serialize/Deserialize for JSON representation
//   - No XML knowledge in the domain layer (XML adapters will be separate)
//   - No async dependencies
//   - Edition 2024

pub mod cbc;
// pub mod cac; — Phase 0-F2 through 0-F4
