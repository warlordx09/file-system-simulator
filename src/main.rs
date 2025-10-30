mod directory;
mod disk;
mod inode;

use crate::directory::Directory;
use crate::disk::{BLOCK_SIZE, VirtualDisk};
use crate::inode::{FileType, Inode};
use colored::*;
use std::io::{self, Write};
fn main() {
    let mut disk = VirtualDisk::new();
    let root = Directory::new("root", None);
    let mut dir_stack: Vec<Directory> = vec![root];

    // ======= Cool Banner =======
    println!(
        "{}",
        r#"
██╗   ██╗██╗████████╗██╗██╗     ███████╗
██║   ██║██║╚══██╔══╝██║██║     ██╔════╝
██║   ██║██║   ██║   ██║██║     █████╗
╚██╗ ██╔╝██║   ██║   ██║██║     ██╔══╝
 ╚████╔╝ ██║   ██║   ██║███████╗███████╗
  ╚═══╝  ╚═╝   ╚═╝   ╚═╝╚══════╝╚══════╝
"#
        .bright_magenta()
        .bold()
    );
    println!(
        "{}",
        "🚀 Virtual File System Simulator (RustFS) 🚀"
            .bold()
            .bright_cyan()
    );
    println!(
        "{}",
        "Type 'help' for list of commands.\n"
            .italic()
            .bright_yellow()
    );

    loop {
        // ======= Prompt =======
        let current_path = dir_stack
            .iter()
            .map(|d| d.name.as_str())
            .collect::<Vec<_>>()
            .join("/")
            .bright_green();

        print!("{}:/{}> ", "RustFS".bright_blue().bold(), current_path);
        io::stdout().flush().unwrap();

        let current_dir = dir_stack.last_mut().unwrap();

        // ======= Read Input =======
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "help" => {
                println!("{}", "\nAvailable Commands:".bold().bright_cyan());
                println!("  mkdir <dir>    - Create new directory");
                println!("  create <file>  - Create new file");
                println!("  cat <file>     - View file content");
                println!("  ls             - List directory");
                println!("  cd <dir>       - Change directory");
                println!("  cd ..          - Go up one directory");
                println!("  pwd            - Show current path");
                println!("  rm <file>      - Delete a file");
                println!("  exit           - Save and quit\n");
            }

            // === Create new directory ===
            "mkdir" => {
                if let Some(dir_name) = parts.get(1) {
                    let dir_inode = Inode::new(
                        current_dir.entries.len(),
                        dir_name,
                        0,
                        vec![],
                        FileType::Directory,
                    );
                    if current_dir.add_entry(dir_name, dir_inode.id, true).is_ok() {
                        println!(
                            "{}",
                            format!("📁 Directory '{}' created.", dir_name).green()
                        );
                    } else {
                        println!(
                            "{}",
                            format!("⚠️ Directory '{}' already exists!", dir_name).red()
                        );
                    }
                } else {
                    println!("{}", "Usage: mkdir <dirname>".yellow());
                }
            }

            // === Create new file ===
            "create" => {
                if let Some(file_name) = parts.get(1) {
                    print!("{}", "Enter file content: ".bright_blue());
                    io::stdout().flush().unwrap();
                    let mut content = String::new();
                    io::stdin().read_line(&mut content).unwrap();
                    let bytes = content.as_bytes();

                    let blocks_needed = (bytes.len() + BLOCK_SIZE - 1) / BLOCK_SIZE;
                    let mut blocks = Vec::new();

                    for _ in 0..blocks_needed {
                        if let Some(block) = disk.allocate_block() {
                            blocks.push(block);
                        }
                    }

                    for (i, &block) in blocks.iter().enumerate() {
                        let start = i * BLOCK_SIZE;
                        let end = ((i + 1) * BLOCK_SIZE).min(bytes.len());
                        disk.write_block(block, &bytes[start..end]);
                    }

                    let inode = Inode::new(
                        current_dir.entries.len(),
                        file_name,
                        bytes.len(),
                        blocks.clone(),
                        FileType::File,
                    );

                    current_dir.add_entry(file_name, inode.id, false).unwrap();
                    println!(
                        "{}",
                        format!("📝 File '{}' created successfully.", file_name).green()
                    );
                } else {
                    println!("{}", "Usage: create <filename>".yellow());
                }
            }

            // === View file contents ===
            "cat" => {
                if let Some(file_name) = parts.get(1) {
                    if let Some(entry) = current_dir.get_entry(file_name) {
                        if entry.is_dir {
                            println!("{}", format!("'{}' is a directory.", file_name).red());
                        } else {
                            println!(
                                "{}",
                                format!("(Simulated) Reading contents of '{}'", file_name)
                                    .bright_cyan()
                            );
                        }
                    } else {
                        println!("{}", format!("File '{}' not found.", file_name).red());
                    }
                } else {
                    println!("{}", "Usage: cat <filename>".yellow());
                }
            }

            // === List directory contents ===
            "ls" => {
                println!("{}", "📂 Directory contents:".bold().bright_magenta());
                current_dir.list_entries();
            }

            // === Change directory ===
            "cd" => {
                if let Some(dir_name) = parts.get(1) {
                    if *dir_name == ".." {
                        if dir_stack.len() > 1 {
                            dir_stack.pop();
                            let parent = dir_stack.last().unwrap();
                            println!("{}", format!("⬆️  Moved up to '{}'", parent.name).blue());
                        } else {
                            println!("{}", "⚠️ Already at root directory.".yellow());
                        }
                    } else if let Some(entry) = current_dir.get_entry(dir_name) {
                        if entry.is_dir {
                            let new_dir =
                                Directory::new(dir_name, Some(current_dir.get_inode_id()));
                            dir_stack.push(new_dir);
                            println!("{}", format!("📂 Entered '{}'", dir_name).blue());
                        } else {
                            println!("{}", format!("'{}' is not a directory.", dir_name).red());
                        }
                    } else {
                        println!("{}", format!("Directory '{}' not found.", dir_name).red());
                    }
                } else {
                    println!("{}", "Usage: cd <dirname>".yellow());
                }
            }

            // === Show current working directory ===
            "pwd" => {
                let path = dir_stack
                    .iter()
                    .map(|d| d.name.clone())
                    .collect::<Vec<String>>()
                    .join("/");
                println!("{}", format!("📍 /{}", path).bright_green());
            }

            // === Remove file ===
            "rm" => {
                if let Some(name) = parts.get(1) {
                    if let Some(entry) = current_dir.get_entry(name) {
                        if entry.is_dir {
                            println!("{}", "Use rmdir for directories!".yellow());
                        } else {
                            current_dir.remove_entry(name).unwrap();
                            println!("{}", format!("🗑️  File '{}' deleted.", name).green());
                        }
                    } else {
                        println!("{}", format!("No such file '{}'.", name).red());
                    }
                } else {
                    println!("{}", "Usage: rm <filename>".yellow());
                }
            }

            // === Exit ===
            "exit" => {
                disk.save_to_file("fs_image.bin");
                println!(
                    "{}",
                    "💾 File system saved to 'fs_image.bin'. Exiting...".bright_cyan()
                );
                break;
            }

            _ => println!(
                "{}",
                "❓ Unknown command. Type 'help' for commands.".yellow()
            ),
        }
    }
}
