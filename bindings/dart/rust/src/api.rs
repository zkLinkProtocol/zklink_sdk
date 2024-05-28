use anyhow::Result;
use flutter_rust_bridge::frb;
use serde_json::to_string;
use std::str::FromStr;
use std::time::UNIX_EPOCH;
use zklink_sdk_interface::signer::{L1SignerType, Signer as InnerSigner};
use zklink_sdk_signers::eth_signer::{PackedEthSignature, H256};
use zklink_sdk_signers::starknet_signer::StarkEcdsaSignature;
use zklink_sdk_signers::zklink_signer::{
    signature::ZkLinkSignature as InnerZkLinkSignature, PubKeyHash,
    ZkLinkSigner as InnerZkLinkSigner,
};
use zklink_sdk_types::basic_types::{BigUint, GetBytes, ZkLinkAddress};
use zklink_sdk_types::tx_builder::*;
use zklink_sdk_types::tx_type::change_pubkey::{
    ChangePubKey as InnerChangePubKey, ChangePubKeyAuthData, Create2Data,
};
use zklink_sdk_types::tx_type::contract::{
    AutoDeleveraging as InnerAutoDeleveraging, Contract as InnerContract,
    ContractMatching as InnerContractMatching, ContractPrice as InnerContractPrice,
    Funding as InnerFunding, FundingInfo as InnerFundingInfo, Liquidation as InnerLiquidation,
    Parameter as InnerParameter, SpotPriceInfo as InnerSpotPriceInfo,
    UpdateGlobalVar as InnerUpdateGlobalVar,
};
use zklink_sdk_types::tx_type::forced_exit::ForcedExit as InnerForcedExit;
use zklink_sdk_types::tx_type::order_matching::{
    Order as InnerOrder, OrderMatching as InnerOrderMatching,
};
use zklink_sdk_types::tx_type::transfer::Transfer as InnerTransfer;
use zklink_sdk_types::tx_type::withdraw::Withdraw as InnerWithdraw;
use zklink_sdk_wallet::eth::EthTxOption as InnerEthTxOption;
use zklink_sdk_wallet::wallet::Wallet as InnerWallet;

macro_rules! tx_default {
    () => {
        #[frb(sync)]
        pub fn sign(&mut self, zk_link_signer: ZkLinkSigner) -> Result<()> {
            self.inner.signature = zk_link_signer.inner.sign_musig(&self.inner.get_bytes())?;
            Ok(())
        }

        #[frb(sync)]
        pub fn to_json(&self) -> Result<String> {
            Ok(to_string(&self.inner)?)
        }
    };
}

#[frb(opaque)]
pub struct ZkLinkSignature {
    pub inner: InnerZkLinkSignature,
}

impl ZkLinkSignature {
    #[frb(sync)]
    pub fn get_pubkey(&self) -> String {
        self.inner.pub_key.as_hex()
    }

    #[frb(sync)]
    pub fn get_signature(&self) -> String {
        self.inner.signature.as_hex()
    }
}

#[frb(opaque)]
pub struct ZkLinkSigner {
    pub inner: InnerZkLinkSigner,
}

impl ZkLinkSigner {
    #[frb(sync)]
    pub fn eth_sig(sig: String) -> Result<Self> {
        let signature = PackedEthSignature::from_hex(&sig)?;
        let seed = signature.serialize_packed();
        Ok(Self {
            inner: InnerZkLinkSigner::new_from_seed(&seed)?,
        })
    }

    #[frb(sync)]
    pub fn starknet_sig(sig: String) -> Result<Self> {
        let signature = StarkEcdsaSignature::from_hex(&sig)?;
        let seed = signature.to_bytes_be();
        Ok(Self {
            inner: InnerZkLinkSigner::new_from_seed(&seed)?,
        })
    }

    #[frb(sync)]
    pub fn get_pubkey(&self) -> String {
        self.inner.public_key().as_hex()
    }

    #[frb(sync)]
    pub fn get_pubkey_hash(&self) -> String {
        self.inner.public_key().public_key_hash().as_hex()
    }

    #[frb(sync)]
    pub fn sign_musig(&self, msg: Vec<u8>) -> Result<ZkLinkSignature> {
        Ok(ZkLinkSignature {
            inner: self.inner.sign_musig(&msg)?,
        })
    }
}

