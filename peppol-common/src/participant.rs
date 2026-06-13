// Participant Identifiers — Peppol's addressing system.
//
// Peppol uses a structured identifier for every participant:
//   <ICD code>:<participant ID>
// e.g. "9933:za1234567890"
//
// ISO 6523 ICD codes identify the issuing agency.
// Peppol EAS codes extend ICD for electronic addressing.

use serde::{Deserialize, Serialize};
use std::sync::LazyLock;

/// ISO 6523 International Code Designator — identifies the agency that issued
/// the participant identifier.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct IcdCode {
    pub code: String,
    pub agency: String,
    pub country: Option<String>,
}

/// Peppol Electronic Address Scheme — identifies the type of electronic address.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct EASCode {
    pub code: String,
    pub scheme: String,
}

/// A fully qualified Peppol participant identifier.
///
/// Format: `<ICD>:<ID>` (e.g., `9933:za1234567890`)
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ParticipantId {
    pub icd: IcdCode,
    pub id: String,
}

impl ParticipantId {
    pub fn new(icd: IcdCode, id: impl Into<String>) -> Self {
        Self { icd, id: id.into() }
    }

    pub fn parse(s: &str) -> Option<Self> {
        let (icd_code, id) = s.split_once(':')?;
        let icd = IcdCode::by_code(icd_code)?;
        Some(Self {
            icd,
            id: id.to_string(),
        })
    }

    pub fn to_canonical(&self) -> String {
        format!("{}:{}", self.icd.code, self.id)
    }
}

// ── ICD code registry ──

/// Static registry of common ICD codes used in Peppol.
static KNOWN_ICDS: LazyLock<Vec<IcdCode>> = LazyLock::new(|| {
    vec![
        IcdCode {
            code: "9933".into(),
            agency: "South African Company Registration Number".into(),
            country: Some("ZA".into()),
        },
        IcdCode {
            code: "9950".into(),
            agency: "South African VAT Number".into(),
            country: Some("ZA".into()),
        },
        IcdCode {
            code: "0002".into(),
            agency: "Système Informatique pour le Répertoire des Entreprises (SIRENE)".into(),
            country: Some("FR".into()),
        },
        IcdCode {
            code: "0007".into(),
            agency: "Swedish Tax Agency (Skatteverket)".into(),
            country: Some("SE".into()),
        },
        IcdCode {
            code: "0009".into(),
            agency: "SIRET (France)".into(),
            country: Some("FR".into()),
        },
        IcdCode {
            code: "0060".into(),
            agency: "Data Universal Numbering System (DUNS)".into(),
            country: None,
        },
        IcdCode {
            code: "0088".into(),
            agency: "Global Location Number (GLN)".into(),
            country: None,
        },
        IcdCode {
            code: "0096".into(),
            agency: "Danish Chamber of Commerce (DAN:989567)".into(),
            country: Some("DK".into()),
        },
        IcdCode {
            code: "0106".into(),
            agency: "Vereniging van Kamers van Koophandel (Netherlands)".into(),
            country: Some("NL".into()),
        },
        IcdCode {
            code: "0130".into(),
            agency: "Directorates of the Central Administration (DIN)".into(),
            country: None,
        },
        IcdCode {
            code: "0135".into(),
            agency: "National Board of Taxes (Finland)".into(),
            country: Some("FI".into()),
        },
        IcdCode {
            code: "0142".into(),
            agency: "Chamber of Commerce (Belgium)".into(),
            country: Some("BE".into()),
        },
        IcdCode {
            code: "0151".into(),
            agency: "Australian Business Number (ABN)".into(),
            country: Some("AU".into()),
        },
        IcdCode {
            code: "0183".into(),
            agency: "DK Chamber of Commerce (CVR)".into(),
            country: Some("DK".into()),
        },
        IcdCode {
            code: "0184".into(),
            agency: "Commercial Register (Austria)".into(),
            country: Some("AT".into()),
        },
        IcdCode {
            code: "0190".into(),
            agency: "Enterprise Register (Slovenia)".into(),
            country: Some("SI".into()),
        },
        IcdCode {
            code: "0191".into(),
            agency: "Central Register (Slovakia)".into(),
            country: Some("SK".into()),
        },
        IcdCode {
            code: "0192".into(),
            agency: "Business Registers Agency (Norway)".into(),
            country: Some("NO".into()),
        },
        IcdCode {
            code: "0193".into(),
            agency: "Register of Economic Operators (Poland)".into(),
            country: Some("PL".into()),
        },
        IcdCode {
            code: "0195".into(),
            agency: "National Tax and Customs Administration (Hungary)".into(),
            country: Some("HU".into()),
        },
        IcdCode {
            code: "0196".into(),
            agency: "Statistical Office (Croatia)".into(),
            country: Some("HR".into()),
        },
        IcdCode {
            code: "0198".into(),
            agency: "Company Registration Office (Ireland)".into(),
            country: Some("IE".into()),
        },
        IcdCode {
            code: "0200".into(),
            agency: "Registro Mercantil (Spain)".into(),
            country: Some("ES".into()),
        },
        IcdCode {
            code: "0201".into(),
            agency: "Companies House (UK)".into(),
            country: Some("GB".into()),
        },
        IcdCode {
            code: "0204".into(),
            agency: "Commercial Register (Portugal)".into(),
            country: Some("PT".into()),
        },
        IcdCode {
            code: "0205".into(),
            agency: "Tax Administration (Portugal)".into(),
            country: Some("PT".into()),
        },
        IcdCode {
            code: "0208".into(),
            agency: "Business Register (Latvia)".into(),
            country: Some("LV".into()),
        },
        IcdCode {
            code: "0210".into(),
            agency: "Register of Legal Entities (Lithuania)".into(),
            country: Some("LT".into()),
        },
        IcdCode {
            code: "0211".into(),
            agency: "Centre of Registers and Information (Estonia)".into(),
            country: Some("EE".into()),
        },
        IcdCode {
            code: "0213".into(),
            agency: "Business Register (Italy)".into(),
            country: Some("IT".into()),
        },
        IcdCode {
            code: "0215".into(),
            agency: "Commercial Register (Luxembourg)".into(),
            country: Some("LU".into()),
        },
        IcdCode {
            code: "0216".into(),
            agency: "Department of Registrar of Companies (Cyprus)".into(),
            country: Some("CY".into()),
        },
        IcdCode {
            code: "0218".into(),
            agency: "Business Register (Malta)".into(),
            country: Some("MT".into()),
        },
        IcdCode {
            code: "0221".into(),
            agency: "Companies Registration Office (New Zealand)".into(),
            country: Some("NZ".into()),
        },
        IcdCode {
            code: "0230".into(),
            agency: "Commercial Register (Germany)".into(),
            country: Some("DE".into()),
        },
    ]
});

