#![no_std]
use gstd::{collections::HashMap, debug, exec, msg, prelude::*, ActorId};

static mut STORAGE: Option<HashMap<ActorId, Data>> = None;

struct Data {
    field_1: String,
    field_2: String,
    field_3: String,
    field_4: String,
    field_5: String,
    field_6: String,
    field_7: String,
    field_8: String,
}
#[no_mangle]
extern "C" fn handle() {
    loop{};
    // panic!("");
    // msg::reply_bytes(vec![], 0).expect("Error in sending a reply");

    // let map = unsafe { STORAGE.as_mut().expect("Unexpected error") };
    // let data = Data {
    //     field_1: "111111111111111111111111111111".to_string(),
    //     field_2: "111111111111111111111111111111".to_string(),
    //     field_3: "111111111111111111111111111111".to_string(),
    //     field_4: "111111111111111111111111111111".to_string(),
    //     field_5: "111111111111111111111111111111".to_string(),
    //     field_6: "111111111111111111111111111111".to_string(),
    //     field_7: "111111111111111111111111111111".to_string(),
    //     field_8: "111111111111111111111111111111".to_string(),
    // };

    // let len = map.len() as u64;
    // let account: ActorId = len.into();
    // map.insert(account, data);
}

#[no_mangle]
extern "C" fn init() {
    unsafe { STORAGE = Some(HashMap::with_capacity(100_000)) };
}
