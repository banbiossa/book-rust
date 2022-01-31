use std::fs::{self, File};
use std::io::{BufWriter, Write};

fn main() {
    let filename = "fizzbuzz_file_result.txt";
    // block for writing to file
    {
        // open file
        let fp = File::create(filename).unwrap();
        let mut writer = BufWriter::new(fp);
        // fizzbuzz to 100
        for i in 1..100 {
            let mut line = format!("{}\n", i);
            if (i % 3 == 0) && (i % 5 == 0) {
                line = format!("{} FizzBuzz\n", i);
            } else if i % 3 == 0 {
                line = format!("{} Fizz\n", i);
            } else if i % 5 == 0 {
                line = format!("{} Buzz\n", i);
            }
            // write to file
            let bytes = line.as_bytes();
            writer.write(bytes).unwrap();
        }
    }

    let s = fs::read_to_string(filename).unwrap();
    println!("{}", s);
}
