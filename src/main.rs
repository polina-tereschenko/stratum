#![allow(non_snake_case)]

mod inspect;
mod pull;
mod mount;
mod help;

#[tokio::main]
async fn main() {
    let mut args = std::env::args().skip(1);

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "-p" | "--pull" => pull::handle_pull(&mut args),
            "-m" | "--mount" => mount::handle_mount(),
            "--help" => help::handle_help(),
            "inspect" => inspect::handle_inspect(),
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
