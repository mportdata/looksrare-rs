use crate::constants;
use serde::{Deserialize, Serialize};
use std::process::ExitCode;
use std::process::Termination;
use ethers::prelude::Address;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Network {
    Mainnet,
}

impl Network {
    pub fn url(&self) -> &str {
        match self {
            Network::Mainnet => constants::API_BASE_MAINNET,
        }
    }

    pub fn api(&self) -> String {
        let url = self.url();
        format!("{}{}{}", url, constants::API_PATH, constants::VERSION)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub address: Address,
    pub name: Option<String>,
    pub biography: Option<String>,
    pub website_link: Option<String>,
    pub instagram_link: Option<String>,
    pub twitter_link: Option<String>,
    pub is_verified: bool,
}

impl Termination for Account {
    fn report(self) -> ExitCode {
        println!("Account Termination Trait");
        ExitCode::from(42)
    }
}