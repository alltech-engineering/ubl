// Address — UBL CAC aggregate (Tier 1 stub)
use crate::cbc::*;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Address {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<Id>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_name: Option<StreetName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_name: Option<CityName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_subentity_name: Option<CountrySubentityName>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country_code: Option<CountryCode>,
}
