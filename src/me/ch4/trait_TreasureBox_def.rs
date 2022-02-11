trait TreasureBox {
    fn open(&self, key_no: i32) -> bool {
        self.get_key_no() == key_no
    }
    fn check(&self);
    fn get_key_no(&self) -> i32;
}

struct JewelryBox {
    price: i32,
    key_no: i32,
}

impl TreasureBox for JewelryBox {
    fn check(&self) {
        println!("{} Galleons", self.price);
    }
    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

struct EmptyBox {
    key_no: i32,
}

impl TreasureBox for EmptyBox {
    fn check(&self) {
        println!("It's empty");
    }
    fn get_key_no(&self) -> i32 {
        self.key_no
    }
}

fn open_box(tbox: &impl TreasureBox, key_no: i32) {
    if tbox.open(key_no) {
        tbox.check();
    } else {
        println!("The box is locked");
    }
}

fn main() {
    let box1 = JewelryBox {
        price: 100,
        key_no: 123,
    };
    let box2 = EmptyBox { key_no: 456 };
    let box3 = JewelryBox {
        price: 200,
        key_no: 456,
    };
    open_box(&box1, 456);
    open_box(&box2, 456);
    open_box(&box3, 456);
}
