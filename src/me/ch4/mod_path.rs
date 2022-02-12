mod aa {
    pub mod bb {
        pub mod cc {
            pub fn print() {
                println!("aa::bb::cc::print");
            }
        }
    }

    pub mod dd {
        pub mod ee {
            pub fn print() {
                println!("aa::dd::ee::print");
            }
        }
        pub mod ff {
            pub fn print() {
                super::ee::print();
                super::super::bb::cc::print();
                println!("aa::dd::ff::print");
            }
        }
    }
}

fn main() {
    aa::bb::cc::print();
    aa::dd::ee::print();
    crate::aa::dd::ff::print();
}
