use clap::Parser;

#[derive(Parser,Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, default_value_t = 100)]
    size: u32,

    #[arg(short = 'F', long = "file", default_value = "No file attached!")]
    file: String,

    #[arg(short = 'Q', long, default_value_t = false)]
    quiet: bool,
}

fn main() {
    let args = Args::parse();
    println!("memory vector usage: {}", args.size);
    if args.quiet {
        println!("shush");
    } else {
        println!("AAAAAHHHHH!!!");
    }
    if args.file!= "No file attached!" {
        println!("File name is {}", args.file);
    }
    else {
        println!("{}", args.file)
    }
}
