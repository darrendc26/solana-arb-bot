// use solana_client::rpc_client::RpcClient;
// use solana_sdk::{signature::{Keypair, Signature, Signer},
//     transaction::VersionedTransaction,
//     message::v0::Message};
// use base64::{decode, encode};
// use std::env;
// use dotenv::dotenv;

// pub async fn sign_txn(txn: String) -> Result<String> {
//     dotenv().ok();
//     let url = "https://api.mainnet-beta.solana.com";
//     let rpc = RpcClient::new(url.to_string());
//     let private_key = env::var("USER_PRIVATE_KEY").unwrap();
//     let keypair = Keypair::from_base58_string(&private_key);
//     let tx_bytes = decode(txn).unwrap();
//     let mut versioned_txn = VersionedTransaction::deserialize(&tx_bytes).unwrap();
//     let recent_blockhash = rpc.get_latest_blockhash();  
//     versioned_txn.messsage.recent_blockhash = recent_blockhash;
//     versioned_txn.

// }