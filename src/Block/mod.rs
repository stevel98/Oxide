extern crate time;
extern crate crypto;

use crypto::sha2::Sha256;
use crypto::digest::Digest;
use serde::Deserialize;
use serde::Serialize;


#[derive(Serialize, Deserialize, Clone)]
pub struct Block {
    index: u32,
    hash: String,
    parent_hash: String,
    timestamp: String,
    meta: Metadata,
    nonce: i32
}

impl Block {
    pub fn new(parent: Block, nonce: i32) -> Block {
        let index: u32 = parent.index() + 1;
        let parent_hash = String::from(parent.hash());
        let timestamp = String::from(""); //get current time
        let meta = Metadata::new();
        let mut hasher = Sha256::new();

        let key = vec![index.to_string(),
                               parent_hash.clone(),
                               timestamp.clone(),
                               meta.to_string(),
                               nonce.to_string()
        ].join("|");
        
        hasher.input_str(&key[..]);
        let hash = hasher.result_str();

        Block {
            index,
            hash,
            parent_hash,
            timestamp,
            meta,
            nonce
        }
    }

    pub fn genesis() -> Block {
        let index = 0;
        let parent_hash = String::from("GENESIS");
        let timestamp = String::from("Sun, 07 Jan 2018 06:23:23 GMT");
        let meta = Metadata::new();
        let nonce = 0;
        let hash = String::from("GENESIS");

        return Block {
            index,
            hash,
            parent_hash,
            timestamp,
            meta,
            nonce
        }
    }
}

impl Block {
    pub fn index(&self) -> u32 {
        self.index
    }

    pub fn hash(&self) -> &str {
        &self.hash
    }

    pub fn parent_hash(&self) -> &str {
        &self.parent_hash
    }

    pub fn timestamp(&self) -> &str {
        &self.timestamp
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct Metadata {
}

impl Metadata {
    fn new() -> Metadata {
        Metadata {}
    }

    fn blank() -> Metadata {
        Self::new()
    }

    /** The metadata for the genesis block. */
    fn genesis() -> Metadata {
        Self::new()
    }
}

impl ToString for Metadata {
    fn to_string(&self) -> String {
        String::from("metadata")
    }
}

#[derive(Serialize, Deserialize)]
struct Transaction {
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

