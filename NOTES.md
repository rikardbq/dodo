### Arguments struct
- Derive macro for enabling reflection of the struct
- adds function `parse_args(args: Vec<String>) -> parse_args(env::args)`
    - i.e
    ```rust
    [#derive(CliArgs)]
    struct Arguments {
        hello: Option<String>,
        world: Option<usize>,
    }

    cargo run -- -hello test -world 123
    fn main() {
        let arguments = Arguments::parse_args(std::env::args);
    }
    ```
- populates the struct with args according to the struct fields data types
- 
