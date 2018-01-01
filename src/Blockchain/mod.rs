extern crate Oxide;
extern crate time;
extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

pub struct Blockchain {
    tail: Block,
}

impl Blockchain {
    /** Return whether NEW_BLOCK is valid. */
    fn validate_new_block(&self, &Block new_block) -> bool {
        false
    }
}

