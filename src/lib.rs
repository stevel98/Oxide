extern crate time;
extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

pub struct Blockchain {
    tail: Block,
}

pub struct Block {
    index: u32,
    hash: str,
    parent_hash: str,
    timestamp: str,
    meta: Metadata
}

impl Block {
    fn new(parent: Block) -> Block {
        let new_block = Block {
            index: parent.index() + 1,
            hash: "",
            parent_hash: parent.hash(),
            timestamp: "", // get current time
            meta: Meta::new() // new metadata
        }
        // set hash?
    }

    fn index(&self) -> u32 { //no lifetime annotation b/c u32 implements Move trait?
        self.index
    }

    fn hash(&self) -> &]tr {
        self.hash
    }

    fn parent_hash(&self) {
        self.parent_hash
    }

    fn timestamp(&self) {
        self.timestamp
    }

    /** Compute my own hash. */
    fn compute_hash(&self) -> str{
        let mut hasher = Sha256::new();
        let key: str = vec![&self.index.to_string(),
                            self.parent_hash(),
                            self.timestamp(),
                            self.meta.to_string()
        ].join("|");
        
        hasher.input_str(key);
        hasher.result_str()
    }
}

impl Block {

    /** Serialize myself. */
    fn serialize(&self) {
    }
}

struct Metadata {
}

impl Metadata {
    fn new() -> Metadata {
        Metadata{}
    }
}

impl Metadata {
    fn to_string(&self) {
        "metadata"
    }
}

struct Transaction {
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

