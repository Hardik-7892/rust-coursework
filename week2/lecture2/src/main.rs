fn update (s:&mut String){
    s.push_str("!");
    println!("stringin function is \"{}\"",s);
}
fn main() {
    // println!("Hello, world!");
    let mut s :String = String::from("Hello");
    println!("Initial string is \"{}\"",s);
    update(&mut s);
    println!("string after function is \"{}\"",s);
}
