//! UBL 2.5 document types — all business document definitions generated from
//! the OASIS UBL 2.5 XSD schemas via `xsd-parser`.
//! Do not edit by hand; regenerate with `cargo run -p xsd-gen` then run the split script.

#![allow(unused_imports, dead_code, non_snake_case, clippy::all)]

// Re-export common's modules at crate root so generated document types
// can reference them with bare paths (cac::Party, cct::Amount, etc.)
pub use common::{
    cac, cbc, cct, ds, dsig_11, ext, qdt, sac, sbc, udt, xades, xs,
};

mod documents;
