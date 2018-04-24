mod sql;
mod secrets;

use sql::SQL;

extern crate libc;
use libc::c_char;
use libc::strncpy;

#[macro_use]
extern crate fern;

#[macro_use]
extern crate log;

extern crate chrono;

#[macro_use]
extern crate mysql;

#[macro_use]
extern crate lazy_static;

use std::ffi::CString;
use std::ffi::CStr;

use std::sync::Mutex;

lazy_static! {
    static ref conn: SQL = SQL::new();
}

static mut replay_id: u64 = 0;

fn get_c<T: Into<Vec<u8>>>(t: T) -> *mut c_char {
    CString::new(t).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn RvExtensionVersion(output: *mut c_char, output_size: usize) {
    let v = get_c("1.0");
    strncpy(output, v, output_size);
}

#[no_mangle]
pub unsafe extern "C" fn RVExtension(output: *mut c_char, output_size: usize, function: *mut c_char ) {
    let size = output_size - 1;
    let r_function = CStr::from_ptr(function).to_str().unwrap();
    if r_function == "db-status" {
        let out = get_c(conn.status.to_string());
        strncpy(output, out, size);
    } else if r_function == "setup" {
        fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}[{}][{}] {}",
                chrono::Local::now().format("[%Y-%m-%d][%H:%M:%S]"),
                record.target(),
                record.level(),
                message
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("hyper", log::LevelFilter::Info)
        .chain(fern::log_file("output.log").unwrap())
        .apply();
        info!("pong");
    }
}

#[no_mangle]
pub unsafe extern "C" fn RVExtensionArgs(output: *mut c_char, output_size: usize, function: *mut c_char, args: &[*mut c_char], arg_count: usize) {
    let size = output_size - 1;
    let r_function = CStr::from_ptr(function).to_str().unwrap();
    if r_function == "init-replay" {
        let mut stmt = conn.pool.prepare("INSERT INTO `replays`(`id``, `mission`, `map`) VALUES (NULL, :mission, :map)").unwrap();
        match stmt.execute(params!{
            "mission" => CStr::from_ptr(args[0]).to_str().unwrap(),
            "map" => CStr::from_ptr(args[1]).to_str().unwrap()
        }) {
            Ok(result) => {
                let out = get_c("Replay created");
                strncpy(output, out, size);
                replay_id = result.last_insert_id();
            },
            Err(_) => {
                let out = get_c("Execute Failed");
                strncpy(output, out, size);
            }
        };
    } else if (r_function == "insert") {
        conn.insert(CStr::from_ptr(args[0]).to_str().unwrap(), replay_id);
    }
}
