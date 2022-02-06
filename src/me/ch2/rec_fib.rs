fn fib(n: i64) -> i64 {
    // base
    if n == 1 {
        return 0;
    }
    // base2
    if n == 2 {
        return 1;
    }
    // recursive case
    return fib(n - 1) + fib(n - 2);
}

fn main() {
    for i in 2..=20 {
        println!("{}", fib(i));
    }
}
