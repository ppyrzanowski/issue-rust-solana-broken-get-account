use solana_sdk::{self};
use solana_rpc_client::rpc_client::RpcClient;
use solana_account_decoder::{UiAccountEncoding, UiAccount};
use solana_rpc_client_api::config::RpcAccountInfoConfig;
use solana_sdk::commitment_config::CommitmentConfig;

use reqwest::{self, header::CONTENT_TYPE};
use serde_json::{json, Value};
use std::str::FromStr;

macro_rules! json_req {
    ($method: expr, $params: expr) => {{
        json!({
           "jsonrpc": "2.0",
           "id": 1,
           "method": $method,
           "params": $params,
        })
    }}
}

fn post_rpc(request: Value, rpc_url: &str) -> Value {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(rpc_url)
        .header(CONTENT_TYPE, "application/json")
        .body(request.to_string())
        .send()
        .unwrap();
    serde_json::from_str(&response.text().unwrap()).unwrap()
}

fn send_manual_rpc_get_account(rpc_url: &str, account_pubkey: &str, config: RpcAccountInfoConfig) -> Value {
    let req = json_req!("getAccountInfo", json!([account_pubkey.to_string(), config]));

    // test raw get_account_info rpc request
    let json = post_rpc(req, &rpc_url);

    json
}

fn send_client_rpc_get_account(rpc_url: &str, account_pubkey: &str, config: RpcAccountInfoConfig) -> Option<Account> {
    let rpc_client = RpcClient::new(rpc_url.to_string());

    // test client get_account_info rpc request
    let account = rpc_client.get_account_with_config(
        &solana_sdk::pubkey::Pubkey::from_str(account_pubkey).unwrap(), 
        config
    ).unwrap().value;

    account
}

fn main() {
    let rpc_url = "https://api.mainnet-beta.solana.com";
    let account_pubkey = solana_sdk::pubkey::Pubkey::from_str("QPfdoqN6vp3LqL6Fq7McEAFEccJfdQ8hotCCR29pump").unwrap();

    let config = RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::JsonParsed),
        commitment: Some(CommitmentConfig::confirmed()),
        ..RpcAccountInfoConfig::default()
    };

    let json = send_manual_rpc_get_account(rpc_url, &account_pubkey.to_string(), config.clone());
    println!("manual rpc getAccountInfo result: {:#?}", json);

    //parse account into UiAccount with serde_json
    let ui_account: UiAccount = serde_json::from_value(json["result"]["value"].clone()).unwrap();
    println!("parse manual rpc request into UiAccount struct: {:#?}", ui_account);

    let account = send_client_rpc_get_account(rpc_url, &account_pubkey.to_string(), config.clone());
    println!("client rpc getAccountInfo result (parsed data is missing): {:#?}", account);
}   