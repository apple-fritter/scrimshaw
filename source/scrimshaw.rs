use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    // Get command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: cargo run -- <username> <log_directory>");
        return Ok(());
    }

    // Extract username and log directory from arguments
    let username = &args[1];
    let log_directory = PathBuf::from(&args[2]);

    // Recursive traversal of log directory
    let log_files = find_log_files(&log_directory)?;

    let mut messages = Vec::new();

    for log_file in log_files {
        let file_path = log_directory.join(&log_file);
        let file_content = fs::read_to_string(&file_path)?;

        for line in file_content.lines() {
            if line.starts_with('#') {
                continue; // Ignore lines starting with '#'
            }

            let fields: Vec<&str> = line.split('☕').collect();
            if fields.len() >= 6 && fields[4] == username {
                messages.push(fields[5]);
            }
        }
    }

    let output_file = format!("{}_quotes.txt", username);
    let mut output = fs::File::create(output_file)?;

    for message in messages {
        writeln!(output, "{}", message)?;
    }

    Ok(())
}

fn find_log_files(directory: &Path) -> io::Result<Vec<String>> {
    let mut log_files = Vec::new();

    if directory.is_dir() {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let file_path = entry.path();
            if file_path.is_file() {
                if let Some(extension) = file_path.extension() {
                    if extension == "txt" {
                        if let Some(file_name) = file_path.file_name() {
                            if let Some(file_name_str) = file_name.to_str() {
                                log_files.push(file_name_str.to_string());
                            }
                        }
                    }
                }
            } else if file_path.is_dir() {
                let nested_files = find_log_files(&file_path)?;
                log_files.extend(nested_files);
            }
        }
    }

    Ok(log_files)
}
