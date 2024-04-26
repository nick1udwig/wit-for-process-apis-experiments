use kinode_process_lib::{
    await_message, call_init, our_capabilities, println, spawn,
    vfs::{create_drive, metadata, open_dir, Directory, FileType},
    Address, OnExit, ProcessId, Request, Response,
};

//use crate::kinode::process::api::*;
//use crate::{TransferRequest, TransferResponse, FileInfo, WorkerRequest};
//use crate::kinode::process::api::*;
use crate::kinode::process::standard::ProcessId as WitProcessId;
use crate::kinode::process::api::{Address as WitAddress, TransferRequest, TransferDownload, TransferRequestProgress, TransferResponse, FileInfo, WorkerRequest, WorkerRequestInitialize};

use crate::kinode::process::api::my_func;

wit_bindgen::generate!({
    path: "wit",
    world: "file-transfer-func",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize],
});

impl From<Address> for WitAddress {
    fn from(address: Address) -> Self {
        WitAddress {
            node: address.node,
            process: address.process.into(),
        }
    }
}

impl From<ProcessId> for WitProcessId {
    fn from(process: ProcessId) -> Self {
        WitProcessId {
            process_name: process.process_name,
            package_name: process.package_name,
            publisher_node: process.publisher_node,
        }
    }
}
impl From<WitAddress> for Address {
    fn from(address: WitAddress) -> Self {
        Address {
            node: address.node,
            process: address.process.into(),
        }
    }
}

impl From<WitProcessId> for ProcessId {
    fn from(process: WitProcessId) -> Self {
        ProcessId {
            process_name: process.process_name,
            package_name: process.package_name,
            publisher_node: process.publisher_node,
        }
    }
}

fn ls_files(files_dir: &Directory) -> anyhow::Result<Vec<FileInfo>> {
    let entries = files_dir.read()?;
    let files: Vec<FileInfo> = entries
        .iter()
        .filter_map(|file| match file.file_type {
            FileType::File => match metadata(&file.path, None) {
                Ok(metadata) => Some(FileInfo {
                    name: file.path.clone(),
                    size: metadata.len,
                }),
                Err(_) => None,
            },
            _ => None,
        })
        .collect();

    Ok(files)
}

fn handle_transfer_request(
    our: &Address,
    source: &Address,
    body: &[u8],
    files_dir: &Directory,
) -> anyhow::Result<()> {
    let transfer_request = serde_json::from_slice::<TransferRequest>(body)?;

    match transfer_request {
        TransferRequest::ListFiles => {
            let files = ls_files(files_dir)?;

            Response::new()
                .body(serde_json::to_vec(&TransferResponse::ListFiles(files))?)
                .send()?;
        }
        //TransferRequest::Download { name, target } => {
        TransferRequest::Download(TransferDownload { name, target })  => {
            // spin up a worker, initialize based on whether it's a downloader or a sender.
            let our_worker = spawn(
                None,
                &format!("{}/pkg/worker.wasm", our.package_id()),
                OnExit::None,
                our_capabilities(),
                vec![],
                false,
            )?;

            let our_worker_address = Address {
                node: our.node.clone(),
                process: our_worker,
            };

            if source.node == our.node {
                // we want to download a file
                let _resp = Request::new()
                    .body(serde_json::to_vec(&WorkerRequest::Initialize(WorkerRequestInitialize {
                        name: name.clone(),
                        target_worker: None,
                    }))?)
                    .target(&our_worker_address)
                    .send_and_await_response(5)??;

                // send our initialized worker address to the other node
                Request::new()
                    .body(serde_json::to_vec(&TransferRequest::Download(TransferDownload {
                        name: name.clone(),
                        target: our_worker_address.into(),
                    }))?)
                    .target(&target.into())
                    .send()?;
            } else {
                // they want to download a file
                Request::new()
                    .body(serde_json::to_vec(&WorkerRequest::Initialize(WorkerRequestInitialize {
                        name: name.clone(),
                        target_worker: Some(target),
                    }))?)
                    .target(&our_worker_address)
                    .send()?;
            }
        }
        //TransferRequest::Progress { name, progress } => {
        TransferRequest::Progress(TransferRequestProgress { name, progress }) => {
            println!("{} progress: {}%", name, progress);
        }
    }

    Ok(())
}

fn handle_message(our: &Address, files_dir: &Directory) -> anyhow::Result<()> {
    let message = await_message()?;
    handle_transfer_request(our, message.source(), message.body(), files_dir)
}

call_init!(init);
fn init(our: Address) {
    println!("begin {:?}", my_func(true));

    let drive_path = create_drive(our.package_id(), "files", None).unwrap();
    let files_dir = open_dir(&drive_path, false, None).unwrap();

    loop {
        match handle_message(&our, &files_dir) {
            Ok(()) => {}
            Err(e) => {
                println!("error: {:?}", e);
            }
        };
    }
}
