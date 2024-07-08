use solana_client::client_error::ClientError;
use solana_pubsub_client::nonblocking::pubsub_client::PubsubClientError;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Error)]
pub enum IndexerError {
    #[error("Websocket client error")]
    WebsocketClientError,
    #[error("Client error")]
    ClientError,
}

impl From<PubsubClientError> for IndexerError {
    fn from(_error: PubsubClientError) -> Self {
        IndexerError::WebsocketClientError
    }
}

impl From<ClientError> for IndexerError {
    fn from(_error: ClientError) -> Self {
        IndexerError::ClientError
    }
}
