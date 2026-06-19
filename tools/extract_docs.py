#!/usr/bin/env python3
"""
Extract documentation from all UBL 2.5 XSD annotations and generate comprehensive
reference documentation.

Handles both annotation styles:
1. CAC/maindoc: <ccts:Component> wrapper
2. CCT/UDT: direct <ccts:Definition> etc. (no Component wrapper)

Output: docs/generated/{cac,maindoc,cct,udt,ext,cbc}.md
"""
import xml.etree.ElementTree as ET
import os, re, sys
from pathlib import Path
from collections import OrderedDict

SPEC_DIR = Path("/Users/stephanus/Github/alltech/ubl/spec/cs01-UBL-2.5/xsd")
COMMON_SRC = Path("/Users/stephanus/Github/alltech/ubl/common/src")
OUTPUT_DIR = Path("/Users/stephanus/Github/alltech/ubl/docs/generated")

NS = {
    "xsd": "http://www.w3.org/2001/XMLSchema",
    "ccts": "urn:un:unece:uncefact:documentation:2",
    "cac": "urn:oasis:names:specification:ubl:schema:xsd:CommonAggregateComponents-2",
    "cbc": "urn:oasis:names:specification:ubl:schema:xsd:CommonBasicComponents-2",
    "ext": "urn:oasis:names:specification:ubl:schema:xsd:CommonExtensionComponents-2",
    "udt": "urn:oasis:names:specification:bdndr:schema:xsd:UnqualifiedDataTypes-1",
    "qdt": "urn:oasis:names:specification:ubl:schema:xsd:QualifiedDataTypes-2",
    "sac": "urn:oasis:names:specification:ubl:schema:xsd:SignatureAggregateComponents-2",
    "sbc": "urn:oasis:names:specification:ubl:schema:xsd:SignatureBasicComponents-2",
    "ds": "http://www.w3.org/2000/09/xmldsig#",
    "cct": "urn:un:unece:uncefact:data:specification:CoreComponentTypeSchemaModule:2",
}

CRATE_MAP = {
    "cac": "cac", "cbc": "cbc", "ext": "ext",
    "sac": "sac", "sbc": "sbc",
    "cct": "cct", "udt": "udt",
}


def strip_type_suffix(name):
    if name.endswith("Type"):
        return name[:-4]
    return name


def text_of(elem):
    return " ".join((elem.text or "").split())


def extract_flat_ccts(doc_elem):
    """Extract ccts fields directly under documentation (CCT/UDT style, no Component wrapper)."""
    fields = {}
    for child in doc_elem:
        tag = child.tag.split("}")[-1] if "}" in child.tag else child.tag
        if tag in ("UniqueID", "CategoryCode", "DictionaryEntryName",
                    "VersionID", "Definition", "RepresentationTermName",
                    "PrimitiveType", "ObjectClass", "PropertyTermName",
                    "PropertyTerm", "Cardinality"):
            fields[tag] = text_of(child)
    return fields


def extract_ccts_component(doc_elem):
    """Extract ccts fields from a <ccts:Component> wrapper (CAC/maindoc style)."""
    comp = doc_elem.find("ccts:Component", NS)
    if comp is not None:
        fields = {}
        for child in comp:
            tag = child.tag.split("}")[-1] if "}" in child.tag else child.tag
            if tag in ("ComponentType", "DictionaryEntryName", "Definition",
                        "ObjectClass", "Cardinality", "PropertyTerm",
                        "PropertyTermQualifier", "AssociatedObjectClass",
                        "RepresentationTerm", "DataType"):
                fields[tag] = text_of(child)
        return fields
    return {}


def extract_annotation_doc(annotation_elem):
    """Extract documentation from an annotation element. Returns dict of ccts fields."""
    doc_elem = annotation_elem.find("xsd:documentation", NS)
    if doc_elem is None:
        return {}
    
    # Try Component wrapper first (CAC/maindoc style)
    fields = extract_ccts_component(doc_elem)
    if fields:
        return fields
    
    # Fall back to flat ccts fields (CCT/UDT style)
    fields = extract_flat_ccts(doc_elem)
    if fields:
        return fields
    
    # Plain text
    text = "".join(doc_elem.itertext()).strip()
    return {"Definition": text} if text else {}


