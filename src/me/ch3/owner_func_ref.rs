fn main() {
    let g1 = String::from("hello");
    show_message(&g1);
    println!("{}", g1);
}

fn show_message(message: &String) {
    println!("{}", message);
}
