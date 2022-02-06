fn main() {
    let ss: &str = "you give what you get";
    let so1: String = String::from(ss);
    let so2: String = ss.to_string();
    let ss2: &str = &so1;
    let ss3: &str = so1.as_str();

    println!("{} {} {} {}", so1, so2, ss2, ss3);
    println!("{:p} {:p}", ss2, ss3);
}
