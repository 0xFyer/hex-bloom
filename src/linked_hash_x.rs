use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
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
        let mut hasher = Sha256::new();
        for item in data {
            hasher.update(&item);
            let hash = hasher.finalize_reset();
            root = xor(&root, &hash);
            root0 = xor(&root0, &hash);
            hashes.insert(item, hash.to_vec());
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

    pub fn get_root(&self) -> &Vec<u8> {
        &self.root
    }

    pub fn get_keys(&self) -> Vec<Vec<u8>> {
        let mut keys: Vec<Vec<u8>> = vec![];
        for (key, _) in self.hashes.clone().into_iter() {
            keys.push(key);
        }
        keys
    }
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}
