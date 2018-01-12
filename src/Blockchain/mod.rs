extern crate time;
extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;
use block::Block;
use std::collections::HashMap;


pub struct Blockchain {
    blocks: HashMap<String, String>,
    head: String,
    fork_heads: Vec<String>
}

impl Blockchain {
    /** Return a new blockchain consisting only of the genesis block. */
    pub fn new_chain() -> Blockchain {
        let mut chain: Blockchain = Self::new();
        chain.add(Block::genesis());

        chain
    }

    /** Return a new, empty blockchain. This function is not exposed. */
    fn new() -> Blockchain {
        let fork_heads: Vec<String> = vec![];

        Blockchain {
            blocks: HashMap::new(),
            head: String::from("GENESIS"),
            fork_heads
        }
    }
}

impl Blockchain {
    /** Deserialize and return the block with hash BLOCK_HASH. */
    fn get_block(
        &self,
        block_hash: &str
    ) -> Result<Block, &'static str> {
        Ok(Block::new(Block::genesis(), 1)) 
    }

    /** Attempt to add BLOCK to the blockchain, regardless of whether its 
     *  parent is the head of the longest fork. */
    fn add(&mut self, block: Block) -> Result<(), &'static str> {
        let parent = self.find_parent(&block)?;
        if !self.verify_block(&block, &parent) {
            return Err("invalid block")
        }

        ///serialize block here?

        let owned_hash = block.hash().to_string();
        let owned_parent_hash = parent.hash().to_string();
        self.advance_head(owned_hash.clone(), owned_parent_hash.clone());
        self.blocks.insert(owned_hash, owned_parent_hash);

        Ok(())
    }

    /** Verify that BLOCK is a valid addition to the me, assuming that its
     *  parent is PARENT. */
    fn verify_block(
        &self,
        block: &Block,
        parent: &Block
    ) -> bool {
        /* {Also return false if the block is already in me (just check hash) */
        true
    }

    /** Return the parent of BLOCK if it is in our blockchain, otherwise
     *  Err. */
    fn find_parent(&self, block: &Block) -> Result<Block, &'static str> {
        for (existing_hash, _ignored) in &self.blocks {
            if block.parent_hash() == &existing_hash[..] {
                return self.get_block(existing_hash);
            }
        }

        Err("Block's parent is invalid or is not a head.")
    }

    /** Update self.fork_heads to make NEW_HEAD the head of OLD_HEAD's fork.
     *  Assumes that NEW_HEAD and OLD_HEAD are valid hashes of valid blocks, 
     *  that OLD_HEAD is the head of a fork, and that OLD_HEAD is the parent of
     *  NEW_HEAD. */
    fn advance_head(&mut self, new_head: String, old_head: String) {
        for i in 0..self.fork_heads.len() {
            if self.fork_heads[i] == old_head {
                self.fork_heads[i] = new_head;
                return; //compiler's not smart enough to know this happens only once :(
            }
        }
    }

    /** Return the hash of the longest fork's head. */
    fn longest_head(&self) -> &str {
        &self.head[..]
    }

    /** Return the hashes of the head of all my forks. */
    fn fork_heads(&self) -> &Vec<String> {
        &self.fork_heads
    }
}

impl Blockchain {
    /** Update myself with the valid blocks in NEW_BLOCKS. */
    fn update(&mut self, new_blocks: Vec<Block>) {
        let mut changed = true;

        while changed { // blocks added iff they're valid and have a parent currently in the chain
            changed = false;
            let valid_blocks: Vec<Block> = new_blocks.clone().into_iter().filter_map(|block| {
                if let Ok(parent) = self.find_parent(&block) {
                    if self.verify_block(&block, &parent) {
                        return Some(block)
                    }          
                }
                None
            }).collect();
            if valid_blocks.len() > 0 {
                changed = true;
            }
            for block in valid_blocks {
                self.add(block);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(3, 3);
    }
}
