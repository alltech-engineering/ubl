#[derive(Debug, Deserialize, Serialize)]
/// A character string (i.e. a finite set of characters), generally in the form of words of a language.
///
/// UBL Dictionary Entry Name: `Text. Type`
///
/// Generated from XSD type `TextType`.
pub struct Text {
    #[serde(default, rename = "@languageID")]
    pub language_id: Option<String>,
/// (Deprecated) The identification of the locale of the language.
    #[serde(default, rename = "@languageLocaleID")]
    pub language_locale_id: Option<String>,
    #[serde(default, rename = "$text")]
    pub content: String,
}
