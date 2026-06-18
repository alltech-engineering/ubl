#[derive(Debug, Deserialize, Serialize)]
pub struct SignedAssertionsListType {
    #[serde(default, rename = "SignedAssertion")]
    pub signed_assertion: Vec<Any>,
}
