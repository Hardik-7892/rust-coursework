fn main() {
    // Hello World
    println!("Hello, world!");
    
    // variables and types
    let n:i8 = 4;
    println!("{}",n);
    
    // Loops
    for _i in 0..n {   // warning unused variable: `i` use _i
        println!("Hello");
    }

    // Reading standard inputs
    let mut name:String = String::new();
    println!("What is your name?");
    std::io::stdin()
        .read_line(&mut name)
        .expect("input error");
    let name:String = name;
    println!("Hello {}", name);

    // More on reading standard input
    {

    }
}