def parse_complex_type(ct_elem, namespace_ctx=None):
    """Parse a complexType element. Returns type info dict or None."""
    name = ct_elem.get("name", "")
    if not name:
        return None
    
    rust_name = strip_type_suffix(name)
    
    # Type-level annotation
    annotation = ct_elem.find("xsd:annotation", NS)
    type_doc = extract_annotation_doc(annotation) if annotation is not None else {}
    
    # Fields (from sequence)
    fields = []
    sequence = ct_elem.find("xsd:sequence", NS)
    if sequence is not None:
        for elem in sequence.findall("xsd:element", NS):
            ref = elem.get("ref", "")
            min_occ = elem.get("minOccurs", "1")
            max_occ = elem.get("maxOccurs", "1")
            
            field_ann = elem.find("xsd:annotation", NS)
            field_doc = extract_annotation_doc(field_ann) if field_ann is not None else {}
            
            ns_prefix, local = (ref.split(":", 1) + [None])[:2] if ":" in ref else (None, ref)
            
            fields.append({
                "ref": ref,
                "local": local or ref,
                "ns_prefix": ns_prefix,
                "min_occurs": min_occ,
                "max_occurs": max_occ,
                "doc": field_doc,
            })
    
    # Attributes (for CCT/UDT types)
    attributes = []
    # Check simpleContent -> extension/restriction -> attribute
    for sc in ct_elem.findall("xsd:simpleContent", NS):
        for ext in sc.findall("xsd:extension", NS) + sc.findall("xsd:restriction", NS):
            for attr in ext.findall("xsd:attribute", NS):
                attr_name = attr.get("name", "")
                attr_type = attr.get("type", "")
                attr_ann = attr.find("xsd:annotation", NS)
                attr_doc = extract_annotation_doc(attr_ann) if attr_ann is not None else {}
                attributes.append({
                    "name": attr_name,
                    "type": attr_type,
                    "doc": attr_doc,
                })
    
    # Check complexContent
    for cc in ct_elem.findall("xsd:complexContent", NS):
        for ext in cc.findall("xsd:extension", NS):
            seq = ext.find("xsd:sequence", NS)
            if seq is not None:
                for elem in seq.findall("xsd:element", NS):
                    ref = elem.get("ref", "")
                    min_occ = elem.get("minOccurs", "1")
                    max_occ = elem.get("maxOccurs", "1")
                    field_ann = elem.find("xsd:annotation", NS)
                    field_doc = extract_annotation_doc(field_ann) if field_ann is not None else {}
                    ns_prefix, local = (ref.split(":", 1) + [None])[:2] if ":" in ref else (None, ref)
                    fields.append({
                        "ref": ref,
                        "local": local or ref,
                        "ns_prefix": ns_prefix,
                        "min_occurs": min_occ,
                        "max_occurs": max_occ,
                        "doc": field_doc,
                    })
            for attr in ext.findall("xsd:attribute", NS):
                attr_name = attr.get("name", "")
                attr_type = attr.get("type", "")
                attr_ann = attr.find("xsd:annotation", NS)
                attr_doc = extract_annotation_doc(attr_ann) if attr_ann is not None else {}
                attributes.append({
                    "name": attr_name,
                    "type": attr_type,
                    "doc": attr_doc,
                })
    
    return {
        "xsd_name": name,
        "rust_name": rust_name,
        "doc": type_doc,
        "fields": fields,
        "attributes": attributes,
    }


def parse_simple_types(filepath):
    """Parse simpleType definitions (used in CBC for type definitions)."""
    tree = ET.parse(filepath)
    root = tree.getroot()
    
    types = []
    for st in root.findall("xsd:simpleType", NS):
        name = st.get("name", "")
        if not name:
            continue
        rust_name = strip_type_suffix(name)
        ann = st.find("xsd:annotation", NS)
        type_doc = extract_annotation_doc(ann) if ann is not None else {}
        types.append({
            "xsd_name": name,
            "rust_name": rust_name,
            "doc": type_doc,
            "fields": [],
            "attributes": [],
        })
    return types


def parse_xsd_file(filepath):
    """Parse an XSD file for all type definitions."""
    tree = ET.parse(filepath)
    root = tree.getroot()
    
    types = []
    for ct in root.findall("xsd:complexType", NS):
        info = parse_complex_type(ct)
        if info:
            types.append(info)
    
    types.extend(parse_simple_types(filepath))
    return types


