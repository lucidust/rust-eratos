pub fn has_prime_number_below(n: u32) -> bool {
    n > 2
}

pub fn is_prime_number(n: u32) -> bool {
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

pub fn get_prime_number_count_below(n: u32) -> usize {
    let count: usize = if n < 3 {
        0
    } else {
        (2..n).filter(|i| is_prime_number(*i)).count()
    };

    count
}

pub fn get_largest_prime_number_below(n: u32) -> u32 {
    let largest_prime: u32 = if has_prime_number_below(n) {
        match (2..n).rev().find(|i| is_prime_number(*i)) {
            Some(i) => i,
            None => 0,
        }
    } else {
        0
    };

    largest_prime
}

pub fn get_prime_numbers_below(n: u32) -> Vec<u32> {
    let mut sieve: Vec<u32> = (0..n).map(|i| i).collect();
    let max_check_index: usize = (n as f32).sqrt().ceil() as usize;

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

    let primes: Vec<u32> = if n < 2 {
        vec![]
    } else if max_check_index < 2 {
        vec![2]
    } else {
        sieve.into_iter().filter(|&element| element > 0).collect()
    };

    primes
}
