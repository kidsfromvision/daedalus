mod component;
use std::str::FromStr;

use clap::Parser;
pub use component::*;

#[derive(Debug, PartialEq, Clone)]
pub enum GenerateType {
  Component,
}

impl FromStr for GenerateType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "component" => Ok(GenerateType::Component),
            _ => Err(format!("'{}' is not a valid generation type", s)),
        }
    }
}

#[derive(Parser)]
pub struct GenerateArgs {
    #[arg(name = "Type of item to generate")]
    pub item_type: GenerateType,
    #[clap(name = "Name of the item")]
    pub item_name: ComponentType,
}
