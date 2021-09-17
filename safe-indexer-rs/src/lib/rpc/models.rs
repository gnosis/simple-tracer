use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct RpcRequest {
    pub jsonrpc: String,
    pub method: String,
    pub id: String,
    pub params: Vec<RequestParam>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RpcResponse<T> {
    pub jsonrpc: String,
    pub id: String,
    pub result: T,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum RequestParam {
    Single(String),
    #[serde(rename_all = "camelCase")]
    Log {
        from_block: Option<BlockNumber>,
        address: String,
        topics: Vec<Vec<String>>,
    },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RpcTransaction {
    pub address: String,
    pub block_hash: String,
    pub block_number: String,
    pub data: String,
    pub log_index: String,
    pub removed: bool,
    pub topics: Vec<String>,
    pub transaction_hash: String,
    pub transaction_index: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum BlockNumber {
    Value(String),
    Earliest,
    Latest,
    Pending,
}

pub enum Topic {
    IncomingEth, ExecutionSuccess, ExecutionFailure
}
