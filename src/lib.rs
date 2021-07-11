//! # NationStates API in Rust
//!
//! This crate consists of a wrapper around the api of the game [NationStates](https://www.nationstates.net).
//! Still a work in progress, currently only supports the standard api's for the nation and region endpoints.
//! The NationStates API documentation is available [here](https://www.nationstates.net/pages/api.html).
//!
//! ## Example
//! ```
//! use nationstates::{NSClient, NSError};
//!
//! #[tokio::main]
//! pub async fn main() -> Result<(), NSError> {
//!    let mut client = NSClient::new("Owl Archipelago's API Test")?;
//!    let nation = client.get_nation("Owl Archipelago").await?;
//!
//!    println!("{}", nation.name);
//!    println!("{}", nation.fullname);
//!    println!("{}", nation.motto);
//!
//!    if nation.in_wa() {
//!        println!("WA Member");
//!    }
//!
//!    println!("{} - {}", nation.founded, nation.firstlogin.to_string());
//!
//!    println!();
//!    println!("Economic Freedom: {}", nation.freedom.economy);
//!    println!();
//!
//!    for cause in nation.deaths.causes {
//!        println!("{} - {}", cause.reason, cause.percentage);
//!    }
//!
//!    println!();
//!    println!("Endo Count: {}", nation.endorsements.count());
//!    for endo in nation.endorsements.get() {
//!        println!("{}", endo);
//!    }
//!
//!    Ok(())
//! }
//! ```

pub mod client;
pub mod errors;
pub mod nation;
pub mod region;

pub use client::NSClient;
pub use errors::NSError;
pub use nation::Nation;
pub use region::Region;
