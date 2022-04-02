pub fn parse_args(args: &[String]) -> u32 {
    let arg1: String = String::from(&args[1]);
    arg1.parse::<u32>().unwrap()
}

pub fn print_vec(vec: &Vec<u32>) {
    for index in 0..vec.len() {
        print!("[{:^4}], ", vec[index]);

        if (index + 1) % 10 == 0 {
            println!();
        }
    }

    println!();
}
