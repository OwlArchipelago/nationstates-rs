use nationstates::{NSClient, NSError};
use std::io;

#[tokio::main]
async fn main() -> Result<(), NSError> {
    let mut client = NSClient::new("Owl Archipelago's API test")?;
    println!("Enter your nation name:");

    let mut nation = String::new();
    io::stdin()
        .read_line(&mut nation)
        .expect("Failed to read line");

    println!("Enter the code from https://www.nationstates.net/page=verify_login:");
    let mut checksum = String::new();
    io::stdin()
        .read_line(&mut checksum)
        .expect("Failed to read line");

    if client.verify(nation.trim(), checksum.trim()).await? {
        println!("Verified successfully!");
    } else {
        println!("Failed to verify!");
    }

    Ok(())
}
