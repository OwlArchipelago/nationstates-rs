use nationstates::NSClient;

#[tokio::main]
pub async fn main() -> Result<(), ()> {
    let mut client = NSClient::new("Owl Archipelago's API Test").unwrap();
    let region = client.get_region("The Owligarchy").await.unwrap();

    println!("{}", region.name);
    println!("{}", region.delegate);
    println!("{}", region.factbook);

    for embassy in region.embassies.get() {
        println!("{}", embassy.region);
    }

    Ok(())
}
