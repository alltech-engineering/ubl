use std::path::PathBuf;

use async_trait::async_trait;
use serde::Deserialize;
use tokio::fs;
use uuid::Uuid;

use crate::{Storage, StorageError, StoredDocument};

/// File-system storage backend that persists each document as an XML file.
///
/// Files are written to a configurable output directory with the naming
/// convention `{document_type}_{document_id}_{uuid}.xml`.
///
/// When the document payload carries Peppol identity markers (a top-level
/// `identity` field with `customization_id` / `profile_id`) and the document
/// type is one of the known UBL types, the XML body is proper Peppol UBL XML
/// produced by `peppol-xml`.  Otherwise the file still contains an XML
/// document, but with the raw JSON payload embedded as a comment and as text
/// content.
///
/// The full `StoredDocument` metadata is always serialized to a JSON comment
/// on the second line so that `get()` and `list()` can reconstruct documents
/// without re-parsing the UBL XML.
#[derive(Debug, Clone)]
pub struct FileStore {
    dir: PathBuf,
}

impl FileStore {
    /// Create a new `FileStore` that writes files under `dir`.
    ///
    /// The directory is created (recursively) if it does not exist.
    pub async fn new(dir: impl Into<PathBuf>) -> Result<Self, StorageError> {
        let dir = dir.into();
        fs::create_dir_all(&dir)
            .await
            .map_err(|e| StorageError::Internal(format!("create directory {:?}: {}", dir, e)))?;
        Ok(Self { dir })
    }

    /// Build the filename for a document: `{doc_type}_{doc_id}_{uuid}.xml`
    fn filename(doc: &StoredDocument) -> String {
        format!("{}_{}_{}.xml", doc.document_type, doc.document_id, doc.id)
    }

    /// Serialize a `StoredDocument` into the XML bytes written to disk.
    fn serialize_document(doc: &StoredDocument) -> Result<String, StorageError> {
        // The stored-document JSON (embedded as a comment so we can
        // reconstruct on read without re-parsing the XML body).
        let meta_json = serde_json::to_string(doc)
            .map_err(|e| StorageError::Internal(format!("serialize metadata: {}", e)))?;

        // Try to detect Peppol identity and produce proper UBL XML.
        let body_xml = try_peppol_serialize(doc).unwrap_or_else(|_| {
            // Fallback: embed the raw JSON payload.
            let payload_json = serde_json::to_string_pretty(&doc.payload)
                .unwrap_or_else(|_| String::from("{}"));
            format!(
                "<!-- Fallback JSON payload -->\n<stored-document>{}</stored-document>",
                xml_escape(&payload_json)
            )
        });

        Ok(format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n<!-- {} -->\n{}",
            meta_json, body_xml
        ))
    }

    /// Read an XML file and extract the `StoredDocument` from the comment.
    fn deserialize_document(xml: &str) -> Result<StoredDocument, StorageError> {
        // The second line is the JSON comment with the full StoredDocument.
        let second_line = xml
            .lines()
            .nth(1)
            .ok_or_else(|| StorageError::Internal("missing metadata comment".into()))?;
        let json = second_line
            .trim()
            .strip_prefix("<!-- ")
            .and_then(|s| s.strip_suffix(" -->"))
            .ok_or_else(|| StorageError::Internal("malformed metadata comment".into()))?;
        serde_json::from_str(json)
            .map_err(|e| StorageError::Internal(format!("deserialize metadata: {}", e)))
    }

    /// Find the file on disk whose name contains the given UUID.
    async fn find_file(&self, id: Uuid) -> Result<Option<PathBuf>, StorageError> {
        let uuid_str = id.to_string();
        let mut entries = fs::read_dir(&self.dir)
            .await
            .map_err(|e| StorageError::Internal(format!("read dir {:?}: {}", self.dir, e)))?;
        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| StorageError::Internal(format!("dir entry: {}", e)))?
        {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if name_str.ends_with(".xml") && name_str.contains(&uuid_str) {
                return Ok(Some(entry.path()));
            }
        }
        Ok(None)
    }
}

// ── Storage trait implementation ───────────────────────────────────────────

#[async_trait]
impl Storage for FileStore {
    async fn store(&self, doc: StoredDocument) -> Result<StoredDocument, StorageError> {
        let name = Self::filename(&doc);
        let path = self.dir.join(&name);
        let xml = Self::serialize_document(&doc)?;
        fs::write(&path, &xml)
            .await
            .map_err(|e| StorageError::Internal(format!("write {:?}: {}", path, e)))?;
        Ok(doc)
    }

    async fn get(&self, id: Uuid) -> Result<Option<StoredDocument>, StorageError> {
        match self.find_file(id).await? {
            Some(path) => {
                let xml = fs::read_to_string(&path)
                    .await
                    .map_err(|e| StorageError::Internal(format!("read {:?}: {}", path, e)))?;
                let doc = Self::deserialize_document(&xml)?;
                Ok(Some(doc))
            }
            None => Ok(None),
        }
    }

