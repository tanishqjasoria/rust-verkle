use ethereum_types::H256;
use ethereum_types::U256;

use crate::parameters::{
    BALANCE_LEAF_KEY, CODE_KECCAK_LEAF_KEY, CODE_SIZE_LEAF_KEY, NONCE_LEAF_KEY, VERSION_LEAF_KEY,
};
use crate::util::hash_addr_int;
use crate::Hasher;
use crate::{util::swap_last_byte, Address32};

pub struct Header {
    balance_tree_key: H256,
    version_tree_key: H256,
    code_size_tree_key: H256,
    nonce_tree_key: H256,
    code_keccak_tree_key: H256,
}

impl Header {
    pub fn new<H: Hasher>(address: Address32) -> Header {
        let tree_index = U256::zero();
        Header::with_tree_index::<H>(address, tree_index)
    }

    pub fn with_tree_index<H: Hasher>(addr: Address32, tree_index: U256) -> Header {
        let base_hash = hash_addr_int::<H>(addr, tree_index);

        let version_tree_key = swap_last_byte(base_hash, VERSION_LEAF_KEY);
        let balance_tree_key = swap_last_byte(base_hash, BALANCE_LEAF_KEY);
        let nonce_tree_key = swap_last_byte(base_hash, NONCE_LEAF_KEY);
        let code_keccak_tree_key = swap_last_byte(base_hash, CODE_KECCAK_LEAF_KEY);
        let code_size_tree_key = swap_last_byte(base_hash, CODE_SIZE_LEAF_KEY);

        Header {
            balance_tree_key,
            version_tree_key,
            code_size_tree_key,
            code_keccak_tree_key,
            nonce_tree_key,
        }
    }

    pub fn balance(&self) -> H256 {
        self.balance_tree_key
    }

    pub fn nonce(&self) -> H256 {
        self.nonce_tree_key
    }

    // Backwards compatibility for EXTCODEHASH
    pub fn code_keccak(&self) -> H256 {
        self.code_keccak_tree_key
    }

    //  Backwards compatibility for EXTCODESIZE
    pub fn code_size(&self) -> H256 {
        self.code_size_tree_key
    }

    pub fn version(&self) -> H256 {
        self.version_tree_key
    }
}

#[cfg(test)]
mod test {
    use crate::hash::PedersenHasher;
    use crate::header::Header;
    use crate::{Address32};
    use hex::FromHex;
    #[test]
    fn header_test() {
        let tests = [
            (
                "0000000000000000000000000000000000000000000000000000000000000000",
                [
                    "695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf900",
                    "695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf901",
                    "695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf902",
                    "695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf903",
                    "695921dca3b16c5cc850e94cdd63f573c467669e89cec88935d03474d6bdf904",
                ],
            ),
            (
                "0002030000000000000000000000000000000000000000000000000000000000",
                [
                    "5010fabfb319bf84136db68445972cdd5476ff2fbf3e5133330b3946b84b4e00",
                    "5010fabfb319bf84136db68445972cdd5476ff2fbf3e5133330b3946b84b4e01",
                    "5010fabfb319bf84136db68445972cdd5476ff2fbf3e5133330b3946b84b4e02",
                    "5010fabfb319bf84136db68445972cdd5476ff2fbf3e5133330b3946b84b4e03",
                    "5010fabfb319bf84136db68445972cdd5476ff2fbf3e5133330b3946b84b4e04",
                ],
            ),
            (
                "0071562b71999873db5b286df957af199ec94617000000000000000000000000",
                [
                    "6fc5ac021ff2468685885ad7fdb31a0c58d1ee93254a58c9e9e0809187c53e00",
                    "6fc5ac021ff2468685885ad7fdb31a0c58d1ee93254a58c9e9e0809187c53e01",
                    "6fc5ac021ff2468685885ad7fdb31a0c58d1ee93254a58c9e9e0809187c53e02",
                    "6fc5ac021ff2468685885ad7fdb31a0c58d1ee93254a58c9e9e0809187c53e03",
                    "6fc5ac021ff2468685885ad7fdb31a0c58d1ee93254a58c9e9e0809187c53e04",
                ],
            ),
        ];
        for (input, output) in tests.iter() {
            let input_bytes = <[u8; 32]>::from_hex(input).expect("Error decoding");
            let add = Address32::from_slice(&input_bytes[..]);
            let header = Header::new::<PedersenHasher>(add);

            assert_eq!(
                header.version(),
                Address32::from_slice(&<[u8; 32]>::from_hex(output[0]).expect("Error decoding"))
            );
            assert_eq!(
                header.balance(),
                Address32::from_slice(&<[u8; 32]>::from_hex(output[1]).expect("Error decoding"))
            );
            assert_eq!(
                header.nonce(),
                Address32::from_slice(&<[u8; 32]>::from_hex(output[2]).expect("Error decoding"))
            );
            assert_eq!(
                header.code_keccak(),
                Address32::from_slice(&<[u8; 32]>::from_hex(output[3]).expect("Error decoding"))
            );
            assert_eq!(
                header.code_size(),
                Address32::from_slice(&<[u8; 32]>::from_hex(output[4]).expect("Error decoding"))
            );
        }
    }
}
