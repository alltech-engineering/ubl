#!/usr/bin/env python3
"""
Inject idiomatic Rust doc comments (///) into generated UBL struct definitions,
sourced from XSD ccts:Component annotations.

Maps XSD type names -> Rust struct names by stripping 'Type' suffix then
converting through snake_case -> PascalCase (matching the xsd-parser generator).

Run: python3 tools/inject_docs.py --dry-run   (preview)
     python3 tools/inject_docs.py --apply      (write changes)
"""
import xml.etree.ElementTree as ET
import os, re, sys
from pathlib import Path

SPEC_DIR = Path("/Users/stephanus/Github/alltech/ubl/spec/cs01-UBL-2.5/xsd")
WORKSPACE = Path("/Users/stephanus/Github/alltech/ubl")

NS = {
    "xsd": "http://www.w3.org/2001/XMLSchema",
    "ccts": "urn:un:unece:uncefact:documentation:2",
}

XSD_SOURCES = [
    (SPEC_DIR / "common/UBL-CommonAggregateComponents-2.5.xsd"),
    (SPEC_DIR / "common/UBL-CommonExtensionComponents-2.5.xsd"),
    (SPEC_DIR / "common/BDNDR-CCTS_CCT_SchemaModule-1.1.xsd"),
    (SPEC_DIR / "common/BDNDR-UnqualifiedDataTypes-1.1.xsd"),
] + sorted((SPEC_DIR / "maindoc").glob("*.xsd"))


def text_of(elem):
    return " ".join((elem.text or "").split()) if elem is not None else ""


def xsd_to_rust_name(xsd_type_name):
    """Convert XSD complexType name to Rust struct name.
    'UBLExtensionsType' -> 'UblExtensions', 'PartyType' -> 'Party'
    """
    base = xsd_type_name
    if base.endswith("Type"):
        base = base[:-4]
    s1 = re.sub(r'([A-Z]+)([A-Z][a-z])', r'\1_\2', base)
    s2 = re.sub(r'([a-z0-9])([A-Z])', r'\1_\2', s1)
    snake = s2.lower()
    return ''.join(w.capitalize() for w in snake.split('_'))


def extract_doc(annotation_elem):
    if annotation_elem is None:
        return {}
    doc_elem = annotation_elem.find("xsd:documentation", NS)
    if doc_elem is None:
        return {}
    defn, den = "", ""
    comp = doc_elem.find("ccts:Component", NS)
    if comp is not None:
        for child in comp:
            tag = child.tag.split("}")[-1] if "}" in child.tag else child.tag
            if tag == "Definition": defn = text_of(child)
            elif tag == "DictionaryEntryName": den = text_of(child)
        return {"definition": defn, "dictionary_entry_name": den}
    for child in doc_elem:
        tag = child.tag.split("}")[-1] if "}" in child.tag else child.tag
        if tag == "Definition": defn = text_of(child)
        elif tag == "DictionaryEntryName": den = text_of(child)
    if defn:
        return {"definition": defn, "dictionary_entry_name": den}
    plain = "".join(doc_elem.itertext()).strip()
    return {"definition": plain} if plain else {}


