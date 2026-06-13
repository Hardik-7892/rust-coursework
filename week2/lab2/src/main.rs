struct Book {
    title: String,
    price: f64,
    got_it: bool
}


fn main() {
    // Q1
    println!("-----------------Defining a Custom Data Structure-----------------");
    {
        let a = Book{
            title: String::from("The Rust Programming Language"),
            price: 47.99,
            got_it: true,
        };
        if a.got_it{
            println!("The book \"{}\" costs {} pounds", a.title, a.price);
            // field access is NOT supported
            // println!("The book \"{a.title}\" costs {a.price} pounds");
        }
    }
    // Q2
    println!("-----------------Traversing a Vector of Books-----------------");
    {
        let mut v: Vec<Book> = Vec::new();
        v.push(Book {
            title: String::from("The Rust Programming Language"),
            price: 47.99,
            got_it: true
        });
        v.push(Book {
            title: String::from("Harry Potter and the Goblet of Fire"),
            price: 8.99,
            got_it: true
        });
        let mut sum = 0.0;
        for b in v {
            sum += b.price;
        }
        println!("The total price is {sum:.2} pounds");
    }
    // Q3
    println!("-----------------Finding IP Addresses-----------------");
    {
        // cargo add get_if_addrs
        use std::net::IpAddr;
        
        for iface in get_if_addrs::get_if_addrs().unwrap(){
            println!("{:?}", iface);
        }
        for iface in get_if_addrs::get_if_addrs().unwrap(){
            let ip = iface.addr.ip();
            if let IpAddr::V4(ip4) = ip {
                println!("{:#?}", ip4);
            }
        }
    }
    // Q4
    println!("-----------------Final Challenge-----------------");
    {
        use std::net::IpAddr;
        fn check_class (class: u8) ->char 
        {
            // println!("{}", class);
            if class<128 {
                return 'A';
            }
            else if class<192 {
                return 'B';
            }
            else if class<224 {
                return 'C';
            }
            else if class<240 {
                return 'D';
            }
            else if class<=255 {
                return 'E';
            }
            return 'F';
        }
        for iface in get_if_addrs::get_if_addrs().unwrap(){
            let ip = iface.addr.ip();
            if let IpAddr::V4(ip4) = ip {
                // println!("{:#?}", ip4);
                // check_class(ip4)
                // println!("{:#?}", ip4.octets()[0]);
                let class = check_class(ip4.octets()[0]);
                // println!("Class of {} is {}", ip4, class);
                println!("{} (class {})", ip4, class);
            }
        }
    }
}

// Cleaner approach for final

// use std::{net::IpAddr, vec};
// fn check_class (class: u8) ->char 
// {
//     // println!("{}", class);
//     if class<128 {
//         return 'a';
//     }
//     else if class<192 {
//         return 'b';
//     }
//     else if class<224 {
//         return 'c';
//     }
//     else if class<240 {
//         return 'd';
//     }
//     else if class<=255 {
//         return 'e';
//     }
//     return 'F';
// }
// fn main(){
//     let mut ips = Vec::new();

//     for iface in get_if_addrs::get_if_addrs().unwrap() {
//         if let IpAddr::V4(ip4) = iface.addr.ip() {
//             ips.push(ip4);
//         }
//     }

//     for ip in &ips {
//         println!("{:#?}", ip);
//     }

//     for ip in &ips {
//         let class = check_class(ip.octets()[0]);
//         println!("class {}", class);
//     }
// }

