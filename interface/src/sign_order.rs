#[cfg(feature = "ffi")]
use std::sync::Arc;
use zklink_sdk_signers::zklink_signer::error::ZkSignerError;
use zklink_sdk_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_sdk_types::tx_type::order_matching::Order;
use zklink_sdk_types::tx_type::TxTrait;

#[cfg(not(feature = "ffi"))]
pub fn create_signed_order(
    zklink_signer: &ZkLinkSigner,
    order: &Order,
) -> Result<Order, ZkSignerError> {
    let mut order = order.clone();
    order.signature = zklink_signer.sign_musig(&order.get_bytes())?;
    Ok(order)
}

#[cfg(feature = "ffi")]
pub fn create_signed_order(
    zklink_signer: Arc<ZkLinkSigner>,
    order: Arc<Order>,
) -> Result<Arc<Order>, ZkSignerError> {
    let mut order = (*order).clone();
    order.signature = zklink_signer.sign_musig(&order.get_bytes())?;
    Ok(Arc::new(order))
}
