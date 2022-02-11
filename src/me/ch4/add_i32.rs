fn add_i32(x: i32, y: i32) -> i32 {
    x + y
}
fn add_f32(x: f32, y: f32) -> f32 {
    x + y
}
fn main() {
    println!("{}", add_i32(1, 2));
    println!("{}", add_f32(1.0, 2.0));
}
