#[derive(Debug, Deserialize, Serialize)]
pub struct TelecommunicationsService {
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<super::ext::ubl_common_extension_components_25::UblExtensions>,
    #[serde(default, rename = "ID")]
    pub id: Option<super::cct::IdentifierType>,
    #[serde(rename = "CallDate")]
    pub call_date: super::udt::DateTimeType,
    #[serde(rename = "CallTime")]
    pub call_time: super::udt::DateTimeType,
    #[serde(rename = "ServiceNumberCalled")]
    pub service_number_called: super::cct::TextType,
    #[serde(default, rename = "TelecommunicationsServiceCategory")]
    pub telecommunications_service_category: Option<super::cct::TextType>,
    #[serde(default, rename = "TelecommunicationsServiceCategoryCode")]
    pub telecommunications_service_category_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "MovieTitle")]
    pub movie_title: Option<super::cct::TextType>,
    #[serde(default, rename = "RoamingPartnerName")]
    pub roaming_partner_name: Option<super::cct::TextType>,
    #[serde(default, rename = "PayPerView")]
    pub pay_per_view: Option<super::cct::TextType>,
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<super::cct::QuantityType>,
    #[serde(default, rename = "TelecommunicationsServiceCall")]
    pub telecommunications_service_call: Option<super::cct::TextType>,
    #[serde(default, rename = "TelecommunicationsServiceCallCode")]
    pub telecommunications_service_call_code: Option<super::cct::CodeType>,
    #[serde(default, rename = "CallBaseAmount")]
    pub call_base_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "CallExtensionAmount")]
    pub call_extension_amount: Option<super::cct::AmountType>,
    #[serde(default, rename = "Price")]
    pub price: Option<Price>,
    #[serde(default, rename = "Country")]
    pub country: Option<Country>,
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Vec<ExchangeRate>,
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<AllowanceCharge>,
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<TaxTotal>,
    #[serde(default, rename = "CallDuty")]
    pub call_duty: Vec<Duty>,
    #[serde(default, rename = "TimeDuty")]
    pub time_duty: Vec<Duty>,
}
