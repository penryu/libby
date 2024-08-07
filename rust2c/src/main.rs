//! rust2c
//!
//! Nothing fancy in this file.
//!
//! Just the bare library invocations necessary to generate the obscenely large factorial of
//! whatever integer is passed on the command line.
//!
//! mp_int/lib.rs
//!

#![warn(clippy::pedantic)]
#![deny(clippy::all)]

mod mp_int;

use clap::{crate_name, crate_version, Parser};

use crate::mp_int::MpInt;

#[derive(Parser)]
#[command(arg_required_else_help = true)]
struct Args {
    #[arg(help = "a non-negative integer in base 10")]
    number: u64,
}

fn main() {
    let Args { number } = Args::parse();

    println!(
        "{} {} using gmp {}\n",
        crate_name!(),
        crate_version!(),
        MpInt::gmp_version()
    );

    if number > 500_000 {
        println!("This could take a while...\n");
    }

    let n = MpInt::from(number);
    let fact = n.factorial();
    println!("{number}! -> {fact}");
}
