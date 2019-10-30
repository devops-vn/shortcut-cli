use sled::{Db, IVec};

use std::path::Path;

use slog::Logger;
use std::collections::HashMap;

use crate::error::EasyErrorHandling;
use colored::*;

const SWW_MSG: &'static str = "Something went wrong";

pub struct KVS<'a> {
    db: Db,
    logger: &'a Logger,
}

impl<'a> KVS<'a> {
    pub fn new(logger: &'a Logger, config: &'a HashMap<String, String>) -> Self {
        let path = Path::new(config.get("root_dir").unwrap());
        let db = Db::start_default(path).unwrap();

        Self { db, logger }
    }

    pub fn save_or_update(&self, key: &String, val: &String) {
        self.db.del(&key).unwrap();

        let val_vec = IVec::from(val.as_bytes().to_vec());

        info!(self.logger, "save-or-update key={} value={}", &key, &val);

        match self.db.set(&key, val_vec) {
            Ok(_) => info!(self.logger, "success"),
            Err(error) => {
                error!(self.logger, "failed {}", error);
            }
        };

        self.db.flush().unwrap_or_panic(SWW_MSG);
    }

    pub fn delete(&self, key: &String) -> bool {
        let r_bool = match self.db.del(&key) {
            Ok(Some(_)) => true,
            Ok(None) => false,
            Err(error) => {
                error!(self.logger, "failed to delete key={} {}", &key, error);
                false
            }
        };

        self.db.flush().unwrap_or_panic(SWW_MSG);

        r_bool
    }

    // Returns Result<Ok<String>> if found
    // Returns Result<Err<0>> if not  found
    // Returns Result<Err<-1>> if error occurred during searching
    pub fn get(&self, key: &String) -> Result<String, i16> {
        match self.db.get(&key) {
            Ok(Some(val)) => {
                let to_byte: &[u8] = &val.to_vec();
                Ok(std::str::from_utf8(to_byte).unwrap().to_owned())
            }
            Ok(None) => Err(0),
            Err(error) => {
                error!(
                    self.logger,
                    "failed to get value for key={} {}", &key, error
                );
                Err(-1)
            }
        }
    }

    pub fn list(&self) -> HashMap<String, String> {
        let mut dict = HashMap::new();

        for kv in self.db.iter() {
            match kv {
                Ok(val_kv) => {
                    let key = self.to_utf8(val_kv.0);
                    let val = self.to_utf8(val_kv.1.to_vec());
                    dict.insert(key, val);
                }
                Err(error) => {
                    error!(self.logger, "cannot get macro key/value {}", error);
                }
            }
        }

        dict
    }

    pub fn show_list(&self) {
        println!("{0: <20} {1: <80}", "KEY".bold(), "CONTENT".bold());
        for (x, y) in self.list() {
            println!("{0: <20} {1: <80}", x, y);
        }
    }

    pub fn contain_key(&self, key: &str) -> bool {
        self.db.contains_key(key).unwrap()
    }

    fn to_utf8(&self, val: std::vec::Vec<u8>) -> String {
        let to_byte: &[u8] = &val;
        std::str::from_utf8(to_byte).unwrap().to_owned()
    }
}
