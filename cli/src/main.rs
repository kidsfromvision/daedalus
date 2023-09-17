use clap::Parser;

#[derive(Parser)] // Derive the Parser trait for argument parsing
#[clap(name = "daedalus", version = "1.0", author = "Sam Brownstone brownstone@hey.com")]
enum DaedalusCli {
	#[clap(alias = "g")]
	Generate(GenerateArgs),
}

#[derive(Parser)]
struct GenerateArgs {
    #[arg(name = "Type of item to generate")]
    item_type: String,
    #[clap(name = "Name of the item")]
    item_name: String,
}

fn main() {
	match DaedalusCli::parse() {
			DaedalusCli::Generate(args) => {
					match args.item_type.as_str() {
							"component" => println!("Generating component named: {}", args.item_name),
							_ => println!("Unknown generation type: {}", args.item_type),
					}
			}
	}
}

