struct Person {
    name: String,
    age: u8,
}

impl Person {
    fn new(name: &str, age: u8) -> Self {
        Person {
            name: name.to_string(),
            age,
        }
    }
}

fn main() {
    let taro = Person::new("Taro", 20);
    println!("{}, {}", taro.name, taro.age);

    let jiro = Person {
        name: String::from("Jiro"),
        ..taro
    };
    println!("{}, {}", jiro.name, jiro.age);
}
