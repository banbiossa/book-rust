fn sum_slice(items: &[i64]) -> i64 {
    let mut total = 0;
    for item in items {
        total += item;
    }
    total
}

fn main() {
    let a = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("a={}", sum_slice(&a[0..5]));
    println!("a={}", sum_slice(&a));
    let b = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("b={}", sum_slice(&b[..]));
}
