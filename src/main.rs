mod eratos;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: u32 = rust_eratosthenes::parse_args(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if eratos::is_prime_number(n) {
        println!("{} is a prime number.", n);
    } else {
        println!("{} is not a prime number.", n);
    }

    if eratos::has_prime_number_below(n) {
        let prime_number_count: usize = eratos::get_prime_number_count_below(n);
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
            eratos::get_largest_prime_number_below(n)
        );
        let prime_numbers: Vec<u32> = eratos::get_prime_numbers_below(n);
        println!("Prime numbers less than {}.", n);

        if let Err(e) = rust_eratosthenes::print_vec(&prime_numbers) {
            println!("Application error: {}", e);
            process::exit(1);
        };
    } else {
        println!("There is no prime number less than {}.", n);
    }
}
