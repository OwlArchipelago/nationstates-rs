use std::str;
use std::thread::sleep;
use std::time::{Duration, SystemTime};

use quick_xml::de::from_str;
use reqwest::Client;

use crate::errors::NSError;
use crate::nation::Nation;
use crate::region::Region;

const NS_API_URL: &str = "https://www.nationstates.net/cgi-bin/api.cgi";
const RATE_LIMIT: usize = 49;
const NS_API_VERSION: &str = "11";

pub struct NSClient {
    client: Client,
    calls: Vec<SystemTime>,
}

impl NSClient {
    pub fn new(user_agent: &str) -> Result<NSClient, NSError> {
        let client = Client::builder()
            .user_agent(user_agent)
            .build()
            .map_err(NSError::HTTPClient)?;

        Ok(NSClient {
            client,
            calls: vec![],
        })
    }

    fn make_call(&mut self) -> bool {
        // TODO Add check for hard rate limit

        // Remove calls that happened more than 30 seconds before
        self.calls
            .retain(|&call| SystemTime::now().duration_since(call).unwrap().as_secs() >= 30);

        if self.calls.len() < RATE_LIMIT {
            self.calls.push(SystemTime::now());
            return true;
        }

        false
    }

    async fn make_request(&mut self, query: &[(&str, &str)]) -> Result<String, NSError> {
        while !self.make_call() {
            sleep(Duration::from_secs(5));
        }

        self.client
            .get(NS_API_URL)
            .query(&[("v", NS_API_VERSION)])
            .query(query)
            .send()
            .await
            .map_err(NSError::HTTPClient)?
            .text()
            .await
            .map_err(NSError::HTTPClient)
    }

    pub async fn get_nation(&mut self, nation: &str) -> Result<Nation, NSError> {
        let data = self.make_request(&[("nation", nation)]).await?;
        // Parse the xml
        from_str(data.as_str()).map_err(NSError::Deserializer)
    }

    pub async fn get_region(&mut self, region: &str) -> Result<Region, NSError> {
        let data = self.make_request(&[("region", region)]).await?;
        // Parse the xml
        from_str(data.as_str()).map_err(NSError::Deserializer)
    }

    pub async fn verify(&mut self, nation: &str, checksum: &str) -> Result<bool, NSError> {
        let data = self
            .make_request(&[("a", "verify"), ("nation", nation), ("checksum", checksum)])
            .await?;
        Ok(data.contains('1'))
    }

    pub async fn verify_with_token(
        &mut self,
        nation: &str,
        checksum: &str,
        token: &str,
    ) -> Result<bool, NSError> {
        let data = self
            .make_request(&[
                ("a", "verify"),
                ("token", token),
                ("nation", nation),
                ("checksum", checksum),
            ])
            .await?;
        Ok(data.contains('1'))
    }
}
