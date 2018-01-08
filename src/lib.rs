#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate time;
extern crate crypto;

use self::crypto::digest::Digest;
use self::crypto::sha2::Sha256;

mod block;
mod blockchain;

