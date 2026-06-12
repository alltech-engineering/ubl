// UBL 2.5 Common Aggregate Components (CAC)
//
// Reference: https://docs.oasis-open.org/ubl/cs01-UBL-2.5/xsd/common/UBL-CommonAggregateComponents-2.5.xsd

// Tier 1 stubs (needed by Tier 2; full implementation in P0-F2)
pub mod address;
pub mod allowance_charge;
pub mod delivery_terms;
pub mod item;
pub mod party;
pub mod payment_means;
pub mod payment_terms;
pub mod price;
pub mod tax;

// Tier 2 — Document support aggregates (this task, P0-F3)
pub mod billing_reference;
pub mod credit_note_line;
pub mod debit_note_line;
pub mod delivery;
pub mod despatch_line;
pub mod document_reference;
pub mod invoice_line;
pub mod line_item;
pub mod order_line;
pub mod order_reference;
pub mod period;
pub mod receipt_line;
pub mod response;
pub mod status;

// Tier 3 — Extended aggregates (P0-F4)
pub mod location;
pub mod transport;
pub mod transport_mode;
pub mod stowage;
pub mod logistics;

// Tier 4 — Remaining aggregates (P0-F4)
pub mod financial;
pub mod goods_item;
pub mod hazardous;
pub mod item_more;
pub mod line;
pub mod organization;
pub mod supporting;
