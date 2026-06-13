use std::fs;
use std::path::Path;
use std::env;

fn main() {
    let mut v: Vec<i32> = Vec::new();
    v.push(0);
    let mut i: usize = 0;
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let file_path = Path::new(file_name);

    let content = fs::read_to_string(file_path)
        .expect("unable to read input file");

    let _happy_face = "\u{1F600}";
    let _sad_face = "\u{1f641}";
    let _right_arrow = "\u{27A1}\u{FE0F}";
    let _left_arrow = "\u{2B05}\u{FE0F}";
    
    let mut iter = content.chars().peekable();
    while let Some(c) = iter.next() {
        match c {
            '\u{1F600}' => {
                v[i] += 1;
                
            }
            '\u{1F641}' => {
                v[i] -= 1;
            }
            '\u{27A1}' => {
                if let Some('\u{FE0F}') = iter.peek() {
                    iter.next(); 
                    i += 1;
                    if i == v.len() {
                        v.push(0);
                    }
                }
            }
            '\u{2B05}' => {
                if let Some('\u{FE0F}') = iter.peek() {
                    iter.next(); 
                    i -= 1;
                }
            }
            _ => {}
        }
    }
    
    let mut highest_non_zero_index = 0;
    for (index, &val) in v.iter().enumerate() {
        if val != 0 {
            highest_non_zero_index = index;
        }
    }
    for (i, &val) in v.iter().enumerate() {
        println!("{} : {}", i, val);
        if i == highest_non_zero_index {
            break;
        }
    }
}
