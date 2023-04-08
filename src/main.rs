use clap::Parser;
use reqwest;

#[derive(Parser)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    domain: String,

    #[arg(short, long)]
    username: String,

    #[arg(short, long)]
    password: String,
}

fn main() -> Result<(), reqwest::Error> {
    let args = Args::parse();

    let ip = reqwest::blocking::get("https://domains.google.com/checkip")?.text()?;
    println!("{}", ip);

    let client = reqwest::blocking::Client::new();
    let update_response = client
        .get(format!(
            "https://domains.google.com/nic/update?hostname={}&myip={}",
            args.domain, ip
        ))
        .basic_auth(args.username, Some(args.password))
        .send()?
        .text()?;
    println!("{}", update_response);

    Ok(())
}
