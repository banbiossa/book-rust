// recursive sum
fn sum(n: i32) -> i32 {
    // base case
    if n <= 0 {
        return 0;
    }
    //  recursive case
    return sum(n - 1) + n;
}

fn main() {
    println!("{}", sum(10));
}
