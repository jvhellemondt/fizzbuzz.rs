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
        let test_cases = vec![1, 2, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

        test_cases
            .iter()
            .for_each(|&num| assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(num), num.to_string()));
    }
}
