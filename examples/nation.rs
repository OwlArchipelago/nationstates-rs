use nationstates::{NSClient, NSError};

#[tokio::main]
pub async fn main() -> Result<(), NSError> {
    let mut client = NSClient::new("Owl Archipelago's API Test")?;
    let nation = client.get_nation("Owl Archipelago").await?;

    println!("{}", nation.name);
    println!("{}", nation.fullname);
    println!("{}", nation.motto);

    if nation.in_wa() {
        println!("WA Member");
    }

    println!("{} - {}", nation.founded, nation.firstlogin.to_string());

    println!();
    println!("Economic Freedom: {}", nation.freedom.economy);
    println!();

    for cause in nation.deaths.causes {
        println!("{} - {}", cause.reason, cause.percentage);
    }

    println!();
    println!("Endo Count: {}", nation.endorsements.count());
    for endo in nation.endorsements.get() {
        println!("{}", endo);
    }

    Ok(())
}
