fn main() {
    let args = std::env::args();
    let mut total: f64 = 0.0;
    // process all command line arguments
    for (i, fname) in args.enumerate() {
        // skip command (arg[0])
        if i == 0 {
            continue;
        };
        // read file
        // let contents = match std::fs::read_to_string(&fname) {
        //     Ok(v) => v,
        //     Err(_) => continue,
        // };
        let contents = std::fs::read_to_string(&fname).unwrap();
        // parse numbers
        for line in contents.lines() {
            let num: f64 = match line.parse() {
                Ok(v) => v,
                Err(_) => continue,
            };
            total += num;
        }
    }
    println!("{}", total);
}
