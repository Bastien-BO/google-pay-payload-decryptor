use derivative::Derivative;

pub struct IntermediateSigningKey {
    signed_key: str,
    signatures: [str],
}

pub struct PaymentMethodToken {
    protocol_version: str,
    signature: str,
    intermediate_signing_key: IntermediateSigningKey,
    signed_message: str,
}

pub struct KeySet {
    key_value: str,
    protocol_version: str,
    key_expiration: str,
}

pub struct PaymentsKeySets {
    keys: [KeySet; 3]
}

#[derive(Derivative)]
#[derivative(Debug, Default)]
pub struct GooglePublicKeysEndpoints {
    #[derivative(Default(value = "https://payments.developers.google.com/paymentmethodtoken/keys.json"))]
    keys_url_production: str,
    #[derivative(Default(value = "https://payments.developers.google.com/paymentmethodtoken/test/keys.json"))]
    keys_url_test: str,

}