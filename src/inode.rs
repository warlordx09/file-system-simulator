use chrono::Local;
use serde::{Deserialize, Serialize};

/// Represents the type of the file (like in Linux)
#[derive(Serialize, Deserialize, Clone, Copy, Debug, PartialEq)]
pub enum FileType {
    File,
    Directory,
    Symlink,
}

/// Basic permission flags (rwx)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Permissions {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
}

impl Permissions {
    pub fn default_file() -> Self {
        Self {
            read: true,
            write: true,
            execute: false,
        }
    }
    pub fn default_dir() -> Self {
        Self {
            read: true,
            write: true,
            execute: true,
        }
    }
}

/// Inode structure representing metadata of a file or directory
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Inode {
    pub id: usize,
    pub name: String,
    pub size: usize,
    pub file_type: FileType,
    pub blocks: Vec<usize>,
    pub permissions: Permissions,
    pub created_at: String,
    pub modified_at: String,
}

impl Inode {
    /// Create a new inode
    pub fn new(
        id: usize,
        name: &str,
        size: usize,
        blocks: Vec<usize>,
        file_type: FileType,
    ) -> Self {
        let now = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        Self {
            id,
            name: name.to_string(),
            size,
            file_type,
            blocks,
            permissions: if file_type == FileType::File {
                Permissions::default_file()
            } else {
                Permissions::default_dir()
            },
            created_at: now.clone(),
            modified_at: now,
        }
    }

    /// Update file metadata after editing
    pub fn update(&mut self, new_size: usize, new_blocks: Vec<usize>) {
        self.size = new_size;
        self.blocks = new_blocks;
        self.modified_at = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    }

    /// Create a safe clone (for directory/file copy simulation)
    pub fn clone_safe(&self) -> Self {
        Self {
            id: self.id,
            name: self.name.clone(),
            size: self.size,
            file_type: self.file_type,
            blocks: self.blocks.clone(),
            permissions: self.permissions.clone(),
            created_at: self.created_at.clone(),
            modified_at: self.modified_at.clone(),
        }
    }

    /// Print inode info (for debugging or listing)
    pub fn print_info(&self) {
        println!(
            "Inode #{} -> Name: {}, Type: {:?}, Size: {} bytes",
            self.id, self.name, self.file_type, self.size
        );
        println!(
            "  Created: {}, Modified: {}",
            self.created_at, self.modified_at
        );
        println!(
            "  Permissions: r={} w={} x={}",
            self.permissions.read, self.permissions.write, self.permissions.execute
        );
        println!("  Blocks: {:?}", self.blocks);
    }
}
