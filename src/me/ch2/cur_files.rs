use std::fs;

fn main() {
    // get files
    let files = fs::read_dir(".").unwrap();
    for ent in files {
        let entry = ent.unwrap();
        let path = entry.path();
        let fname = path.to_str().unwrap_or("bad file name");
        println!("{}", fname);
    }
}
