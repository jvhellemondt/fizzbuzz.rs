fn main() {
    println!("Hello, world!");
}

struct FizzBuzzChecker {}

impl FizzBuzzChecker {
    fn is_a_fizzbuzz(num: u8) -> String {
        match num {
            n if n % 15 == 0 => "FizzBuzz".to_string(),
            n if n % 3 == 0 => "Fizz".to_string(),
            n if n % 5 == 0 => "Buzz".to_string(),
            _ => num.to_string(),
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
            .for_each(|&num| assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(num), "Fizz"));
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
            .for_each(|&num| assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(num), "Buzz"));
    }

    #[test]
    fn test_a_multiple_of_fifteen_should_be_fizzbuzz() {
        let test_cases = TestCaseBuilder::new()
            .with_step_by(15)
            .with_range(15, 100)
            .build();

        test_cases
            .iter()
            .for_each(|&num| assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(num), "FizzBuzz"));
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
            .for_each(|&num| assert_eq!(FizzBuzzChecker::is_a_fizzbuzz(num), num.to_string()));
    }
}
