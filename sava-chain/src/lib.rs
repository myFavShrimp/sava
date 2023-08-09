use std::error::Error;

pub type SanitizerFn<T> = fn(&T) -> T;
pub type ValidatorFn<T> = fn(&T) -> bool;

pub type FieldExtractorFn<T, R> = fn(&T) -> R;
pub type FieldCombinatorFn<R, T> = fn(&mut T, R);

// #[cfg(test)]
// mod dev_tests {
//     use thiserror::Error;

//     use super::*;

//     pub trait Net2
//     where
//         Self::G: From<Self::E1> + From<Self::E2>,
//         Self::S1: Chain<Type = Self::T1, Error = Self::E1>,
//         Self::E1: std::error::Error,
//         Self::S2: Chain<Type = Self::T2, Error = Self::E2>,
//         Self::E2: std::error::Error,
//     {
//         type I;
//         type G;
//         type S1;
//         type T1;
//         type E1;
//         type S2;
//         type T2;
//         type E2;

//         fn net() -> (
//             (
//                 FieldExtractorFn<Self::I, Self::T1>,
//                 FieldCombinatorFn<Self::T1, Self::I>,
//             ),
//             (
//                 FieldExtractorFn<Self::I, Self::T2>,
//                 FieldCombinatorFn<Self::T2, Self::I>,
//             ),
//         );

//         fn execute(input: Self::I) -> Result<Self::I, Self::G> {
//             let mut net_data = input;
//             let sava_net = Self::net();

//             let extracted_field = sava_net.0 .0(&net_data);
//             let chain_result = Self::S1::execute(extracted_field)?;
//             sava_net.0 .1(&mut net_data, chain_result);

//             let extracted_field = sava_net.1 .0(&net_data);
//             let chain_result = Self::S2::execute(extracted_field)?;
//             sava_net.1 .1(&mut net_data, chain_result);

//             Ok(net_data)
//         }
//     }

//     #[derive(Error, Debug, PartialEq)]
//     enum ChainError {
//         #[error("Error1")]
//         Error1,
//     }

//     #[derive(Error, Debug, PartialEq)]
//     enum Chain2Error {
//         #[error("Error1")]
//         Error1,
//     }

//     #[derive(Error, Debug, PartialEq)]
//     enum NetError {
//         #[error("Error1 - {0}")]
//         Error1(#[from] ChainError),
//         #[error("Error2 - {0}")]
//         Error2(#[from] Chain2Error),
//     }

//     fn sanitize_string(input: &String) -> String {
//         input.trim().to_owned()
//     }
//     fn validate_email(input: &String) -> bool {
//         input.contains("@")
//     }

//     fn validate_i8(input: &i8) -> bool {
//         input > &16
//     }

//     struct Chain1;
//     impl Chain for Chain1 {
//         type Type = String;
//         type Error = ChainError;

//         fn chain() -> Vec<Link<String, ChainError>> {
//             vec![
//                 Link::Sa(sanitize_string),
//                 Link::Va(validate_email, ChainError::Error1),
//             ]
//         }
//     }

//     struct Chain2;
//     impl Chain for Chain2 {
//         type Type = i8;
//         type Error = Chain2Error;

//         fn chain() -> Vec<Link<i8, Chain2Error>> {
//             vec![Link::Va(validate_i8, Chain2Error::Error1)]
//         }
//     }

//     struct NetDataValidator;
//     impl Net2 for NetDataValidator {
//         type I = NetData;
//         type G = NetError;
//         type S1 = Chain1;
//         type S2 = Chain2;

//         type T1 = <Chain1 as Chain>::Type;
//         type E1 = <Chain1 as Chain>::Error;
//         type T2 = <Chain2 as Chain>::Type;
//         type E2 = <Chain2 as Chain>::Error;

//         fn net() -> (
//             (
//                 FieldExtractorFn<Self::I, Self::T1>,
//                 FieldCombinatorFn<Self::T1, Self::I>,
//             ),
//             (
//                 FieldExtractorFn<Self::I, Self::T2>,
//                 FieldCombinatorFn<Self::T2, Self::I>,
//             ),
//         ) {
//             (
//                 (
//                     |item| item.field1.clone(),
//                     |item, field_data| item.field1 = field_data,
//                 ),
//                 (
//                     |item| item.field2.clone(),
//                     |item, field_data| item.field2 = field_data,
//                 ),
//             )
//         }
//     }

//     #[derive(PartialEq, Debug)]
//     struct NetData {
//         field1: String,
//         field2: i8,
//     }

//     #[test]
//     fn run_test() {
//         let data = NetData {
//             field1: String::from("  Test@Test  "),
//             field2: 32,
//         };

//         let result = NetDataValidator::execute(data);

//         assert_eq!(
//             result.unwrap(),
//             NetData {
//                 field1: String::from("Test@Test"),
//                 field2: 32
//             }
//         )
//     }
// }

// pub trait Net<I, G, S1, T1, E1>
// where
//     G: From<E1>,
//     S1: Chain<T1, E1>,
//     E1: std::error::Error,
// {
//     fn net() -> ((FieldExtractorFn<I, T1>, FieldCombinatorFn<T1, I>, S1),);

//     fn execute(input: I) -> Result<I, G> {
//         let mut net_data = input;
//         let sava_net = Self::net();

//         let extracted_field = sava_net.0 .0(&net_data);
//         let chain_result = S1::execute(extracted_field)?;
//         sava_net.0 .1(&mut net_data, chain_result);

//         Ok(net_data)
//     }
// }

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
