use std::env;
use std::fs::File;
use std::path::{Path, PathBuf};

use csv::Reader;

pub fn import_csv_data() -> Reader<File> {
    let path: String = get_path_arg();
    let mut relative: PathBuf = env::current_dir().unwrap();
    relative.push(path.clone());

    let mut reader: Result<Reader<File>, csv::Error> = Reader::from_path(relative.as_path());

    if reader.is_err() {
        reader = Reader::from_path(Path::new(&path)); //absolute
    }

    match reader {
        Ok(csv) => csv,
        Err(err) => {
            panic!("{}", err);
        }
    }
}

fn get_path_arg() -> String {
    let args: Vec<String> = env::args().collect();
    if args.len() > 2 {
        panic!(
            "Too many args provided. Got {} : Expected 1",
            (args.len() - 1)
        );
    } else if args.len() <= 1 {
        panic!("Need csv file path argument");
    }
    args.get(1).unwrap().clone()
}
