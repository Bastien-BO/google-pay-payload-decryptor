use crate::payment_method_token::{GooglePublicKeysEndpoints, PaymentsKeySets};


pub trait PublicKeysManager {
    fn new(production: bool) -> Self;
    fn refresh_trusted_signing_key_json(google_trusted_cert_url: &str) -> PaymentsKeySets;
    fn get_trusted_signing_key_json(&self) -> PaymentsKeySets;
}

impl PublicKeysManager for PaymentsKeySets {

    fn new(production: bool) -> Self {
        let endpoints = GooglePublicKeysEndpoints::default();
        let url: String;

        if production{
            url = endpoints.keys_url_production;
        }
        else {
            url = endpoints.keys_url_test;
        }
        let test = Self::refresh_trusted_signing_key_json(&url);

        Self {
            keys: test.keys
        }
    }

    fn refresh_trusted_signing_key_json(google_trusted_cert_url: &str) -> PaymentsKeySets {
        let key_sets: PaymentsKeySets = reqwest::get(google_trusted_cert_url)?.json()?;
        return key_sets;
    }

    fn get_trusted_signing_key_json(&self) -> PaymentsKeySets {
        todo!()
    }
}