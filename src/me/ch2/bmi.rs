fn main() {
    let height_cm = input("Height (cm): ");
    let weight_kg = input("Weight (kg): ");

    // bmi
    let height = height_cm / 100.0;
    let bmi = weight_kg / (height * height);
    println!("BMI: {}", bmi);

    // bmi category
    if bmi < 18.5 {
        println!("Underweight");
    } else if bmi < 25.0 {
        println!("Normal");
    } else if bmi < 30.0 {
        println!("Obese Class I");
    } else if bmi < 35.0 {
        println!("Obese Class II");
    } else if bmi < 40.0 {
        println!("Obese Class III");
    } else {
        println!("Obese Class IV");
    }
}

fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut s = String::new();
    std::io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line");
    return s.trim().parse().expect("Failed to parse f64");
}
