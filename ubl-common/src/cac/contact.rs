// UBL Contact aggregate — person or department contact information.

use crate::cbc::*;
use serde::{Deserialize, Serialize};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_roundtrip() {
        let c = Contact {
            id: None,
            department: None,
            electronic_mail: Some(Text {
                value: "john@example.com".into(),
                language_id: None,
            }),
            job_title: None,
            name: Some(Name::new("John Doe")),
            note: None,
            telefax: None,
            telephone: Some(Telephone::new("+27 21 555 1234")),
            other_communication: vec![],
        };
        let json = serde_json::to_string(&c).unwrap();
        let c2: Contact = serde_json::from_str(&json).unwrap();
        assert_eq!(c.name.unwrap().0, c2.name.unwrap().0);
    }
}
