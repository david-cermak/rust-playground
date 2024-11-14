use std::fs;
use std::path::{Path, PathBuf};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CompileCommand {
    directory: String,
    command: String,
    file: String,
}

fn main() {
    // Path to the compile_commands.json file
    let compile_commands_path = Path::new("compile_commands.json");

    // Parse compile_commands.json
    let compile_commands: Vec<CompileCommand> = {
        let data = fs::read_to_string(compile_commands_path)
            .expect("Failed to read compile_commands.json");
        serde_json::from_str(&data)
            .expect("Failed to parse compile_commands.json")
    };

    // Directory of compile_commands.json, used to resolve relative paths
    let base_dir = compile_commands_path
        .parent()
        .expect("Failed to get base directory of compile_commands.json");

    // Find the entry corresponding to `mdns_networking_socket.c`
    let cmd = compile_commands.iter()
        .find(|entry| entry.file.ends_with("mdns_networking_socket.c"))
        .expect("mdns_networking_socket.c not found in compile_commands.json");

    // Initialize the build with the target C file
    let mut build = cc::Build::new();
    build.file("src/mdns_networking_socket.c");

    // Parse flags and include paths
    for part in cmd.command.split_whitespace() {
        if part.starts_with("-I") {
            // Handle include directories
            let include_path = &part[2..];
            let full_include_path = if Path::new(include_path).is_relative() {
                base_dir.join(include_path).canonicalize()
                    .expect("Failed to resolve relative include path")
            } else {
                PathBuf::from(include_path)
            };
            build.include(full_include_path);
        } else if part.starts_with("-D") || part.starts_with("-std") {
            // Add other compilation flags
            build.flag(part);
        }
    }

    // Compile with the gathered information
    build.compile("mdns");
}
