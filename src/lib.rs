use quick_xml::de::from_str;
use reqwest::Client;

mod errors;
mod nation;
mod region;

pub use errors::NSError;
pub use nation::{Nation, WA};
pub use region::Region;

const NS_API_URL: &str = "https://www.nationstates.net/cgi-bin/api.cgi";

pub struct NSClient {
    client: Client,
}

impl NSClient {
    pub fn new(user_agent: &str) -> Result<NSClient, NSError> {
        let client = Client::builder().user_agent(user_agent).build();

        match client {
            Ok(client) => Ok(NSClient { client }),
            Err(error) => Err(NSError::HTTPClient(error)),
        }
    }

    pub async fn get_nation(&self, nation: &str) -> Result<Nation, NSError> {
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

    pub async fn get_region(&self, region: &str) -> Result<Region, NSError> {
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
}
