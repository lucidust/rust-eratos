fn main() {
    let n: u32 = 100;
    let primes: Vec<u32> = get_primes_below(n);
    print_vec(&primes);
}

fn get_primes_below(n: u32) -> Vec<u32> {
    let mut sieve: Vec<u32> = create_sieve(n);
    let max_check_index: usize = (sieve.len() as f32).sqrt() as usize;
    // println!("Max check index: {}", max_check_index);
    // print_vec(&sieve);

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

    let primes: Vec<u32> = sieve.into_iter().filter(|&element| element > 0).collect();

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
