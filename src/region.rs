use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Region {
    pub name: String,
    pub factbook: String,
    pub numnations: u32,
    // TODO NATIONS
    pub delegate: String,
    #[serde(rename = "DELEGATEVOTES")]
    pub delegate_votes: u32,
    // DELEGATEAUTH
    pub founder: String,
    // OFFICERS
    pub power: String,
    pub flag: String,
    pub embassies: Embassies,
}

#[derive(Debug, Deserialize)]
pub struct Embassies {
    #[serde(rename = "$value")]
    embassies: Vec<Embassy>,
}

impl Embassies {
    pub fn get(&self) -> Vec<&Embassy> {
        self.embassies.iter().filter(|e| e.t.is_none()).collect()
    }
}

#[derive(Debug, Deserialize)]
pub struct Embassy {
    #[serde(rename = "type")]
    pub t: Option<String>,
    #[serde(rename = "$value")]
    pub region: String,
}
