struct Body {
    height: f64,
    weight: f64,
}

impl Body {
    fn calc_bmi(&self) -> f64 {
        self.weight / (self.height * self.height)
    }
    fn calc_per(&self) -> f64 {
        self.calc_bmi() / 22.0 * 100.0
    }
}

fn main() {
    let taro = Body {
        height: 1.75,
        weight: 80.0,
    };
    println!("BMI={:.2}", taro.calc_bmi());
    println!("PER={:.1}%", taro.calc_per());
}
