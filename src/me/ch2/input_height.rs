fn main() {
    let mut height;
    loop {
        println!("Height(cm)?");
        height = input_f(0.0);
        if height > 0.0 {
            break;
        }
        println!("Invalid height, try again.");
    }

    let weight = 22.0 * (height / 100.0).powf(2.0);
    println!("Normal weight(kg): {}", weight);
}

// get input from stdin
fn input_str() -> String {
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
    return s.trim_end().to_string();
}

fn input_f(def: f64) -> f64 {
    let s = input_str();
    match s.trim().parse() {
        Ok(v) => v,
        Err(_) => def,
    }
}
