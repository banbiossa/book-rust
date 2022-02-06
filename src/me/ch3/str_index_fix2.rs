fn main() {
    let s2 = "abcdefg";
    println!("{}", &s2[0..3]);

    let s = "こんにちは";
    let ch = &s[..3];
    println!("{}", ch);

    let ch = &s[6..9];
    println!("{}", ch);
}
