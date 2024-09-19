fn main() {
    println!("Hello, world!");
}

struct FizzBuzzChecker {}

impl FizzBuzzChecker {
    fn is_a_fizzbuzz(num: u8) -> String {
        if num % 15 == 0 {
            return "FizzBuzz".to_string()
        } else if num % 5 == 0 {
            return "Buzz".to_string()
        } else if num % 3 == 0 {
            return "Fizz".to_string()
        }
        num.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a_multiple_of_three_which_is_not_a_multiple_of_15_should_be_fizz() {
        let test_cases: Vec<_> = (3..=100)
            .step_by(3)
            .filter(|&n| n % 15 != 0 && n % 5 != 0)
            .collect();

        test_cases
            .iter()
            .for_each(|&num| assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(num), "Fizz"));
    }

    #[test]
    fn test_a_multiple_of_five_which_is_not_a_multiple_of_15_should_be_buzz() {
        let test_cases: Vec<_> = (5..=100)
            .step_by(5)
            .filter(|&n| n % 15 != 0)
            .collect();

        test_cases
            .iter()
            .for_each(|&num| assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(num), "Buzz"));
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
    fn test_a_number_not_a_multiple_of_three_or_five_should_be_the_number_given() {
        let test_cases: Vec<_> = (1..=100)
            .step_by(1)
            .filter(|&n| n % 3 != 0 && n % 5 != 0)
            .collect();

        test_cases
            .iter()
            .for_each(|&num| assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(num), num.to_string()));
    }
}