def build_xsd_db():
    """Parse all XSDs, keyed by RUST struct name."""
    db = {}
    for xsd_path in XSD_SOURCES:
        try:
            tree = ET.parse(xsd_path)
            root = tree.getroot()
            for ct in root.findall("xsd:complexType", NS):
                xsd_name = ct.get("name", "")
                if not xsd_name:
                    continue
                rust_name = xsd_to_rust_name(xsd_name)
                ann = ct.find("xsd:annotation", NS)
                doc = extract_doc(ann)
                fields = {}
                # Sequence elements
                sequence = ct.find("xsd:sequence", NS)
                if sequence is not None:
                    for elem in sequence.findall("xsd:element", NS):
                        ref = elem.get("ref", "")
                        if not ref:
                            continue
                        local = ref.split(":")[-1] if ":" in ref else ref
                        fann = elem.find("xsd:annotation", NS)
                        fdoc = extract_doc(fann)
                        fields[local] = fdoc.get("definition", "")
                # Attributes (CCT/UDT types with simpleContent)
                for sc in ct.findall("xsd:simpleContent", NS):
                    for ext in sc.findall("xsd:extension", NS) + sc.findall("xsd:restriction", NS):
                        for attr in ext.findall("xsd:attribute", NS):
                            attr_name = attr.get("name", "")
                            if not attr_name:
                                continue
                            ann = attr.find("xsd:annotation", NS)
                            adoc = extract_doc(ann)
                            fields[f"@{attr_name}"] = adoc.get("definition", "")
                # Attributes from complexContent
                for cc in ct.findall("xsd:complexContent", NS):
                    for ext in cc.findall("xsd:extension", NS):
                        for attr in ext.findall("xsd:attribute", NS):
                            attr_name = attr.get("name", "")
                            if not attr_name:
                                continue
                            ann = attr.find("xsd:annotation", NS)
                            adoc = extract_doc(ann)
                            fields[f"@{attr_name}"] = adoc.get("definition", "")
                db[rust_name] = {
                    "xsd_name": xsd_name,
                    "definition": doc.get("definition", ""),
                    "dictionary_entry_name": doc.get("dictionary_entry_name", ""),
                    "fields": fields,
                }
        except Exception as e:
            print(f"  WARNING: {xsd_path}: {e}")
    return db


def find_structs_in_file(filepath):
    """Find all pub struct definitions.
    Returns [(struct_name, struct_line, end_line, fields: [(field_name, xml_rename, doc_insert_line)])]
    doc_insert_line is the line BEFORE which to insert the field's doc comment (the serde attr line).
    """
    with open(filepath) as f:
        lines = f.readlines()
    
    structs = []
    i = 0
    while i < len(lines):
        line = lines[i]
        
        if 'pub struct' not in line and not (line.strip().startswith('#[') and 'derive' in line):
            i += 1
            continue
        
        m = re.match(r'pub\s+struct\s+(\w+)', line)
        if m:
            struct_name = m.group(1)
            struct_line = i
        else:
            j = i
            while j < len(lines) and (lines[j].strip().startswith('#[') or 
                                        lines[j].strip().startswith('///') or
                                        lines[j].strip() == ''):
                j += 1
            if j < len(lines):
                m2 = re.match(r'pub\s+struct\s+(\w+)', lines[j])
                if m2:
                    struct_name = m2.group(1)
                    struct_line = j
                    i = j
                else:
                    i += 1
                    continue
            else:
                i += 1
                continue
        
        # Find closing }
        depth = 0
        end_line = struct_line
        for k in range(struct_line, len(lines)):
            depth += lines[k].count('{') - lines[k].count('}')
            if depth == 0 and k > struct_line:
                end_line = k
                break
        
        # Extract fields with correct doc insertion points
        fields = []
        k = struct_line + 1
        while k < end_line:
            fline = lines[k]
            
            # Check if this is a field line
            fm = re.match(r'\s*pub\s+(\w+)\s*:', fline)
            if fm:
                field_name = fm.group(1)
                
                # Look backwards for serde rename attribute (up to 3 lines)
                rename = None
                attr_line = k  # default: insert doc before the pub field line
                for back in range(1, 4):
                    prev_idx = k - back
                    if prev_idx < struct_line:
                        break
                    prev = lines[prev_idx].strip()
                    if prev.startswith('#[serde') and 'rename' in prev:
                        rm = re.search(r'rename\s*=\s*"([^"]+)"', prev)
                        if rm:
                            rename = rm.group(1)
                        attr_line = prev_idx
                        break
                    elif prev.startswith('#[') or prev.startswith('///') or prev == '':
                        continue
                    else:
                        break
                
                fields.append((field_name, rename, attr_line))
            
            k += 1
        
        structs.append((struct_name, struct_line, end_line, fields))
        i = end_line + 1
    
    return structs


def wrap_text(text, max_width=100):
    if not text:
        return ""
    text = " ".join(text.split())
    if len(text) <= max_width:
        return text
    words = text.split()
    lines = []
    current = ""
    for w in words:
        if len(current) + len(w) + 1 <= max_width:
            current = (current + " " + w).strip()
        else:
            lines.append(current)
            current = w
    if current:
        lines.append(current)
    return "\n/// ".join(lines)


