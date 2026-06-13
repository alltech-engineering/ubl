// UBL Customer Party aggregate.

use crate::cac::contact::Contact;
use crate::cac::party::Party;
use crate::cbc::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerParty {
    pub customer_assigned_account_id: Option<CustomerAssignedAccountID>,
    pub supplier_assigned_account_id: Option<SupplierAssignedAccountID>,
    #[serde(default)]
    pub additional_account_id: Vec<AdditionalAccountID>,
    pub party: Option<Party>,
    pub delivery_contact: Option<Contact>,
    pub accounting_contact: Option<Contact>,
    pub buyer_contact: Option<Contact>,
}
