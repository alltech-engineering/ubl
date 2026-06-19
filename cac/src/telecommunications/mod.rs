use serde::{Deserialize, Serialize};

include!("supply_line.rs");
include!("supply.rs");

#[derive(Debug, Deserialize, Serialize)]
/// A class to describe a telecommunications service (e.g., a telephone call or a video on demand
/// service).
///
/// UBL Dictionary Entry Name: `Telecommunications Service. Details`
///
/// Generated from XSD type `TelecommunicationsServiceType`.
pub struct TelecommunicationsService {
/// A container for extensions foreign to the document.
    #[serde(default, rename = "UBLExtensions")]
    pub ubl_extensions:
        Option<ext::UblExtensions>,
/// An identifier for this telecommunications service.
    #[serde(default, rename = "ID")]
    pub id: Option<cct::Identifier>,
/// In the case of a telephone call, the date of the call.
    #[serde(rename = "CallDate")]
    pub call_date: udt::DateTime,
/// In the case of a telephone call, the time of the call.
    #[serde(rename = "CallTime")]
    pub call_time: udt::DateTime,
/// In the case of a telephone call, the phone number called.
    #[serde(rename = "ServiceNumberCalled")]
    pub service_number_called: cct::Text,
/// The telecommunications category, expressed as text.
    #[serde(default, rename = "TelecommunicationsServiceCategory")]
    pub telecommunications_service_category: Option<cct::Text>,
/// The telecommunications category, expressed as a code.
    #[serde(default, rename = "TelecommunicationsServiceCategoryCode")]
    pub telecommunications_service_category_code: Option<cct::Code>,
/// The title of a movie delivered via this telecommunications service.
    #[serde(default, rename = "MovieTitle")]
    pub movie_title: Option<cct::Text>,
/// Statement of the roaming partner name.
    #[serde(default, rename = "RoamingPartnerName")]
    pub roaming_partner_name: Option<cct::Text>,
/// A pay-per-view delivered via this telecommunications service.
    #[serde(default, rename = "PayPerView")]
    pub pay_per_view: Option<cct::Text>,
/// The number of calls.
    #[serde(default, rename = "Quantity")]
    pub quantity: Option<cct::Quantity>,
/// The telecommunications call described as a text
    #[serde(default, rename = "TelecommunicationsServiceCall")]
    pub telecommunications_service_call: Option<cct::Text>,
/// The telecommunications call described as a code
    #[serde(default, rename = "TelecommunicationsServiceCallCode")]
    pub telecommunications_service_call_code: Option<cct::Code>,
/// The amount to be payed as the base for one call
    #[serde(default, rename = "CallBaseAmount")]
    pub call_base_amount: Option<cct::Amount>,
/// The amount to be payed for the call
    #[serde(default, rename = "CallExtensionAmount")]
    pub call_extension_amount: Option<cct::Amount>,
/// The price for using the telecommunication service
    #[serde(default, rename = "Price")]
    pub price: Option<crate::Price>,
/// The country to which the service is provided. In case of a telephone call it is the country where
/// the receiver is located.
    #[serde(default, rename = "Country")]
    pub country: Option<crate::Country>,
/// A exchanges rates used in the pricing e.g.. when phone calls has crossed border lines.
    #[serde(default, rename = "ExchangeRate")]
    pub exchange_rate: Vec<crate::ExchangeRate>,
/// An allowance or charge that applies to the UtilityStatement as a whole.
    #[serde(default, rename = "AllowanceCharge")]
    pub allowance_charge: Vec<crate::AllowanceCharge>,
/// A total amount of taxes of a particular kind applicable to this telecommunications service.
    #[serde(default, rename = "TaxTotal")]
    pub tax_total: Vec<crate::TaxTotal>,
/// In the case of a telephone call, a duty on this call.
    #[serde(default, rename = "CallDuty")]
    pub call_duty: Vec<crate::Duty>,
/// A duty on a consumption of time.
    #[serde(default, rename = "TimeDuty")]
    pub time_duty: Vec<crate::Duty>,
}
