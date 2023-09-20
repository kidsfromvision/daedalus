use std::{fs, io::Write};
use super::helpers::find_components_directory;
use clap::{Command, Arg};

pub async fn write_button(variant: String) {
	let url = format!("https://raw.githubusercontent.com/kidsfromvision/daedalus/main/demo/src/lib/components/button/button-{}.svelte", variant);

	let content = reqwest::get(&url)
			.await
			.expect("Failed to fetch content")
			.text()
			.await
			.expect("Failed to read content");

	let current_dir = std::env::current_dir().expect("Failed to get current directory");
	let components_dir = find_components_directory(&current_dir)
			.expect("Failed to find components directory in the Svelte project");

	let button_dir = components_dir.join("button");
	let file_path = button_dir.join(format!("{}.svelte", variant));
	let mut file = fs::File::create(&file_path)
		.expect("Failed to create file");

	file.write_all(content.as_bytes())
		.expect("Failed to write to file");

	println!("Generated {} component at {}", variant, &file_path.display());
}

pub fn button_command() -> Command {
	Command::new("button")
		.about("Generate a new button")
		.subcommand_required(false)
			.arg(
				Arg::new("variant")
					.short('v')
					.long("variant")
					.value_parser(["light", "dark"])
					.num_args(0..=1)
					.require_equals(true)
					.default_value("light")
					.default_missing_value("light")
			)

}
