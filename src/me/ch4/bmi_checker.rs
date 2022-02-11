struct BmiRange {
    min: f64,
    max: f64,
    label: String,
}

impl BmiRange {
    fn new(min: f64, max: f64, label: &str) -> Self {
        BmiRange {
            min,
            max,
            label: label.to_string(),
        }
    }
    fn test(&self, bmi: f64) -> bool {
        bmi >= self.min && bmi < self.max
    }
}

struct Body {
    height: f64,
    weight: f64,
    name: String,
}

impl Body {
    fn new(name: &str, height: f64, weight: f64) -> Self {
        Body {
            name: name.to_string(),
            height,
            weight,
        }
    }
    fn calc_bmi(&self) -> f64 {
        self.weight / (self.height * self.height)
    }
    fn print_result(&self) {
        let bmi = self.calc_bmi();
        let mut label = String::new();
        let bmi_list = [
            BmiRange::new(0.0, 18.5, "痩せ型"),
            BmiRange::new(18.5, 25.0, "標準"),
            BmiRange::new(25.0, 30.0, "肥満（１度）"),
            BmiRange::new(30.0, 35.0, "肥満（２度）"),
            BmiRange::new(35.0, 40.0, "肥満（３度）"),
            BmiRange::new(40.0, 100.0, "肥満（４度）"),
        ];
        for range in bmi_list {
            if range.test(bmi) {
                label = range.label.clone();
                break;
            }
        }
        println!("{}さんのBMIは{:.2}です。hantei: {}", self.name, bmi, label);
    }
}

fn main() {
    let body = Body::new("Taro", 1.75, 80.0);
    body.print_result();
    let body = Body::new("Jiro", 1.75, 60.0);
    body.print_result();
    let body = Body::new("Saburo", 1.75, 40.0);
    body.print_result();
}
