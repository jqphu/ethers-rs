use ethers_core::types::U256;

use crate::{transports::common::JsonRpcError, ProviderError};

use super::WsError;

#[derive(Debug, thiserror::Error)]
pub enum WsClientError {
    /// Thrown if deserialization failed
    #[error(transparent)]
    JsonError(#[from] serde_json::Error),

    /// Thrown if the response could not be parsed
    #[error(transparent)]
    JsonRpcError(#[from] JsonRpcError),

    /// Internal lib error
    #[error(transparent)]
    InternalError(#[from] WsError),

    /// Remote server sent a Close message
    #[error("Websocket closed unexpectedly")]
    UnexpectedClose,

    /// Unexpected channel closure
    #[error("Unexpected internal channel closure. This is likely a bug. Please report via github")]
    DeadChannel,

    /// Thrown if the websocket responds with binary data
    #[error("Websocket responded with unexpected binary data")]
    UnexpectedBinary(Vec<u8>),

    /// PubSubClient asked to listen to an unknown subscription id
    #[error("Attempted to listen to unknown subscription: {0:?}")]
    UnknownSubscription(U256),

    /// Too Many Reconnects
    #[error("Reconnect limit reached")]
    TooManyReconnects,
}

impl From<WsClientError> for ProviderError {
    fn from(src: WsClientError) -> Self {
        ProviderError::JsonRpcClientError(Box::new(src))
    }
}
