use std::rc::Rc;
fn main() {
    let mut a_rc = Rc::new(5);
    let mut b_rc = Rc::clone(&a_rc);
    *b_rc += 1;
    println!("a_rc: {}, b_rc: {}", a_rc, b_rc);
}
