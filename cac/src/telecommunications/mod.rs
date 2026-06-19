use serde::{Deserialize, Serialize};

include!("supply_line.rs");
include!("supply.rs");

#[derive(Debug, Deserialize, Serialize)]
pub struct TelecommunicationsService {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
    #[serde(rename = "CallDate")]
    pub call_date: udt::DateTime,
    #[serde(rename = "CallTime")]
    pub call_time: udt::DateTime,
    #[serde(rename = "ServiceNumberCalled")]
    pub service_number_called: cct::Text,
    #[serde(default, rename = "TelecommunicationsServiceCategory")]
    pub telecommunications_service_category: Option<cct::Text>,
    #[serde(default, rename = "TelecommunicationsServiceCategoryCode")]
    pub telecommunications_service_category_code: Option<cct::Code>,
    #[serde(default, rename = "MovieTitle")]
    pub movie_title: Option<cct::Text>,
    #[serde(default, rename = "RoamingPartnerName")]
    pub roaming_partner_name: Option<cct::Text>,
    #[serde(default, rename = "PayPerView")]
    pub pay_per_view: Option<cct::Text>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
    #[serde(default, rename = "TelecommunicationsServiceCall")]
    pub telecommunications_service_call: Option<cct::Text>,
    #[serde(default, rename = "TelecommunicationsServiceCallCode")]
    pub telecommunications_service_call_code: Option<cct::Code>,
    #[serde(default, rename = "CallBaseAmount")]
    pub call_base_amount: Option<cct::Amount>,
    #[serde(default, rename = "CallExtensionAmount")]
    pub call_extension_amount: Option<cct::Amount>,
    #[serde(default, rename = "Price")]
    pub price: Option<crate::Price>,
    #[serde(default, rename = "Country")]
    pub country: Option<crate::Country>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Vec<crate::ExchangeRate>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<crate::TaxTotal>,
    #[serde(default, rename = "CallDuty")]
    pub call_duty: Vec<crate::Duty>,
    #[serde(default, rename = "TimeDuty")]
    pub time_duty: Vec<crate::Duty>,
}
