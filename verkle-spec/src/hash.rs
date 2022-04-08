use crate::Hasher;
use ethereum_types::H256;
use verkle_trie::{committer::Committer, from_to_bytes::ToBytes, Fr};

pub struct PedersenHasher;
impl Hasher for PedersenHasher {
    fn hash64(bytes64: [u8; 64]) -> H256 {
        use verkle_trie::committer::test::TestCommitter;
        // let hasher = PedersenHasher::new(TestCommitter);
        let chunks = crate::util::chunk64(bytes64);
        let fr_data: Vec<_> = chunks.iter().map(|x| -> Fr { Fr::from(*x) }).collect();
        let bytes = TestCommitter.commit_lagrange(&fr_data[..]).to_bytes();
        return H256::from_slice(&bytes[..]);
        // hasher.hash(&chunks[..])
    }
}

#[cfg(test)]
mod test {
    use crate::hash::{Hasher, PedersenHasher};
    use crate::H256;
    use hex::FromHex;
    #[test]
    fn hash_test() {
        let tests = [
            (
                <[u8;64]>::from_hex("00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").expect("Decoding failed"),
                <[u8;32]>::from_hex("695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf9d4").expect("Decoding failed")
            ),
            (
                <[u8;64]>::from_hex("00020300000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").expect("Decoding failed"),
                <[u8;32]>::from_hex("5010fabfb319bf84136db68445972cdd5476ff2fbf3e5133330b3946b84b4e6a").expect("Decoding failed")
            ),
            (
                <[u8;64]>::from_hex("0071562b71999873db5b286df957af199ec946170000000000000000000000000000000000000000000000000000000000000000000000000000000000000000").expect("Decoding failed"),
                <[u8;32]>::from_hex("6fc5ac021ff2468685885ad7fdb31a0c58d1ee93254a58c9e9e0809187c53e71").expect("Decoding failed")
            )
        ];
        for (input, output) in tests.iter() {
            assert_eq!(
                PedersenHasher::hash64(*input),
                H256::from_slice(&output[..])
            );
        }
    }
}
