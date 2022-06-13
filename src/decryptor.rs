use crate::payment_method_token::PaymentsKeySets;

pub trait PublicKeysManager {
    fn refresh_trusted_signing_key_json(&self) -> PaymentsKeySets;
    fn get_trusted_signing_key_json(&self) -> PaymentsKeySets;
}

impl PublicKeysManager for PaymentsKeySets {
    fn refresh_trusted_signing_key_json(&self) -> PaymentsKeySets {
        todo!()
    }

    fn get_trusted_signing_key_json(&self) -> PaymentsKeySets {
        todo!()
    }
}