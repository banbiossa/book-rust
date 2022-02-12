fn main() {
    let array = [
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Durian".to_string(),
    ];
    for a in array.iter() {
        println!("{}", a);
    }
    println!("len={}", array.len());
}