    async fn list(
        &self,
        doc_type: Option<&str>,
        limit: usize,
    ) -> Result<Vec<StoredDocument>, StorageError> {
        let mut docs: Vec<StoredDocument> = Vec::new();
        let mut entries = fs::read_dir(&self.dir)
            .await
            .map_err(|e| StorageError::Internal(format!("read dir {:?}: {}", self.dir, e)))?;
        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| StorageError::Internal(format!("dir entry: {}", e)))?
        {
            if docs.len() >= limit {
                break;
            }
            let name = entry.file_name();
            let name_str = name.to_string_lossy();
            if !name_str.ends_with(".xml") {
                continue;
            }
            let xml = fs::read_to_string(entry.path())
                .await
                .map_err(|e| StorageError::Internal(format!("read {:?}: {}", entry.path(), e)))?;
            match Self::deserialize_document(&xml) {
                Ok(doc) => {
                    if let Some(dt) = doc_type {
                        if doc.document_type != dt {
                            continue;
                        }
                    }
                    docs.push(doc);
                }
                Err(_) => {
                    // Skip malformed files silently.
                }
            }
        }
        Ok(docs)
    }

    async fn delete(&self, id: Uuid) -> Result<(), StorageError> {
        match self.find_file(id).await? {
            Some(path) => {
                fs::remove_file(&path)
                    .await
                    .map_err(|e| StorageError::Internal(format!("remove {:?}: {}", path, e)))?;
                Ok(())
            }
            None => Err(StorageError::NotFound(id.to_string())),
        }
    }
}

// ── Peppol XML serialization helpers ───────────────────────────────────────

/// Shape of a Peppol-wrapped JSON payload.
#[derive(Debug, Deserialize)]
struct PeppolWrapped {
    identity: Option<peppol_common::identity::DocumentIdentity>,
    #[serde(flatten)]
    _rest: serde_json::Value,
}

/// Attempt to serialize the document payload as proper Peppol UBL XML.
///
/// Returns `Ok(xml_string)` when the payload contains Peppol identity
/// markers *and* we know how to deserialize the document type.
fn try_peppol_serialize(doc: &StoredDocument) -> Result<String, StorageError> {
    // For Order documents, always produce proper UBL XML even without Peppol identity.
    // For other types, require Peppol identity markers.
    let is_order = doc.document_type == "Order";

    let has_identity = doc
        .payload
        .as_object()
        .map_or(false, |o| o.contains_key("identity"));

    if !has_identity && !is_order {
        return Err(StorageError::Internal("no Peppol identity in payload".into()));
    }

    // For non-Order with identity, extract it. For Order without identity, use defaults.
    let identity = if has_identity {
        let wrapped: PeppolWrapped = serde_json::from_value(doc.payload.clone())
            .map_err(|e| StorageError::Internal(format!("deserialize wrapped payload: {e}")))?;
        wrapped
            .identity
            .unwrap_or_else(|| peppol_common::identity::identities::ordering_3_0("Order"))
    } else {
        // Order without explicit identity — use Ordering BIS defaults
        peppol_common::identity::identities::ordering_3_0("Order")
    };

    // Match on document type
    match doc.document_type.as_str() {
        "Invoice" => {
            let invoice: ubl_documents::billing::Invoice =
                serde_json::from_value(doc.payload.clone())
                    .map_err(|e| StorageError::Internal(format!("deserialize Invoice: {e}")))?;
            let peppol = peppol_xml::PeppolDocument {
                document: invoice,
                identity,
                root_element: "Invoice",
                root_namespace: "urn:oasis:names:specification:ubl:schema:xsd:Invoice-2",
            };
            peppol_xml::to_peppol_xml(&peppol)
                .map_err(|e| StorageError::Internal(format!("peppol xml: {e}")))
        }
        "Order" => {
            let order: ubl_documents::ordering::Order =
                serde_json::from_value(doc.payload.clone())
                    .map_err(|e| StorageError::Internal(format!("deserialize Order: {e}")))?;
            let peppol = peppol_xml::PeppolDocument {
                document: order,
                identity,
                root_element: "Order",
                root_namespace: "urn:oasis:names:specification:ubl:schema:xsd:Order-2",
            };
            peppol_xml::to_peppol_xml(&peppol)
                .map_err(|e| StorageError::Internal(format!("peppol xml: {e}")))
        }
        // Future: CreditNote, Order, etc. once ToXml impls exist.
        _ => {
            let payload_json = serde_json::to_string_pretty(&doc.payload)
                .unwrap_or_else(|_| String::from("{}"));
            Ok(format!(
                "<!-- Fallback JSON payload (unsupported Peppol type: {}) -->\n<stored-document>{}</stored-document>",
                doc.document_type,
                xml_escape(&payload_json)
            ))
        }
    }
}

