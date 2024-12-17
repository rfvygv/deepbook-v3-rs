use deepbook_v3_rs::client::DeepBookClient;
use deepbook_v3_rs::types::index::Environment;
use std::sync::Arc;
use sui_sdk::SuiClientBuilder;
use sui_types::{base_types::SuiAddress, crypto::SuiKeyPair};

#[tokio::main]
async fn main() {
    // let sui_mainnet = SuiClientBuilder::default()
    //     .build("https://fullnode.mainnet.sui.io:443")
    //     .await
    //     .unwrap();

    // let sui = SuiClientBuilder::default()
    //     .build("http:// 127.0.0.1:9000")
    //     .await
    //     .unwrap();

    let sui_testnet = SuiClientBuilder::default().build_testnet().await.unwrap();

    // Do not use this account on the mainnet.
    let skp: SuiKeyPair = SuiKeyPair::decode(
        "suiprivkey1qpta0trskvfdwhhnvlczuxtuujuycccx2uf56ucpm7j4qhuu6x8yxp43y5q",
    )
    .map_err(|_| anyhow::anyhow!("Invalid PrivateKey"))
    .unwrap();

    let pk = skp.public();
    let sender = SuiAddress::from(&pk);

    let env = Environment::Testnet;

    let db_client =
        DeepBookClient::new(Arc::new(sui_testnet), sender, env, None, None, None, None).unwrap();

    // Create and Share BalanceManager
    let pt = db_client
        .balance_manager
        .create_and_share_balance_manager()
        .unwrap();

    let res = db_client
        .clone()
        .execute_and_sign_transaction(pt, 50000000)
        .await
        .unwrap();

    println!("{:?}", res);
}
