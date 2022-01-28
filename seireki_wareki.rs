/*
- 1926: showa
- 1989: heisei
- 2019: reiwa

初年は元年とする
 */
fn main() {
    for i in 1926..2027 {
        print!("西暦{}年 = ", i);
        if i < 1989 {
            print!("昭和");
            if i == 1926 {
                print!("元");
            } else {
                print!("{}", i - 1926 + 1);
            }
            println!("年");
        } else if i < 2019 {
            print!("平成");
            if i == 1989 {
                print!("元");
            } else {
                print!("{}", i - 1989 + 1);
            }
            println!("年");
        } else {
            print!("令和");
            if i == 2019 {
                print!("元");
            } else {
                print!("{}", i - 2019 + 1);
            }
            println!("年");
        }
    }
}
