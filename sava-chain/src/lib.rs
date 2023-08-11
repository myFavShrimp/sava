use std::error::Error;

pub type SanitizerFn<T> = fn(&T) -> T;
pub type ValidatorFn<T> = fn(&T) -> bool;

pub type FieldExtractorFn<T, R> = fn(&T) -> R;
pub type FieldCombinatorFn<R, T> = fn(&mut T, R);

pub enum Link<T, E>
where
    E: Error,
{
    Sa(SanitizerFn<T>),
    Va(ValidatorFn<T>, E),
}

pub trait Chain
where
    Self::Error: Error,
{
    type Type;
    type Error;

    fn chain() -> Vec<Link<Self::Type, Self::Error>>;
}

pub trait ChainExec {
    type Type;
    type Error;

    fn execute(input: Self::Type) -> Result<Self::Type, Self::Error>;
}

impl<T: Chain> ChainExec for T {
    type Type = <Self as Chain>::Type;
    type Error = <Self as Chain>::Error;

    fn execute(input: Self::Type) -> Result<Self::Type, Self::Error> {
        let mut chain_data = input;

        for link in <Self as Chain>::chain() {
            match link {
                Link::Sa(sanitizer) => chain_data = sanitizer(&chain_data),
                Link::Va(validator, error) => match validator(&chain_data) {
                    true => {}
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

    impl Chain for SomeDataValidator {
        type Type = SomeData;
        type Error = SomeError;

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
