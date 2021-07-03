use nationstates::{NSClient, NSError};

#[tokio::main]
pub async fn main() -> Result<(), NSError> {
    let client = NSClient::new("Owl Archipelago's API Test")?;
    let region = client.get_region("Owl Archipelago").await?;

    println!("{}", region.name);
    println!("{}", nation.fullname);
    println!("{}", nation.motto);

    if nation.in_wa() {
        println!("WA Member");
    }

    if !nation.endorsements.is_empty() {
        println!("Endorsements: ");
        for endorsement in nation.endorsements {
            println!("{}", endorsement);
        }
    }

    Ok(())
}
