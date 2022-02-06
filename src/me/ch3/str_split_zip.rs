fn main() {
    let zipcode = "105-0011";
    println!("slice");
    println!("1st: {}", &zipcode[0..3]);
    println!("2nd: {}", &zipcode[4..]);

    // split_at
    println!("split_at");
    let (zip1, zip2) = zipcode.split_at(3);
    let (zip2, zip3) = zip2.split_at(1);
    println!("1st: {}", zip1);
    println!("mark: {}", zip2);
    println!("2nd: {}", zip3);

    // split_off
    println!("split_off");
    let mut zip1 = String::from(zipcode);
    let mut zip2 = zip1.split_off(3);
    let zip3 = zip2.split_off(1);
    println!("1st: {}", zip1);
    println!("mark: {}", zip2);
    println!("2nd: {}", zip3);

    // split
    println!("split");
    let zip_a: Vec<&str> = zipcode.split("-").collect();
    println!("1st: {}", zip_a[0]);
    println!("2nd: {}", zip_a[1]);
}
