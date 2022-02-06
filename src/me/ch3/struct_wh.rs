struct Body {
    weight: f64,
    size: f64,
}

fn main() {
    let ichiro = Body {
        weight: 65.0,
        size: 1.8,
    };
    let jiro = Body {
        weight: 75.0,
        size: 1.9,
    };
    println!("ichiro={:.2}", calc_bmi(&ichiro));
    println!("jiro={:.1}", calc_bmi(&jiro));
}

fn calc_bmi(body: &Body) -> f64 {
    body.weight / (body.size * body.size)
}
