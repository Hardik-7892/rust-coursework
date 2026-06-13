use std::fs;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // println!("{}", &args[0]); // This prints the executable file name
    let file_name = &args[1];
    let file_path = Path::new(file_name);
    // let file_path = Path::new("foo.txt"); // We use the above commands to pass foo.txt as arguments
    let content = fs::read_to_string(file_path)
        .expect("unable to read input file");
    // println!("The content of the file is:\n{}", content );
    let mut happy: u16 = 0;
    let mut sad: u16 = 0;
    // using for loop
    /* 
    for emoji in content.chars() {
        if emoji == '\u{1F600}' {
            happy +=1;
        }
        else if emoji == '\u{1f641}'{
            sad +=1;
        }
    }
    */

    // using match
    for c in content.chars(){
        match c{
            '\u{1F600}' => happy +=1,
            '\u{1f641}' => sad +=1,
            // _ => println!("not happy or sad emoji")
            _ => ()
            // _ => todo!()  // equivalent to default case
        };
    }
    if happy >sad {
        println!("HAPPY");
    }
    else if sad > happy{
        println!("SAD");
    }
    else {
        println!("OK");
    }
}