fn main() {
    let afile = "./fizzbuzz_python.txt";
    let bfile = "./fizzbuzz_rust.txt";

    // file as string
    let afile_str = std::fs::read_to_string(afile).expect("Unable to read file");
    let bfile_str = std::fs::read_to_string(bfile).expect("Unable to read file");

    // trim
    let afile_str = afile_str.trim();
    let bfile_str = bfile_str.trim();

    if afile_str == bfile_str {
        println!("Files are the same");
    } else {
        println!("Files are different");
    }
}
