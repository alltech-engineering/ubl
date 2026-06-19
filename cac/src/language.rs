#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a language.
///
/// UBL Dictionary Entry Name: `Language. Details`
///
/// Generated from XSD type `LanguageType`.
pub struct Language {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this language.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// The name of this language.
    #[serde(default, rename = "Name")]
    pub name: Option<cct::Text>,
/// A code signifying the locale in which this language is used.
    #[serde(default, rename = "LocaleCode")]
    pub locale_code: Option<cct::Code>,
}
