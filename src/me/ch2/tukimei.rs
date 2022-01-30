use std::collections::HashMap;

fn main() {
    let tuki = ["睦月", "如月", "弥生", "卯月"];
    let mut tuki_map: HashMap<&str, usize> = HashMap::new();
    for (i, v) in tuki.iter().enumerate() {
        tuki_map.insert(v, i + 1);
    }

    for s in ["卯月", "神無月"] {
        let month = tuki_map.get(s);
        if month == None {
            println!("{}はありません", s);
        } else {
            println!("{} = {}月", s, tuki_map[s]);
        }
    }

    // using None/Some
    for s in ["卯月", "神無月"] {
        match tuki_map.get(s) {
            Some(v) => println!("{} = {}月", s, v),
            None => println!("{}はありません", s),
        }
    }
}
