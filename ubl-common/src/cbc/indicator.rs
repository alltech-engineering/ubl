// UBL Indicator types — boolean flags.

use serde::{Deserialize, Serialize};

macro_rules! define_indicator {
    ($name:ident, $doc:expr) => {
        #[doc = $doc]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(pub bool);
        impl $name {
            pub fn new(value: bool) -> Self { Self(value) }
            pub fn is_true(&self) -> bool { self.0 }
        }
        impl From<bool> for $name {
            fn from(b: bool) -> Self { Self(b) }
        }
    };
}

define_indicator!(Indicator, "A generic indicator (true/false).");
define_indicator!(ChargeIndicator, "Whether this is a charge (true) or allowance (false).");
define_indicator!(CopyIndicator, "Whether this is a copy of the original document.");
define_indicator!(BackOrderAllowedIndicator, "Whether back orders are allowed.");
define_indicator!(CatalogueIndicator, "Whether a catalogue is referenced.");
define_indicator!(CompletionIndicator, "Whether something is complete.");
define_indicator!(ConsolidatableIndicator, "Whether goods can be consolidated.");
define_indicator!(ConsolidatedIndicator, "Whether goods are consolidated.");
define_indicator!(ContainerizedIndicator, "Whether goods are containerized.");
define_indicator!(DangerousGoodsApprovedIndicator, "Whether dangerous goods approval exists.");
define_indicator!(HumanFoodIndicator, "Whether the item is for human consumption.");
define_indicator!(HazardousRiskIndicator, "Whether there is a hazardous risk.");
define_indicator!(LivestockIndicator, "Whether livestock is involved.");
define_indicator!(OrderableIndicator, "Whether the item is orderable.");
define_indicator!(PartialDeliveryIndicator, "Whether partial delivery is allowed.");
define_indicator!(ReturnableMaterialIndicator, "Whether material is returnable.");
define_indicator!(TaxIncludedIndicator, "Whether tax is included in the amount.");
define_indicator!(AcceptedIndicator, "Whether something was accepted.");
define_indicator!(AdValoremIndicator, "Ad valorem indicator.");
define_indicator!(BulkCargoIndicator, "Bulk cargo indicator.");
define_indicator!(CabotageIndicator, "Cabotage indicator.");
define_indicator!(DangerousGoodsIndicator, "Whether goods are dangerous.");
define_indicator!(GovernmentAgreementConstraintIndicator, "Government agreement constraint.");
define_indicator!(LegalStatusIndicator, "Legal status indicator.");
define_indicator!(OnCarriageIndicator, "On-carriage indicator.");
define_indicator!(PreCarriageIndicator, "Pre-carriage indicator.");
define_indicator!(PricingUpdateRequestIndicator, "Whether a pricing update was requested.");
define_indicator!(SmallBusinessInclusionIndicator, "Small business inclusion indicator.");
define_indicator!(SoleProprietorshipIndicator, "Sole proprietorship indicator.");
define_indicator!(SpecialSecurityIndicator, "Special security indicator.");
define_indicator!(ThirdPartyPayerIndicator, "Third-party payer indicator.");
define_indicator!(ToOrderIndicator, "To-order indicator.");
define_indicator!(TransitDirectionIndicator, "Transit direction indicator.");
define_indicator!(VariantConstraintIndicator, "Variant constraint indicator.");

