fn main() {
    println!("Hello, world!");
}

struct FizzBuzzChecker {}

impl FizzBuzzChecker {
    fn is_a_fizzbuzz(_num: u8) -> String {
        "Fizz".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_should_be_fizz() {
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(3), "Fizz");
    }

    #[test]
    fn test_one_should_be_one() {
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(1), "1");
    }
}
