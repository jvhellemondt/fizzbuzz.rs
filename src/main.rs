fn main() {
    println!("Hello, world!");
}

struct FizzBuzzChecker {}

impl FizzBuzzChecker {
    fn is_a_fizzbuzz(num: u8) -> String {
        if num != 3 {
            return num.to_string();
        }
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
    fn test_a_number_not_three_should_be_the_number_given() {
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(1), "1");
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(2), "2");
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(4), "4");
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(5), "5");
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(6), "6");
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(7), "7");
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(8), "8");
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(9), "9");
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(10), "10");
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(11), "11");
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(12), "12");
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(13), "13");
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(14), "14");
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(15), "15");
    }
}
