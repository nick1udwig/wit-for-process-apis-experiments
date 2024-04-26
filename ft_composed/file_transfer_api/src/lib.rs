//use serde::{Deserialize, Serialize};

//use crate::kinode::process::api::Guest;
//use crate::kinode::process::standard::Address;

wit_bindgen::generate!({
    path: "wit",
    world: "file-transfer-api",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize],
});

//#[derive(Serialize, Deserialize, Debug)]
//pub enum TransferRequest {
//    ListFiles,
//    Download { name: String, target: Address },
//    Progress { name: String, progress: u64 },
//}
//
//#[derive(Serialize, Deserialize, Debug)]
//pub enum TransferResponse {
//    ListFiles(Vec<FileInfo>),
//    Download { name: String, worker: Address },
//    Done,
//    Started,
//}
//
//#[derive(Serialize, Deserialize, Debug)]
//pub struct FileInfo {
//    pub name: String,
//    pub size: u64,
//}
//
//#[derive(Serialize, Deserialize, Debug)]
//pub enum WorkerRequest {
//    Initialize {
//        name: String,
//        target_worker: Option<Address>,
//    },
//    Chunk {
//        name: String,
//        offset: u64,
//        length: u64,
//    },
//    Size(u64),
//}
//
//struct Api;
////impl Guest for Api {
////}
//export!(Api);
