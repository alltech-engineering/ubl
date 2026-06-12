// UBL Customer Party aggregate.

use serde::{Deserialize, Serialize};
use crate::cbc::*;
use crate::cac::address::PostalAddress;
use crate::cac::contact::Contact;
use crate::cac::party::{Party, PartyIdentification, PartyName, PartyTaxScheme, PartyLegalEntity, Person};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CustomerParty {
    pub customer_assigned_account_id: Option<CustomerAssignedAccountID>,
    pub supplier_assigned_account_id: Option<SupplierAssignedAccountID>,
    pub additional_account_id: Vec<AdditionalAccountID>,
    pub party: Option<Party>,
    pub delivery_contact: Option<Contact>,
    pub accounting_contact: Option<Contact>,
    pub buyer_contact: Option<Contact>,
}
