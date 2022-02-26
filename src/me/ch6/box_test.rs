fn main() {
    let x_box = Box::new(100);
    println!("{}", x_box);

    let x_val = *x_box;
    println!("{}", x_val);
}
