fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    println!("{}", add(1, 2));
    println!("{}", add(1.0, 2.0));
    // println!("{}", add('a', 'b'));
    println!("{}", add::<i32>(10, 25));
}
