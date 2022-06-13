use derivative::Derivative;
use serde::Deserialize;

pub struct IntermediateSigningKey {
    pub signed_key: String,
    pub signatures: [String; 3],
}

pub struct PaymentMethodToken {
    pub protocol_version: String,
    pub signature: String,
    pub intermediate_signing_key: IntermediateSigningKey,
    pub signed_message: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KeySet {
    pub key_value: String,
    pub protocol_version: String,
    pub key_expiration: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaymentsKeySets {
    pub keys: [KeySet; 3],
}

#[derive(Derivative)]
#[derivative(Default)]
pub struct GooglePublicKeysEndpoints {
    #[derivative(Default(value="https://payments.developers.google.com/paymentmethodtoken/keys.json"))]
    pub keys_url_production: String,
    #[derivative(Default(value="https://payments.developers.google.com/paymentmethodtoken/test/keys.json"))]
    pub keys_url_test: String,
}