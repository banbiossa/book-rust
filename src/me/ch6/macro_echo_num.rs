macro_rules! echo_num {
    ($num:expr) => {
        println!("{}", $num);
    };
}

fn main() {
    echo_num!(1);
    echo_num!(2);
    echo_num!(3);
}
