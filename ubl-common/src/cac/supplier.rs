// UBL Supplier Party aggregate.

use crate::cac::contact::Contact;
use crate::cac::party::Party;
use crate::cbc::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupplierParty {
    pub customer_assigned_account_id: Option<CustomerAssignedAccountID>,
    #[serde(default)]
    pub additional_account_id: Vec<AdditionalAccountID>,
    pub data_sending_capability: Option<Text>,
    pub party: Option<Party>,
    pub despatch_contact: Option<Contact>,
    pub accounting_contact: Option<Contact>,
    pub seller_contact: Option<Contact>,
}
