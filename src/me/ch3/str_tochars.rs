fn main() {
    let pr = "ねこにこばん";
    for c in pr.chars() {
        print!("{} ", c);
    }
    println!("\n文字数={}", pr.chars().count());

    let pr_chars: Vec<char> = pr.chars().collect();
    for c in pr_chars.iter() {
        print!("{} ", c);
    }
    println!("\n文字数={}", pr_chars.len());
}
