use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonRpcResult<T> {
    pub jsonrpc: String,
    pub id: String,
    pub result: Option<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonRpcErrorMessage<T> {
    pub message: String,
    pub code: i64,
    pub data: Option<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct JsonRpcError<T> {
    pub jsonrpc: String,
    pub id: String,
    pub error: JsonRpcErrorMessage<T>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Tag {
    Latest,
    Earliest,
    Pending,
}

impl From<Tag> for String {
    fn from(t: Tag) -> Self {
        String::from(match t {
            Tag::Latest => "latest",
            Tag::Earliest => "earliest",
            Tag::Pending => "pending",
        })
    }
}
