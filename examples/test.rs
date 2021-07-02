use nationstates::NSAPIClient;

#[tokio::main]
pub async fn main() -> Result<(), ()> {
    let client = NSAPIClient::new("Owl Archipelago's API Test").unwrap();
    let nation = client.get_nation("Owl Archipelago").await.unwrap();

    println!("{}", nation.ntype);
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
