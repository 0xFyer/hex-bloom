use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug)]
pub struct LinkedHashX {
    root: Vec<u8>,
    root0: Vec<u8>,
    hashes: HashMap<Vec<u8>, Vec<u8>>,
}

impl LinkedHashX {
    pub fn new(data: Vec<Vec<u8>>) -> LinkedHashX {
        let mut root = vec![0; 32];
        let mut root0 = vec![0; 32];
        let mut hashes = HashMap::new();
        for block in data {
            let mut hasher = Sha256::new();
            hasher.update(&block);
            let hash = hasher.finalize();
            root = xor(&root, &hash);
            root0 = xor(&root0, &hash);
            hashes.insert(block, hash.to_vec());
        }
        LinkedHashX {
            root,
            root0,
            hashes,
        }
    }

    pub fn insert(&mut self, data: Vec<u8>) {
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let hash = hasher.finalize();
        self.root = xor(&self.root, &hash);
        self.root0 = xor(&self.root0, &hash);
        self.hashes.insert(data, hash.to_vec());
    }

    pub fn delete(&mut self, data: Vec<u8>) {
        if let Some(hash) = self.hashes.remove(&data) {
            self.root = xor(&self.root, &hash);
            self.root0 = xor(&self.root0, &hash);
        }
    }

    pub fn update(&mut self, old_data: Vec<u8>, new_data: Vec<u8>) {
        self.delete(old_data);
        self.insert(new_data);
    }
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}
