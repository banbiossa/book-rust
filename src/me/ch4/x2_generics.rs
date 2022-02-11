fn x2<T: std::ops::Add<Output = T> + Copy>(n: T) -> T {
    n + n
}

fn main() {
    println!("{}", x2(10));
    println!("{}", x2(10.0f64));
    println!("{}", x2::<u64>(3));
}
