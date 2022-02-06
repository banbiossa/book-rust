use encoding_rs;
use std::fs;
use std::fs::File;
use std::io::Write;

fn main() {
    let filename = "test-sjis.txt";
    save_sjis(filename, "こっそり食べるものは美味しい.");
    let s = load_sjis(filename);
    println!("{}", s);
}

fn save_sjis(filename: &str, text: &str) {
    let (enc, _, _) = encoding_rs::SHIFT_JIS.encode(text);
    let buf = enc.into_owned();
    let mut file = File::create(filename).expect("file create failed");
    file.write(&buf[..]).expect("file write failed");
}

fn load_sjis(filename: &str) -> String {
    let buf = fs::read(filename).expect("file read failed");
    let (dec, _, _) = encoding_rs::SHIFT_JIS.decode(&buf);
    return dec.into_owned();
}
