mod examples;

fn main() -> () {
    use std::env;
    let binary = env::var("BINARY").expect("Environment variable $BINARY must be present!");

    match binary.as_ref() {
        "guess_number" => examples::guess_number::main(),
        "println" => examples::println::main(),
        "primitives" => examples::primitives::main(),
        "custom_types" => examples::custom_types::main(),
        _ => panic!("no such binary configuration '{}'", binary),
    }
}
