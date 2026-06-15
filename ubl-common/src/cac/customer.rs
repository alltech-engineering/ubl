// UBL Customer Party aggregate.

use crate::cac::contact::Contact;
use crate::cac::party::Party;
use crate::cbc::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerParty {
    #[serde(default)]
    pub customer_assigned_account_id: Option<CustomerAssignedAccountID>,
    #[serde(default)]
    pub supplier_assigned_account_id: Option<SupplierAssignedAccountID>,
    #[serde(default)]
    pub additional_account_id: Vec<AdditionalAccountID>,
    #[serde(default)]
    pub party: Option<Party>,
    #[serde(default)]
    pub delivery_contact: Option<Contact>,
    #[serde(default)]
    pub accounting_contact: Option<Contact>,
    #[serde(default)]
    pub buyer_contact: Option<Contact>,
}
