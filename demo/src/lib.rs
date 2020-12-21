#![no_std]
#![feature(default_alloc_error_handler)]

extern crate alloc;

use alloc::string;
use mw_std::debug;
use mw_std::fs;
use mw_std::sql;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[mw_rt::async_main]
async fn main() {
    let _r = fs::read_file("./test.txt").await;
    debug::println("ok");
}

#[no_mangle]
pub extern "C" fn sql() {
    let runtime = mw_rt::runtime::Runtime::new();

    runtime.spawn(async move {
        let result = sql::sql_run("select * from test_db").await;
        let s = unsafe { alloc::slice::from_raw_parts(result.0, result.1) };
        let str = string::String::from_utf8(s.to_vec()).unwrap();
        debug::println(&alloc::format!("{:?}", result));
        debug::println(str.as_str());
    });
}

// #[mw_rt::main]
// fn main() {
//     let _r = fs::read_file_callback("./test.txt", |_result| {
//         debug::println("ok");
//     });
// }
