fn main() {
    let pr = "豚に真珠";
    let mut sub1 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if i < 2 {
            sub1.push(c);
            continue;
        }
        break;
    }
    println!("先頭2文字={}", sub1);

    let mut sub2 = String::new();
    for (i, c) in pr.chars().enumerate() {
        if 2 <= i && i <= 3 {
            sub2.push(c);
        }
    
    println!("3-4文字目={}", sub2);
}
