use std::path::{Path, PathBuf};

pub fn find_components_directory(starting_path: &Path) -> Option<PathBuf> {
	let mut current_path = starting_path.to_path_buf();
	loop {
		if current_path.join("package.json").exists() {
			let src_lib_components = current_path.join("src/lib/components");
			if src_lib_components.exists() {
					return Some(src_lib_components);
			}

			// Check for both possible component directories
			let lib_components = current_path.join("lib/components");
			if lib_components.exists() {
					return Some(lib_components);
			}

		}

		if !current_path.pop() {
			break;
		}
	}
	None
}
