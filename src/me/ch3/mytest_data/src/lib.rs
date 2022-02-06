#[cfg(test)]
mod tests {
    #[test]
    fn array_test() {
        let a1 = [10, 20, 30];
        let a2 = [10, 20, 30];
        assert_eq!(a1, a2);

        let a3 = ["apple".to_string(), "banana".to_string()];
        let a4 = [Sring::from("apple"), String::from("banana")];
        assert_eq!(a3, a4);
    }
    #[test]
    fn vec_test() {
        let v1 = vec!["apple", "banana", "mango"];
        let mut v2: Vec<&str> = Vec::new();
        v2.push("apple");
        v2.push("banana");
        v2.push("mango");
        assert_eq!(v1, v2);
    }
}
