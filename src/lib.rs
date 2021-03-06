//! # rust_eratos
//!
//! `rust_eratos` is one of the Sieve of Eratosthenes implementation for rust language practice.

fn get_upper_range_n(n: u32) -> u32 {
    (n as f32).sqrt() as u32 + 1
}

/// Check has prime number below the number given.
///
/// # Examples
/// ```
/// assert_eq!(rust_eratos::has_prime_number_below(0), false);
/// assert_eq!(rust_eratos::has_prime_number_below(1), false);
/// assert_eq!(rust_eratos::has_prime_number_below(2), false);
/// assert_eq!(rust_eratos::has_prime_number_below(3), true);
/// ```
pub fn has_prime_number_below(n: u32) -> bool {
    n > 2
}

/// Check is prime the number given.
///
/// # Examples
/// ```
/// assert_eq!(rust_eratos::is_prime_number(2), true);
/// assert_eq!(rust_eratos::is_prime_number(3), true);
/// assert_eq!(rust_eratos::is_prime_number(11), true);
/// assert_eq!(rust_eratos::is_prime_number(12), false);
/// ```
pub fn is_prime_number(n: u32) -> bool {
    match n {
        0..=1 => false,
        _ => !(2..get_upper_range_n(n)).any(|i| n % i == 0),
    }
}

/// Get prime number count below the number given.
///
/// # Examples
/// ```
/// assert_eq!(rust_eratos::get_prime_number_count_below(2), 0);
/// assert_eq!(rust_eratos::get_prime_number_count_below(3), 1);
/// assert_eq!(rust_eratos::get_prime_number_count_below(11), 4);
/// assert_eq!(rust_eratos::get_prime_number_count_below(12), 5);
/// ```
pub fn get_prime_number_count_below(n: u32) -> usize {
    match n {
        0..=2 => 0,
        _ => (2..n).filter(|i| is_prime_number(*i)).count(),
    }
}

/// Get largest a prime number below the number given.
///
/// # Examples
/// ```
/// assert_eq!(rust_eratos::get_largest_prime_number_below(2), 0);
/// assert_eq!(rust_eratos::get_largest_prime_number_below(3), 2);
/// assert_eq!(rust_eratos::get_largest_prime_number_below(11), 7);
/// assert_eq!(rust_eratos::get_largest_prime_number_below(12), 11);
/// ```
pub fn get_largest_prime_number_below(n: u32) -> u32 {
    match (2..n).rev().find(|i| is_prime_number(*i)) {
        Some(i) => i,
        None => 0,
    }
}

/// Get prime numbers below the number given.
///
/// # Examples
/// ```
/// assert_eq!(rust_eratos::get_prime_numbers_below(2), vec![]);
/// assert_eq!(rust_eratos::get_prime_numbers_below(3), vec![2]);
/// assert_eq!(rust_eratos::get_prime_numbers_below(11), vec![2, 3, 5, 7]);
/// assert_eq!(rust_eratos::get_prime_numbers_below(12), vec![2, 3, 5, 7, 11]);
/// ```
pub fn get_prime_numbers_below(n: u32) -> Vec<u32> {
    let upper_range_n: u32 = get_upper_range_n(n);
    let mut sieve: Vec<u32> = vec![0, 0];
    sieve.append(&mut (2..n).collect());

    for (i, _) in (0..upper_range_n).enumerate() {
        if sieve[i] <= 0 {
            continue;
        }

        let mut next_index = i * i;

        while next_index < sieve.len() {
            sieve[next_index] = 0;
            next_index += i;
        }
    }

    sieve.into_iter().filter(|&element| element > 0).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_prime() {
        assert_eq!(has_prime_number_below(0), false);
        assert_eq!(has_prime_number_below(1), false);
        assert_eq!(has_prime_number_below(2), false);
        assert_eq!(has_prime_number_below(3), true);
        assert_eq!(has_prime_number_below(4), true);
        assert_eq!(has_prime_number_below(5), true);
    }

    #[test]
    fn is_prime() {
        assert_eq!(is_prime_number(0), false);
        assert_eq!(is_prime_number(1), false);
        assert_eq!(is_prime_number(2), true);
        assert_eq!(is_prime_number(3), true);
        assert_eq!(is_prime_number(4), false);
        assert_eq!(is_prime_number(5), true);
        assert_eq!(is_prime_number(6), false);
        assert_eq!(is_prime_number(7), true);
        assert_eq!(is_prime_number(8), false);
        assert_eq!(is_prime_number(9), false);
        assert_eq!(is_prime_number(10), false);
        assert_eq!(is_prime_number(11), true);
        assert_eq!(is_prime_number(12), false);
    }

    #[test]
    fn prime_count_below() {
        assert_eq!(get_prime_number_count_below(0), 0);
        assert_eq!(get_prime_number_count_below(1), 0);
        assert_eq!(get_prime_number_count_below(2), 0);
        assert_eq!(get_prime_number_count_below(3), 1);
        assert_eq!(get_prime_number_count_below(4), 2);
        assert_eq!(get_prime_number_count_below(5), 2);
        assert_eq!(get_prime_number_count_below(6), 3);
        assert_eq!(get_prime_number_count_below(7), 3);
        assert_eq!(get_prime_number_count_below(8), 4);
        assert_eq!(get_prime_number_count_below(9), 4);
        assert_eq!(get_prime_number_count_below(10), 4);
        assert_eq!(get_prime_number_count_below(11), 4);
        assert_eq!(get_prime_number_count_below(12), 5);
        assert_eq!(get_prime_numbers_below(12).len(), 5);
    }

    #[test]
    fn largest_prime_below() {
        assert_eq!(get_largest_prime_number_below(0), 0);
        assert_eq!(get_largest_prime_number_below(1), 0);
        assert_eq!(get_largest_prime_number_below(2), 0);
        assert_eq!(get_largest_prime_number_below(3), 2);
        assert_eq!(get_largest_prime_number_below(4), 3);
        assert_eq!(get_largest_prime_number_below(5), 3);
        assert_eq!(get_largest_prime_number_below(6), 5);
        assert_eq!(get_largest_prime_number_below(7), 5);
        assert_eq!(get_largest_prime_number_below(8), 7);
        assert_eq!(get_largest_prime_number_below(9), 7);
        assert_eq!(get_largest_prime_number_below(10), 7);
        assert_eq!(get_largest_prime_number_below(11), 7);
        assert_eq!(get_largest_prime_number_below(12), 11);

        assert_eq!(get_prime_numbers_below(12).contains(&11), true);
    }

    #[test]
    fn primes_below() {
        assert_eq!(get_prime_numbers_below(0), vec![]);
        assert_eq!(get_prime_numbers_below(1), vec![]);
        assert_eq!(get_prime_numbers_below(2), vec![]);
        assert_eq!(get_prime_numbers_below(3), vec![2]);
        assert_eq!(get_prime_numbers_below(4), vec![2, 3]);
        assert_eq!(get_prime_numbers_below(5), vec![2, 3]);
        assert_eq!(get_prime_numbers_below(6), vec![2, 3, 5]);
        assert_eq!(get_prime_numbers_below(7), vec![2, 3, 5]);
        assert_eq!(get_prime_numbers_below(8), vec![2, 3, 5, 7]);
        assert_eq!(get_prime_numbers_below(9), vec![2, 3, 5, 7]);
        assert_eq!(get_prime_numbers_below(10), vec![2, 3, 5, 7]);
        assert_eq!(get_prime_numbers_below(11), vec![2, 3, 5, 7]);
        assert_eq!(get_prime_numbers_below(12), vec![2, 3, 5, 7, 11]);
    }
}
