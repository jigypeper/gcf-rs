use promptuity::prompts::Number;
use promptuity::themes::FancyTheme;
use promptuity::{Error, Promptuity, Term};

fn calculate_gcf(mut a: isize, mut b: isize) -> isize {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() -> Result<(), Error> {
    let mut term = Term::default();
    let mut theme = FancyTheme::default();
    let mut p = Promptuity::new(&mut term, &mut theme);

    p.term().clear()?;

    p.with_intro("Greatest Common Factor Calculator").begin()?;

    let number_1 = p.prompt(Number::new("Enter the first number").with_min(0))?;
    let number_2 = p.prompt(Number::new("Enter the second number").with_min(0))?;
    let gcf = calculate_gcf(number_1, number_2);

    p.with_outro(format!(
        "The GCF for {} and {} is {}",
        number_1, number_2, gcf
    ))
    .finish()?;

    Ok(())
}
