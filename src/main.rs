use std::{fmt::Debug, io, str::FromStr};

fn ask_question<T>(question: &str) -> T
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    loop {
        println!("{question}");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<T>() {
            Ok(num) => return num,
            Err(e) => println!("{:?}, please try again.", e),
        }
    }
}

fn calculate_gcf(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    println!("Greatest Common Factor Calculator\n---------------------------------");

    let number_1: u32 = ask_question("Enter the first number:");
    let number_2: u32 = ask_question("Enter the second number:");

    let gcf = calculate_gcf(number_1, number_2);

    println!("The GCF for {} and {} is {}", number_1, number_2, gcf);
}
