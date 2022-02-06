fn main() {
    let pr = "隗より始めよ";

    let sub3: String = pr.chars().take(2).collect();
    println!("先頭2文字={}", sub3);

    let pr_chars: Vec<char> = pr.chars().collect();
    let sub_chars = &pr_chars[2..4];

    // let sub4: String = sub_chars.into_iter().collect();
    let sub4: String = sub_chars.iter().collect();
    println!("{}", sub4);
}
