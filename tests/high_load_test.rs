// use datasize::DataSize;
// use futures::executor::block_on;
use gclient::{EventProcessor, GearApi, Result, WSAddress};
use gear_core::ids::CodeId;
use gear_core::ids::ProgramId;
use gstd::{prelude::*, ActorId};
use std::fs::read_to_string;
use std::mem::size_of;
pub const PROGRAM_ID: &str = "24a6fc596c48e33e5cb05ea37d6cfef93a6c5f178892683115e61069c78144d6";
#[tokio::test]
async fn upload_storage() -> Result<()> {
    let api = GearApi::init(WSAddress::new("wss://vit.vara-network.io", 443)).await?;
    //let mut api = GearApi::dev().await?;

    let pid = hex::decode(PROGRAM_ID).unwrap();
    let pid = ProgramId::decode(&mut pid.as_slice()).unwrap();
    let mut messages = Vec::new();

    for i in 0..10_000 {
        let message: (ProgramId, Vec<u8>, u64, u128, bool) =
            (pid, vec![], 600_000_000_000, 0, false);
        messages.push(message);
    }

    let msg_list = messages.chunks(40);
    let rpc_nonce = api.rpc_nonce().await?;
    println!("RPC NONCE {:?}", rpc_nonce);
    //    api.set_nonce(rpc_nonce + 1000);
    for (i, msgs) in msg_list.enumerate() {
        println!("Sending messages {}", i);
        api.send_message_bytes_batch(msgs.to_vec()).await?;
    }

    Ok(())
}

// #[derive(Debug, Clone, PartialEq, Eq, Encode, Decode, TypeInfo)]
// pub enum NFTMasterAction {
//     AddNFTContract { nft_contract: ActorId, meta: String },
//     RemoveNFTContract { nft_contract: ActorId },
//     AddOperator { operator: ActorId },
//     RemoveOperator { operator: ActorId },
// }
// // async fn upload_media(
// //     pid: ProgramId,
// //     messages: &[(ProgramId, Vec<u8>, u64, u128)],
// //     nonce: u32,
// // ) -> Result<()> {
// //     let mut api = GearApi::dev().await?;

// //     let msg_list = messages.chunks(25);

// //     for (i, msgs) in msg_list.enumerate() {
// //         println!("Sending messages {}", i);
// //         let rpc_nonce = api.rpc_nonce().await?;
// //         println!("RPC NONCE {:?}", rpc_nonce);
// //         api.set_nonce(rpc_nonce + nonce);
// //         api.send_message_bytes_batch(msgs.to_vec()).await?;
// //     }
// //     Ok(())
// // }
