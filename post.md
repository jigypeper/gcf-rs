# Improving a Greatest Common Factor Calculator in Rust

Calculating the Greatest Common Factor (GCF) of two numbers is a common mathematical task. In this blog post, we'll explore how to enhance a GCF calculator written in Rust for better clarity and efficiency.

## Original Code

```rust
use std::{cmp::Ordering, fmt::Debug, io, str::FromStr};

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

fn main() {
    println!("Greatest Common Factor Calculator\n---------------------------------");
    let number_1: u32 = ask_question("What is your first number?");
    let number_2: u32 = ask_question("What is your second number?");
    let mut smaller_number: u32 = match number_1.cmp(&number_2) {
        Ordering::Less => number_1,
        Ordering::Greater => number_2,
        Ordering::Equal => number_1,
    };

    while smaller_number > 0 {
        if number_1 % smaller_number == 0 && number_2 % smaller_number == 0 {
            println!(
                "The GCF for {} and {} is {}",
                number_1, number_2, smaller_number
            );
            break;
        }

        smaller_number -= 1;
    }
}

```
## Improvements

### Prompt
The first obvious improvment is to simply the prompt, given that any user will know what they are trying to accomplish by running the program, keeping the question succinct improves user experience:
'What is your first number?' becomes 'Enter the first number: '.

### Main logic
Currently the program figures out the smaller of the 2 numbers and loops back to 0 until it hits a number divisible by both. For very large numbers, this may become resource intensive. It becomes necessary to employ Euclids Algorithm for calculating the greatest common factor:

```rust
fn calculate_gcf(mut a: u32, mut b: u32) -> u32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

```
Using Euclid means we will reach an answer quicker and no longer need to check each value. Furthermore, we can also remove the comparison of the entered numbers and hence the unnecessary imports.

Tha program then becomes:

```rust
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

```
In our previous programs, we consistently relied on a custom prompt function. However, this approach presents a challenge: the need for repetitive implementation. To address this, we are considering two alternatives:

1. Create a Utility Crate: 

- Package our prompt function into a utility crate and publish it to crates.io.  
- This option provides the benefit of having our own reusable crate. However, it may lead to namespace clutter on crates.io, especially if similar functionalities already exist.  

2. Explore Existing Crates:  

- Investigate available crates to find one that meets our prompt function requirements.  
- This approach leverages the existing Rust ecosystem, potentially saving us from reinventing the wheel.
While the first option offers a dedicated solution, it may be considered wasteful if comparable functionality is readily available. The second option encourages us to integrate seamlessly into the Rust ecosystem by adopting existing and proven solutions.

A quick search on crates.io shows that there are many existing prompt implementations. Promptuity is a good option, it even comes with a fancy theme and  
error handling/data type conversions built in.  
Our first step is to add promptuity as a dependency to our Cargo.toml file:  
```toml
[dependencies]
promptuity = "0.0.5"
```
We can now import the crates and prompts.  
```rust
use promptuity::prompts::Number;
use promptuity::themes::FancyTheme;
use promptuity::{Error, Promptuity, Term};
```
Switching over to promptuity means we can also remove our ask_question function, since this will all be handled by promptuity now. The new process is to create the terminal, add a theme, and then a mutible prompt.
```rust
let mut term = Term::default();
let mut theme = FancyTheme::default();
let mut p = Promptuity::new(&mut term, &mut theme);
```
