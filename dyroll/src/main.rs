#![warn(clippy::pedantic)]
#![deny(clippy::all)]

use clap::Parser;
use dlopen2::wrapper::{Container, WrapperApi};

#[derive(WrapperApi)]
struct RollApi {
    roll: fn(count: u8, sides: u8) -> u16,
}

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

    let libroll: Container<RollApi> =
        unsafe { Container::load("libroll.so") }.expect("failed to load libroll library");
    let sum = libroll.roll(count, sides);
    println!("{count}d{sides} => {sum}");
}
