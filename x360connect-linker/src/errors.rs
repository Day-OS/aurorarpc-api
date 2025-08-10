use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuroraRpcError {
    #[error("This device does not have local IP")]
    localIpNotFound,

    #[error("RPC Error `{0}`")]
    RPCError(String),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader {
        expected: String,
        found: String,
    },
}