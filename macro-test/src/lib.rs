use sava_chain::*;
use sava_chain_macros::chaining;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum SomeError {
    #[error("Error1")]
    Error1,
    #[error("Error2")]
    Error2,
}

#[derive(PartialEq, Debug)]
pub struct SomeData(pub String);

fn sanitization_one(input: &SomeData) -> SomeData {
    SomeData(input.0.trim().to_owned())
}

fn validation_one(input: &SomeData) -> bool {
    input.0.starts_with("a")
}

fn validation_two(input: &SomeData) -> bool {
    input.0.ends_with("b")
}

#[derive(Error, Debug, PartialEq)]
pub enum ToError {
    #[error("Error1")]
    Error1(#[from] SomeError),
}

pub struct SomeDataValidator;

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

#[derive(Default, Debug, PartialEq)]
pub struct ToValidate {
    my_data: String,
    my_data2: String,
}

chaining! {
    (ToValidate, ToError) => MyValidator: [
        (|struct_data| SomeData(struct_data.my_data.clone()), |struct_data, SomeData(data)| struct_data.my_data = data, SomeDataValidator),
        (|struct_data| SomeData(struct_data.my_data2.clone()), |struct_data, SomeData(data)| struct_data.my_data2 = data, SomeDataValidator),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_error() {
        let to_validate = ToValidate {
            my_data: String::from("a   "),
            my_data2: String::from("   b   "),
        };

        let result = MyValidator::execute(to_validate);

        assert_eq!(result.unwrap_err(), ToError::Error1(SomeError::Error2))
    }

    #[test]
    fn basic_ok() {
        let to_validate = ToValidate {
            my_data: String::from("ab   "),
            my_data2: String::from(" a c b   "),
        };

        let result = MyValidator::execute(to_validate);

        assert_eq!(
            result.unwrap(),
            ToValidate {
                my_data: String::from("ab"),
                my_data2: String::from("a c b"),
            }
        )
    }
}
