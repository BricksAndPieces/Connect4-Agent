use serde::{Serialize, Deserialize};

// smallest prime number larger than 8 million (64mb table)
pub const TABLE_SIZE: usize = 8_388_593;

// todo: maybe take table size as a param so it can be dynamically sized
#[derive(Serialize, Deserialize, Clone)]
pub struct TranspositionTable {
    keys: Vec<u32>,
    vals: Vec<i8>,
}

impl TranspositionTable {
    pub fn new() -> Self {
        Self {
            keys: vec![!0; TABLE_SIZE],
            vals: vec![!0; TABLE_SIZE],
        }
    }

    pub fn from(keys: Vec<u32>, vals: Vec<i8>) -> Self {
        Self { keys, vals }
    }

    pub fn set(&mut self, key: u64, value: i8) {
        let index = key as usize % TABLE_SIZE;
        self.keys[index] = key as u32;
        self.vals[index] = value;
    }

    pub fn get(&self, key: u64) -> Option<i8> {
        let index = key as usize % TABLE_SIZE;
        if self.keys[index] == key as u32 {
            return Some(self.vals[index]);
        }

        None
    }

    pub fn keys(&self) -> Vec<u32> {
        self.keys.clone()
    }

    pub fn vals(&self) -> Vec<i8> {
        self.vals.clone()
    }
}
