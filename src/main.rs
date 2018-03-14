#[macro_use] extern crate failure;
extern crate rand;
#[macro_use] extern crate failure_derive;
#[macro_use] extern crate ferris_print;
#[macro_use] extern crate structopt;

use std::io;

use failure::Error;
use rand::Rng;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "gfrh-rs-club-petals-around-the-rose")]
struct Opt {
    #[structopt(short = "d", long = "dice", default_value = "5")]
    dice: u16,
}


fn run() -> Result<(), Error> {
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
        ferrisprint!("You're right!");
    } else {
        ferrisprint!("Sorry, the right answer was {}.", soln);
    }

    Ok(())
}


fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        writeln!(stderr, "caused by: {}", e.cause()).expect(errmsg);

        writeln!(stderr, "backtrace: {:?}", e.backtrace()).expect(errmsg);

        ::std::process::exit(1);
    }
}
