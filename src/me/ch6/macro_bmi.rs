macro_rules! bmi_select {
    ($bmi:expr; $( $label:expr => $range:expr  );+ ) => {{
        let mut result = "error";
        $(
            if $range.start <= $bmi && $bmi <= $range.end {
                result = $label;
            }
        )+
        result
    }};
}

fn main() {
    let h: f32 = 158.0;
    let w: f32 = 63.0;
    let bmi = w / (h / 100.0).powf(2.0);
    let label = bmi_select![
        bmi;
        "underweight" => 0.0..18.5;
        "normal" => 18.5..25.0;
        "overweight1" => 25.0..30.0;
        "overweight2" => 30.0..35.0;
        "overweight3" => 35.0..40.0;
        "obese" => 40.0..100.0 ];
    println!("{}", label);
}
