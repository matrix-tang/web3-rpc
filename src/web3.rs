use crate::client::Client;
use crate::model::{JsonRpcResult, Tag};
use serde_json::json;

#[derive(Clone)]
pub struct Web3 {
    client: Client,
}

impl Web3 {
    pub fn new(url: String) -> Self {
        Web3 {
            client: Client::new(url),
        }
    }

    // web3
    pub async fn web3_client_version(&self) -> anyhow::Result<JsonRpcResult<String>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "net_version", "params": [], "id": "101" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn web3_sha3(&self, sha3: &str) -> anyhow::Result<JsonRpcResult<String>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "web3_sha3", "params": [sha3], "id": "102" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    // net
    pub async fn net_version(&self) -> anyhow::Result<JsonRpcResult<String>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "net_version", "params": [], "id": "301" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn net_listening(&self) -> anyhow::Result<JsonRpcResult<bool>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "net_listening", "params": [], "id": "302" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<bool> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn net_peer_count(&self) -> anyhow::Result<JsonRpcResult<i64>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "net_peerCount", "params": [], "id": "303" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<i64> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    // eth
    pub async fn eth_protocol_version(&self) -> anyhow::Result<JsonRpcResult<String>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_protocolVersion", "params": [], "id": "304" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_syncing(&self) -> anyhow::Result<JsonRpcResult<bool>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_syncing", "params": [], "id": "305" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<bool> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_coinbase(&self) -> anyhow::Result<JsonRpcResult<String>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_coinbase", "params": [], "id": "306" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_mining(&self) -> anyhow::Result<JsonRpcResult<bool>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_mining", "params": [], "id": "307" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<bool> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_hashrate(&self) -> anyhow::Result<JsonRpcResult<String>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_hashrate", "params": [], "id": "308" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_gas_price(&self) -> anyhow::Result<JsonRpcResult<String>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_gasPrice", "params": [], "id": "309" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_accounts(&self) -> anyhow::Result<JsonRpcResult<Vec<String>>> {
        let payload =
            json!({ "jsonrpc": "2.0", "method": "eth_accounts", "params": [], "id": "310" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<Vec<String>> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_balance(
        &self,
        address: &str,
        tag: Option<Tag>,
    ) -> anyhow::Result<JsonRpcResult<String>> {
        let mut t = String::from(Tag::Latest);
        if let Some(tag) = tag {
            t = String::from(tag);
        }
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getBalance", "params": [address, t], "id": "311" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_storage_at(
        &self,
        data: &str,
        quantity: &str,
        tag: Option<Tag>,
    ) -> anyhow::Result<JsonRpcResult<String>> {
        let mut t = String::from(Tag::Latest);
        if let Some(tag) = tag {
            t = String::from(tag);
        }
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getStorageAt", "params": [data, quantity, t], "id": "312" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_transaction_count(
        &self,
        address: &str,
        tag: Option<Tag>,
    ) -> anyhow::Result<JsonRpcResult<String>> {
        let mut t = String::from(Tag::Latest);
        if let Some(tag) = tag {
            t = String::from(tag);
        }
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getTransactionCount", "params": [address, t], "id": "313" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_block_transaction_count_by_hash(
        &self,
        hash: &str,
    ) -> anyhow::Result<JsonRpcResult<String>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getBlockTransactionCountByHash", "params": [hash], "id": "314" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_block_transaction_count_by_number(
        &self,
        number: &str,
    ) -> anyhow::Result<JsonRpcResult<String>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getBlockTransactionCountByNumber", "params": [number], "id": "315" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_uncle_count_by_block_hash(
        &self,
        hash: &str,
    ) -> anyhow::Result<JsonRpcResult<String>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getUncleCountByBlockHash", "params": [hash], "id": "316" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_uncle_count_by_block_number(
        &self,
        number: &str,
    ) -> anyhow::Result<JsonRpcResult<String>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getUncleCountByBlockNumber", "params": [number], "id": "317" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_get_code(
        &self,
        address: &str,
        tag: Option<Tag>,
    ) -> anyhow::Result<JsonRpcResult<String>> {
        let mut t = String::from(Tag::Latest);
        if let Some(tag) = tag {
            t = String::from(tag);
        }
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_getCode", "params": [address, t], "id": "318" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }

    pub async fn eth_sign(
        &self,
        address: &str,
        data: &str,
    ) -> anyhow::Result<JsonRpcResult<String>> {
        let payload = json!({ "jsonrpc": "2.0", "method": "eth_sign", "params": [address, data], "id": "319" });
        let result = self.client.post(payload).await?;
        let r: JsonRpcResult<String> = serde_json::from_str(result.as_str())?;

        Ok(r)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn get_block_transaction_count_by_hash() {
        let rpc = Web3::new("https://mainnet.infura.io/v3/xxx".to_string());
        let r = rpc
            .eth_get_block_transaction_count_by_hash(
                "0xe812a49745d691961893d7cfd3902d78d710751bab872f12215ee23f27f3efa9",
            )
            .await;
        println!("{:?}", r);
    }
}
