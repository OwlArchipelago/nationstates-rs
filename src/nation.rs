use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Nation information returned by the Standard Nation API
#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct Nation {
    pub name: String,
    #[serde(rename = "TYPE")]
    pub ntype: String,
    pub fullname: String,
    pub motto: String,
    pub category: String,
    pub influence: String,
    pub unstatus: WAStatus,
    pub endorsements: Endorsements,
    pub issues_answered: u32,
    pub freedom: Freedom<String>,
    #[serde(rename = "FREEDOMSCORES")]
    pub freedom_scores: Freedom<f32>,
    pub region: String,
    pub population: u32,
    pub tax: f32,
    pub animal: String,
    pub currency: String,
    #[serde(rename = "DEMONYM")]
    pub adjective_demonym: String,
    #[serde(rename = "DEMONYM2")]
    pub noun_demonym: String,
    #[serde(rename = "DEMONYM2PLURAL")]
    pub plural_demonym: String,
    pub flag: String,
    // Industry/Gov stuff
    #[serde(rename = "GOVT")]
    pub govt_budget: GovtBudget,
    #[serde(rename = "MAJORINDUSTRY")]
    pub major_industry: String,
    #[serde(rename = "GOVTPRIORITY")]
    pub govt_priority: String,
    #[serde(rename = "PUBLICSECTOR")]
    pub public_sector: f32,
    pub founded: String,
    #[serde(with = "ts_seconds")]
    pub firstlogin: DateTime<Utc>,
    #[serde(with = "ts_seconds")]
    pub lastlogin: DateTime<Utc>,
    pub lastactivity: String,
    pub deaths: Deaths,
    pub leader: String,
    pub capital: String,
    pub religion: String,
    pub factbooks: u32,
    pub dispatches: u32,
    pub dbid: u32,
}

impl Nation {
    pub fn in_wa(&self) -> bool {
        self.unstatus == WAStatus::Delegate || self.unstatus == WAStatus::NonMember
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct GovtBudget {
    pub administration: f32,
    pub defence: f32,
    pub education: f32,
    pub environment: f32,
    pub healthcare: f32,
    pub commerce: f32,
    #[serde(rename = "INTERNATIONALAID")]
    pub international_aid: f32,
    #[serde(rename = "LAWANDORDER")]
    pub law_and_order: f32,
    #[serde(rename = "PUBLICTRANSPORT")]
    pub public_transport: f32,
    #[serde(rename = "SOCIALEQUALITY")]
    pub social_equality: f32,
    pub spirituality: f32,
    pub welfare: f32,
}

#[derive(Debug, Deserialize)]
pub struct Endorsements {
    #[serde(rename = "$value")]
    endos: Option<Vec<String>>,
}

impl Endorsements {
    pub fn count(&self) -> usize {
        match self.endos.as_ref() {
            Some(endos) => endos.len(),
            None => 0,
        }
    }

    pub fn get(&self) -> Vec<String> {
        self.endos.as_ref().unwrap_or(vec![].as_ref()).clone()
    }
}

/// Causes of death in a nation
#[derive(Debug, Deserialize)]
pub struct Deaths {
    #[serde(rename = "CAUSE")]
    pub causes: Vec<Cause>,
}

/// Cause of death and its percentage
#[derive(Debug, Deserialize)]
pub struct Cause {
    #[serde(rename = "type")]
    pub reason: String,
    #[serde(rename = "$value")]
    pub percentage: f32,
}

/// Freedom scores for a nation
#[derive(Debug, Deserialize)]
pub struct Freedom<T> {
    #[serde(rename = "CIVILRIGHTS")]
    pub civil_rights: T,
    #[serde(rename = "ECONOMY")]
    pub economy: T,
    #[serde(rename = "POLITICALFREEDOM")]
    pub political: T,
}

/// WA membership status for a nation
#[derive(Debug, PartialEq)]
pub enum WAStatus {
    Delegate,
    Member,
    NonMember,
    Invalid,
}

impl<'de> Deserialize<'de> for WAStatus {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Ok(match s.as_str() {
            "WA Delegate" => WAStatus::Delegate,
            "WA Member" => WAStatus::Member,
            "Non-member" => WAStatus::NonMember,
            _ => WAStatus::Invalid,
        })
    }
}
