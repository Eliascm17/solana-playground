use std::str::FromStr;

use solana_extra_wasm::transaction_status::UiTransactionEncoding;
use solana_sdk::signature::Signature;

use crate::client::SerializableTransaction;
use crate::utils::rpc_config::{serialize_and_encode, RpcSendTransactionConfig};
use crate::{ClientRequest, ClientResponse};

#[derive(Debug, Clone)]
pub struct SendTransactionRequest<'a, T: SerializableTransaction> {
    transaction: &'a T,
    config: Option<RpcSendTransactionConfig>,
}

impl<'a, T: SerializableTransaction> SendTransactionRequest<'a, T> {
    pub fn new(transaction: &'a T) -> Self {
        Self {
            transaction,
            config: None,
        }
    }

    pub fn new_with_config(transaction: &'a T, config: RpcSendTransactionConfig) -> Self {
        Self {
            transaction,
            config: Some(config),
        }
    }
}

impl<'a, T: SerializableTransaction + serde::Serialize> From<SendTransactionRequest<'a, T>>
    for serde_json::Value
{
    fn from(value: SendTransactionRequest<'a, T>) -> Self {
        let encoding = match value.config {
            Some(ref c) => c.encoding.unwrap_or(UiTransactionEncoding::Base64),
            None => UiTransactionEncoding::Base64,
        };

        let serialized_encoded = serialize_and_encode(&value.transaction, encoding).unwrap();

        match value.config {
            Some(config) => serde_json::json!([serialized_encoded, config]),
            None => serde_json::json!([serialized_encoded]),
        }
    }
}

impl<'a, T: SerializableTransaction + serde::Serialize> From<SendTransactionRequest<'a, T>>
    for ClientRequest
{
    fn from(val: SendTransactionRequest<'a, T>) -> Self {
        let mut request = ClientRequest::new("sendTransaction");
        let params: serde_json::Value = val.into();

        request.params(params).clone()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendTransactionResponse(Signature);

impl From<SendTransactionResponse> for Signature {
    fn from(val: SendTransactionResponse) -> Self {
        val.0
    }
}

impl From<ClientResponse> for SendTransactionResponse {
    fn from(response: ClientResponse) -> Self {
        let signature = response.result.as_str().expect("invalid response");
        let signature = Signature::from_str(signature).expect("invalid signature");

        SendTransactionResponse(signature)
    }
}
