use std::fs;
use std::env;

fn ls(file_path: &str) {
    let paths = fs::read_dir(file_path).unwrap();

    for path in paths {
        println!("{}", path.unwrap().path().display())
    }
}

fn find_and_print(file_path: String, target: &str) {
    let paths = fs::read_dir(file_path).unwrap();
    let mut files: Vec<String> = Vec::new();

    for path in paths {
        let file_path = path.unwrap().path();
        let file_name = file_path.file_name().unwrap().to_str().unwrap();
        files.push(file_path.display().to_string());

        if file_path.is_dir() {
            find_and_print(file_path.display().to_string(), target)
        }

        if file_name == target {
            println!("{}", file_path.display());
            break
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        ls(args[1].as_str());
    }

    if args.len() >= 3 && args[2] == "--find" {
        find_and_print(args[1].to_string(), args[3].as_str());
    }
}