use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Region {
    pub name: String,
    pub factbook: String,
    pub numnations: u32,
    // NATIONS
    pub delegate: String,
    // DELEGATE VOTES
    // DELEGATEAUTH
    // OFFICERS
    // POWER
    pub flag: String,
    // EMBASSIES
}
