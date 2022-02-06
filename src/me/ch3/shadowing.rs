fn main() {
    {
        let mut v = 300;
        v += 5;
        println!("v={}", v);
    }
    {
        let v = 300;
        let v = v + 5;
        println!("v={}", v);
    }
}
