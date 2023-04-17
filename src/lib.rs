use std::error::Error;

pub type SanitizerFn<T> = fn(&T) -> T;
pub type ValidatorFn<T> = fn(&T) -> bool;

pub enum Link<T, E>
where E: Error,
{
    Sa(SanitizerFn<T>),
    Va(ValidatorFn<T>, E),
}

pub trait Chain<T, E>
where E: Error,
{
    fn chain() -> Vec<Link<T, E>>;

    fn execute(input: T) -> Result<T, E> {
        let mut chain_data = input;

        for link in Self::chain() {
            match link {
                Link::Sa(sanitizer) => chain_data = sanitizer(&chain_data),
                Link::Va(validator, error) => match validator(&chain_data) {
                    true => {},
                    false => return Err(error),
                },
            }
        }

        Ok(chain_data)
    }
}

#[cfg(test)]
mod tests {
    use thiserror::Error;

    use super::*;

    #[derive(Error, Debug, PartialEq)]
    enum SomeError {
        #[error("Error1")]
        Error1,
        #[error("Error2")]
        Error2,
    }

    #[derive(PartialEq, Debug)]
    struct SomeData(pub String);

    fn sanitization_one(input: &SomeData) -> SomeData {
        SomeData(input.0.trim().to_owned())
    }

    fn validation_one(input: &SomeData) -> bool {
        input.0.starts_with("a")
    }

    fn validation_two(input: &SomeData) -> bool {
        input.0.ends_with("b")
    }

    struct SomeDataValidator;

    impl Chain<SomeData, SomeError> for SomeDataValidator {
        fn chain() -> Vec<Link<SomeData, SomeError>> {
            vec![
                Link::Sa(sanitization_one),
                Link::Va(validation_one, SomeError::Error1),
                Link::Va(validation_two, SomeError::Error2),
            ]
        }
    }

    #[test]
    fn simple_chain() {
        let test_data = SomeData("  a nice string b  ".to_owned());

        let sava_result = SomeDataValidator::execute(test_data);

        assert_eq!(sava_result.unwrap(), SomeData("a nice string b".to_owned()));
    }

    #[test]
    fn chain_error_one() {
        let test_data = SomeData("  b nice string b  ".to_owned());

        let sava_result = SomeDataValidator::execute(test_data);

        assert_eq!(sava_result.unwrap_err(), SomeError::Error1);
    }

    #[test]
    fn chain_error_two() {
        let test_data = SomeData("  a nice string a  ".to_owned());

        let sava_result = SomeDataValidator::execute(test_data);

        assert_eq!(sava_result.unwrap_err(), SomeError::Error2);
    }
    
    #[test]
    fn input_same_as_sanitized() {
        let test_data = SomeData("a nice string b".to_owned());

        let sava_result = SomeDataValidator::execute(test_data);

        assert_eq!(sava_result.unwrap(), SomeData("a nice string b".to_owned()));
    }
}
