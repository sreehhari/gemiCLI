use clap::Parser;
use colored::*;
use reqwest;
use std::io::{self, Write};

//for getting the cli arguments
#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    discrete: bool,
}
