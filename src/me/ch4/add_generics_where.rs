fn add<T>(a: T, b: T) -> T
where
    T: std::ops::Add<Output = T>,
{
    a + b
}

fn main() {
    println!("{}", add(1, 2));
    println!("{}", add(1.0, 2.0));
    println!("{}", add::<i32>(10, 25));
}
