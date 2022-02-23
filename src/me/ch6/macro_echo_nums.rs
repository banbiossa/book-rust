#[macro_export]
macro_rules! echo_nums {
    ( $( $num:expr ),* ) => {
        $ (
            print!("{}, ", $num);
        ) *
        println!("");
    }
}

fn main() {
    echo_nums![1, 2, 3, 4, 5];
}
