// UBL 2.5 XML namespace constants.
//
// Reference: UBL 2.5 XSD schemas

/// UBL 2.5 document namespace (no prefix, default for document root)
pub const UBL: &str = "urn:oasis:names:specification:ubl:schema:xsd:Invoice-2";

/// Common Aggregate Components namespace (cac prefix)
pub const CAC: &str = "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2";

/// Common Basic Components namespace (cbc prefix)
pub const CBC: &str = "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2";

/// XMLSchema-instance namespace
pub const XSI: &str = "http://www.w3.org/2001/XMLSchema-instance";

/// Extension namespace
pub const EXT: &str = "urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2";

/// Document namespace for specific types — map from document type name to namespace
pub fn doc_ns(doc_type: &str) -> String {
    format!("urn:oasis:names:specification:ubl:schema:xsd:{doc_type}-2")
}

/// Convert a snake_case Rust field name to UBL PascalCase element name.
/// Handles special cases like ID -> ID, UUID -> UUID, URI -> URI.
pub fn to_element_name(snake: &str) -> String {
    let mut result = String::new();
    let mut chars = snake.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '_' {
            if let Some(&next) = chars.peek() {
                result.push(next.to_ascii_uppercase());
                chars.next();
            }
        } else if result.is_empty() {
            result.push(c.to_ascii_uppercase());
        } else {
            // Check for acronym patterns (consecutive uppercase after _)
            result.push(c);
        }
    }

    // Fix known acronyms that should be all-uppercase
    fix_acronyms(&result)
}

/// Ensure known acronyms are properly cased in UBL element names.
fn fix_acronyms(s: &str) -> String {
    let mut result = s.to_string();
    // Common UBL acronyms that should remain uppercase
    for (pattern, replacement) in [
        ("Id", "ID"),
        ("Uuid", "UUID"),
        ("Uri", "URI"),
        ("Gln", "GLN"),
        ("Gtin", "GTIN"),
        ("Sscc", "SSCC"),
        ("Cv2Id", "CV2ID"),
        ("Undgcode", "UNDGCode"),
        ("Xpath", "XPath"),
        ("Ubldocument", "UBLDocument"),
    ] {
        if result.ends_with(pattern) && result.len() > pattern.len() {
            let prefix_len = result.len() - pattern.len();
            result = format!("{}{}", &result[..prefix_len], replacement);
        } else if result == pattern {
            result = replacement.to_string();
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_element_name() {
        assert_eq!(to_element_name("id"), "ID");
        assert_eq!(to_element_name("uuid"), "UUID");
        assert_eq!(to_element_name("issue_date"), "IssueDate");
        assert_eq!(to_element_name("invoice_type_code"), "InvoiceTypeCode");
        assert_eq!(
            to_element_name("line_extension_amount"),
            "LineExtensionAmount"
        );
        assert_eq!(to_element_name("website_uri"), "WebsiteURI");
        assert_eq!(to_element_name("tax_amount"), "TaxAmount");
    }
}
