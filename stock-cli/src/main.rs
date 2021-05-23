use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut company: String = "AAPL".to_string();

    if args.len() < 2 {
        println!("Since you didn't specify a company symbol, it is defaulted to AAPL.");
    } else {
        company = args[1].clone();
    }

    println!("{:?}", company);
}
