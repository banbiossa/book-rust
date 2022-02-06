fn main() {
    let pr = "ねこにこばん";
    for c in pr.bytes() {
        print!("{:2x} ", c);
    }
    println!("バイト数={}B", pr.len());
}
