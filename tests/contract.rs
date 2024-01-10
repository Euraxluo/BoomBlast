use near_workspaces::types::NearToken;
use serde_json::json;

const NFT_WASM_FILEPATH: &str = "./contract/target/wasm32-unknown-unknown/release/contract.wasm";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = near_workspaces::sandbox().await?;
    let wasm = std::fs::read(NFT_WASM_FILEPATH)?;
    let contract = worker.dev_deploy(&wasm).await?;

    let outcome = contract
        .call("init")
        .args_json(json!({
                "owner_id": contract.id(),
        }))
        .transact()
        .await?;

    println!("new_default_meta outcome: {:#?}", outcome);

    let deposit = NearToken::from_yoctonear(10000000000000000000000);
    let outcome = contract
        .call("mint")
        .args_json(json!({
            "account_id": contract.id(),
            "metadata": {
                "title": "Test NFT title",
                "description": "Test NFT",
                "copies": 1,
            },
            "memo":"test",
        }))
        // .deposit(deposit)
        .transact()
        .await?;

    println!("nft_mint outcome: {:#?}", outcome);

    let result: serde_json::Value = contract.call("nft_metadata").view().await?.json()?;

    println!("--------------\n{}", result);
    println!("Dev Account ID: {}", contract.id());
    
    let result: serde_json::Value = contract.call("nft_token").args_json(json!({
        "token_id": "1"
    })).view().await?.json()?;

    println!("--------------\n{}", result);

    Ok(())
}