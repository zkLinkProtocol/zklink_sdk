use ethers::abi::Contract;
use std::collections::HashMap;
use std::str::FromStr;

const ERC20_JSON: &str = include_str!("abi/IERC20.json");
const ZKLINK_JSON: &str = include_str!("abi/Zklink.json");
const L1_GATEWAY_JSON: &str = include_str!("abi/L1_gateway.json");

pub fn load_abi(content: &str) -> String {
    if let Some(abi_str) = serde_json::Value::from_str(content).unwrap().get("abi") {
        abi_str.to_string()
    } else {
        content.to_string()
    }
}

pub fn load_contracts() -> HashMap<String, Contract> {
    let zklink_contract = Contract::load(load_abi(ZKLINK_JSON).as_bytes()).unwrap();
    let erc20_contract = Contract::load(load_abi(ERC20_JSON).as_bytes()).unwrap();
    let l1_gw_contract = Contract::load(load_abi(L1_GATEWAY_JSON).as_bytes()).unwrap();
    let mut contracts = HashMap::new();
    contracts.insert("zklink".to_owned(), zklink_contract);
    contracts.insert("erc20".to_owned(), erc20_contract);
    contracts.insert("l1_gateway".to_owned(), l1_gw_contract);
    contracts
}
