mod generator;
use generator::{GenerateArgs, GenerateType, generate_component};
use clap::Parser;

#[derive(Parser)] // Derive the Parser trait for argument parsing
#[clap(name = "daedalus", version = "1.0", author = "Sam Brownstone brownstone@hey.com")]
enum DaedalusCli {
    #[clap(alias = "g")]
    Generate(GenerateArgs),
}

#[tokio::main]
async fn main() {
    match DaedalusCli::parse() {
        DaedalusCli::Generate(args) => {
            match args.item_type {
                GenerateType::Component => generate_component(args.item_name).await,
            };
        }
    }
}
