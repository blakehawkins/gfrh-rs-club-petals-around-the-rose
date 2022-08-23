use std::io;

use anyhow::Result;
use rand::Rng;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "gfrh-rs-club-petals-around-the-rose")]
struct Opt {
    #[structopt(short = "d", long = "dice", default_value = "5")]
    dice: u16,
}


fn run() -> Result<()> {
    let opt = Opt::from_args();
    let mut dice = Vec::with_capacity(opt.dice as usize);

    let mut rng = rand::thread_rng();

    (0..opt.dice + 1).for_each(|_| dice.push(rng.gen_range(1, 7)));

    println!("You rolled {:?} - how many petals around the rose?", dice);

    // Evaluate the correct answer.
    dice.retain(|&x| x == 3 || x == 5);

    let soln = dice.into_iter().fold(0, |acc, x| {
        acc + match x {
            3 => Ok(2),
            5 => Ok(4),
            _ => Err("Unexpected retained value"),
        }.unwrap()
    });

    // Let the user try.
    println!("Please input your answer.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Failed to parse u32");

    if guess == soln {
        println!("You're right!");
    } else {
        println!("Sorry, the right answer was {}.", soln);
    }

    Ok(())
}


fn main() -> Result<()> {
    run()
}
