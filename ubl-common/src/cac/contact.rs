// UBL Contact aggregate — person or department contact information.

use serde::{Deserialize, Serialize};
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Contact {
    pub id: Option<ID>,
    pub name: Option<Name>,
    pub telephone: Option<Telephone>,
    pub telefax: Option<Telephone>,
    pub electronic_mail: Option<Text>,
    pub note: Option<Note>,
    #[serde(default)]
    pub other_communication: Vec<Communication>,
    pub job_title: Option<JobTitle>,
    pub department: Option<Department>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Communication {
    pub channel_code: Option<ChannelCode>,
    pub channel: Option<Channel>,
    pub value: Option<Text>,
}
