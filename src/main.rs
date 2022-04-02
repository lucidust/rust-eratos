fn main() {
    let n: u32 = 13;

    if is_prime(n) {
        println!("{} is a prime number.", n);
    } else {
        println!("{} is not a prime number.", n);
    }

    if has_prime_below(n) {
        println!("There are prime numbers less than {}.", n);
        let prime_numbers: Vec<u32> = get_primes_below(n);
        println!("Prime numbers less than {}.", n);
        print_vec(&prime_numbers);
    } else {
        println!("There is no prime number less than {}.", n);
    }
}

fn is_prime(n: u32) -> bool {
    let is_prime: bool = if n < 2 {
        false
    } else if n < 4 {
        true
    } else {
        let max_check_n: u32 = (n as f32).sqrt().ceil() as u32 + 1;
        !(2..max_check_n).any(|i| n % i == 0)
    };

    is_prime
}

fn has_prime_below(n: u32) -> bool {
    n > 2
}

fn get_primes_below(n: u32) -> Vec<u32> {
    let mut sieve: Vec<u32> = create_sieve(n);
    let max_check_index: usize = (sieve.len() as f32).sqrt().ceil() as usize;

    for index in 0..max_check_index {
        if sieve[index] <= 0 {
            continue;
        }

        if index < 2 {
            sieve[index] = 0;
        } else {
            let mut next_index = index + index;

            while next_index < sieve.len() {
                sieve[next_index] = 0;
                next_index += index;
            }
        }
    }

    let primes: Vec<u32> = if max_check_index < 2 {
        vec![2]
    } else {
        sieve.into_iter().filter(|&element| element > 0).collect()
    };

    primes
}

fn create_sieve(n: u32) -> Vec<u32> {
    let size = n as usize;
    let mut sieve: Vec<u32> = vec![0; size];

    for index in 0..size {
        sieve[index] = index as u32;
    }

    sieve
}

fn print_vec(vec: &Vec<u32>) {
    for index in 0..vec.len() {
        print!("[{:^4}], ", vec[index]);

        if (index + 1) % 10 == 0 {
            println!();
        }
    }

    println!();
}
