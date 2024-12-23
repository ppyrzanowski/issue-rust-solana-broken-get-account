#### Problem
I noticed that the RpcClient method `get_account_with_config()` never takes the encoding field of the config struct into account and thus never includes the parsed data in the result. Since the method `account_subscribe()` on the pubsub struct already is a `UiAccount`, i would propose to keep the interfaces of pubsub and client uniform and fix `get_account...()` to return `UiAccount` wich already supports different encodings.

Maybe there is a reason, which i haven't grasped yet, why the result of the rpc call is parsed with serde into the `UiAccount` and than reduced to the simpler `Account` struct on [this line](https://github.com/anza-xyz/agave/blame/89235bb88642b7b72275142f78443d1efe03f447/rpc-client/src/nonblocking/rpc_client.rs#L3505).

I would be happy to implement this fix, as i have it already worked out.

Reproduction repo for Issue: https://github.com/ppyrzanowski/issue-rust-solana-broken-get-account

To reproduce see the [src/main.rs](src/main.rs) file.


#### Output:
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
