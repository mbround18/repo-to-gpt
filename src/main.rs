mod cli;
mod prompt;

use cli::Args;

use clap::Parser;
use ignore::gitignore::GitignoreBuilder;
use ignore::WalkBuilder;
use serde_json::json;
use std::collections::HashSet;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

fn main() -> io::Result<()> {
    let args = Args::parse();

    let input_folder = &args.input;
    let output_file = &args.output;
    let ignored_extensions: HashSet<String> = args.ignored.split(',').map(String::from).collect();

    let input_path = Path::new(input_folder);
    if !input_path.exists() || !input_path.is_dir() {
        eprintln!("The specified input folder does not exist or is not a directory.");
        std::process::exit(1);
    }

    let mut file_data = Vec::new();
    let mut skipped_files = 0;

    println!("Scanning folder: {}", input_folder);
    println!("Ignoring file extensions: {:?}", ignored_extensions);

    let mut walk_builder = WalkBuilder::new(input_path);

    let gitignore_path = Path::new(input_folder).join(".gitignore");
    if gitignore_path.exists() {
        let mut gitignore_builder = GitignoreBuilder::new(input_path);
        gitignore_builder.add(Path::new(input_folder).join(".gitignore"));
        if let Ok(gitignore) = gitignore_builder.build() {
            walk_builder.add_ignore(gitignore.path());
        }
    }

    let walker = walk_builder.build();

    for result in walker {
        match result {
            Ok(entry) => {
                let path = entry.path();
                if entry.file_type().map_or(false, |ft| ft.is_file()) {
                    if let Some(ext) = path.extension() {
                        if ignored_extensions.contains(&format!(".{}", ext.to_string_lossy())) {
                            skipped_files += 1;
                            continue;
                        }
                    }

                    if is_binary_file(path) {
                        skipped_files += 1;
                        continue;
                    }

                    match fs::read_to_string(path) {
                        Ok(content) => {
                            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
                            let file_path = path.display().to_string();
                            let data = content
                                .replace("\r", "")
                                .strip_prefix("\n")
                                .unwrap_or(content.as_str())
                                .strip_suffix("\r\n")
                                .unwrap_or(content.as_str())
                                .trim()
                                .to_string();
                            file_data.push(json!({
                                    "FileName": file_name,
                                    "FilePath": file_path,
                                    "Content": data,
                            }));
                        }
                        Err(e) => {
                            eprintln!("Could not read file {}: {}", path.display(), e);
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Error reading file: {}", err);
            }
        }
    }

    if file_data.is_empty() {
        eprintln!("No files found or processed in the folder. Check the input folder or exclusion filters.");
        std::process::exit(1);
    }

    println!(
        "Processed {} files, skipped {} files.",
        file_data.len(),
        skipped_files
    );

    println!("Converting and minifying file data to JSON...");
    let output_json = json!({
        "Prompt": prompt::PROMPT,
        "Files": file_data
    });

    let json = if args.pretty {
        serde_json::to_string_pretty(&output_json).expect("Failed to serialize file data")
    } else {
        serde_json::to_string(&output_json).expect("Failed to serialize file data")
    };

    let mut output = File::create(output_file)?;
    output.write_all(json.as_bytes())?;

    println!(
        "Folder contents successfully exported to minified JSON: {}",
        output_file
    );

    Ok(())
}

fn is_binary_file(path: &Path) -> bool {
    match fs::read(path) {
        Ok(content) => content.iter().any(|&byte| byte == 0),
        Err(_) => false,
    }
}
