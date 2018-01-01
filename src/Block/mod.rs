extern crate Oxide;
extern crate time;
extern crate crypto;





pub struct Block {
    index: u32,
    hash: Option<str>,
    parent_hash: Option<str>,
    timestamp: str,
    meta: Metadata
}

const GENESIS_BLOCK: Block = Block {
    index: 0;
    hash: "Genesis block.",
    parent_hash: None,
    timestamp: "Sat, 30 Dec 2017 08:19:06 +0000",
    meta: Meta::blank() //no metadata for the genesis block.
}

impl Block {
    fn new(parent: Block) -> Block {
        let new_block = Block {
            index: parent.index() + 1,
            hash: None,
            parent_hash: parent.hash(),
            timestamp: "", // get current time
            meta: Meta::new() // new metadata
        }

        new_block.set_hash();
    }

    fn index(&self) -> u32 { //no lifetime annotation b/c u32 implements Move?
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

    /** Compute and set my own hash. */
    fn set_hash(&self) -> Result<str>{
        if let self.hash = Some(str) {
            Err("Hash already set")
        }

        let mut hasher = Sha256::new();
        let key: str = vec![&self.index.to_string(),
                            self.parent_hash(),
                            self.timestamp(),
                            self.meta.to_string()
        ].join("|");
        
        hasher.input_str(key);
        self.hash = hasher.result_str())

        Ok("Hash set")
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
        blank()
    }

    fn blank() -> Metadata {
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

