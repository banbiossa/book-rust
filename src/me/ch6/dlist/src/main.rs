mod dlist;
fn main() {
    let mut list = dlist::List::new();
    list.push(1);
    list.push(2);
    list.unshift(0);
    list.unshift(-1);
    for v in list.iter() {
        println!("{}", v);
    }
}
