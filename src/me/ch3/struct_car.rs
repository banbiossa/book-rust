struct CarSpec {
    name: String,
    weight: u32,
    color: String,
}

fn main() {
    let car1 = CarSpec {
        name: String::from("Ford"),
        weight: 2000,
        color: String::from("red"),
    };
    let car2 = CarSpec {
        name: String::from("Tesla"),
        weight: 3000,
        color: String::from("blue"),
    };

    println!("car1: {:>8}, {}kg, {}", car1.name, car1.weight, car1.color);
    println!("car2: {:>8}, {}kg, {}", car2.name, car2.weight, car2.color);
}
