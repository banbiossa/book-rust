use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let a = Rc::new(RefCell::new(1000));
    let b = Rc::clone(&a);
    *b.borrow_mut() += 10;
    println!("a={}, b={}", a.borrow(), b.borrow());
}
