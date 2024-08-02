#![warn(clippy::pedantic)]
#![deny(clippy::all)]

use clap::Parser;

use roll::roll;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about=None, arg_required_else_help=true)]
struct Args {
    #[arg(required = true)]
    count: u8,

    #[arg(required = true)]
    sides: u8,
}

fn main() {
    let Args { count, sides } = Args::parse();
    let sum = roll(count, sides);
    println!("{count}d{sides} => {sum}");
}
