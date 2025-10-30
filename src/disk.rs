use std::fs::{File, OpenOptions};
use std::io::{Read, Write};

pub const BLOCK_SIZE: usize = 512;
pub const TOTAL_BLOCKS: usize = 100;

pub struct VirtualDisk {
    pub data: Vec<Vec<u8>>, // blocks
    pub free_blocks: Vec<bool>,
}

impl VirtualDisk {
    pub fn new() -> Self {
        Self {
            data: vec![vec![0u8; BLOCK_SIZE]; TOTAL_BLOCKS],
            free_blocks: vec![true; TOTAL_BLOCKS],
        }
    }

    pub fn allocate_block(&mut self) -> Option<usize> {
        for (i, free) in self.free_blocks.iter_mut().enumerate() {
            if *free {
                *free = false;
                return Some(i);
            }
        }
        None
    }

    pub fn free_block(&mut self, index: usize) {
        self.free_blocks[index] = true;
    }

    pub fn write_block(&mut self, index: usize, data: &[u8]) {
        let len = data.len().min(BLOCK_SIZE);
        self.data[index][..len].copy_from_slice(&data[..len]);
    }

    pub fn read_block(&self, index: usize) -> &[u8] {
        &self.data[index]
    }

    // Save entire virtual disk to binary file
    pub fn save_to_file(&self, path: &str) {
        let mut file = File::create(path).unwrap();
        for block in &self.data {
            file.write_all(block).unwrap();
        }
        println!("Disk saved to {}", path);
    }

    // Load from binary file
    pub fn load_from_file(path: &str) -> Self {
        let mut file = OpenOptions::new().read(true).open(path).unwrap();
        let mut data = vec![vec![0u8; BLOCK_SIZE]; TOTAL_BLOCKS];
        for block in &mut data {
            file.read_exact(block).unwrap();
        }
        println!("Disk loaded from {}", path);
        Self {
            data,
            free_blocks: vec![true; TOTAL_BLOCKS],
        }
    }
}
