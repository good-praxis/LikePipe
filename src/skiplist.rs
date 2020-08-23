use dirs::data_dir;
use ron::{
    de::from_reader,
    ser::{to_string_pretty, PrettyConfig},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    fs::{DirBuilder, File},
    io::Write,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Skiplist {
    pub skiplist: HashSet<String>,
}

impl Skiplist {
    fn input_path() -> String {
        format!("{}/NewpipeEngager/data.ron", data_dir().unwrap().display())
    }

    fn create_file() -> File {
        let path = Skiplist::input_path();
        let dirs = String::from(path.get(0..path.rfind('/').unwrap()).unwrap());

        DirBuilder::new().recursive(true).create(dirs).unwrap();

        File::create(path).unwrap()
    }

    fn new() -> Skiplist {
        Skiplist {
            skiplist: HashSet::new(),
        }
    }

    pub fn load() -> Skiplist {
        let f = match File::open(&Skiplist::input_path()) {
            Ok(f) => f,
            Err(_) => Skiplist::create_file(),
        };

        let skiplist: Skiplist = match from_reader(f) {
            Ok(x) => x,
            Err(_e) => Skiplist::new(),
        };
        skiplist
    }

    pub fn save(&self) {
        let s = to_string_pretty(&self, PrettyConfig::new()).unwrap();
        let mut f = File::create(Skiplist::input_path()).unwrap();
        f.write_all(s.as_bytes()).unwrap();
    }
}

impl Drop for Skiplist {
    fn drop(&mut self) {
        self.save();
    }
}
