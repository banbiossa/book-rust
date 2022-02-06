fn main() {
    let banana = ("banana", "yellow", 300);
    let apple = ("apple", "red", 500);
    let total = banana.2 + apple.2;
    print_tuple(&banana);
    print_tuple(&apple);
    println!("{}", total);
}

fn print_tuple(item: &(&str, &str, i64)) {
    println!("{} {} {}", item.0, item.1, item.2);
}
