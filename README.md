#### Problem
I noticed that the RpcClient method `get_account_with_config()` never takes the encoding field of the config struct into account and thus never includes the parsed data in the result. Since the method `account_subscribe()` on the pubsub struct already is a `UiAccount`, i would propose to keep the interfaces of pubsub and client uniform and fix `get_account...()` to return `UiAccount` wich already supports different encodings.

Maybe there is a reason, which i haven't grasped yet, why the result of the rpc call is parsed with serde into the `UiAccount` and than reduced to the simpler `Account` struct on [this line](https://github.com/anza-xyz/agave/blame/89235bb88642b7b72275142f78443d1efe03f447/rpc-client/src/nonblocking/rpc_client.rs#L3505).

I would be happy to implement this fix, as i have it already worked out.

Reproduction repo for Issue: https://github.com/ppyrzanowski/issue-rust-solana-broken-get-account

To reproduce:
```rs
use solana_sdk::{self};
use solana_rpc_client::rpc_client::RpcClient;
use solana_account_decoder::UiAccount;

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

fn send_manual_rpc_get_account(rpc_url: &str, account_pubkey: &str) -> Value {
    use {
        solana_account_decoder::UiAccountEncoding,
        solana_rpc_client_api::config::RpcAccountInfoConfig,
        solana_sdk::commitment_config::CommitmentConfig,
    };
    let config = RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::JsonParsed),
        commitment: Some(CommitmentConfig::confirmed()),
        ..RpcAccountInfoConfig::default()
    };

    // test raw get_account_info rpc request
    let req = json_req!("getAccountInfo", json!([account_pubkey.to_string(), config]));
    let json = post_rpc(req, &rpc_url);

    json
}

fn send_client_rpc_get_account(rpc_url: &str, account_pubkey: &str) -> Option<UiAccount> {
    let rpc_client = RpcClient::new(rpc_url.to_string());
    use {
        solana_account_decoder::UiAccountEncoding,
        solana_rpc_client_api::config::RpcAccountInfoConfig,
        solana_sdk::commitment_config::CommitmentConfig,
    };
    let config = RpcAccountInfoConfig {
        encoding: Some(UiAccountEncoding::JsonParsed),
        commitment: Some(CommitmentConfig::confirmed()),
        ..RpcAccountInfoConfig::default()
    };

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

    let json = send_manual_rpc_get_account(rpc_url, &account_pubkey.to_string());
    println!("manual rpc getAccountInfo result: {:#?}", json);

    //parse account into UiAccount with serde_json
    let ui_account: UiAccount = serde_json::from_value(json["result"]["value"].clone()).unwrap();
    println!("parse manual rpc request into UiAccount struct: {:#?}", ui_account);

    let account = send_client_rpc_get_account(rpc_url, &account_pubkey.to_string());
    println!("client rpc getAccountInfo result (parsed data is missing): {:#?}", account);
}   
```

Result:
```js
manual rpc getAccountInfo result: Object {
    "id": Number(1),
    "jsonrpc": String("2.0"),
    "result": Object {
        "context": Object {
            "apiVersion": String("2.0.19"),
            "slot": Number(309258989),
        },
        "value": Object {
            "data": Object {
                "parsed": Object {
                    "info": Object {
                        "decimals": Number(6),
                        "freezeAuthority": Null,
                        "isInitialized": Bool(true),
                        "mintAuthority": Null,
                        "supply": String("998269279442489"),
                    },
                    "type": String("mint"),
                },
                "program": String("spl-token"),
                "space": Number(82),
            },
            "executable": Bool(false),
            "lamports": Number(1461600),
            "owner": String("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"),
            "rentEpoch": Number(18446744073709551615),
            "space": Number(82),
        },
    },
}

parse manual rpc request into UiAccount struct: UiAccount {
    lamports: 1461600,
    data: Json(
        ParsedAccount {
            program: "spl-token",
            parsed: Object {
                "info": Object {
                    "decimals": Number(6),
                    "freezeAuthority": Null,
                    "isInitialized": Bool(true),
                    "mintAuthority": Null,
                    "supply": String("998269279442489"),
                },
                "type": String("mint"),
            },
            space: 82,
        },
    ),
    owner: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA",
    executable: false,
    rent_epoch: 18446744073709551615,
    space: Some(
        82,
    ),
}

client rpc getAccountInfo result (parsed data is missing): Some(
    Account {
        lamports: 1461600,
        data.len: 82,
        owner: TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA,
        executable: false,
        rent_epoch: 18446744073709551615,
        data: 0000000006c5c1ce638d2567d26468b05eb951d1a28dcc6e123482b5c675149770e62bf23996caadeb8b03000601000000000000000000000000000000000000,
    },
)
```


#### Proposed Solution

RpcClient method `get_account_with_config()` should return `UiAccount` instead of `Account`.
