use crate::zklink_signer::{EddsaPrivKey, Engine};

pub struct PackedPrivateKey(EddsaPrivKey<Engine>);

impl AsRef<EddsaPrivKey<Engine>> for PackedPrivateKey {
    fn as_ref(&self) -> &EddsaPrivKey<Engine> {
        &self.0
    }
}

impl From<EddsaPrivKey<Engine>> for PackedPrivateKey {
    fn from(value: EddsaPrivKey<Engine>) -> Self {
        Self(value)
    }
}
