fn main() {
    //u32
    let a_vec: Vec<u32> = vec![1, 2, 3, 4, 5];
    for i in a_vec {
        println!("{}", i);
    }

    // &str vec
    let s_vec: Vec<&str> = vec!["hello", "world"];
    for i in s_vec {
        println!("{}", i);
    }
}
