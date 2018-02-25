mod secrets;

extern crate libc;
use libc::c_char;
use libc::strncpy;

#[macro_use]
extern crate mysql;

use std::ffi::CString;

fn get_c<T: Into<Vec<u8>>>(t: T) -> *mut c_char {
    CString::new(t).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn RvExtensionVersion(output: *mut c_char, output_size: usize) {
    let v = get_c("1.0");
    strncpy(output, v, output_size);
}

#[no_mangle]
pub unsafe extern "C" fn RVExtension(output: *mut c_char, output_size: usize, function: char ) {
    let size = output_size - 1;
    let out = get_c("Wow so cool");
    strncpy(output, out, size);
}

#[no_mangle]
pub unsafe extern "C" fn RVExtensionArgs(output: *mut c_char, output_size: usize, function: *mut c_char, args: &[*mut c_char], arg_count: usize) {
    let size = output_size - 1;
    if *function == *get_c("init-replay") {
        let pool = get_pool();
        match pool {
            Ok(conn) => {
                setup_database(&conn);
                let mut stmt = conn.prepare("INSERT INTO `replays`(`id``, `mission`, `map`) VALUES (NULL, :mission, :map)").unwrap();
                //let mission = CStr::from_ptr(args[0]).to_str().unwrap();
                //let map = CStr::from_ptr(args[1]).to_str().unwrap();
                match stmt.execute(params!{
                    "mission" => "Hello",
                    "map" => "Tanoie"
                }) {
                    Ok(_) => {
                        let out = get_c("Replay created");
                        strncpy(output, out, size);
                    },
                    Err(_) => {
                        let out = get_c("Execute Failed");
                        strncpy(output, out, size);
                    }
                };
            },
            Err(e) => {
                let out = get_c(format!("Unable to create pool with error: {}", e));
                strncpy(output, out, size);
            }
        };
    }
}

fn get_pool() -> mysql::Result<mysql::Pool> {
    let host = format!("mysql://{}:{}@{}:3307",
        secrets::MySQL::USERNAME,
        secrets::MySQL::PASSWORD,
        secrets::MySQL::HOST);
    mysql::Pool::new(host)
}

fn setup_database(conn: &mysql::Pool) {
    conn.prep_exec("CREATE TABLE IF NOT EXISTS `synixe_aar`.`replays` (
      `id` int(6) NOT NULL AUTO_INCREMENT,
      `date` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
      `mission` varchar(64) NOT NULL,
      `map` varchar(32) NOT NULL,
      PRIMARY KEY (`id`)
  ) DEFAULT CHARSET=latin1;", ()).unwrap();
}