#[frb(opaque)]
pub struct Signer {
    pub inner: InnerSigner,
}

impl Signer {
    #[frb(sync)]
    pub fn eth_signer(eth_private_key: String) -> Result<Self> {
        Ok(Self {
            inner: InnerSigner::new(&eth_private_key, L1SignerType::Eth)?,
        })
    }

    #[frb(sync)]
    pub fn starknet_signer(
        eth_private_key: String,
        starknet_chain_id: String,
        starknet_addr: String,
    ) -> Result<Self> {
        let signer_type = L1SignerType::Starknet {
            chain_id: starknet_chain_id,
            address: starknet_addr,
        };
        Ok(Self {
            inner: InnerSigner::new(&eth_private_key, signer_type)?,
        })
    }

    #[frb(sync)]
    pub fn sign_change_pubkey_with_onchain(&self, tx: ChangePubKey) -> Result<String> {
        let sig = self
            .inner
            .sign_change_pubkey_with_onchain_auth_data(tx.inner)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn sign_change_pubkey_with_eth_ecdsa_auth(&self, tx: ChangePubKey) -> Result<String> {
        let sig = self
            .inner
            .sign_change_pubkey_with_eth_ecdsa_auth(tx.inner)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn sign_change_pubkey_with_create2data_auth(
        &self,
        tx: ChangePubKey,
        creator_address: String,
        salt_arg: String,
        code_hash: String,
    ) -> Result<String> {
        let create2_data = Create2Data {
            creator_address: ZkLinkAddress::from_hex(&creator_address)?,
            code_hash: H256::from_str(&code_hash)?,
            salt_arg: H256::from_str(&salt_arg)?,
        };
        let sig = self
            .inner
            .sign_change_pubkey_with_create2data_auth(tx.inner, create2_data)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn sign_transfer(
        &self,
        tx: Transfer,
        token_symbol: String,
        chain_id: Option<String>,
        addr: Option<String>,
    ) -> Result<String> {
        let sig = self
            .inner
            .sign_transfer(tx.inner, &token_symbol, chain_id, addr)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn sign_withdraw(
        &self,
        tx: Withdraw,
        token_symbol: String,
        chain_id: Option<String>,
        addr: Option<String>,
    ) -> Result<String> {
        let sig = self
            .inner
            .sign_withdraw(tx.inner, &token_symbol, chain_id, addr)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn sign_forced_exit(&self, tx: ForcedExit) -> Result<String> {
        let sig = self.inner.sign_forced_exit(tx.inner)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn create_signed_order(&self, order: Order) -> Result<Order> {
        Ok(Order {
            inner: self.inner.create_signed_order(&order.inner)?,
        })
    }

    #[frb(sync)]
    pub fn sign_order_matching(&self, tx: OrderMatching) -> Result<String> {
        let sig = self.inner.sign_order_matching(tx.inner)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn create_signed_contract(&self, contract: Contract) -> Result<Contract> {
        Ok(Contract {
            inner: self.inner.create_signed_contract(&contract.inner)?,
        })
    }

    #[frb(sync)]
    pub fn sign_contract_matching(&self, tx: ContractMatching) -> Result<String> {
        let sig = self.inner.sign_contract_matching(tx.inner)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn sign_auto_deleveraging(&self, tx: AutoDeleveraging) -> Result<String> {
        let sig = self.inner.sign_auto_deleveraging(tx.inner)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn sign_funding(&self, tx: Funding) -> Result<String> {
        let sig = self.inner.sign_funding(tx.inner)?;
        Ok(to_string(&sig.tx)?)
    }

    #[frb(sync)]
    pub fn sign_liquidation(&self, tx: Liquidation) -> Result<String> {
        let sig = self.inner.sign_liquidation(tx.inner)?;
        Ok(to_string(&sig.tx)?)
    }
}

#[frb(opaque)]
pub struct ChangePubKey {
    pub inner: InnerChangePubKey,
}

impl ChangePubKey {
    #[frb(sync)]
    pub fn new(
        chain_id: u8,
        account_id: u32,
        sub_account_id: u8,
        new_pubkey_hash: String,
        fee_token: u32,
        fee: String,
        nonce: u32,
        eth_signature: Option<String>,
        ts: Option<u32>,
    ) -> Result<Self> {
        let ts = if let Some(time_stamp) = ts {
            time_stamp
        } else {
            UNIX_EPOCH.elapsed().unwrap().as_secs() as u32
        };
        let eth_signature = if let Some(s) = eth_signature {
            Some(PackedEthSignature::from_hex(&s)?)
        } else {
            None
        };
        Ok(Self {
            inner: ChangePubKeyBuilder {
                chain_id: chain_id.into(),
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                new_pubkey_hash: PubKeyHash::from_hex(&new_pubkey_hash)?,
                fee_token: fee_token.into(),
                fee: BigUint::from_str(&fee)?,
                nonce: nonce.into(),
                eth_signature,
                timestamp: ts.into(),
            }
            .build(),
        })
    }

    #[frb(sync)]
    pub fn to_eip712_request_payload(&self, chain_id: u32, address: String) -> Result<String> {
        let eth_data = self
            .inner
            .to_eip712_request_payload(chain_id, &ZkLinkAddress::from_hex(&address)?)?;
        Ok(to_string(&eth_data)?)
    }

    #[frb(sync)]
    pub fn get_eth_sign_msg(&self, nonce: u32, account_id: u32) -> String {
        format!(
            "ChangePubKey\nPubKeyHash: {}\nNonce: {}\nAccountId: {}",
            self.inner.new_pk_hash.as_hex(),
            nonce,
            account_id
        )
    }

    #[frb(sync)]
    pub fn set_eth_auth_data(&mut self, sig: String) -> Result<()> {
        let eth_signature = PackedEthSignature::from_hex(&sig)?;
        let eth_authdata = ChangePubKeyAuthData::EthECDSA { eth_signature };
        self.inner.eth_auth_data = eth_authdata;
        Ok(())
    }

    tx_default!();
}

#[frb(opaque)]
pub struct Transfer {
    pub inner: InnerTransfer,
}

impl Transfer {
    #[frb(sync)]
    pub fn new(
        account_id: u32,
        to_address: String,
        from_sub_account_id: u8,
        to_sub_account_id: u8,
        token: u32,
        fee: String,
        amount: String,
        nonce: u32,
        ts: Option<u32>,
    ) -> Result<Self> {
        let ts = if let Some(time_stamp) = ts {
            time_stamp
        } else {
            UNIX_EPOCH.elapsed().unwrap().as_secs() as u32
        };
        Ok(Self {
            inner: TransferBuilder {
                account_id: account_id.into(),
                to_address: ZkLinkAddress::from_hex(&to_address)?,
                from_sub_account_id: from_sub_account_id.into(),
                to_sub_account_id: to_sub_account_id.into(),
                token: token.into(),
                fee: BigUint::from_str(&fee)?,
                nonce: nonce.into(),
                timestamp: ts.into(),
                amount: BigUint::from_str(&amount)?,
            }
            .build(),
        })
    }

    #[frb(sync)]
    pub fn get_eth_sign_msg(&self, token_symbol: String) -> String {
        self.inner.get_eth_sign_msg(&token_symbol)
    }

    tx_default!();
}

#[frb(opaque)]
pub struct Withdraw {
    pub inner: InnerWithdraw,
}

impl Withdraw {
    #[frb(sync)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        to_chain_id: u8,
        to_address: String,
        l2_source_token: u32,
        l1_target_token: u32,
        amount: String,
        call_data: Option<String>,
        fee: String,
        nonce: u32,
        withdraw_to_l1: bool,
        withdraw_fee_ratio: u16,
        ts: Option<u32>,
    ) -> Result<Self> {
        let ts = if let Some(time_stamp) = ts {
            time_stamp
        } else {
            UNIX_EPOCH.elapsed().unwrap().as_secs() as u32
        };
        let call_data = if let Some(call_data) = call_data {
            Some(hex::decode(call_data.trim_start_matches("0x"))?)
        } else {
            None
        };
        Ok(Self {
            inner: WithdrawBuilder {
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                to_chain_id: to_chain_id.into(),
                to_address: ZkLinkAddress::from_hex(&to_address)?,
                l2_source_token: l2_source_token.into(),
                l1_target_token: l1_target_token.into(),
                amount: BigUint::from_str(&amount)?,
                call_data,
                fee: BigUint::from_str(&fee)?,
                nonce: nonce.into(),
                withdraw_to_l1,
                withdraw_fee_ratio,
                timestamp: ts.into(),
            }
            .build(),
        })
    }

    #[frb(sync)]
    pub fn get_eth_sign_msg(&self, token_symbol: String) -> String {
        self.inner.get_eth_sign_msg(&token_symbol)
    }

    tx_default!();
}

#[frb(opaque)]
pub struct ForcedExit {
    pub inner: InnerForcedExit,
}

impl ForcedExit {
    #[frb(sync)]
    pub fn new(
        to_chain_id: u8,
        initiator_account_id: u32,
        initiator_sub_account_id: u8,
        target_sub_account_id: u8,
        target: String,
        l2_source_token: u32,
        l1_target_token: u32,
        exit_amount: String,
        initiator_nonce: u32,
        withdraw_to_l1: bool,
        ts: Option<u32>,
    ) -> Result<Self> {
        let ts = if let Some(time_stamp) = ts {
            time_stamp
        } else {
            UNIX_EPOCH.elapsed().unwrap().as_secs() as u32
        };
        Ok(Self {
            inner: ForcedExitBuilder {
                to_chain_id: to_chain_id.into(),
                initiator_account_id: initiator_account_id.into(),
                initiator_sub_account_id: initiator_sub_account_id.into(),
                target: ZkLinkAddress::from_hex(&target)?,
                l2_source_token: l2_source_token.into(),
                timestamp: ts.into(),
                l1_target_token: l1_target_token.into(),
                initiator_nonce: initiator_nonce.into(),
                target_sub_account_id: target_sub_account_id.into(),
                withdraw_to_l1,
                exit_amount: BigUint::from_str(&exit_amount)?,
            }
            .build(),
        })
    }

    tx_default!();
}

#[frb(opaque)]
pub struct ContractPrice {
    pub inner: InnerContractPrice,
}

impl ContractPrice {
    #[frb(sync)]
    pub fn new(pair_id: u16, market_price: String) -> Result<Self> {
        Ok(Self {
            inner: InnerContractPrice {
                pair_id: pair_id.into(),
                market_price: BigUint::from_str(&market_price)?,
            },
        })
    }
}

#[frb(opaque)]
pub struct SpotPriceInfo {
    pub inner: InnerSpotPriceInfo,
}

impl SpotPriceInfo {
    #[frb(sync)]
    pub fn new(token_id: u32, price: String) -> Result<Self> {
        Ok(Self {
            inner: InnerSpotPriceInfo {
                token_id: token_id.into(),
                price: BigUint::from_str(&price)?,
            },
        })
    }
}

#[frb(opaque)]
pub struct Order {
    pub inner: InnerOrder,
}

impl Order {
    #[frb(sync)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        slot_id: u32,
        nonce: u32,
        base_token_id: u32,
        quote_token_id: u32,
        amount: String,
        price: String,
        is_sell: bool,
        maker_fee_rate: u8,
        taker_fee_rate: u8,
        has_subsidy: bool,
    ) -> Result<Self> {
        Ok(Self {
            inner: InnerOrder {
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                slot_id: slot_id.into(),
                nonce: nonce.into(),
                base_token_id: base_token_id.into(),
                quote_token_id: quote_token_id.into(),
                amount: BigUint::from_str(&amount)?,
                price: BigUint::from_str(&price)?,
                is_sell: is_sell as u8,
                fee_rates: [maker_fee_rate, taker_fee_rate],
                has_subsidy: has_subsidy as u8,
                signature: Default::default(),
            },
        })
    }

    tx_default!();
}

#[frb(opaque)]
pub struct OrderMatching {
    pub inner: InnerOrderMatching,
}

impl OrderMatching {
    #[frb(sync)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        taker: Order,
        maker: Order,
        fee: String,
        fee_token: u32,
        contract_prices: Vec<ContractPrice>,
        margin_prices: Vec<SpotPriceInfo>,
        expect_base_amount: String,
        expect_quote_amount: String,
    ) -> Result<Self> {
        let contract_prices = contract_prices.iter().map(|e| e.inner.clone()).collect();
        let margin_prices = margin_prices.iter().map(|e| e.inner.clone()).collect();
        Ok(Self {
            inner: OrderMatchingBuilder {
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                taker: taker.inner,
                fee: BigUint::from_str(&fee)?,
                fee_token: fee_token.into(),
                expect_base_amount: BigUint::from_str(&expect_base_amount)?,
                maker: maker.inner,
                expect_quote_amount: BigUint::from_str(&expect_quote_amount)?,
                contract_prices,
                margin_prices,
            }
            .build(),
        })
    }

    tx_default!();
}

#[frb(opaque)]
pub struct Contract {
    pub inner: InnerContract,
}

impl Contract {
    #[frb(sync)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        slot_id: u32,
        nonce: u32,
        pair_id: u16,
        size: String,
        price: String,
        direction: bool,
        maker_fee_rate: u8,
        taker_fee_rate: u8,
        has_subsidy: bool,
    ) -> Result<Self> {
        Ok(Self {
            inner: ContractBuilder {
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                slot_id: slot_id.into(),
                nonce: nonce.into(),
                pair_id: pair_id.into(),
                size: BigUint::from_str(&size)?,
                price: BigUint::from_str(&price)?,
                direction,
                maker_fee_rate,
                taker_fee_rate,
                has_subsidy,
            }
            .build(),
        })
    }

    tx_default!();
}

#[frb(opaque)]
pub struct ContractMatching {
    pub inner: InnerContractMatching,
}

impl ContractMatching {
    #[frb(sync)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        taker: Contract,
        maker: Vec<Contract>,
        fee: String,
        fee_token: u16,
        contract_prices: Vec<ContractPrice>,
        margin_prices: Vec<SpotPriceInfo>,
    ) -> Result<Self> {
        let maker = maker.iter().map(|e| e.inner.clone()).collect();
        let contract_prices = contract_prices.iter().map(|e| e.inner.clone()).collect();
        let margin_prices = margin_prices.iter().map(|e| e.inner.clone()).collect();
        Ok(Self {
            inner: ContractMatchingBuilder {
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                taker: taker.inner,
                maker,
                fee: BigUint::from_str(&fee)?,
                fee_token: fee_token.into(),
                contract_prices,
                margin_prices,
            }
            .build(),
        })
    }

    tx_default!();
}

#[frb(opaque)]
pub struct AutoDeleveraging {
    pub inner: InnerAutoDeleveraging,
}

impl AutoDeleveraging {
    #[frb(sync)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        sub_account_nonce: u32,
        contract_prices: Vec<ContractPrice>,
        margin_prices: Vec<SpotPriceInfo>,
        adl_account_id: u32,
        pair_id: u16,
        adl_size: String,
        adl_price: String,
        fee: String,
        fee_token: u16,
    ) -> Result<Self> {
        let contract_prices = contract_prices.iter().map(|e| e.inner.clone()).collect();
        let margin_prices = margin_prices.iter().map(|e| e.inner.clone()).collect();
        Ok(Self {
            inner: AutoDeleveragingBuilder {
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                sub_account_nonce: sub_account_nonce.into(),
                contract_prices,
                margin_prices,
                adl_account_id: adl_account_id.into(),
                pair_id: pair_id.into(),
                adl_size: BigUint::from_str(&adl_size)?,
                adl_price: BigUint::from_str(&adl_price)?,
                fee: BigUint::from_str(&fee)?,
                fee_token: fee_token.into(),
            }
            .build(),
        })
    }

    tx_default!();
}

#[frb(opaque)]
pub struct Funding {
    pub inner: InnerFunding,
}

impl Funding {
    #[frb(sync)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        sub_account_nonce: u32,
        funding_account_ids: Vec<u32>,
        fee: String,
        fee_token: u16,
    ) -> Result<Self> {
        let funding_account_ids = funding_account_ids
            .iter()
            .map(|id| (*id).into())
            .collect::<Vec<_>>();
        Ok(Self {
            inner: FundingBuilder {
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                sub_account_nonce: sub_account_nonce.into(),
                fee: BigUint::from_str(&fee)?,
                fee_token: fee_token.into(),
                funding_account_ids,
            }
            .build(),
        })
    }

    tx_default!();
}

#[frb(opaque)]
pub struct Liquidation {
    pub inner: InnerLiquidation,
}

impl Liquidation {
    #[frb(sync)]
    pub fn new(
        account_id: u32,
        sub_account_id: u8,
        sub_account_nonce: u32,
        contract_prices: Vec<ContractPrice>,
        margin_prices: Vec<SpotPriceInfo>,
        liquidation_account_id: u32,
        fee: String,
        fee_token: u16,
    ) -> Result<Self> {
        let contract_prices = contract_prices.iter().map(|e| e.inner.clone()).collect();
        let margin_prices = margin_prices.iter().map(|e| e.inner.clone()).collect();
        Ok(Self {
            inner: LiquidationBuilder {
                account_id: account_id.into(),
                sub_account_id: sub_account_id.into(),
                sub_account_nonce: sub_account_nonce.into(),
                contract_prices,
                margin_prices,
                liquidation_account_id: liquidation_account_id.into(),
                fee: BigUint::from_str(&fee)?,
                fee_token: fee_token.into(),
            }
            .build(),
        })
    }

    tx_default!();
}

#[frb(opaque)]
pub struct FundingInfo {
    pub inner: InnerFundingInfo,
}

impl FundingInfo {
    #[frb(sync)]
    pub fn new(pair_id: u16, price: String, funding_rate: i16) -> Result<Self> {
        Ok(Self {
            inner: InnerFundingInfo {
                pair_id: pair_id.into(),
                price: BigUint::from_str(&price)?,
                funding_rate,
            },
        })
    }
}

#[frb(opaque)]
pub struct Parameter {
    pub inner: InnerParameter,
}

impl Parameter {
    #[frb(sync)]
    pub fn fee_account(account_id: u32) -> Result<Self> {
        Ok(Self {
            inner: InnerParameter::FeeAccount {
                account_id: account_id.into(),
            },
        })
    }

    #[frb(sync)]
    pub fn insurance_fund_account(account_id: u32) -> Result<Self> {
        Ok(Self {
            inner: InnerParameter::InsuranceFundAccount {
                account_id: account_id.into(),
            },
        })
    }

    #[allow(unused_variables)]
    #[frb(sync)]
    pub fn margin_info(
        margin_id: u8,
        symbol: Option<String>,
        token_id: u32,
        ratio: u8,
    ) -> Result<Self> {
        Ok(Self {
            inner: InnerParameter::MarginInfo {
                margin_id: margin_id.into(),
                token_id: token_id.into(),
                ratio,
            },
        })
    }

    #[frb(sync)]
    pub fn funding_infos(infos: Vec<FundingInfo>) -> Result<Self> {
        let infos = infos.iter().map(|e| e.inner.clone()).collect();
        Ok(Self {
            inner: InnerParameter::FundingInfos { infos },
        })
    }

    #[frb(sync)]
    pub fn contract_info(
        pair_id: u16,
        symbol: String,
        initial_margin_rate: u16,
        maintenance_margin_rate: u16,
    ) -> Result<Self> {
        Ok(Self {
            inner: InnerParameter::ContractInfo {
                pair_id: pair_id.into(),
                symbol,
                initial_margin_rate,
                maintenance_margin_rate,
            },
        })
    }
}

#[frb(opaque)]
pub struct UpdateGlobalVar {
    pub inner: InnerUpdateGlobalVar,
}

impl UpdateGlobalVar {
    #[frb(sync)]
    pub fn new(
        from_chain_id: u8,
        sub_account_id: u8,
        parameter: Parameter,
        serial_id: f64,
    ) -> Result<Self> {
        Ok(Self {
            inner: UpdateGlobalVarBuilder {
                from_chain_id: from_chain_id.into(),
                sub_account_id: sub_account_id.into(),
                parameter: parameter.inner,
                serial_id: serial_id as u64,
            }
            .build(),
        })
    }

    #[frb(sync)]
    pub fn to_json(&self) -> Result<String> {
        Ok(to_string(&self.inner)?)
    }
}

#[frb(opaque)]
pub struct EthTxOption {
    pub inner: InnerEthTxOption,
}

impl EthTxOption {
    #[frb(sync)]
    pub fn new(
        is_support_eip1559: bool,
        to: String,
        nonce: Option<f64>,
        value: Option<String>,
        gas: Option<f64>,
        gas_price: Option<String>,
    ) -> Result<Self> {
        let value = if let Some(v) = value {
            Some(BigUint::from_str(&v)?)
        } else {
            None
        };
        let gas_price = if let Some(g) = gas_price {
            Some(BigUint::from_str(&g)?)
        } else {
            None
        };
        Ok(Self {
            inner: InnerEthTxOption {
                is_support_eip1559,
                to: ZkLinkAddress::from_hex(&to)?,
                nonce: nonce.map(|n| n as u64),
                value,
                gas: gas.map(|g| g as u64),
                gas_price,
            },
        })
    }
}

#[frb(opaque)]
pub struct Wallet {
    pub inner: InnerWallet,
}

impl Wallet {
    #[frb(sync)]
    pub fn new(url: String, private_key: String) -> Result<Self> {
        Ok(Self {
            inner: InnerWallet::new(&url, &private_key),
        })
    }

    pub async fn get_balance(&self) -> Result<String> {
        let balance = self.inner.get_balance().await?;
        Ok(balance.to_string())
    }

    pub async fn get_nonce(&self, block_number: String) -> Result<f64> {
        let nonce = self.inner.get_nonce(block_number).await?;
        Ok(nonce.as_u64() as f64)
    }

    pub async fn get_deposit_fee(&self, eth_params: EthTxOption) -> Result<String> {
        let fee = self.inner.get_fee(eth_params.inner).await?;
        Ok(fee.to_string())
    }

    pub async fn wait_for_transaction(&self, tx_hash: String, timeout: Option<u32>) -> Result<u8> {
        let tx_hash = H256::from_str(&tx_hash)?;
        let status = self.inner.wait_for_transaction(tx_hash, timeout).await?;
        Ok(status as u8)
    }

    pub async fn approve_erc20(
        &self,
        contract: String,
        amount: String,
        eth_params: EthTxOption,
    ) -> Result<String> {
        let contract = ZkLinkAddress::from_hex(&contract)?;
        let amount = BigUint::from_str(&amount)?;
        let tx_hash = self
            .inner
            .approve_erc20(contract, amount, eth_params.inner)
            .await?;
        Ok(hex::encode(tx_hash.as_bytes()))
    }

    pub async fn deposit_erc20(
        &self,
        sub_account_id: u8,
        deposit_to: String,
        token_addr: String,
        amount: String,
        mapping: bool,
        eth_params: EthTxOption,
        is_gateway: bool,
    ) -> Result<String> {
        let deposit_to = ZkLinkAddress::from_hex(&deposit_to)?;
        let token_addr = ZkLinkAddress::from_hex(&token_addr)?;
        let amount = BigUint::from_str(&amount)?;
        let tx_hash = if !is_gateway {
            self.inner
                .deposit_erc20_to_layer1(
                    sub_account_id,
                    deposit_to,
                    token_addr,
                    amount,
                    mapping,
                    eth_params.inner,
                )
                .await?
        } else {
            self.inner
                .deposit_erc20_to_gateway(
                    sub_account_id,
                    deposit_to,
                    token_addr,
                    amount,
                    mapping,
                    eth_params.inner,
                )
                .await?
        };
        Ok(hex::encode(tx_hash.as_bytes()))
    }

    pub async fn deposit_eth(
        &self,
        sub_account_id: u8,
        deposit_to: String,
        eth_params: EthTxOption,
        is_gateway: bool,
    ) -> Result<String> {
        let deposit_to = ZkLinkAddress::from_hex(&deposit_to)?;
        let tx_hash = if !is_gateway {
            self.inner
                .deposit_eth_to_layer1(sub_account_id, deposit_to, eth_params.inner)
                .await?
        } else {
            self.inner
                .deposit_eth_to_gateway(sub_account_id, deposit_to, eth_params.inner)
                .await?
        };
        Ok(hex::encode(tx_hash.as_bytes()))
    }

    pub async fn set_auth_pubkey_hash(
        &self,
        nonce: f64,
        new_pubkey_hash: String,
        eth_params: EthTxOption,
    ) -> Result<String> {
        let new_pubkey_hash = PubKeyHash::from_hex(&new_pubkey_hash)?;
        let tx_hash = self
            .inner
            .set_auth_pubkey_hash(nonce as u64, new_pubkey_hash, eth_params.inner)
            .await?;
        Ok(hex::encode(tx_hash.as_bytes()))
    }

    pub async fn full_exit(
        &self,
        account_id: u32,
        sub_account_id: u8,
        token_id: u16,
        mapping: bool,
        eth_params: EthTxOption,
    ) -> Result<String> {
        let tx_hash = self
            .inner
            .full_exit(
                account_id,
                sub_account_id,
                token_id,
                mapping,
                eth_params.inner,
            )
            .await?;
        Ok(hex::encode(tx_hash.as_bytes()))
    }
}

#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}
