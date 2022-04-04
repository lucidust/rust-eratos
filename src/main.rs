use std::error::Error;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u32 = parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if rust_eratos::is_prime_number(n) {
        println!("{} is a prime number.", n);
    } else {
        println!("{} is not a prime number.", n);
    }

    if rust_eratos::has_prime_number_below(n) {
        let prime_number_count: usize = rust_eratos::get_prime_number_count_below(n);
        match prime_number_count {
            1 => print!(
                "There is {} prime number less than {},",
                prime_number_count, n
            ),
            _ => print!(
                "There are {} prime numbers less than {},",
                prime_number_count, n
            ),
        }

        println!(
            " and the largest number is {}.",
            rust_eratos::get_largest_prime_number_below(n)
        );
        let prime_numbers: Vec<u32> = rust_eratos::get_prime_numbers_below(n);
        println!("Prime numbers less than {}.", n);

        if let Err(e) = print_vec(&prime_numbers) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        };
    } else {
        println!("There is no prime number less than {}.", n);
    }
}

fn parse_args(args: &[String]) -> Result<u32, &'static str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }

    let arg1: String = String::from(&args[1]);

    Ok(arg1.parse::<u32>().unwrap())
}

fn print_vec(vec: &Vec<u32>) -> Result<(), Box<dyn Error>> {
    for index in 0..vec.len() {
        print!("[{:^4}], ", vec[index]);

        if (index + 1) % 10 == 0 {
            println!();
        }
    }

    println!();

    Ok(())
}
