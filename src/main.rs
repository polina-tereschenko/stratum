#![allow(non_snake_case)]

use std::env;
mod pull;
mod mount;
mod help;

#[tokio::main]
async fn main() {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-p" | "--pull" => pull::handle_pull(&mut args).await,
            "-m" | "--mount" => mount::handle_mount(),
            "--help" => help::handle_help(),
            _ => {
                if arg.starts_with('-') {
                    println!("Unknown argument {}", arg);
                } else {
                    println!("Unknown positional argument {}", arg);
                }
            }
        }
    }
}