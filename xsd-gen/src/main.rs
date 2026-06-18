//! UBL 2.5 XSD → Rust code generator.
//!
//! Generates ALL types (common + maindoc) from the OASIS UBL 2.5 XSD distribution
//! in a single pass using the `xsd-parser` library, then writes the output to a
//! staging directory (`_generated/`) for post-processing into the `common` and
//! `maindoc` crates.
//!
//! Usage:
//!   cargo run -p xsd-gen
//!
//! After generation, run `./scripts/split.sh` to move modules into the two crates.

use std::fs;
use std::path::{Path, PathBuf};

use anyhow::{Context, Error};
use xsd_parser::config::{
    GeneratorFlags, IdentQuadruple, InterpreterFlags, OptimizerFlags, ParserFlags, Resolver,
    Schema,
};
use xsd_parser::models::meta::{CustomMeta, MetaType};
use xsd_parser::{generate_modules, Config, IdentType};

fn main() -> Result<(), Error> {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .context("No workspace parent directory")?
        .to_path_buf();

    let xsd_dir = workspace.join("spec/cs01-UBL-2.5/xsd");
    let output_dir = workspace.join("_generated");

    // Collect schema entry points:
    // - All maindoc XSDs (they import CAC, CBC, EXT → which transitively import CCT, UDT, QDT)
    // - All common XSDs (signature, datatypes) loaded directly
    // The file resolver handles cross-references via schemaLocation imports.
    let mut schemas = Vec::new();
    collect_xsds(&xsd_dir.join("maindoc"), &mut schemas)?;
    eprintln!("Loaded {} schema entry points", schemas.len());

    let config = Config::default()
        .with_schemas(schemas)
        .with_interpreter_flags(InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT)
        .with_optimizer_flags(OptimizerFlags::all())
        .with_generator_flags(GeneratorFlags::all() | GeneratorFlags::BUILD_IN_ABSOLUTE_PATHS - GeneratorFlags::ANY_TYPE_SUPPORT - GeneratorFlags::MIXED_TYPE_SUPPORT - GeneratorFlags::NILLABLE_TYPE_SUPPORT)
        .with_serde_quick_xml();

    // File resolver so xs:include / xs:import resolves locally
    let mut config = config;
    config.parser.resolver = vec![Resolver::File];
    config.parser.flags = ParserFlags::all();

    // Map base64Binary / hexBinary → String (no base64 feature in published xsd-parser)
    config.interpreter.types = vec![
        (
            IdentQuadruple::from((IdentType::Type, "xs:base64Binary")),
            MetaType::from(CustomMeta::new("String").include_from("std::string::String")),
        ),
        (
            IdentQuadruple::from((IdentType::Type, "xs:hexBinary")),
            MetaType::from(CustomMeta::new("String").include_from("std::string::String")),
        ),
    ];

    eprintln!("Generating...");
    let modules = generate_modules(config)?;

    // Clean staging directory
    let _ = fs::remove_dir_all(&output_dir);
    fs::create_dir_all(&output_dir)?;

    modules.write_to_files(&output_dir)?;

    // List what we got
    list_tree(&output_dir, 0)?;
    eprintln!("\nDone. Output at: {}", output_dir.display());
    Ok(())
}

fn collect_xsds(dir: &Path, schemas: &mut Vec<Schema>) -> Result<(), Error> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            collect_xsds(&path, schemas)?;
        } else if path.extension().map_or(false, |e| e == "xsd") {
            schemas.push(Schema::File(path.canonicalize()?));
        }
    }
    Ok(())
}

fn list_tree(dir: &Path, depth: usize) -> Result<(), Error> {
    let indent = "  ".repeat(depth);
    let entries: Vec<_> = fs::read_dir(dir)?.filter_map(|e| e.ok()).collect();
    for entry in &entries {
        let name = entry.file_name();
        let path = entry.path();
        if path.is_dir() {
            eprintln!("{indent}{}/", name.to_string_lossy());
            if depth < 2 {
                list_tree(&path, depth + 1)?;
            }
        }
    }
    Ok(())
}
