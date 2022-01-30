fn main() {
    let a = 'a';
    let b = b'a';
    let c = '\x61';
    println!("{} {:2x} {}", a, b, c);

    let d = '愛';
    let e = '愛' as u32;
    let f = '\u{611b}';
    println!("{} {:4x} {}", d, e, f);
}
