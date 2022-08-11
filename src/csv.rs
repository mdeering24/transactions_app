use std::env;
use std::fs::File;
use std::path::Path;

use csv::Reader;

pub fn import_csv_data() -> Reader<File> {
    let reader = Reader::from_path(Path::new(&get_path_arg()));
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
