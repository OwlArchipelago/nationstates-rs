use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Nation {
    pub name: String,
    #[serde(rename = "TYPE")]
    pub ntype: String,
    pub fullname: String,
    pub motto: String,
    pub category: String,
    pub unstatus: WA,
    pub endorsements: Vec<String>,
    pub issues_answered: u32,
    // FREEDOM
    pub region: String,
    // POPULATION
    pub tax: f32,
    pub animal: String,
    pub currency: String,
    // DEMONYMS
    pub flag: String,
    // Industry/Gov stuff
    pub founded: String,
    pub leader: String,
    pub capital: String,
    pub religion: String,
}

impl Nation {
    pub fn in_wa(&self) -> bool {
        self.unstatus == WA::Delegate || self.unstatus == WA::NonMember
    }
}

#[derive(Debug, PartialEq)]
pub enum WA {
    Delegate,
    Member,
    NonMember,
    Invalid,
}

impl<'de> Deserialize<'de> for WA {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "WA Delegate" => WA::Delegate,
            "WA Member" => WA::Member,
            "Non-member" => WA::NonMember,
            _ => WA::Invalid,
        })
    }
}
