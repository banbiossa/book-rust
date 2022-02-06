fn main() {
    let s = "苦しむ人にはどの日も悪い日である";
    let s2 = s.replace("苦しむ人", "陽気な人");
    let s3 = s2.replace("悪い日", "いい日");
    println!("before: {}, after: {}", s, s3);
}
