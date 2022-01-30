fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename = &args[1];
    let text = std::fs::read_to_string(filename).expect("Something went wrong reading the file");
    println!("{}", text);
}
