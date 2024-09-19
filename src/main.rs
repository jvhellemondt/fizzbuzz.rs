fn main() {
    println!("Hello, world!");
}

const INVALID_NUMBER_INPUT_ERROR: &str = "Invalid input: number must be between 1 and 100";

#[derive(Debug, PartialEq)]
enum FizzBuzzResult {
    Fizz,
    Buzz,
    FizzBuzz,
    Number(u8),
}

struct FizzBuzzChecker {}

impl FizzBuzzChecker {
    fn is_a_fizzbuzz(num: u8) -> Result<FizzBuzzResult, &'static str> {
        match num {
            n if n == 0 || n > 100 => Err(INVALID_NUMBER_INPUT_ERROR),
            n if n % 15 == 0 => Ok(FizzBuzzResult::FizzBuzz),
            n if n % 3 == 0 => Ok(FizzBuzzResult::Fizz),
            n if n % 5 == 0 => Ok(FizzBuzzResult::Buzz),
            _ => Ok(FizzBuzzResult::Number(num)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCaseBuilder {
        step: usize,
        start: u8,
        end: u8,
        filter: Option<Box<dyn Fn(u8) -> bool>>,
    }

    impl TestCaseBuilder {
        fn new() -> Self {
            TestCaseBuilder {
                step: 1,
                start: 1,
                end: 100,
                filter: None,
            }
        }

        fn with_step_by(mut self, step: usize) -> Self {
            self.step = step;
            self
        }

        fn with_range(mut self, start: u8, end: u8) -> Self {
            self.start = start;
            self.end = end;
            self
        }

        fn with_filter(mut self, filter: impl Fn(u8) -> bool + 'static) -> Self {
            self.filter = Some(Box::new(filter));
            self
        }

        fn build(self) -> Vec<u8> {
            let mut test_cases: Vec<u8> = (self.start..=self.end)
                .step_by(self.step)
                .collect();

            if let Some(filter_fn) = self.filter {
                test_cases.retain(|&n| filter_fn(n));
            }

            test_cases
        }
    }

    #[test]
    fn test_a_multiple_of_three_which_is_not_a_multiple_of_15_should_be_fizz() {
        let test_cases = TestCaseBuilder::new()
            .with_step_by(3)
            .with_range(3, 100)
            .with_filter(|n| n % 15 != 0 && n % 5 != 0)
            .build();

        test_cases
            .iter()
            .for_each(|&num| assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(num), Ok(FizzBuzzResult::Fizz)));
    }

    #[test]
    fn test_a_multiple_of_five_which_is_not_a_multiple_of_15_should_be_buzz() {
        let test_cases = TestCaseBuilder::new()
            .with_step_by(5)
            .with_range(5, 100)
            .with_filter(|n| n % 15 != 0)
            .build();

        test_cases
            .iter()
            .for_each(|&num| assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(num), Ok(FizzBuzzResult::Buzz)));
    }

    #[test]
    fn test_a_multiple_of_fifteen_should_be_fizzbuzz() {
        let test_cases = TestCaseBuilder::new()
            .with_step_by(15)
            .with_range(15, 100)
            .build();

        test_cases
            .iter()
            .for_each(|&num| assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(num), Ok(FizzBuzzResult::FizzBuzz)));
    }

    #[test]
    fn test_a_number_not_a_multiple_of_three_or_five_should_be_the_number_given() {
        let test_cases = TestCaseBuilder::new()
            .with_step_by(1)
            .with_range(1, 100)
            .with_filter(|n| n % 3 != 0 && n % 5 != 0)
            .build();

        test_cases
            .iter()
            .for_each(|&num| assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(num), Ok(FizzBuzzResult::Number(num))));
    }

    #[test]
    fn it_should_support_graceful_error_handling() {
        let test_cases = TestCaseBuilder::new()
            .with_step_by(1)
            .with_range(1, 100)
            .build();

        test_cases
            .iter()
            .for_each(|&num| assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(num).is_ok(), true));
    }

    #[test]
    fn test_the_number_zero_and_above_hundred_are_not_allowed() {
        let test_cases = vec![0, 101, 150, 255];

        test_cases
            .iter()
            .for_each(|&num| assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(num), Err(INVALID_NUMBER_INPUT_ERROR)));
    }

    #[test]
    fn test_fizzbuzz_enum_performance_win_over_string() {
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(3), Ok(FizzBuzzResult::Fizz));
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(7), Ok(FizzBuzzResult::Number(7)));
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(5), Ok(FizzBuzzResult::Buzz));
        assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(15), Ok(FizzBuzzResult::FizzBuzz));
        assert!(FizzBuzzChecker::is_a_fizzbuzz(101).is_err());
    }
}
