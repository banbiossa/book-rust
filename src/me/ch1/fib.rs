fn main() {
    let mut a = 1;
    let mut b = 1;

    for _i in 1..30 {
        let c = a + b;
        println!("{}", c);
        a = b;
        b = c;
    }
}
