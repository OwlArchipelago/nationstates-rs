use std::thread::sleep;
use std::time::{Duration, SystemTime};

use quick_xml::de::from_str;
use reqwest::Client;

use crate::errors::NSError;
use crate::nation::Nation;
use crate::region::Region;

const NS_API_URL: &str = "https://www.nationstates.net/cgi-bin/api.cgi";
const RATE_LIMIT: usize = 49;

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

        return false;
    }

    pub async fn get_nation(&mut self, nation: &str) -> Result<Nation, NSError> {
        while !self.make_call() {
            sleep(Duration::from_secs(5));
        }

        let res = self
            .client
            .get(NS_API_URL)
            .query(&[("nation", nation)])
            .send()
            .await
            .map_err(|error| NSError::HTTPClient(error))?;
        // Get text from the response or return an error
        let data = res
            .text()
            .await
            .map_err(|error| NSError::HTTPClient(error))?;
        // Deserialization
        from_str(data.as_str()).map_err(|error| NSError::Deserializer(error))
    }

    pub async fn get_region(&mut self, region: &str) -> Result<Region, NSError> {
        while !self.make_call() {
            sleep(Duration::from_secs(5));
        }

        let res = self
            .client
            .get(NS_API_URL)
            .query(&[("region", region)])
            .send()
            .await
            .map_err(|error| NSError::HTTPClient(error))?;
        // Get text from the response or return an error
        let data = res
            .text()
            .await
            .map_err(|error| NSError::HTTPClient(error))?;
        // Deserialization
        from_str(data.as_str()).map_err(|error| NSError::Deserializer(error))
    }

    pub async fn verify(&mut self, nation: &str, checksum: &str) -> Result<bool, NSError> {
        while !self.make_call() {
            sleep(Duration::from_secs(5));
        }

        let res = self
            .client
            .get(NS_API_URL)
            .query(&[("a", "verify"), ("nation", nation), ("checksum", checksum)])
            .send()
            .await
            .map_err(|error| NSError::HTTPClient(error))?;

        // Get text from the response or return an error
        let data = res
            .text()
            .await
            .map_err(|error| NSError::HTTPClient(error))?;

        Ok(data == "1")
    }
}
