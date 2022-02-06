fn main() {
    let pr = "知恵は武器よりも価値がある";
    println!("先頭2文字: {}", &pr[0..6]);

    // 武器
    println!("{}", &pr[9..15]);
    // panic
    println!("{}", &pr[9..14]);
}