def format_struct_doc(type_info):
    lines = []
    defn = type_info.get("definition", "")
    den = type_info.get("dictionary_entry_name", "")
    xsd_name = type_info.get("xsd_name", "")
    if defn:
        lines.append(f"/// {wrap_text(defn)}")
    if den:
        lines.append("///")
        lines.append(f"/// UBL Dictionary Entry Name: `{den}`")
    lines.append("///")
    lines.append(f"/// Generated from XSD type `{xsd_name}`.")
    return lines


def inject_docs(filepath, xsd_db, dry_run=False):
    """Inject docs into one file. Returns (modified_content_or_None, num_documented)."""
    structs = find_structs_in_file(filepath)
    if not structs:
        return None, 0
    
    with open(filepath) as f:
        lines = f.readlines()
    
    new_lines = list(lines)
    total_documented = 0
    
    # Process bottom-up (by struct)
    for struct_name, struct_line, end_line, fields in reversed(structs):
        type_info = xsd_db.get(struct_name)
        if not type_info:
            continue
        
        # Collect all insertions for this struct: (line_idx, text)
        # Then apply bottom-up within the struct so indices stay valid
        insertions = []
        
        # Struct-level doc
        struct_doc = format_struct_doc(type_info)
        insertions.append((struct_line, "\n".join(struct_doc) + "\n"))
        total_documented += 1
        
        # Field docs
        xsd_fields = type_info.get("fields", {})
        for field_name, xml_rename, attr_line in fields:
            xml_key = xml_rename if xml_rename else field_name
            if xml_key and xml_key.startswith("$"):
                continue
            
            field_def = xsd_fields.get(xml_key, "")
            if field_def:
                insertions.append((attr_line, f"/// {wrap_text(field_def)}\n"))
                total_documented += 1
        
        # Apply all insertions for this struct, highest index first
        for line_idx, text in sorted(insertions, key=lambda x: -x[0]):
            new_lines.insert(line_idx, text)
    
    if total_documented == 0:
        return None, 0
    return "".join(new_lines), total_documented


def main():
    dry_run = "--apply" not in sys.argv
    
    print("Building XSD doc database...")
    xsd_db = build_xsd_db()
    print(f"  {len(xsd_db)} Rust-mapped types indexed")
    
    for name in ["Party", "Invoice", "Address", "Amount", "UblExtensions"]:
        if name in xsd_db:
            info = xsd_db[name]
            print(f"  {name}: '{info['definition'][:60]}...' ({len(info['fields'])} fields)")
    
    rust_roots = [
        WORKSPACE / "cac/src",
        WORKSPACE / "cct/src",
        WORKSPACE / "udt/src",
        WORKSPACE / "ext/src",
        WORKSPACE / "maindoc/src/documents",
    ]
    
    stats = {"files": 0, "structs": 0}
    
    # Dry run: check what changes would be made
    if dry_run:
        print("\nDRY RUN — checking changes...")
    
    for root in rust_roots:
        if not root.exists():
            continue
        for filepath in sorted(root.rglob("*.rs")):
            if filepath.name == "lib.rs" and filepath.stat().st_size < 200:
                # Tiny lib.rs files are likely just module declarations; skip
                # Larger ones (like udt/src/lib.rs) contain real struct definitions
                continue
            result, count = inject_docs(filepath, xsd_db, dry_run)
            if result:
                if not dry_run:
                    with open(filepath, "w") as f:
                        f.write(result)
                stats["files"] += 1
                stats["structs"] += count
                if dry_run:
                    print(f"  [DRY] {filepath.relative_to(WORKSPACE)} ({count} docs)")
    
    if dry_run:
        print(f"\nDRY RUN complete — {stats['files']} files, {stats['structs']} doc comments would be added.")
        print("Run with --apply to write changes.")
    else:
        print(f"\nDone. {stats['files']} files modified, {stats['structs']} doc comments added.")


if __name__ == "__main__":
    main()
