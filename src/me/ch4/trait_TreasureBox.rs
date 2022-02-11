trait TreasureBox {
    fn open(&self, key_no: i32) -> bool;
    fn check(&self);
}

struct JewelryBox {
    price: i32,
    key_no: i32,
}

impl TreasureBox for JewelryBox {
    fn open(&self, key_no: i32) -> bool {
        self.key_no == key_no
    }
    fn check(&self) {
        println!("{} Galleons", self.price);
    }
}

struct TrapBox {
    damage: i32,
}

impl TreasureBox for TrapBox {
    // always opens
    fn open(&self, _key_no: i32) -> bool {
        return true;
    }
    fn check(&self) {
        println!("It's a trap! {} damage", self.damage);
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
    let box2 = TrapBox { damage: 10 };
    let box3 = JewelryBox {
        price: 200,
        key_no: 456,
    };

    let my_key = 456;
    open_box(&box1, my_key);
    open_box(&box2, my_key);
    open_box(&box3, my_key);
}
