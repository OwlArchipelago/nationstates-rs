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
        let client = Client::builder().user_agent(user_agent).build();

        match client {
            Ok(client) => Ok(NSClient {
                client,
                calls: vec![],
            }),
            Err(error) => Err(NSError::HTTPClient(error)),
        }
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

    pub async fn get_nation(&mut self, nation: &str) -> Result<Nation, NSError> {
        while !self.make_call() {
            sleep(Duration::from_secs(5));
        }

        let res = self
            .client
            .get(NS_API_URL)
            .query(&[("nation", nation), ("v", NS_API_VERSION)])
            .send()
            .await
            .map_err(NSError::HTTPClient)?;
        // Get text from the response or return an error
        let data = res.text().await.map_err(NSError::HTTPClient)?;
        // Deserialization
        from_str(data.as_str()).map_err(NSError::Deserializer)
    }

    pub async fn get_region(&mut self, region: &str) -> Result<Region, NSError> {
        while !self.make_call() {
            sleep(Duration::from_secs(5));
        }

        let res = self
            .client
            .get(NS_API_URL)
            .query(&[("region", region), ("v", NS_API_VERSION)])
            .send()
            .await
            .map_err(NSError::HTTPClient)?;
        // Get text from the response or return an error
        let data = res.text().await.map_err(NSError::HTTPClient)?;
        // Deserialization
        from_str(data.as_str()).map_err(NSError::Deserializer)
    }

    pub async fn verify(&mut self, nation: &str, checksum: &str) -> Result<bool, NSError> {
        self.handle_verification(nation, checksum, Option::None)
            .await
    }

    pub async fn verify_with_token(
        &mut self,
        nation: &str,
        checksum: &str,
        token: &str,
    ) -> Result<bool, NSError> {
        self.handle_verification(nation, checksum, Option::Some(token))
            .await
    }

    async fn handle_verification(
        &mut self,
        nation: &str,
        checksum: &str,
        token: Option<&str>,
    ) -> Result<bool, NSError> {
        while !self.make_call() {
            sleep(Duration::from_secs(5));
        }

        let mut req = self.client.get(NS_API_URL).query(&[
            ("a", "verify"),
            ("nation", nation),
            ("checksum", checksum),
            ("v", NS_API_VERSION),
        ]);

        if let Some(site_token) = token {
            req = req.query(&[("token", site_token)]);
        }

        let res = req.send().await.map_err(NSError::HTTPClient)?;

        // Get text from the response or return an error
        let data = res.text().await.map_err(NSError::HTTPClient)?;

        Ok(data.contains('1'))
    }
}
