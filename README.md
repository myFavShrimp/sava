# 🧬 sava-chain

sava-chain is a Rust crate for building customizable data validation and sanitization chains, using a simple and composable API.

## 🌟 Features

- Define custom data validation and sanitization chains using reusable links
- Customize validation error types with Rust's `Error` trait
- Simple and intuitive API for defining chains and executing them on input data

## 🚀 Getting Started

Define your custom error and your data validation and sanitization links:

```rust
#[derive(thiserror::Error, Debug)]
enum MyError {
    #[error("my custom error")]
    CustomError,
}

fn sanitize_string(input: &String) -> String {
    input.trim().to_owned()
}

fn validate_email(input: &String) -> bool {
    input.contains("@")
}
```

Create a struct for your data validation and sanitization chain, implementing the `Chain` trait:

```rust
struct MyChain;

impl Chain<String, MyError> for MyChain {
    fn chain() -> Vec<Link<String, MyError>> {
        vec![
            Link::Sa(sanitize_string),
            Link::Va(validate_email, MyError::CustomError),
        ]
    }
}
```

Execute your chain on your input data, handling any validation errors as necessary:

```rust
let input_data = "  someemail@example.com  ".to_owned();

let result = MyChain::execute(input_data);

match result {
    Ok(clean_data) => println!("Clean data: {}", clean_data),
    Err(err) => println!("Validation error: {}", err),
}
```

## 🤝 Contributing

Contributions to sava-chain are welcome and encouraged! If you would like to contribute, please open a pull request or issue on the [GitHub repository](https://github.com/myFavShrimp/sava-chain).

## 📝 License

This project is licensed under the **MIT License**.


---

Disclaimer: This README was generated using ChatGPT. The crate author acknowledges their own limitations and laziness, and advises that this README should not be solely relied upon for accuracy or completeness.
