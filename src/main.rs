use rust_eratos::core;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u32 = rust_eratos::parse_args(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if core::is_prime_number(n) {
        println!("{} is a prime number.", n);
    } else {
        println!("{} is not a prime number.", n);
    }

    if core::has_prime_number_below(n) {
        let prime_number_count: usize = core::get_prime_number_count_below(n);
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
            core::get_largest_prime_number_below(n)
        );
        let prime_numbers: Vec<u32> = core::get_prime_numbers_below(n);
        println!("Prime numbers less than {}.", n);

        if let Err(e) = rust_eratos::print_vec(&prime_numbers) {
            eprintln!("Application error: {}", e);
            process::exit(1);
        };
    } else {
        println!("There is no prime number less than {}.", n);
    }
}
