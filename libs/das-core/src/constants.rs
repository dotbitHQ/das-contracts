use alloc::vec;
use alloc::vec::Vec;

pub use ckb_std::ckb_types::core::ScriptHashType;
use ckb_std::ckb_types::packed::*;

use super::types::ScriptLiteral;
use super::util;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ScriptType {
    Lock,
    Type,
}

#[derive(Debug, Clone)]
pub enum TypeScript {
    AccountCellType,
    AccountSaleCellType,
    AccountAuctionCellType,
    ApplyRegisterCellType,
    BalanceCellType,
    ConfigCellType,
    IncomeCellType,
    OfferCellType,
    PreAccountCellType,
    ProposalCellType,
    ReverseRecordCellType,
    SubAccountCellType,
    ReverseRecordRootCellType,
    EIP712Lib,
}

#[derive(Debug)]
pub enum LockScript {
    AlwaysSuccessLock,
    DasLock,
    Secp256k1Blake160SignhashLock,
    Secp256k1Blake160MultisigLock,
}

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u8)]
pub enum OracleCellType {
    Quote = 0,
    Time = 1,
    Height = 2,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum CellField {
    Capacity,
    Lock,
    Type,
    Data,
}

pub const CKB_HASH_DIGEST: usize = 32;
pub const CKB_HASH_PERSONALIZATION: &[u8] = b"ckb-default-hash";

pub const CELL_BASIC_CAPACITY: u64 = 6_100_000_000;

pub const RATE_BASE: u64 = 10_000;

pub const ACCOUNT_ID_LENGTH: usize = 20;
pub const ACCOUNT_SUFFIX: &str = ".bit";
pub const ACCOUNT_MAX_PRICED_LENGTH: u8 = 8;

pub const CUSTOM_KEYS_NAMESPACE: &[u8] = b"0123456789abcdefghijklmnopqrstuvwxyz_";
pub const COIN_TYPE_DIGITS: &[u8] = b"0123456789";

pub const SECP_SIGNATURE_SIZE: usize = 65;
// This is smaller than the real data type in solidity, but it is enough for now.
pub const EIP712_CHAINID_SIZE: usize = 8;

pub const DAY_SEC: u64 = 86400;
pub const DAYS_OF_YEAR: u64 = 365;
pub const YEAR_SEC: u64 = DAY_SEC * DAYS_OF_YEAR;

pub const PRE_ACCOUNT_CELL_TIMEOUT: u64 = DAY_SEC;
pub const PRE_ACCOUNT_CELL_SHORT_TIMEOUT: u64 = 3600;

pub const CROSS_CHAIN_BLACK_ARGS: [u8; 20] = [0; 20];

pub const TYPE_ID_CODE_HASH: [u8; 32] = [
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 84, 89, 80, 69, 95, 73, 68,
];

pub fn super_lock() -> Script {
    #[cfg(feature = "dev")]
    let super_lock = ScriptLiteral {
        code_hash: [
            143, 45, 124, 176, 101, 18, 242, 119, 114, 7, 70, 29, 16, 11, 5, 98, 176, 33, 50, 50, 161, 189, 112, 38,
            30, 87, 243, 127, 220, 97, 72, 61,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![0; 20],
    };

    #[cfg(feature = "local")]
    let super_lock = ScriptLiteral {
        code_hash: [
            155, 215, 224, 111, 62, 207, 75, 224, 242, 252, 210, 24, 139, 35, 241, 185, 252, 200, 142, 93, 75, 101,
            168, 99, 123, 23, 114, 59, 189, 163, 204, 232,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![
            188, 80, 42, 52, 164, 48, 227, 225, 103, 200, 42, 36, 219, 111, 146, 55, 177, 94, 191, 53,
        ],
    };

    #[cfg(feature = "testnet")]
    let super_lock = ScriptLiteral {
        code_hash: [
            155, 215, 224, 111, 62, 207, 75, 224, 242, 252, 210, 24, 139, 35, 241, 185, 252, 200, 142, 93, 75, 101,
            168, 99, 123, 23, 114, 59, 189, 163, 204, 232,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![
            188, 80, 42, 52, 164, 48, 227, 225, 103, 200, 42, 36, 219, 111, 146, 55, 177, 94, 191, 53,
        ],
    };

    #[cfg(feature = "mainnet")]
    let super_lock = ScriptLiteral {
        code_hash: [
            92, 80, 105, 235, 8, 87, 239, 198, 94, 27, 202, 12, 7, 223, 52, 195, 22, 99, 179, 98, 47, 211, 135, 108,
            135, 99, 32, 252, 150, 52, 226, 168,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![
            193, 38, 99, 94, 206, 86, 124, 113, 197, 15, 116, 130, 197, 219, 128, 96, 56, 82, 195, 6,
        ],
    };

    util::script_literal_to_script(super_lock)
}

pub fn das_wallet_lock() -> Script {
    #[cfg(feature = "dev")]
    let das_wallet_lock = ScriptLiteral {
        code_hash: [
            143, 45, 124, 176, 101, 18, 242, 119, 114, 7, 70, 29, 16, 11, 5, 98, 176, 33, 50, 50, 161, 189, 112, 38,
            30, 87, 243, 127, 220, 97, 72, 61,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };

    #[cfg(feature = "local")]
    let das_wallet_lock = ScriptLiteral {
        code_hash: [
            157, 111, 41, 25, 227, 40, 243, 33, 125, 125, 211, 218, 181, 247, 206, 233, 216, 224, 98, 190, 230, 168,
            13, 93, 5, 205, 73, 92, 163, 65, 99, 120,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };

    #[cfg(feature = "testnet")]
    let das_wallet_lock = ScriptLiteral {
        code_hash: [
            155, 215, 224, 111, 62, 207, 75, 224, 242, 252, 210, 24, 139, 35, 241, 185, 252, 200, 142, 93, 75, 101,
            168, 99, 123, 23, 114, 59, 189, 163, 204, 232,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![
            239, 191, 73, 127, 117, 47, 247, 166, 85, 168, 236, 111, 60, 143, 63, 234, 174, 214, 228, 16,
        ],
    };

    #[cfg(feature = "mainnet")]
    let das_wallet_lock = ScriptLiteral {
        code_hash: [
            92, 80, 105, 235, 8, 87, 239, 198, 94, 27, 202, 12, 7, 223, 52, 195, 22, 99, 179, 98, 47, 211, 135, 108,
            135, 99, 32, 252, 150, 52, 226, 168,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![
            193, 38, 99, 94, 206, 86, 124, 113, 197, 15, 116, 130, 197, 219, 128, 96, 56, 82, 195, 6,
        ],
    };

    util::script_literal_to_script(das_wallet_lock)
}

pub fn das_lock() -> Script {
    #[cfg(feature = "dev")]
    let das_lock: ScriptLiteral = ScriptLiteral {
        code_hash: [
            235, 210, 202, 67, 121, 125, 241, 234, 226, 31, 90, 13, 32, 160, 154, 56, 81, 190, 171, 6, 60, 160, 109,
            123, 134, 161, 225, 232, 239, 156, 118, 152,
        ],
        hash_type: ScriptHashType::Type,
        args: Vec::new(),
    };

    #[cfg(feature = "local")]
    let das_lock: ScriptLiteral = ScriptLiteral {
        code_hash: [
            89, 52, 16, 137, 210, 202, 237, 168, 209, 186, 241, 211, 135, 176, 100, 84, 249, 115, 140, 61, 28, 36, 81,
            174, 51, 44, 6, 228, 46, 179, 38, 243,
        ],
        hash_type: ScriptHashType::Type,
        args: Vec::new(),
    };

    #[cfg(feature = "testnet")]
    let das_lock: ScriptLiteral = ScriptLiteral {
        code_hash: [
            50, 109, 241, 102, 227, 240, 169, 0, 160, 174, 224, 67, 227, 26, 77, 234, 15, 1, 234, 51, 7, 230, 226, 53,
            240, 157, 27, 66, 32, 183, 95, 189,
        ],
        hash_type: ScriptHashType::Type,
        args: Vec::new(),
    };

    #[cfg(feature = "mainnet")]
    let das_lock: ScriptLiteral = ScriptLiteral {
        code_hash: [
            147, 118, 195, 181, 129, 25, 66, 150, 10, 132, 102, 145, 225, 110, 71, 124, 244, 61, 124, 127, 166, 84, 6,
            124, 153, 72, 223, 205, 9, 163, 33, 55,
        ],
        hash_type: ScriptHashType::Type,
        args: Vec::new(),
    };

    util::script_literal_to_script(das_lock)
}

pub fn cross_chain_lock() -> Script {
    #[cfg(not(feature = "mainnet"))]
    let cross_chain_lock: ScriptLiteral = ScriptLiteral {
        code_hash: [
            92, 80, 105, 235, 8, 87, 239, 198, 94, 27, 202, 12, 7, 223, 52, 195, 22, 99, 179, 98, 47, 211, 135, 108,
            135, 99, 32, 252, 150, 52, 226, 168,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![
            7, 189, 69, 77, 230, 250, 195, 106, 195, 109, 54, 2, 32, 199, 40, 195, 154, 36, 73, 87,
        ],
    };

    #[cfg(feature = "mainnet")]
    let cross_chain_lock: ScriptLiteral = ScriptLiteral {
        code_hash: [
            92, 80, 105, 235, 8, 87, 239, 198, 94, 27, 202, 12, 7, 223, 52, 195, 22, 99, 179, 98, 47, 211, 135, 108,
            135, 99, 32, 252, 150, 52, 226, 168,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![
            127, 242, 240, 247, 135, 3, 140, 22, 193, 245, 198, 43, 4, 190, 100, 115, 251, 115, 179, 142,
        ],
    };

    util::script_literal_to_script(cross_chain_lock)
}

pub fn profit_manager_lock() -> Script {
    #[cfg(any(feature = "dev", feature = "local"))]
    let lock = ScriptLiteral {
        // CAREFUL: If you edit the code_hash here, you need also make the code_hash in fn das_wallet_lock() consistent.
        code_hash: [
            143, 45, 124, 176, 101, 18, 242, 119, 114, 7, 70, 29, 16, 11, 5, 98, 176, 33, 50, 50, 161, 189, 112, 38,
            30, 87, 243, 127, 220, 97, 72, 61,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };

    #[cfg(feature = "testnet")]
    let lock = ScriptLiteral {
        code_hash: [
            155, 215, 224, 111, 62, 207, 75, 224, 242, 252, 210, 24, 139, 35, 241, 185, 252, 200, 142, 93, 75, 101,
            168, 99, 123, 23, 114, 59, 189, 163, 204, 232,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![
            208, 225, 249, 167, 154, 185, 54, 24, 33, 203, 195, 179, 31, 204, 238, 9, 76, 175, 172, 208,
        ],
    };

    #[cfg(feature = "mainnet")]
    let lock = ScriptLiteral {
        code_hash: [
            155, 215, 224, 111, 62, 207, 75, 224, 242, 252, 210, 24, 139, 35, 241, 185, 252, 200, 142, 93, 75, 101,
            168, 99, 123, 23, 114, 59, 189, 163, 204, 232,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    };

    util::script_literal_to_script(lock)
}

pub fn time_cell_type() -> Script {
    #[cfg(any(feature = "dev", feature = "local"))]
    let time_cell_type = ScriptLiteral {
        code_hash: [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![1],
    };

    #[cfg(feature = "testnet")]
    let time_cell_type = ScriptLiteral {
        code_hash: [
            150, 36, 140, 222, 251, 9, 238, 217, 16, 1, 138, 132, 124, 251, 81, 173, 4, 76, 45, 125, 182, 80, 17, 41,
            49, 118, 14, 62, 243, 74, 126, 154,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![1],
    };

    #[cfg(feature = "mainnet")]
    let time_cell_type = ScriptLiteral {
        code_hash: [
            158, 83, 123, 245, 184, 236, 4, 76, 163, 245, 51, 85, 232, 121, 243, 253, 136, 50, 33, 126, 74, 155, 65,
            217, 153, 76, 240, 197, 71, 36, 26, 121,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![1],
    };

    util::script_literal_to_script(time_cell_type)
}

pub fn height_cell_type() -> Script {
    #[cfg(feature = "dev")]
    let height_cell_type = ScriptLiteral {
        code_hash: [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![2],
    };

    #[cfg(feature = "local")]
    let height_cell_type = ScriptLiteral {
        code_hash: [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![2],
    };

    #[cfg(feature = "testnet")]
    let height_cell_type = ScriptLiteral {
        code_hash: [
            150, 36, 140, 222, 251, 9, 238, 217, 16, 1, 138, 132, 124, 251, 81, 173, 4, 76, 45, 125, 182, 80, 17, 41,
            49, 118, 14, 62, 243, 74, 126, 154,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![2],
    };

    #[cfg(feature = "mainnet")]
    let height_cell_type = ScriptLiteral {
        code_hash: [
            158, 83, 123, 245, 184, 236, 4, 76, 163, 245, 51, 85, 232, 121, 243, 253, 136, 50, 33, 126, 74, 155, 65,
            217, 153, 76, 240, 197, 71, 36, 26, 121,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![2],
    };

    util::script_literal_to_script(height_cell_type)
}

pub fn quote_cell_type() -> Script {
    #[cfg(feature = "dev")]
    let quote_cell_type = ScriptLiteral {
        code_hash: [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![0],
    };

    #[cfg(feature = "local")]
    let quote_cell_type = ScriptLiteral {
        code_hash: [
            1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![0],
    };

    #[cfg(feature = "testnet")]
    let quote_cell_type = ScriptLiteral {
        code_hash: [
            150, 36, 140, 222, 251, 9, 238, 217, 16, 1, 138, 132, 124, 251, 81, 173, 4, 76, 45, 125, 182, 80, 17, 41,
            49, 118, 14, 62, 243, 74, 126, 154,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![0],
    };

    #[cfg(feature = "mainnet")]
    let quote_cell_type = ScriptLiteral {
        code_hash: [
            158, 83, 123, 245, 184, 236, 4, 76, 163, 245, 51, 85, 232, 121, 243, 253, 136, 50, 33, 126, 74, 155, 65,
            217, 153, 76, 240, 197, 71, 36, 26, 121,
        ],
        hash_type: ScriptHashType::Type,
        args: vec![0],
    };

    util::script_literal_to_script(quote_cell_type)
}

#[cfg(feature = "dev")]
pub const CONFIG_CELL_TYPE: ScriptLiteral = ScriptLiteral {
    code_hash: [
        123, 140, 211, 76, 213, 227, 55, 74, 169, 223, 172, 16, 140, 241, 35, 54, 233, 49, 147, 62, 137, 47, 84, 71,
        30, 70, 159, 193, 179, 26, 60, 202,
    ],
    hash_type: ScriptHashType::Type,
    args: Vec::new(),
};

#[cfg(feature = "local")]
pub const CONFIG_CELL_TYPE: ScriptLiteral = ScriptLiteral {
    code_hash: [
        220, 123, 89, 43, 36, 20, 178, 229, 192, 147, 85, 89, 198, 7, 98, 141, 137, 24, 161, 12, 127, 28, 226, 8, 187,
        193, 50, 2, 72, 61, 5, 42,
    ],
    hash_type: ScriptHashType::Type,
    args: Vec::new(),
};

#[cfg(feature = "testnet")]
pub const CONFIG_CELL_TYPE: ScriptLiteral = ScriptLiteral {
    code_hash: [
        3, 10, 194, 172, 217, 192, 22, 249, 164, 171, 19, 213, 44, 36, 77, 35, 170, 234, 99, 110, 12, 189, 56, 110,
        198, 96, 183, 153, 116, 148, 101, 23,
    ],
    hash_type: ScriptHashType::Type,
    args: Vec::new(),
};

#[cfg(feature = "mainnet")]
pub const CONFIG_CELL_TYPE: ScriptLiteral = ScriptLiteral {
    code_hash: [
        144, 59, 255, 2, 33, 183, 43, 47, 93, 84, 146, 54, 182, 49, 35, 75, 41, 79, 16, 245, 62, 108, 199, 50, 138,
        240, 119, 118, 227, 42, 102, 64,
    ],
    hash_type: ScriptHashType::Type,
    args: Vec::new(),
};

pub fn config_cell_type() -> Script {
    util::script_literal_to_script(CONFIG_CELL_TYPE)
}

pub fn always_success_lock() -> Script {
    #[cfg(feature = "dev")]
    let always_success_lock = ScriptLiteral {
        code_hash: [
            52, 240, 82, 252, 69, 95, 206, 124, 113, 244, 144, 95, 34, 54, 83, 165, 251, 230, 66, 97, 198, 178, 83,
            113, 36, 222, 0, 241, 213, 40, 32, 233,
        ],
        hash_type: ScriptHashType::Type,
        args: Vec::new(),
    };

    #[cfg(feature = "local")]
    let always_success_lock = ScriptLiteral {
        code_hash: [
            184, 243, 231, 77, 189, 72, 86, 149, 58, 151, 112, 104, 42, 255, 194, 137, 221, 0, 152, 153, 45, 17, 214,
            103, 205, 243, 84, 151, 226, 103, 190, 50,
        ],
        hash_type: ScriptHashType::Type,
        args: Vec::new(),
    };

    #[cfg(feature = "testnet")]
    let always_success_lock = ScriptLiteral {
        code_hash: [
            241, 239, 97, 182, 151, 117, 8, 217, 236, 86, 254, 67, 57, 154, 1, 229, 118, 8, 106, 118, 207, 15, 124,
            104, 125, 20, 24, 51, 94, 140, 64, 31,
        ],
        hash_type: ScriptHashType::Type,
        args: Vec::new(),
    };

    #[cfg(feature = "mainnet")]
    let always_success_lock = ScriptLiteral {
        code_hash: [
            48, 62, 173, 55, 190, 94, 235, 252, 243, 80, 72, 71, 21, 85, 56, 203, 98, 58, 38, 242, 55, 96, 157, 242,
            75, 210, 150, 117, 12, 18, 48, 120,
        ],
        hash_type: ScriptHashType::Type,
        args: Vec::new(),
    };

    util::script_literal_to_script(always_success_lock)
}

pub fn signall_lock() -> Script {
    #[cfg(feature = "dev")]
    let signall_lock = ScriptLiteral {
        // CAREFUL: If you edit the code_hash here, you need also make the code_hash in fn das_wallet_lock() consistent.
        code_hash: [
            143, 45, 124, 176, 101, 18, 242, 119, 114, 7, 70, 29, 16, 11, 5, 98, 176, 33, 50, 50, 161, 189, 112, 38,
            30, 87, 243, 127, 220, 97, 72, 61,
        ],
        hash_type: ScriptHashType::Type,
        args: Vec::new(),
    };

    #[cfg(not(feature = "dev"))]
    let signall_lock = ScriptLiteral {
        code_hash: [
            155, 215, 224, 111, 62, 207, 75, 224, 242, 252, 210, 24, 139, 35, 241, 185, 252, 200, 142, 93, 75, 101,
            168, 99, 123, 23, 114, 59, 189, 163, 204, 232,
        ],
        hash_type: ScriptHashType::Type,
        args: Vec::new(),
    };

    util::script_literal_to_script(signall_lock)
}

pub fn multisign_lock() -> Script {
    #[cfg(feature = "dev")]
    let multisign_lock = ScriptLiteral {
        code_hash: [
            75, 9, 147, 1, 216, 0, 229, 2, 51, 47, 158, 77, 1, 173, 66, 126, 7, 230, 225, 199, 153, 166, 131, 41, 132,
            58, 196, 115, 232, 50, 24, 72,
        ],
        hash_type: ScriptHashType::Type,
        args: Vec::new(),
    };

    #[cfg(not(feature = "dev"))]
    let multisign_lock = ScriptLiteral {
        code_hash: [
            92, 80, 105, 235, 8, 87, 239, 198, 94, 27, 202, 12, 7, 223, 52, 195, 22, 99, 179, 98, 47, 211, 135, 108,
            135, 99, 32, 252, 150, 52, 226, 168,
        ],
        hash_type: ScriptHashType::Type,
        args: Vec::new(),
    };

    util::script_literal_to_script(multisign_lock)
}
