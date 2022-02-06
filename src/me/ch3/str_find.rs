fn main() {
    let s = "隣の客はよく柿食う客だ";
    match s.find("柿") {
        Some(i) => println!("{}", i),
        None => println!("柿not found"),
    };

    match s.find("バナナ") {
        Some(i) => println!("{}", i),
        None => println!("バナナnot found"),
    };
}
