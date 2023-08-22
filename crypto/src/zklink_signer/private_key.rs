use crate::zklink_signer::{EddsaPrivKey, Engine};

pub struct PrivateKey(EddsaPrivKey<Engine>);

impl AsRef<EddsaPrivKey<Engine>> for PrivateKey {
    fn as_ref(&self) -> &EddsaPrivKey<Engine> {
        &self.0
    }
}

impl From<EddsaPrivKey<Engine>> for PrivateKey {
    fn from(value: EddsaPrivKey<Engine>) -> Self {
        Self(value)
    }
}
