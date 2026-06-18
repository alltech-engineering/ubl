#[derive(Debug, Deserialize, Serialize)]
pub struct SignedSignatureProperties {
    #[serde(default, rename = "@Id")]
    pub id: Option<String>,
    #[serde(default, rename = "SigningTime")]
    pub signing_time: Option<String>,
    #[serde(default, rename = "SigningCertificate")]
    pub signing_certificate: Option<CertIdListType>,
    #[serde(default, rename = "SigningCertificateV2")]
    pub signing_certificate_v2: Option<CertIdListV2Type>,
    #[serde(default, rename = "SignaturePolicyIdentifier")]
    pub signature_policy_identifier: Option<SignaturePolicyIdentifier>,
    #[serde(default, rename = "SignatureProductionPlace")]
    pub signature_production_place: Option<SignatureProductionPlace>,
    #[serde(default, rename = "SignatureProductionPlaceV2")]
    pub signature_production_place_v2: Option<SignatureProductionPlaceV2>,
    #[serde(default, rename = "SignerRole")]
    pub signer_role: Option<SignerRole>,
    #[serde(default, rename = "SignerRoleV2")]
    pub signer_role_v2: Option<SignerRoleV2>,
    #[serde(default, rename = "any73")]
    pub any: Vec<String>,
}
