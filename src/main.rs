/// Returns the string "Fizz" if the number is divisible by 3,
/// "Buzz" if the number is divisible by 5, and "FizzBuzz" if
/// the number is divisible by both 3 and 5. Otherwise, returns
/// the number as a string.
fn fizz_buzz(n: i32) -> String {
    let mut result = String::new();
    if n % 3 == 0 {
        result.push_str("Fizz");
    }
    if n % 5 == 0 {
        result.push_str("Buzz");
    }
    if result.is_empty() {
        result.push_str(&n.to_string());
    }
    result
}

/// Prints the N strings where N is the number of times to call
/// the fn fizz_buzz.
fn main() {
    // Check if the user has provided the number of times to call
    // and print help if not.
    if std::env::args().len() != 2 {
        println!("Usage: fizzbuzz [N]");
        std::process::exit(1);
    }

    // Get the number of times to call fizz_buzz from the command line
    // and print the result
    let args: Vec<String> = std::env::args().collect();
    let n = args[1].parse::<i32>().unwrap();
    for i in 1..=n {
        println!("{}", fizz_buzz(i));
    }
}

/// Tests for the fizz_buzz function
/// Run with `cargo test`
/// Path: src/main.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fizz_buzz() {
        assert_eq!(fizz_buzz(1), "1");
        assert_eq!(fizz_buzz(2), "2");
        assert_eq!(fizz_buzz(3), "Fizz");
        assert_eq!(fizz_buzz(4), "4");
        assert_eq!(fizz_buzz(5), "Buzz");
        assert_eq!(fizz_buzz(6), "Fizz");
        assert_eq!(fizz_buzz(7), "7");
        assert_eq!(fizz_buzz(8), "8");
        assert_eq!(fizz_buzz(9), "Fizz");
        assert_eq!(fizz_buzz(10), "Buzz");
        assert_eq!(fizz_buzz(11), "11");
        assert_eq!(fizz_buzz(12), "Fizz");
        assert_eq!(fizz_buzz(13), "13");
        assert_eq!(fizz_buzz(14), "14");
        assert_eq!(fizz_buzz(15), "FizzBuzz");
        assert_eq!(fizz_buzz(16), "16");
        assert_eq!(fizz_buzz(17), "17");
        assert_eq!(fizz_buzz(18), "Fizz");
        assert_eq!(fizz_buzz(19), "19");
        assert_eq!(fizz_buzz(20), "Buzz");
        assert_eq!(fizz_buzz(21), "Fizz");
        assert_eq!(fizz_buzz(22), "22");
        assert_eq!(fizz_buzz(23), "23");
        assert_eq!(fizz_buzz(24), "Fizz");
        assert_eq!(fizz_buzz(25), "Buzz");
        assert_eq!(fizz_buzz(26), "26");
        assert_eq!(fizz_buzz(27), "Fizz");
        assert_eq!(fizz_buzz(28), "28");
        assert_eq!(fizz_buzz(29), "29");
        assert_eq!(fizz_buzz(30), "FizzBuzz");
    }
}