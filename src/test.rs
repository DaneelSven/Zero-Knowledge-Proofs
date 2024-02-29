use starknet::{
    core::types::{BlockId, BlockTag},
    providers::{
        jsonrpc::{HttpTransport, JsonRpcClient},
        Provider, Url,
    },
};

#[tokio::main]
async fn main() {
    let provider = JsonRpcClient::new(HttpTransport::new(
        Url::parse("https://starknet-goerli.g.alchemy.com/v2/oEHNP00oHKcxVYxo0T3xFNbYVtm7WskT").unwrap(),
    ));

    let block_number = provider.block_number().await.unwrap();

    // Convert the block number to BlockId
    let block_id = BlockId::Number(block_number);

    println!("Current block number of starkent {block_number} ");
    // println!("Current block id of starkent {block_id} ");



    let block_hash: starknet::core::types::MaybePendingBlockWithTxHashes = provider.get_block_with_tx_hashes(BlockId::Tag(BlockTag::Latest)).await.unwrap();
// Handle the result of the future
// match block_hash {
//     Ok(block_hash) => println!("Block hash: {:?}", block_hash), // Use {:?} or {:#?} for Debug printing
//     Err(e) => println!("Error fetching block hash: {:?}", e),
// }
println!("{block_hash:#?}");

println!("block with tx {:?}", block_hash)

}