fn main() {
    /* price: 98_000
    1200 *0.8
    *0.9
     */

    let price = 98_000.0;
    let a = price * 0.8 + 1200.0;
    let b = price * 0.9;

    println!("a is {}, b is {}", a, b);
    if a < b {
        println!("a is cheaper: {}", a);
    } else {
        println!("b is cheaper {}", b);
    }
}
