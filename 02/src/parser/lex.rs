use std::str::FromStr;

pub fn tokenize(input: &str) -> Vec<usize> {
    let str_tokens: Vec<&str> = input.split(",").collect();
    let mut tokens: Vec<usize> = vec!();
    for token in str_tokens {
        tokens.push(usize::from_str(token).unwrap())
    }

    tokens
}