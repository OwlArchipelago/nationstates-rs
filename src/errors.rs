use thiserror::Error;

#[derive(Error, Debug)]
pub enum NSError {
    #[error("error from the http client")]
    HTTPClient(#[from] reqwest::Error),
    #[error("error from the deserializer")]
    Deserializer(#[from] quick_xml::de::DeError),
}