def parse_cbc_elements(filepath):
    """Parse CBC XSD for element -> type mappings (since CBC types have no annotations)."""
    tree = ET.parse(filepath)
    root = tree.getroot()
    
    elements = []
    for elem in root.findall("xsd:element", NS):
        elem_name = elem.get("name", "")
        elem_type = elem.get("type", "")
        if elem_name and elem_type:
            rust_type = strip_type_suffix(elem_type)
            elements.append({
                "element_name": elem_name,
                "xsd_type": elem_type,
                "rust_type": rust_type,
            })
    return elements


def format_def(text, max_len=0):
    """Clean and optionally truncate a definition string."""
    if not text:
        return ""
    text = " ".join(text.split())
    if max_len and len(text) > max_len:
        return text[:max_len-3] + "..."
    return text


def load_cbc_rust_aliases():
    """Load the CBC type aliases from the generated Rust source to map names -> base types."""
    cbc_file = COMMON_SRC / "cbc.rs"
    if not cbc_file.exists():
        return {}
    
    aliases = {}
    with open(cbc_file) as f:
        for line in f:
            m = re.match(r'pub type (\w+) = (\w+::\w+);', line.strip())
            if m:
                aliases[m.group(1)] = m.group(2)
    return aliases


def generate_cbc_doc(cbc_elements, aliases, output_dir):
    """Generate CBC documentation (type aliases with descriptions from element names)."""
    filepath = output_dir / "cbc.md"
    lines = []
    lines.append("# CBC — UBL 2.5 Common Basic Components")
    lines.append("")
    lines.append(f"Generated from UBL 2.5 XSD schema. {len(cbc_elements)} basic component types.")
    lines.append("")
    lines.append("CBC types are simple type aliases that restrict CCT/UDT base types.")
    lines.append("They carry semantic meaning through their names rather than structural differences.")
    lines.append("")
    
    # Group by base type
    by_base = OrderedDict()
    for el in cbc_elements:
        rust_name = el["rust_type"]
        base = aliases.get(rust_name, "unknown")
        by_base.setdefault(base, []).append(el)
    
    lines.append("## By Base Type")
    lines.append("")
    for base, items in by_base.items():
        lines.append(f"### `{base}` ({len(items)} types)")
        lines.append("")
        for item in sorted(items, key=lambda x: x["rust_type"]):
            lines.append(f"- `{item['rust_type']}` — element: `{item['element_name']}`")
        lines.append("")
    
    lines.append("## Alphabetical Index")
    lines.append("")
    lines.append("| Rust Type | XSD Type | Element Name |")
    lines.append("|-----------|----------|-------------|")
    for item in sorted(cbc_elements, key=lambda x: x["rust_type"]):
        lines.append(f"| `{item['rust_type']}` | `{item['xsd_type']}` | `{item['element_name']}` |")
    lines.append("")
    
    with open(filepath, "w") as f:
        f.write("\n".join(lines))
    print(f"Wrote {filepath} ({len(cbc_elements)} types)")


