fn main() {
    let src = "2 3 4 * +".to_string();
    let ans = rpn_calc_by_banbiossa::eval(src).unwrap();
    println!("{}", ans);
}
