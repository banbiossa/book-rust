fn main () {
    let mut total = 0;
    let nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    for i in nums {
        total += i;
    }
    println!("{}", total);
}