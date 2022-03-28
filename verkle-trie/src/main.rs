use sha2::{Digest, Sha256};

use ark_ec::ProjectiveCurve;
use once_cell::sync::Lazy;
use verkle_db::{BareMetalDiskDb, RocksDb};
use verkle_trie::{database::{memory_db::MemoryDb, VerkleDb}, trie::Trie, VerkleConfig, Config, TrieTrait};
use verkle_trie::committer::test::TestCommitter;

fn main() {

    use tempfile::tempdir;
    let temp_dir = tempdir().unwrap();

    let db = VerkleDb::<RocksDb>::from_path(&temp_dir);

    let committer = TestCommitter;
    let config = Config { db, committer };
    let mut trie = Trie::new(config);
    let key = [
        121, 85, 7, 198, 131, 230, 143, 90, 165, 129, 173, 81, 186, 89, 19, 191, 13, 107, 197,
        120, 243, 229, 224, 183, 72, 25, 6, 8, 210, 159, 31, 0,
    ];
    let value = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 2,
    ];
    trie.insert_single(key, value);
    let sr1 = trie.root_hash();
    trie.flush_database();
    let sr2 = trie.root_hash();

    // use std::time::Instant;

    // let now = Instant::now();
    // for key in KEYS_10K.iter() {
    //     trie.insert(*key, *key);
    // }
    // println!("insert keys time : {}", now.elapsed().as_millis());
    // let now = Instant::now();
    // trie.flush_database();
    // println!(
    //     "total flush time (inc write to batch) : {}",
    //     now.elapsed().as_millis()
    // );
}
