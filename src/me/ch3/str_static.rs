fn echo(s: &'static str) {
    println!("{}", s);
}

fn main() {
    echo("愚かな人でも黙っていると");
    echo("賢いとみられる");
    // let s = "test".to_string();
    // echo(&s);
}
