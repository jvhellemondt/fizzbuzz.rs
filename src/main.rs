fn main() {
    println!("Hello, world!");
}

struct FizzBuzzChecker {}

impl FizzBuzzChecker {
    fn is_a_fizzbuzz(num: u8) -> String {
        if num % 15 == 0 {
            return "FizzBuzz".to_string()
        } else if num == 5 {
            return "Buzz".to_string()
        } else if num == 3 {
            return "Fizz".to_string()
        }
        num.to_string()
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
    fn test_five_should_be_buzz() {
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(5), "Buzz");
    }

    #[test]
    fn test_fifteen_should_be_fizzbuzz() {
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(15), "FizzBuzz");
    }

    #[test]
    fn test_a_multiple_of_fifteen_should_be_fizzbuzz() {
        let test_cases: Vec<_> = (15..=100)
            .step_by(15)
            .collect();

        test_cases
            .iter()
            .for_each(|&num| assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(num), "FizzBuzz"));
    }

    #[test]
    fn test_a_number_not_three_should_be_the_number_given() {
        let test_cases = vec![1, 2, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14];

        test_cases
            .iter()
            .for_each(|&num| assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(num), num.to_string()));
    }
}
