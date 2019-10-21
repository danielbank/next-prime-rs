use clap::{App, Arg};
use primes::PrimeSet;

fn next_prime(x: u64) -> u64 {
    let mut pset = PrimeSet::new();
    let (_ix, n) = pset.find(x);
    n
}

fn main() {
    let matches = App::new("next-prime")
        .version("1.0")
        .about("Calculate the next prime after a given integer")
        .author("Daniel Bank")
        .arg(
            Arg::with_name("number")
                .help("a number after which the next prime will occur")
                .index(1)
                .required(true),
        )
        .get_matches();

    if matches.is_present("input") {
        println!("An input file was specified");
    }

    if let Some(ref number) = matches.value_of("number") {
        match number.parse::<u64>() {
            Ok(n) => {
                let foo = next_prime(n + 1);
                println!("The next prime after {} is {:?}", number, foo);
            }
            Err(_e) => println!("Failed to find next prime after {}", number),
        }
    }
}
