use quick_xml::de::from_str;
use reqwest::Client;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
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

#[derive(Debug, PartialEq)]
pub enum WA {
    Delegate,
    Member,
    NonMember,
    Invalid,
}

impl Nation {
    pub fn in_wa(&self) -> bool {
        self.unstatus == WA::Delegate || self.unstatus == WA::NonMember
    }
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

const NS_API_URL: &str = "https://www.nationstates.net/cgi-bin/api.cgi";

#[derive(Debug, Clone)]
pub struct NSAPIError;

pub struct NSAPIClient {
    client: Client,
}

impl NSAPIClient {
    pub fn new(user_agent: &str) -> Result<NSAPIClient, NSAPIError> {
        let client = Client::builder().user_agent(user_agent).build();

        match client {
            Ok(client) => Ok(NSAPIClient { client }),
            Err(_) => Err(NSAPIError),
        }
    }

    pub async fn get_nation(&self, nation: &str) -> Result<Nation, NSAPIError> {
        let result = self
            .client
            .get(NS_API_URL)
            .query(&[("nation", nation)])
            .send()
            .await;

        // Get response from request or return an error
        let res = match result {
            Ok(res) => res,
            Err(_) => {
                return Err(NSAPIError);
            }
        };

        // Get text from the response or return an error
        let data = match res.text().await {
            Ok(text) => text,
            Err(_) => {
                return Err(NSAPIError);
            }
        };

        // Deserialize
        match from_str(data.as_str()) {
            Ok(nation) => Ok(nation),
            Err(_) => Err(NSAPIError),
        }
    }
}
