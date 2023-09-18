use std::fs;
use std::io::Write;
use std::str::FromStr;
use reqwest::{get};

#[derive(Debug, PartialEq, Clone)]
pub enum ComponentType {
    Button,
}

impl FromStr for ComponentType {
	type Err = String;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
			match s {
					"button" => Ok(ComponentType::Button),
					_ => Err(format!("'{}' is not a valid component type", s)),
			}
	}
}

pub async fn generate_component(component: ComponentType) {
	match component {
		ComponentType::Button => {
			let url = "https://raw.githubusercontent.com/kidsfromvision/daedalus/main/demo/src/lib/components/button/button-dark.svelte";
			let content = reqwest::get(url)
				.await
				.expect("Failed to fetch content")
				.text()
				.await
				.expect("Failed to read content");

			// Ensure the directory exists
			let path = "./lib/components/button";
			fs::create_dir_all(path).expect("Failed to create directory");

			// Write to the file
			let mut file = fs::File::create(format!("{}/button.svelte", path))
					.expect("Failed to create file");
			file.write_all(content.as_bytes())
					.expect("Failed to write to file");

			println!("Generated button component at {}/button.svelte", path);
		}
	}
}
