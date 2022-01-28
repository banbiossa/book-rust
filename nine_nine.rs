/* 9 * 9 の表
 */
fn main() {
    for i in 1..10 {
        for j in 1..10 {
            /*/
            if i * j < 10 {
                print!(" {}, ", i * j);
            } else {
                print!("{}, ", i * j);
            }
            */
            print!("{:3},", i * j);
        }
        println!("");
    }
}