/// Minimal XML escaping for text content.
fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json::json;

    fn make_doc(doc_type: &str, doc_id: &str, payload: serde_json::Value) -> StoredDocument {
        StoredDocument {
            id: Uuid::new_v4(),
            document_type: doc_type.into(),
            document_id: doc_id.into(),
            payload,
            created_at: Utc::now(),
            validated: false,
            validation_errors: json!({}),
        }
    }

    #[test]
    fn test_serialize_non_peppol_payload() {
        let doc = make_doc("Invoice", "INV-001", json!({"amount": 100}));
        let xml = FileStore::serialize_document(&doc).unwrap();
        // Must contain the metadata comment and fallback body.
        assert!(xml.contains("<!-- {"), "missing metadata comment");
        assert!(xml.contains("<stored-document>"), "missing fallback wrapper");
    }

    #[test]
    fn test_serialize_peppol_invoice() {
        let payload = json!({
            "identity": {
                "customization_id": "urn:fdc:peppol.eu:2017:poacc:billing:3.0::2.1",
                "profile_id": "urn:fdc:peppol.eu:2017:poacc:billing:01:1.0"
            },
            "id": {"value": "INV-001"},
            "issue_date": "2026-06-13",
            "invoice_type_code": {"value": "380"},
            "document_currency_code": {"value": "ZAR"},
            "note": [{"value": "Test"}],
            "accounting_supplier_party": {
                "party": {
                    "party_name": [{"name": "Acme Corp"}],
                    "party_identification": [{"id": {"value": "9933:za1234567890"}}],
                    "postal_address": {
                        "street_name": "123 Main St",
                        "city_name": "Cape Town",
                        "postal_zone": {"value": "8001"},
                        "country": {"identification_code": {"value": "ZA"}},
                        "address_line": []
                    }
                }
            },
            "legal_monetary_total": {
                "line_extension_amount": {"value": "100.00", "currency_id": "ZAR"},
                "payable_amount": {"value": "115.00", "currency_id": "ZAR"}
            },
            "invoice_line": [{
                "id": {"value": "1"},
                "line_extension_amount": {"value": "100.00", "currency_id": "ZAR"},
                "item": {"name": "Widget"}
            }]
        });
        let doc = make_doc("Invoice", "INV-001", payload);
        let xml = FileStore::serialize_document(&doc).unwrap();
        // Should contain Peppol identifiers and UBL content.
        assert!(xml.contains("CustomizationID"), "missing CustomizationID");
        assert!(xml.contains("ProfileID"), "missing ProfileID");
        assert!(xml.contains("<cbc:ID>INV-001</cbc:ID>"), "missing invoice ID");
    }

    #[test]
    fn test_roundtrip_deserialize() {
        let doc = make_doc("Invoice", "INV-002", json!({"total": 250.0}));
        let xml = FileStore::serialize_document(&doc).unwrap();
        let restored = FileStore::deserialize_document(&xml).unwrap();
        assert_eq!(restored.id, doc.id);
        assert_eq!(restored.document_type, "Invoice");
        assert_eq!(restored.document_id, "INV-002");
        assert_eq!(restored.payload, doc.payload);
    }

    #[test]
    fn test_filename_format() {
        let doc = make_doc("Order", "ORD-42", json!({}));
        let name = FileStore::filename(&doc);
        assert!(name.starts_with("Order_ORD-42_"));
        assert!(name.ends_with(".xml"));
    }

    #[tokio::test]
    async fn test_store_and_get_and_delete() {
        let dir = std::env::temp_dir().join("peppol-filestore-test");
        let _ = fs::remove_dir_all(&dir).await;
        let store = FileStore::new(&dir).await.unwrap();

        let doc = make_doc("Invoice", "INV-TEST", json!({"total": 99.99}));

        // store
        let stored = store.store(doc.clone()).await.unwrap();
        assert_eq!(stored.id, doc.id);

        // get
        let fetched = store.get(doc.id).await.unwrap();
        assert!(fetched.is_some());
        let fetched = fetched.unwrap();
        assert_eq!(fetched.id, doc.id);
        assert_eq!(fetched.document_type, "Invoice");
        assert_eq!(fetched.payload, doc.payload);

        // list
        let list = store.list(None, 10).await.unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].id, doc.id);

        // list with filter
        let list = store.list(Some("Order"), 10).await.unwrap();
        assert_eq!(list.len(), 0);
        let list = store.list(Some("Invoice"), 10).await.unwrap();
        assert_eq!(list.len(), 1);

        // delete
        store.delete(doc.id).await.unwrap();
        let fetched = store.get(doc.id).await.unwrap();
        assert!(fetched.is_none());

        // delete non-existent should error
        let err = store.delete(doc.id).await.unwrap_err();
        assert!(matches!(err, StorageError::NotFound(_)));

        // cleanup
        let _ = fs::remove_dir_all(&dir).await;
    }
}
