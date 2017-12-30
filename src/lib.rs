extern crate time;

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
            parent.index() + 1,
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

    fn hash(&self) -> &str {
        self.hash
    }

    fn parent_hash(&self) {
        self.parent_hash
    }

    fn timestamp(&self) {
        self.timestamp
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

struct Transaction {
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

