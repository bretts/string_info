/// A struct to contain different values of a decoded char
#[derive(Debug)]
pub struct Char {
    pub scalar: char,
    pub codepoint: String,
    pub binary: Vec<String>,
    pub decimal: Vec<u8>,
}
