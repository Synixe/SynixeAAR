use secrets;

use chrono;
use fern;
use log;
use mysql;

pub struct SQL {
    pub pool: mysql::Pool,
    pub status: i32
}

impl SQL {
    pub fn new() -> SQL {
        info!("Requesting pool");
        let pool = get_pool().unwrap();
        info!("Pool Received");
        let mut status = -1;
        match pool.prep_exec(
            "CREATE TABLE IF NOT EXISTS `synixe_aar`.`replays` (
                `id` int(6) NOT NULL AUTO_INCREMENT,
                `date` timestamp NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
                `mission` varchar(64) NOT NULL,
                `map` varchar(32) NOT NULL,
                PRIMARY KEY (`id`)
            ) DEFAULT CHARSET=latin1;", ()
        ) {
            Ok(_) => {
                status = 10;
            },
            Err(_) => {
                status = -10;
            }
        };

        //["`shots` (`id`,`mission`,`p`,`w`,`a`) VALUES (NULL, :mission, '%1','%2','%3');", _name, _weapon, _ammo]
        match pool.prep_exec(
            "CREATE TABLE IF NOT EXISTS `synixe_aar`.`shots` (
                `id` int(6) NOT NULL AUTO_INCREMENT,
                `mission` varchar(64) NOT NULL,
                `p` varchar(32) NOT NULL,
                `w` varchar(32) NOT NULL,
                `a` varchar(32) NOT NULL,
                PRIMARY KEY (`id`)
            ) DEFAULT CHARSET=latin1;", ()
        ) {
            Ok(_) => {
                status = 11;
            },
            Err(_) => {
                status = -11;
            }
        };

        //["`hits` (`id`,`mission`,`p`,`c`,`d`,`i`) VALUES (NULL,:mission,'%1','%2','%3','%4');", _vname, _cause, _damage, _iname]
        match pool.prep_exec(
            "CREATE TABLE IF NOT EXISTS `synixe_aar`.`hits` (
                `id` int(6) NOT NULL AUTO_INCREMENT,
                `mission` varchar(64) NOT NULL,
                `p` varchar(32) NOT NULL,
                `c` varchar(32) NOT NULL,
                `d` varchar(32) NOT NULL,
                `i` varchar(32) NOT NULL,
                PRIMARY KEY (`id`)
            ) DEFAULT CHARSET=latin1;", ()
        ) {
            Ok(_) => {
                status = 12;
            },
            Err(_) => {
                status = -12;
            }
        };

        //["`deaths` (`id`,`mission``v`,`k`,`i`) VALUES (NULL,:mission,'%1','%2','%3');", _vname, _killer, _iname]
        match pool.prep_exec(
            "CREATE TABLE IF NOT EXISTS `synixe_aar`.`deaths` (
                `id` int(6) NOT NULL AUTO_INCREMENT,
                `mission` varchar(64) NOT NULL,
                `v` varchar(32) NOT NULL,
                `k` varchar(32) NOT NULL,
                `i` varchar(32) NOT NULL,
                PRIMARY KEY (`id`)
            ) DEFAULT CHARSET=latin1;", ()
        ) {
            Ok(_) => {
                status = 13;
            },
            Err(_) => {
                status = -13;
            }
        };

        if status > 0 {
            status = 1;
        }

        SQL {
            pool: pool,
            status: status
        }
    }

    pub fn insert(&self, text: &str, replay: u64) {
        let mut stmt = self.pool.prepare(text).unwrap();
        stmt.execute(params!{
            "mission" => replay
        });
    }
}

fn get_pool() -> mysql::Result<mysql::Pool> {
    let host = format!("mysql://{}:{}@{}:3306",
        secrets::MySQL::USERNAME,
        secrets::MySQL::PASSWORD,
        secrets::MySQL::HOST);
    info!("Creating pool: {}", host);
    mysql::Pool::new(host)
}
