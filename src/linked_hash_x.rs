use sha2::{Digest, Sha256};
use std::collections::HashMap;

#[derive(Debug)]
pub struct LinkedHashX<T> {
    root: Vec<u8>,
    root0: Vec<u8>,
    hashes: HashMap<T, Vec<u8>>,
}

impl<T: AsRef<[u8]> + std::fmt::Debug + std::cmp::Eq + std::hash::Hash + Clone> LinkedHashX<T> {
    pub fn new(data: Vec<T>) -> LinkedHashX<T> {
        let mut root = vec![0; 32];
        let mut root0 = vec![0; 32];
        let mut hashes = HashMap::new();
        for item in data {
            let mut hasher = Sha256::new();
            hasher.update(item.as_ref());
            let hash = hasher.finalize();
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

    pub fn insert(&mut self, data: T) {
        let mut hasher = Sha256::new();
        hasher.update(data.as_ref());
        let hash = hasher.finalize();
        self.root = xor(&self.root, &hash);
        self.root0 = xor(&self.root0, &hash);
        self.hashes.insert(data, hash.to_vec());
    }

    pub fn delete(&mut self, data: T) {
        if let Some(hash) = self.hashes.remove(&data) {
            self.root = xor(&self.root, &hash);
            self.root0 = xor(&self.root0, &hash);
        }
    }

    pub fn update(&mut self, old_data: T, new_data: T) {
        self.delete(old_data);
        self.insert(new_data);
    }

    pub fn get_root(&self) -> &Vec<u8> {
        &self.root
    }
}

fn xor(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().zip(b.iter()).map(|(a, b)| a ^ b).collect()
}
