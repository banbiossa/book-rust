// documents
/// Eval RPN
/// # Example
/// ```
/// let src = String::from("1 2 + 3 *");
/// a = rpn_calc::eval(src).unwrap();
/// println!("{}", a); // -> 9
/// ```

pub fn eval(src: String) -> Result<f64, String> {
    let tokens = src.split_whitespace();
    let mut stack: Vec<f64> = vec![];
    for tok in tokens {
        let t = tok.trim();
        if t == "" {
            continue;
        }
        match t.parse::<f64>() {
            Ok(v) => {
                stack.push(v);
                continue;
            }
            Err(_) => 0.0,
        };
        let b = stack.pop().unwrap_or(0.0);
        let a = stack.pop().unwrap_or(0.0);

        match t {
            "+" => stack.push(a + b),
            "-" => stack.push(a - b),
            "/" => stack.push(a / b),
            "*" => stack.push(a * b),
            "%" => stack.push(a % b),
            _ => return Err(format!("invalid operator: {}", t)),
        }
    }
    if stack.len() == 0 {
        return Err(format!("No result"));
    }
    if stack.len() > 1 {
        return Err(format!("Too many values in stack"));
    }
    Ok(stack.pop().unwrap_or(0.0))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(eval("1 3 +".to_string()), Ok(4.0));
        assert_eq!(eval("2 3 *".to_string()), Ok(6.0));
        assert_eq!(eval("6 3 /".to_string()), Ok(2.0));
        assert_eq!(eval("6 3 - 1 -".to_string()), Ok(2.0));
    }
}
