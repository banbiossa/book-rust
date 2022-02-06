use std::{env, path};

fn main() {
    // check command line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: {} <dir> <pattern>", args[0]);
        return;
    }

    // get command line arguments
    let target_dir = &args[1];
    let keyword = &args[2];
    // to PathBuf
    let target = path::PathBuf::from(target_dir);
    findfile(&target, keyword);
}

// recursively find file with given keyword
fn findfile(target: &path::PathBuf, keyword: &str) {
    // get list of files in target directory
    let files = target
        .read_dir()
        .expect(&format!("{} is not a directory", target.display()));
    for dir_entry in files {
        // get PathBuf
        let path = dir_entry.unwrap().path();
        // recusive if path
        if path.is_dir() {
            findfile(&path, keyword);
        }
        // to string
        let fname = path.file_name().unwrap().to_string_lossy();
        // check if file name contains keyword
        if None == fname.find(keyword) {
            continue;
        }
        // display path and file name
        println!("{}", path.to_string_lossy());
    }
}
