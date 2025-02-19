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

fn main() {
    let args = Args::parse();

    if !args.discrete {
        println!(
            "{}",
            r#"
            _
                         | |
                         | |===( )   //////
                         |_|   |||  | o o|
                                ||| ( c  )                  ____
                                 ||| \= /                  ||   \_
                                  ||||||                   ||     |
                                  ||||||                ...||__/|-"
                                  ||||||             __|________|__
                                    |||             |______________|
                                    |||             || ||      || ||
                                    |||             || ||      || ||
            ------------------------|||-------------||-||------||-||-------
                                    |__>            || ||      || ||


                 hit enter to send

            "#
        );
    }
    loop {
        let prompt = if args.discrete {
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to read line or you just entered an empty string");
            input.trim().to_string()
        } else {
            print!("{}", "Enter your prompt: ".bright_blue());
            io::stdout().flush().unwrap();
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("failed to get the input");
            input.trim().to_string()
        };

        if prompt == "qui" {
            println!("{}", "またね bye bye".bright_red());
            break;
        }

        match send_request(&prompt) {
            Ok(response) => println!("{}", response.bright_yellow()),
            Err(e) => println!("{}:{}", "Error:".bright_red(), e),
        }
    }
}

fn send_request(prompt: &str) -> Result<String, reqwest::Error> {
    let url = format!("https://gemicli.vercel.app/gemini?prompt={}", prompt);
    let response = reqwest::blocking::get(&url)?;
    let response_text = response.text()?;
    Ok(response_text)
}
