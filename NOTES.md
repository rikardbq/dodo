### Arguments struct
- **This will become its own crate later**
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




### filter idea
    parse filter from left to right
    "simplest complex" case (name:task1 or name:task2) and tag:test
        parse each char, look for starter "(" and ender ")", keep chars inbetween
    store the string in struct
    split string on " or " if contains "or"
    split string on " and " if contains "and"
    filter tasks on split elements afterwards
    continue parsing string, encounter "and" keyword
    split on " and " and keep right side
    filter remaining tasks on right side filter

    ---
    
    Use a linked list
    parse filter string as nodes where each node is initiated by "(" and terminated by its corresponding ")"
    in the case of "(name:task1 or name:task2) and tag:test"

    {
        node: {
            str: "name:task1 or name:task2",
            node: {
                str: "and tag:test",
                node: null
            }
        }
    }

    traverse the tree and the order of precedence becomes natural
    higher nodes carry higher precedence
    parse each node str and convert to whatever logical expression they represent
