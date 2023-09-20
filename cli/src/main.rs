mod components;
use components::*;
use clap::{Command};

#[tokio::main]
async fn main() {
	let cli = Command::new("daedalus")
		.about("A CLI tool to manage your Daedalus components")
		.author("Sam Brownstone brownstone@hey.com")
		.version("1.0")
		.subcommand_required(true)
		.subcommand(
			Command::new("generate")
				.about("Generate a new entity")
				.alias("g")
				.subcommand_required(true)
				.subcommand(
						Command::new("component")
						.about("Generate a new component")
						.alias("c")
						.subcommand_required(true)
						.subcommand(button_command())
				)
		);

	match cli.get_matches().subcommand() {
		Some(("generate", sub_matches)) => {
			match sub_matches.subcommand() {
				Some(("component", sub_matches)) => {
					match sub_matches.subcommand() {
						Some(("button", sub_matches)) => {
							let variant = sub_matches
								.get_many::<String>("variant")
								.expect("contains_id")
								.map(|s| s.as_str())
								.collect::<String>();

							// TODO: add --dir option to specify output directory
							components::write_button(variant).await;
						}
						_ => unreachable!()
					}
				}
				_ => unreachable!()
			}
		}
		_ => unreachable!()
	}
}
