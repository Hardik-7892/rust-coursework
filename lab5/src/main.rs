use std::path::Path;
use std::env;
use std::fs;

fn check_brackets(stack: &mut Vec<char>, ch: char) -> &'static str {
    match ch {
        '(' | '{' | '[' => {
            stack.push(ch);
            "VALID"
        }
        ')' => {
            if stack.pop() != Some('(') {
                "INVALID"
            } else {
                "VALID"
            }
        }
        '}' => {
            if stack.pop() != Some('{') {
                "INVALID"
            } else {
                "VALID"
            }
        }
        ']' => {
            if stack.pop() != Some('[') {
                "INVALID"
            } else {
                "VALID"
            }
        }
        '\n' | '\r' => {
            "VALID"
        }
        _ => { println!(); "INVALID"},
    }
}

fn main() {
    let mut stack: Vec<char> = Vec::new();
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file_path = Path::new(file_name);
    let content = fs::read_to_string(file_path)
        .expect("unable to read input file");
    let mut res = "VALID";
    for ch in content.chars(){
        res = check_brackets(&mut stack, ch);
        if res == "INVALID"{
            break;
        }
    }
    if stack.is_empty(){
        println!("{}", res);
    }
    else {println!("INVALID");}
}
