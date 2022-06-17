extern crate unicode_normalization;
extern crate unidecode;

use regex::Regex;
use std::env;
use unicode_normalization::UnicodeNormalization;
use unidecode::unidecode;

pub fn is_cancerous(str: String) -> bool {
    str.chars().any(|c| !c.is_ascii())
}

pub fn decancer(str: String) -> String {
    let mut str = str.nfkc().collect::<String>();
    str = str.nfd().collect::<String>();
    str = unidecode(&str);
    let re = Regex::new(r"[^a-zA-Z0-9 \n.]").unwrap();
    str = re.replace_all(&str, "").to_string();
    str = str.trim().to_string();
    str
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let input_string = &args[1];
    let check_cancerous = is_cancerous(input_string.to_string());
    let output_string = decancer(input_string.to_string());
    println!("cancerous: {}\noutput: {}", check_cancerous, output_string);
}
