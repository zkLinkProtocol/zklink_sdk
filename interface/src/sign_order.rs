#[cfg(feature = "ffi")]
use std::sync::Arc;
use zklink_signers::zklink_signer::error::ZkSignerError;
use zklink_signers::zklink_signer::pk_signer::ZkLinkSigner;
use zklink_types::tx_type::order_matching::Order;

#[cfg(feature = "sync")]
pub fn sign(order: &mut Order, signer: &ZkLinkSigner) -> Result<(), ZkSignerError> {
    let bytes = order.get_bytes();
    order.signature = signer.sign_musig(&bytes)?;
    Ok(())
}

#[cfg(feature = "ffi")]
pub fn signed_order(
    zklink_signer: Arc<ZkLinkSigner>,
    order: Arc<Order>,
) -> Result<Arc<Order>, ZkSignerError> {
    let mut order = (*order).clone();
    order.signature = zklink_signer.sign_musig(&order.get_bytes())?;
    Ok(Arc::new(order))
}
