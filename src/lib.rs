
struct Blockchain {
    tail: Block,
}

struct Block {
    parent_hash: str,
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
