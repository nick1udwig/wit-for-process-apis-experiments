//use serde::{Deserialize, Serialize};

//use crate::kinode::process::api::Guest;
//use crate::kinode::process::standard::Address;
use crate::exports::kinode::process::api::Guest;

wit_bindgen::generate!({
    path: "wit",
    world: "file-transfer-func-api",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize],
});

struct Api;
impl Guest for Api {
    fn my_func(lol: bool) -> bool {
        true
    }
}
export!(Api);
