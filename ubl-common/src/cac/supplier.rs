// UBL Supplier Party aggregate.

use crate::cac::contact::Contact;
use crate::cac::party::Party;
use crate::cbc::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupplierParty {
    #[serde(default)]
    pub customer_assigned_account_id: Option<CustomerAssignedAccountID>,
    #[serde(default)]
    pub additional_account_id: Vec<AdditionalAccountID>,
    #[serde(default)]
    pub data_sending_capability: Option<Text>,
    #[serde(default)]
    pub party: Option<Party>,
    #[serde(default)]
    pub despatch_contact: Option<Contact>,
    #[serde(default)]
    pub accounting_contact: Option<Contact>,
    #[serde(default)]
    pub seller_contact: Option<Contact>,
}
