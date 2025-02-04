#[derive(Debug, PartialEq)]
struct GItem {
    name: String,
    price: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn item_test() {
        let apple1 = GItem {
            name: "apple".to_string(),
            price: 100,
        };
        let mut apple2 = GItem {
            name: "apple".to_string(),
            price: 0,
        };
        apple2.price = 100;

        assert_eq!(apple1.name, apple2.name);
        assert_eq!(apple1.price, apple2.price);
        assert_eq!(apple1, apple2);
    }
}
