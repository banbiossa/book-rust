fn main() {
    let args = std::env::args();
    let mut total = 0.0;
    // add all command line arguments
    for (i, s) in args.enumerate() {
        // skip command (arg[0])
        if i == 0 {
            continue;
        };
        // to number
        let num: f64 = match s.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };
        total += num;
    }
    println!("{}", total);
}
