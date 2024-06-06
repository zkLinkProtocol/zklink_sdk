mod auto_deleveraging;
mod contract_matching;
mod funding;
mod liquidation;
pub mod prices;
mod update_global_var;

pub use auto_deleveraging::AutoDeleveraging;
pub use contract_matching::{Contract, ContractMatching};
pub use funding::{Funding, FundingInfo};
pub use liquidation::Liquidation;
pub use prices::{ContractPrice, OraclePrices, SpotPriceInfo};
pub use update_global_var::{Parameter, UpdateGlobalVar};

use super::zklink_tx::ZkLinkTxType;

impl Contract {
    pub const MSG_TYPE: u8 = 0xfe;
}

impl ContractMatching {
    pub const TX_TYPE: u8 = ZkLinkTxType::ContractMatching as u8;
}
impl Liquidation {
    pub const TX_TYPE: u8 = ZkLinkTxType::Liquidation as u8;
}
impl AutoDeleveraging {
    pub const TX_TYPE: u8 = ZkLinkTxType::AutoDeleveraging as u8;
}
impl UpdateGlobalVar {
    pub const TX_TYPE: u8 = ZkLinkTxType::UpdateGlobalVar as u8;
}
impl Funding {
    pub const TX_TYPE: u8 = ZkLinkTxType::Funding as u8;
}
