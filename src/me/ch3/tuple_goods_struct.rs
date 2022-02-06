struct Item(String, i64);

fn main() {
    let banana = Item("banana".to_string(), 100);
    let apple = Item("apple".to_string(), 300);
    let orange = Item("orage".to_string(), 200);
    let items = vec![banana, apple, orange];
    let total = print_and_sum_items(&items);
    println!("{}", total);
}

fn print_and_sum_items(items: &Vec<Item>) -> i64 {
    let mut total = 0;
    for item in items {
        println!("{} {}", item.0, item.1);
        total += item.1;
    }
    total
}
