use std::fs;
use std::env;

fn ls(file_path: &str) {
    let paths = fs::read_dir(file_path).unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    ls(args[1].as_str());
}