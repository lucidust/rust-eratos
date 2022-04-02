use std::error::Error;

pub fn parse_args(args: &[String]) -> Result<u32, &'static str> {
    if args.len() < 2 {
        return Err("not enough arguments");
    }

    let arg1: String = String::from(&args[1]);

    Ok(arg1.parse::<u32>().unwrap())
}

pub fn print_vec(vec: &Vec<u32>) -> Result<(), Box<dyn Error>> {
    for index in 0..vec.len() {
        print!("[{:^4}], ", vec[index]);

        if (index + 1) % 10 == 0 {
            println!();
        }
    }

    println!();

    Ok(())
}

pub mod core;
