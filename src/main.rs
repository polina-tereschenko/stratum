#![allow(non_snake_case)]

use std::env;
mod pull;
mod mount;

#[tokio::main]
async fn main() {
    let mut args = env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "stratum -p" | "stratum -pull" | "-p" | "--pull" => pull::handle_pull(&mut args).await,
            "stratum -m" | "stratum --mount" | "-m" | "--mount" => mount::handle_mount(),
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