def generate_markdown(types_by_crate, output_dir):
    """Generate per-crate markdown documentation."""
    output_dir.mkdir(parents=True, exist_ok=True)
    
    for crate_name, types in types_by_crate.items():
        if not types:
            continue
        
        filepath = output_dir / f"{crate_name}.md"
        lines = []
        
        title_map = {
            "cac": "CAC — UBL 2.5 Common Aggregate Components",
            "maindoc": "MAINDOC — UBL 2.5 Document Types",
            "cct": "CCT — UN/CEFACT Core Component Types",
            "udt": "UDT — UN/CEFACT Unqualified Data Types",
            "ext": "EXT — UBL 2.5 Extension Components",
        }
        title = title_map.get(crate_name, f"{crate_name.upper()} — UBL 2.5 Types")
        lines.append(f"# {title}")
        lines.append("")
        lines.append(f"Generated from OASIS UBL 2.5 XSD annotations. {len(types)} types.")
        lines.append("")
        
        # Table of contents
        lines.append("## Types")
        lines.append("")
        for t in types:
            defn = format_def(t["doc"].get("Definition", ""))
            defn_short = (defn[:80] + "...") if len(defn) > 80 else defn
            lines.append(f"- [{t['rust_name']}](#{t['rust_name'].lower()}) — {defn_short}")
        lines.append("")
        
        for t in types:
            lines.append(f"### {t['rust_name']}")
            lines.append("")
            lines.append(f"**XSD type:** `{t['xsd_name']}`")
            
            doc = t["doc"]
            for key in ["Definition", "DictionaryEntryName", "ObjectClass",
                         "RepresentationTermName", "PrimitiveType",
                         "UniqueID", "CategoryCode", "VersionID",
                         "ComponentType", "DataType"]:
                if doc.get(key):
                    label = key.replace("RepresentationTermName", "Representation Term") \
                                .replace("PrimitiveType", "Primitive Type") \
                                .replace("UniqueID", "Unique ID") \
                                .replace("CategoryCode", "Category") \
                                .replace("VersionID", "Version")
                    lines.append(f"**{label}:** {doc[key]}")
            
            if t["attributes"]:
                lines.append("")
                lines.append("**Attributes:**")
                lines.append("")
                lines.append("| Attribute | Type | Definition |")
                lines.append("|-----------|------|------------|")
                for a in t["attributes"]:
                    adef = format_def(a["doc"].get("Definition", ""))
                    lines.append(f"| `{a['name']}` | `{a['type']}` | {adef} |")
            
            if t["fields"]:
                lines.append("")
                lines.append("**Fields:**")
                lines.append("")
                lines.append("| Field | Type | Card. | Definition |")
                lines.append("|-------|------|-------|------------|")
                for f in t["fields"]:
                    local = f["local"]
                    type_ref = f["ref"] or local
                    card = f"{f['min_occurs']}..{f['max_occurs']}"
                    if f["max_occurs"] == "unbounded":
                        card = f"{f['min_occurs']}..n"
                    
                    fdef = format_def(f["doc"].get("Definition", ""), max_len=80)
                    lines.append(f"| `{local}` | `{type_ref}` | {card} | {fdef} |")
            
            lines.append("")
        
        with open(filepath, "w") as f:
            f.write("\n".join(lines))
        print(f"Wrote {filepath} ({len(types)} types)")


def main():
    common_dir = SPEC_DIR / "common"
    maindoc_dir = SPEC_DIR / "maindoc"
    
    types_by_crate = OrderedDict()
    
    # 1. CAC
    print("Parsing CAC...")
    cac = parse_xsd_file(common_dir / "UBL-CommonAggregateComponents-2.5.xsd")
    types_by_crate["cac"] = cac
    print(f"  {len(cac)} types")
    
    # 2. Maindoc
    print("Parsing maindoc...")
    maindoc = []
    for xf in sorted(maindoc_dir.glob("*.xsd")):
        maindoc.extend(parse_xsd_file(xf))
    types_by_crate["maindoc"] = maindoc
    print(f"  {len(maindoc)} types")
    
    # 3. CCT
    print("Parsing CCT...")
    cct = parse_xsd_file(common_dir / "BDNDR-CCTS_CCT_SchemaModule-1.1.xsd")
    types_by_crate["cct"] = cct
    print(f"  {len(cct)} types")
    
    # 4. UDT
    print("Parsing UDT...")
    udt = parse_xsd_file(common_dir / "BDNDR-UnqualifiedDataTypes-1.1.xsd")
    types_by_crate["udt"] = udt
    print(f"  {len(udt)} types")
    
    # 5. EXT
    print("Parsing EXT...")
    ext = parse_xsd_file(common_dir / "UBL-CommonExtensionComponents-2.5.xsd")
    types_by_crate["ext"] = ext
    print(f"  {len(ext)} types")
    
    # 6. CBC elements (for the index)
    print("Parsing CBC elements...")
    cbc_el = parse_cbc_elements(common_dir / "UBL-CommonBasicComponents-2.5.xsd")
    aliases = load_cbc_rust_aliases()
    print(f"  {len(cbc_el)} elements, {len(aliases)} Rust aliases")
    
    # Generate markdown
    print("\nGenerating docs...")
    generate_markdown(types_by_crate, OUTPUT_DIR)
    generate_cbc_doc(cbc_el, aliases, OUTPUT_DIR)
    
    # Summary
    total = sum(len(v) for v in types_by_crate.values())
    print(f"\nDone. {total} types documented across {len(types_by_crate)} crates.")
    print(f"Plus {len(cbc_el)} CBC basic components indexed.")
    print(f"Output: {OUTPUT_DIR}/")


if __name__ == "__main__":
    main()
