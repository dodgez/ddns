use dotenv;
use reqwest;

fn main() -> Result<(), reqwest::Error> {
    dotenv::from_filename(".env.local").expect("No environment variables found");

    let domain = std::env::var("GOOGLE_DOMAINS_DOMAIN").expect("GOOGLE_DOMAINS_DOMAIN is required");
    let username = std::env::var("GOOGLE_DOMAINS_USERNAME").expect("GOOGLE_DOMAINS_USERNAME is required");
    let password = std::env::var("GOOGLE_DOMAINS_PASSWORD").expect("GOOGLE_DOMAINS_PASSWORD is required");

    let ip = reqwest::blocking::get("https://domains.google.com/checkip")?.text()?;
    println!("{}", ip);

    let client = reqwest::blocking::Client::new();
    let update_response = client
        .get(format!(
            "https://domains.google.com/nic/update?hostname={}&myip={}",
            domain, ip
        ))
        .basic_auth(username, Some(password))
        .send()?
        .text()?;
    println!("{}", update_response);

    Ok(())
}
