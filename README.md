# web3-rpc
Ethereum JSON-RPC client

<a href="https://crates.io/crates/web3-rpc" rel="nofollow noopener noreferrer"><img src="https://img.shields.io/crates/v/web3-rpc" alt="Crates.io"></a>

## Usage

Add this to your Cargo.toml

```toml
[dependencies]
web3-rpc = "0.1.10"
```

```rust
use serde_json::json;
use web3_rpc::model::Tag;
use web3_rpc::web3::Web3;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let rpc = Web3::new("http://127.0.0.1:8545".to_string());
    let r = rpc.web3_client_version().await?;
    println!("{:?}", r);
    let r = rpc.web3_sha3("0x68656c6c6f20776f726c64").await?;
    println!("{:?}", r);

    let r = rpc.net_version().await?;
    println!("{:?}", r);

    let r = rpc.net_listening().await?;
    println!("{:?}", r);

    let r = rpc.net_peer_count().await?;
    println!("{:?}", r);

    let r = rpc.eth_protocol_version().await?;
    println!("{:?}", r);

    let r = rpc.eth_syncing().await?;
    println!("{:?}", r);

    let r = rpc.eth_coinbase().await?;
    println!("{:?}", r);

    let r = rpc.eth_mining().await?;
    println!("{:?}", r);

    let r = rpc.eth_hashrate().await?;
    println!("{:?}", r);

    let r = rpc.eth_gas_price().await?;
    println!("{:?}", r);

    let r = rpc.eth_accounts().await?;
    println!("{:?}", r);

    let r = rpc
        .eth_get_balance(
            "0x846c4dc9f4e2514206ef179eaa0bcfae007e37d2",
            Some(Tag::Latest),
        )
        .await?;
    println!("{:?}", r);

    let r = rpc
        .eth_get_storage_at("0x295a70b2de5e3953354a6a8344e616ed314d7251", "0x0", None)
        .await?;
    println!("{:?}", r);

    let r = rpc
        .eth_get_transaction_count("0x846c4dc9f4e2514206ef179eaa0bcfae007e37d2", None)
        .await?;
    println!("{:?}", r);

    match rpc
        .eth_get_block_transaction_count_by_hash(
            "0xe812a49745d691961893d7cfd3902d78d710751bab872f12215ee23f27f3efa9",
        )
        .await
    {
        Ok(r) => {
            println!("{:?}", r);
        }
        Err(e) => {
            println!("{:?}", e);
        }
    }

    /*
    let r = rpc
        .eth_get_block_transaction_count_by_number("0xdf969d")
        .await?;
    println!("{:?}", r);

    let r = rpc
        .eth_get_uncle_count_by_block_hash(
            "0xb903239f8543d04b5dc1ba6579132b143087c68db1b2168786408fcbce568238",
        )
        .await?;
    println!("{:?}", r);

    let r = rpc.eth_get_uncle_count_by_block_number("0xdf969d").await?;
    println!("{:?}", r);*/

    /*let r = rpc
        .eth_get_code("0xc00e94cb662c3520282e6f5717214004a7f26888", None)
        .await?;
    println!("{:?}", r);*/

    let r = rpc
        .eth_sign("0x846c4dc9f4e2514206ef179eaa0bcfae007e37d2", "0x84")
        .await?;
    println!("{:?}", r);

    let r = rpc
        .eth_send_transaction(
            "0x846c4dc9f4e2514206ef179eaa0bcfae007e37d2",
            "0x4e910ef89c176119744977d90093858089454223",
            "0x76c0",
            "0x9184e72a000",
            "0x9184e72a",
            "0x",
        )
        .await?;
    println!("{:?}", r);

    let r = rpc
        .eth_call(json!({"to": "0x846c4dc9f4e2514206ef179eaa0bcfae007e37d2", "data": "0x313ce567"}))
        .await?;
    println!("{:?}", r);

    Ok(())
}
```

## api

#### web3

- [x] web3_clientVersion
- [x] web3_sha3

#### net
- [x] net_version
- [x] net_listening
- [x] net_peerCount

#### eth
- [x] eth_protocolVersion
- [x] eth_syncing
- [x] eth_coinbase
- [x] eth_mining
- [x] eth_hashrate
- [x] eth_gasPrice
- [x] eth_accounts
- [x] eth_getBalance
- [x] eth_getStorageAt
- [x] eth_getTransactionCount
- [x] eth_getBlockTransactionCountByHash
- [x] eth_getBlockTransactionCountByNumber
- [x] eth_getUncleCountByBlockHash
- [x] eth_getUncleCountByBlockNumber
- [x] eth_getCode
- [x] eth_sign
- [x] eth_sendTransaction
- [x] eth_sendRawTransaction
- [x] eth_call
- [x] eth_estimateGas
- [x] eth_getBlockByHash
- [x] eth_getBlockByNumber
- [x] eth_getTransactionByHash
- [x] eth_getTransactionByBlockHashAndIndex
- [x] eth_getTransactionByBlockNumberAndIndex
- [x] eth_getTransactionReceipt
- [x] eth_getUncleByBlockHashAndIndex
- [x] eth_getUncleByBlockNumberAndIndex
- [x] eth_getCompilers
- [ ] eth_compileSolidity
- [ ] eth_compileLLL
- [ ] eth_compileSerpent
- [ ] eth_newFilter
- [ ] eth_newBlockFilter
- [ ] eth_newPendingTransactionFilter
- [ ] eth_uninstallFilter
- [ ] eth_getFilterChanges
- [ ] eth_getFilterLogs
- [ ] eth_getLogs
- [ ] eth_getWork
- [ ] eth_submitWork
- [ ] eth_submitHashrate
- [x] eth_blockNumber

#### db
- [ ] db_putString
- [ ] db_getString
- [ ] db_putHex
- [ ] db_getHex

#### shh
- [ ] shh_version
- [ ] shh_post
- [ ] shh_newIdentity
- [ ] shh_hasIdentity
- [ ] shh_newGroup
- [ ] shh_addToGroup
- [ ] shh_newFilter
- [ ] shh_uninstallFilter
- [ ] shh_getFilterChanges
- [ ] shh_getMessages

## Your Self Request
```rust
async fn your_self_rest() -> anyhow::Result<JsonRpcResult<Vec<String>>> {
    // example
    // eth_accounts
    let rpc = Web3::new("http://127.0.0.1:8545".to_string());
    let payload = json!({ "jsonrpc": "2.0", "method": "eth_accounts", "params": [], "id": "310" });
    let result = rpc.client.post(payload).await?;
    let r: JsonRpcResult<Vec<String>> = serde_json::from_str(result.as_str())?;

    Ok(r)
}
```