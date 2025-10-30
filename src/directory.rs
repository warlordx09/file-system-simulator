use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Represents a directory entry mapping: name -> inode_id
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DirEntry {
    pub name: String,
    pub inode_id: usize,
    pub is_dir: bool,
}

/// Directory structure containing subdirectories and files
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Directory {
    pub name: String,
    pub parent: Option<usize>, // inode_id of parent dir
    pub entries: HashMap<String, DirEntry>,
}

impl Directory {
    /// Create a new empty directory
    pub fn new(name: &str, parent: Option<usize>) -> Self {
        Directory {
            name: name.to_string(),
            parent,
            entries: HashMap::new(),
        }
    }
    /// Add a new file or directory entry
    pub fn add_entry(&mut self, name: &str, inode_id: usize, is_dir: bool) -> Result<(), String> {
        if self.entries.contains_key(name) {
            return Err(format!(
                "Entry '{}' already exists in directory '{}'",
                name, self.name
            ));
        }

        let entry = DirEntry {
            name: name.to_string(),
            inode_id,
            is_dir,
        };

        self.entries.insert(name.to_string(), entry);
        Ok(())
    }

    /// Remove a file or subdirectory
    pub fn remove_entry(&mut self, name: &str) -> Result<(), String> {
        if self.entries.remove(name).is_none() {
            return Err(format!("No such entry '{}' in '{}'", name, self.name));
        }
        Ok(())
    }

    /// List all entries (like `ls`)
    pub fn list_entries(&self) {
        println!("Contents of directory '{}':", self.name);
        if self.entries.is_empty() {
            println!("  (empty)");
            return;
        }

        for (name, entry) in &self.entries {
            let entry_type = if entry.is_dir { "DIR" } else { "FILE" };
            println!("  {} ({}) -> inode {}", name, entry_type, entry.inode_id);
        }
    }

    /// Check if entry exists
    pub fn has_entry(&self, name: &str) -> bool {
        self.entries.contains_key(name)
    }

    /// Resolve an entry by name
    pub fn get_entry(&self, name: &str) -> Option<&DirEntry> {
        self.entries.get(name)
    }

    /// Navigate to subdirectory or parent
    pub fn change_dir(
        &self,
        dirs: &Vec<Directory>,
        current_inode: usize,
        target: &str,
    ) -> Option<usize> {
        if target == ".." {
            // Go up to parent if possible
            return self.parent;
        }

        // Go down into child folder
        if let Some(entry) = self.get_entry(target) {
            if entry.is_dir {
                return Some(entry.inode_id);
            }
        }

        println!("No such directory '{}'", target);
        None
    }

    /// Get full path by walking up the parent chain
    pub fn get_path(&self, dirs: &Vec<Directory>, self_inode: usize) -> String {
        let mut path = vec![self.name.clone()];
        let mut current_parent = self.parent;

        while let Some(parent_inode) = current_parent {
            if let Some(parent_dir) = dirs.iter().find(|d| d.get_inode_id() == parent_inode) {
                path.push(parent_dir.name.clone());
                current_parent = parent_dir.parent;
            } else {
                break;
            }
        }

        path.reverse();
        format!("/{}", path.join("/"))
    }

    /// Helper to get fake inode ID of directory (for simulation)
    pub fn get_inode_id(&self) -> usize {
        self.name
            .as_bytes()
            .iter()
            .map(|b| *b as usize)
            .sum::<usize>()
            % 10000
    }
}
