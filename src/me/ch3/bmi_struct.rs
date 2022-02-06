// bmi
use std::io;

// bmi struct
struct BmiRange {
    min: f64,
    max: f64,
    label: &'static str,
}

fn main() {
    let height_cm = input("height (cm)");
    let weigth_kg = input("weight (kg)");
    let height = height_cm as f64 / 100.0;
    let bmi = weigth_kg as f64 / (height * height);
    let bmi_list = vec![
        BmiRange {
            min: 0.0,
            max: 18.5,
            label: "underweight",
        },
        BmiRange {
            min: 18.5,
            max: 25.0,
            label: "normal",
        },
        BmiRange {
            min: 25.0,
            max: 30.0,
            label: "overweight",
        },
        BmiRange {
            min: 30.0,
            max: 40.0,
            label: "obese",
        },
        BmiRange {
            min: 40.0,
            max: 100.0,
            label: "morbidly obese",
        },
    ];
    // check
    for range in bmi_list {
        if bmi >= range.min && bmi < range.max {
            println!("BMI: {:.2}, you are {}", bmi, range.label);
            break;
        }
    }
}

fn input(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input.trim().parse::<f64>().expect("failed to parse f64")
}
