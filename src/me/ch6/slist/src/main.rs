mod slist;
fn main() {
    let mut list = slist::List::new();
    list.push(1);
    list.push(2);
    list.unshift(0);
    list.unshift(-1);
    println!("{}", list.get(0).unwrap());
    println!("{}", list.get(1).unwrap());
    println!("{}", list.get(2).unwrap());
    println!("{}", list.get(3).unwrap());
}