/// Static registry of EAS codes.
static KNOWN_EAS: LazyLock<Vec<EASCode>> = LazyLock::new(|| {
    vec![
        EASCode {
            code: "0002".into(),
            scheme: "SIRENE (France)".into(),
        },
        EASCode {
            code: "0007".into(),
            scheme: "Swedish Org Number".into(),
        },
        EASCode {
            code: "0009".into(),
            scheme: "SIRET (France)".into(),
        },
        EASCode {
            code: "0088".into(),
            scheme: "GLN (Global Location Number)".into(),
        },
        EASCode {
            code: "0183".into(),
            scheme: "DK CVR Number".into(),
        },
        EASCode {
            code: "0184".into(),
            scheme: "Austrian VAT".into(),
        },
        EASCode {
            code: "0192".into(),
            scheme: "Norwegian Org Number".into(),
        },
        EASCode {
            code: "0196".into(),
            scheme: "Croatian OIB".into(),
        },
        EASCode {
            code: "0201".into(),
            scheme: "UK Company Number".into(),
        },
        EASCode {
            code: "0208".into(),
            scheme: "Latvian Registration Number".into(),
        },
        EASCode {
            code: "0210".into(),
            scheme: "Lithuanian Company Code".into(),
        },
        EASCode {
            code: "9933".into(),
            scheme: "South African CIPC Registration".into(),
        },
        EASCode {
            code: "9950".into(),
            scheme: "South African VAT Number".into(),
        },
    ]
});

impl IcdCode {
    /// Look up an ICD code by its 4-digit number.
    pub fn by_code(code: &str) -> Option<IcdCode> {
        KNOWN_ICDS.iter().find(|icd| icd.code == code).cloned()
    }

    /// Look up an EAS code.
    pub fn by_eas(code: &str) -> Option<IcdCode> {
        KNOWN_EAS
            .iter()
            .find(|eas| eas.code == code)
            .map(|eas| IcdCode {
                code: eas.code.clone(),
                agency: eas.scheme.clone(),
                country: None,
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_participant_id_parse() {
        let pid = ParticipantId::parse("9933:za1234567890").unwrap();
        assert_eq!(pid.icd.code, "9933");
        assert_eq!(pid.id, "za1234567890");
        assert_eq!(pid.to_canonical(), "9933:za1234567890");
    }

    #[test]
    fn test_icd_by_code() {
        let icd = IcdCode::by_code("9933").unwrap();
        assert_eq!(icd.agency, "South African Company Registration Number");
        assert_eq!(icd.country, Some("ZA".into()));
    }

    #[test]
    fn test_participant_id_serialization() {
        let pid = ParticipantId::new(IcdCode::by_code("9933").unwrap(), "za1234567890");
        let json = serde_json::to_string(&pid).unwrap();
        let pid2: ParticipantId = serde_json::from_str(&json).unwrap();
        assert_eq!(pid.to_canonical(), pid2.to_canonical());
    }
}
