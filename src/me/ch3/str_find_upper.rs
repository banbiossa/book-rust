fn main() {
    let s = format!(
        "{}{}",
        "There is more than one way to do it.", "But I don't know which is better.",
    );
    let res = s.find(|c: char| c.to_ascii_uppercase() == 'S");
    match res {
        Some(i) => println!("{}", i),
        None => println!("S not found"),
    };
}
