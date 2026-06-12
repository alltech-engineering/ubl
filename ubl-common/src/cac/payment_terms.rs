// PaymentTerms — UBL CAC aggregate (Tier 1 stub)
use crate::cbc::*;

#[derive(Debug, Clone, Partialserde::Serialize, serde::Deserialize)]
pub struct PaymentTerms {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub note: Vec<Note>,
}
