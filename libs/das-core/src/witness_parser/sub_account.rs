use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::convert::{TryFrom, TryInto};
use core::str::FromStr;

use ckb_std::ckb_constants::Source;
use ckb_std::error::SysError;
use ckb_std::syscalls;
use das_types::constants::*;
use das_types::packed::*;
use das_types::prelude::*;
#[cfg(all(debug_assertions))]
use das_types::prettier::Prettier;
use simple_ast::{types as ast_types, util as ast_util};

use super::super::error::*;
use super::super::{data_parser, util};

// Binary format: 'das'(3) + DATA_TYPE(4) + binary_data

#[derive(Debug)]
pub struct SubAccountMintSignWitness {
    // The index of the transaction's witnesses, this field is mainly used for debug.
    pub index: usize,
    pub version: u32,
    pub signature: Vec<u8>,
    pub sign_role: Option<LockRole>,
    pub sign_type: Option<DasLockType>,
    pub sign_args: Vec<u8>,
    pub expired_at: u64,
    pub account_list_smt_root: Vec<u8>,
}

#[derive(Debug)]
pub struct SubAccountWitness {
    // The index of the transaction's witnesses, this field aaaaaaaaaaaaaaaaaaaais mainly used for debug.
    pub index: usize,
    pub version: u32,
    pub signature: Vec<u8>,
    pub sign_role: Option<LockRole>,
    pub sign_type: Option<DasLockType>,
    pub sign_args: Vec<u8>,
    pub sign_expired_at: u64,
    pub new_root: Vec<u8>,
    pub proof: Vec<u8>,
    pub action: SubAccountAction,
    pub sub_account: SubAccount,
    pub edit_key: Vec<u8>,
    pub edit_value: SubAccountEditValue,
    pub edit_value_bytes: Vec<u8>,
}

#[derive(Debug)]
pub enum SubAccountEditValue {
    None,
    Owner(Vec<u8>),
    Manager(Vec<u8>),
    Records(Records),
    Proof,
    Channel(Vec<u8>, u64),
    ExpiredAt(u64),
}

pub struct SubAccountWitnessesIter<'a> {
    parser: &'a SubAccountWitnessesParser,
    current: usize,
}

impl<'a> Iterator for SubAccountWitnessesIter<'a> {
    type Item = Result<SubAccountWitness, Box<dyn ScriptError>>;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.parser.get(self.current);
        self.current += 1;

        ret
    }
}

#[derive(Debug)]
pub struct SubAccountWitnessesParser {
    pub flag: SubAccountConfigFlag,
    pub contains_creation: bool,
    pub contains_edition: bool,
    pub contains_renew: bool,
    pub contains_recycle: bool,
    pub mint_sign_index: Option<usize>,
    pub renew_sign_index: Option<usize>,
    pub price_rule_indexes: Vec<usize>,
    pub preserved_rule_indexes: Vec<usize>,
    pub indexes: Vec<usize>,
}

impl SubAccountWitnessesParser {
    pub fn new(flag: SubAccountConfigFlag) -> Result<Self, Box<dyn ScriptError>> {
        let mut contains_creation = false;
        let mut contains_edition = false;
        let mut contains_renew = false;
        let mut contains_recycle = false;
        let mut mint_sign_index = None;
        let mut renew_sign_index = None;
        let mut price_rule_indexes = Vec::new();
        let mut preserved_rule_indexes = Vec::new();
        let mut indexes = Vec::new();
        let mut i = 0;
        let mut das_witnesses_started = false;
        let mut count = 0;
        loop {
            let mut buf = [0u8; (WITNESS_HEADER_BYTES
                + WITNESS_TYPE_BYTES
                + SUB_ACCOUNT_WITNESS_VERSION_BYTES
                + SUB_ACCOUNT_WITNESS_ACTION_BYTES)];
            let ret = syscalls::load_witness(&mut buf, 0, i, Source::Input);

            match ret {
                // Data which length is too short to be DAS witnesses, so ignore it.
                Ok(_) => i += 1,
                Err(SysError::LengthNotEnough(_)) => {
                    if let Some(raw) = buf.get(..WITNESS_HEADER_BYTES) {
                        if das_witnesses_started {
                            // If it is parsing DAS witnesses currently, end the parsing.
                            if raw != &WITNESS_HEADER {
                                break;
                            }
                        } else {
                            // If it is not parsing DAS witnesses currently, continue to detect the next witness.
                            if raw != &WITNESS_HEADER {
                                i += 1;
                                continue;
                            }
                        }
                    }
                    das_witnesses_started = true;

                    let data_type_in_int = u32::from_le_bytes(
                        buf.get(WITNESS_HEADER_BYTES..(WITNESS_HEADER_BYTES + WITNESS_TYPE_BYTES))
                            .unwrap()
                            .try_into()
                            .unwrap(),
                    );
                    match DataType::try_from(data_type_in_int) {
                        Ok(DataType::SubAccountMintSign) => {
                            count += 1;
                            mint_sign_index = Some(i);
                        }
                        Ok(DataType::SubAccountRenewSign) => {
                            count += 1;
                            renew_sign_index = Some(i);
                        }
                        Ok(DataType::SubAccount) => {
                            count += 1;
                            indexes.push(i);

                            let start = WITNESS_HEADER_BYTES + WITNESS_TYPE_BYTES;
                            // Every sub-account witness has the next fields, here we parse it one by one.
                            let (start, _) = Self::parse_field("version", &buf, start)?;
                            let (_, action_bytes) = Self::parse_field("action", &buf, start)?;
                            if action_bytes == SubAccountAction::Create.to_string().as_bytes() {
                                contains_creation = true;
                            } else if action_bytes == SubAccountAction::Edit.to_string().as_bytes() {
                                contains_edition = true;
                            } else if action_bytes == SubAccountAction::Renew.to_string().as_bytes() {
                                contains_renew = true;
                            } else if action_bytes == SubAccountAction::Recycle.to_string().as_bytes() {
                                contains_recycle = true;
                            }
                        }
                        Ok(DataType::SubAccountPriceRule) => {
                            count += 1;
                            price_rule_indexes.push(i);
                        }
                        Ok(DataType::SubAccountPreservedRule) => {
                            count += 1;
                            preserved_rule_indexes.push(i);
                        }
                        Ok(_) => {
                            // Ignore other witnesses in this parser.
                        }
                        Err(_) => {
                            // Ignore unknown DataTypes which will make adding new DataType much easier and no need to update every contracts.
                            debug!(
                                "Ignored unknown DataType {:?} for compatible purpose.",
                                data_type_in_int
                            );
                        }
                    }

                    i += 1;
                }
                Err(SysError::IndexOutOfBound) => break,
                Err(e) => return Err(e.into()),
            }
        }

        if count <= 0 {
            warn!("Can not find any sub-account witness in this transaction.");
            return Err(code_to_error!(ErrorCode::WitnessEmpty));
        }

        Ok(SubAccountWitnessesParser {
            flag,
            contains_creation,
            contains_edition,
            contains_renew,
            contains_recycle,
            mint_sign_index,
            renew_sign_index,
            price_rule_indexes,
            preserved_rule_indexes,
            indexes,
        })
    }

    fn parse_mint_sign_witness(
        &self,
        index: usize,
        lock_args: &[u8],
    ) -> Result<SubAccountMintSignWitness, Box<dyn ScriptError>> {
        debug!("  witnesses[{:>2}] Parsing SubAccountMint/RenewSignWitness ...", index);

        let raw = util::load_das_witnesses(index)?;
        let start = WITNESS_HEADER_BYTES + WITNESS_LENGTH_BYTES;

        let (start, version_bytes) = Self::parse_field("version", &raw, start)?;
        let (start, signature) = Self::parse_field("signature", &raw, start)?;
        let (start, sign_role_byte) = Self::parse_field("sign_role", &raw, start)?;
        let (start, expired_at_bytes) = Self::parse_field("expired_at", &raw, start)?;
        let (_, account_list_smt_root) = Self::parse_field("account_list_smt_root", &raw, start)?;

        das_assert!(
            version_bytes.len() == 4,
            ErrorCode::WitnessStructureError,
            "  witnesses[{:>2}] SubAccountMintSignWitness.version should be 4 bytes.",
            index
        );
        let version = u32::from_le_bytes(version_bytes.try_into().unwrap());

        das_assert!(
            expired_at_bytes.len() == 8,
            ErrorCode::WitnessStructureError,
            "  witnesses[{:>2}] SubAccountMintSignWitness.expired_at_bytes should be 8 bytes.",
            index
        );
        let expired_at = u64::from_le_bytes(expired_at_bytes.try_into().unwrap());

        let (sign_role, sign_type, sign_args) = Self::parse_sign_info(index, sign_role_byte, lock_args)?;

        Ok(SubAccountMintSignWitness {
            index,
            version: version,
            signature: signature.to_vec(),
            sign_role,
            sign_type,
            sign_args,
            expired_at,
            account_list_smt_root: account_list_smt_root.to_vec(),
        })
    }

    fn parse_rule_witnesses(
        &self,
        data_type: DataType,
    ) -> Result<([u8; 32], Vec<ast_types::SubAccountRule>), Box<dyn ScriptError>> {
        let indexes = match data_type {
            DataType::SubAccountPriceRule => &self.price_rule_indexes,
            DataType::SubAccountPreservedRule => &self.preserved_rule_indexes,
            _ => unreachable!(),
        };

        debug!("Start calculating {:?}Witness ...", data_type);

        // Hash the concat bytes first so that we can release the memory of concat_bytes.
        let hash = {
            let mut concat_bytes = Vec::new();
            for index in indexes {
                debug!("  witnesses[{:>2}] Parsing bytes to {:?}Witness ...", index, data_type);

                let raw = util::load_das_witnesses(*index)?;
                let start = WITNESS_HEADER_BYTES + WITNESS_TYPE_BYTES;

                let (start, _) = Self::parse_field("version", &raw, start)?;
                let (_, rules_bytes) = Self::parse_field("rules", &raw, start)?;

                concat_bytes.extend(util::blake2b_256(rules_bytes));
            }

            util::blake2b_256(&concat_bytes)
        };

        debug!("Start parsing {:?}Witness ...", data_type);

        let mut rules = Vec::new();
        for index in indexes {
            debug!("  witnesses[{:>2}] Parsing bytes to {:?}Witness ...", index, data_type);

            let raw = util::load_das_witnesses(*index)?;
            let start = WITNESS_HEADER_BYTES + WITNESS_TYPE_BYTES;

            let (start, version_bytes) = Self::parse_field("version", &raw, start)?;
            let (_, rules_bytes) = Self::parse_field("rules", &raw, start)?;

            das_assert!(
                version_bytes.len() == 4,
                ErrorCode::WitnessStructureError,
                "  witnesses[{:>2}] SubAccountMintSignWitness.version should be 4 bytes.",
                index
            );
            let version = u32::from_le_bytes(version_bytes.try_into().unwrap());

            let sub_rules = match version {
                1 => {
                    let mol_rules = SubAccountRules::from_compatible_slice(rules_bytes).map_err(|e| {
                        warn!(
                            "  witnesses[{:>2}] Decoding bytes to SubAccountRules failed (expected to be SubAccountRules): {}",
                            index,
                            e.to_string()
                        );

                        code_to_error!(ErrorCode::WitnessEntityDecodingError)
                    })?;

                    ast_util::mol_reader_to_sub_account_rules(String::new(), mol_rules.as_reader()).map_err(|err| {
                        warn!(
                            "witnesses[{:>2}] Parsing witness to SubAccountRules instances failed: {}",
                            index,
                            err.to_string()
                        );

                        code_to_error!(SubAccountCellErrorCode::WitnessParsingError)
                    })?
                }
                _ => {
                    warn!(
                        "  witnesses[{:>2}] Unsupported version {} for {:?}Witness.",
                        index, version, data_type
                    );

                    return Err(code_to_error!(ErrorCode::WitnessVersionUndefined));
                }
            };

            rules.extend(sub_rules.into_iter());
        }

        for (i, rule) in rules.iter().enumerate() {
            das_assert!(
                rule.index == i as u32,
                SubAccountCellErrorCode::WitnessParsingError,
                "  rules[{:>2}] SubAccountMintSignWitness.index should be ordered.",
                rule.index
            );
        }

        Ok((hash, rules))
    }

    fn parse_witness(flag: SubAccountConfigFlag, i: usize) -> Result<SubAccountWitness, Box<dyn ScriptError>> {
        debug!("  witnesses[{:>2}] Parsing SubAccountWitness ...", i);

        let raw = util::load_das_witnesses(i)?;
        let start = WITNESS_HEADER_BYTES + WITNESS_TYPE_BYTES;

        // Every sub-account witness has the next fields, here we parse it one by one.
        let (start, version_bytes) = Self::parse_field("version", &raw, start)?;
        let (start, action_bytes) = Self::parse_field("action", &raw, start)?;
        let (start, signature) = Self::parse_field("signature", &raw, start)?;
        let (start, sign_role_byte) = Self::parse_field("sign_role", &raw, start)?;
        let (start, sign_expired_at_bytes) = Self::parse_field("sign_expired_at", &raw, start)?;
        let (start, new_root) = Self::parse_field("new_root", &raw, start)?;
        let (start, proof) = Self::parse_field("proof", &raw, start)?;
        let (start, sub_account_bytes) = Self::parse_field("sub_account", &raw, start)?;
        let (start, edit_key) = Self::parse_field("edit_key", &raw, start)?;
        let (_, edit_value_bytes) = Self::parse_field("edit_value", &raw, start)?;

        das_assert!(
            version_bytes.len() == 4,
            ErrorCode::WitnessStructureError,
            "  witnesses[{:>2}] SubAccountMintSignWitness.version should be 4 bytes.",
            i
        );
        let version = u32::from_le_bytes(version_bytes.try_into().unwrap());

        if version == 2 {
            // TODO Support multiple version of sub-account witness.
        } else {
            warn!(
                "  witnesses[{:>2}] SubAccountWitness.version is {} which is invalid for now.",
                i, version
            );
            return Err(code_to_error!(ErrorCode::WitnessVersionOrTypeInvalid));
        }

        let action = match String::from_utf8(action_bytes.to_vec()) {
            Ok(action) => match SubAccountAction::from_str(action.as_str()) {
                Ok(val) => val,
                Err(e) => {
                    warn!(
                        "  witnesses[{:>2}] SubAccountWitness.action field parse failed: {:?}",
                        i, e
                    );
                    return Err(code_to_error!(ErrorCode::WitnessStructureError));
                }
            },
            Err(e) => {
                warn!(
                    "  witnesses[{:>2}] SubAccountWitness.action field parse failed: {}",
                    i, e
                );
                return Err(code_to_error!(ErrorCode::WitnessStructureError));
            }
        };

        let sub_account = match SubAccount::from_compatible_slice(sub_account_bytes) {
            Ok(val) => val,
            Err(e) => {
                warn!(
                    "  witnesses[{:>2}] SubAccountWitness.sub_account field parse failed: {}",
                    i, e
                );
                return Err(code_to_error!(ErrorCode::WitnessStructureError));
            }
        };

        debug!(
            "  witnesses[{:>2}] SubAccountWitness.action is {} .",
            i,
            action.to_string()
        );

        let mut sign_role = None;
        let mut sign_type = None;
        let mut sign_args = vec![];
        let mut sign_expired_at = 0;
        let mut _lock_args = vec![];
        let edit_value;
        match action {
            SubAccountAction::Create => {
                edit_value = match edit_key {
                    b"manual" => {
                        das_assert!(
                            !edit_value_bytes.len() >= 8,
                            SubAccountCellErrorCode::WitnessEditValueError,
                            "  witnesses[{:>2}] SubAccountMintSignWitness.edit_value_bytes should not be empty.",
                            i
                        );

                        SubAccountEditValue::Proof
                    }
                    b"custom_script" => {
                        das_assert!(
                            flag == SubAccountConfigFlag::CustomScript,
                            SubAccountCellErrorCode::WitnessEditKeyInvalid,
                            "  witnesses[{:>2}] The flag is {}, so the 'custom_script' is not allowed in edit_key.",
                            i,
                            flag.to_string()
                        );

                        das_assert!(
                            edit_value_bytes.is_empty(),
                            SubAccountCellErrorCode::WitnessEditValueError,
                            "  witnesses[{:>2}] SubAccountMintSignWitness.edit_value_bytes should be empty.",
                            i
                        );

                        SubAccountEditValue::None
                    }
                    b"custom_rule" => {
                        das_assert!(
                            flag == SubAccountConfigFlag::CustomRule,
                            SubAccountCellErrorCode::WitnessEditKeyInvalid,
                            "  witnesses[{:>2}] The flag is {}, so the 'custom_rule' is not allowed in edit_key.",
                            i,
                            flag.to_string()
                        );

                        das_assert!(
                            edit_value_bytes.len() == 28,
                            SubAccountCellErrorCode::WitnessEditValueError,
                            "  witnesses[{:>2}] SubAccountMintSignWitness.edit_value_bytes should be 28 bytes.",
                            i
                        );

                        let value = u64::from_le_bytes(edit_value_bytes[20..].try_into().unwrap());

                        SubAccountEditValue::Channel(edit_value_bytes[..20].to_vec(), value)
                    }
                    _ => SubAccountEditValue::None,
                };
            }
            SubAccountAction::Renew => {
                let new_expired_at = match data_parser::sub_account_cell::get_exipred_at_from_edit_value(
                    &edit_value_bytes,
                ) {
                    Some(value) => value,
                    None => {
                        warn!(
                                "  witnesses[{:>2}] The edit_value should contains expired_at when renewing the sub-account.",
                                i
                            );
                        return Err(code_to_error!(SubAccountCellErrorCode::NewExpiredAtIsRequired));
                    }
                };
                edit_value = SubAccountEditValue::ExpiredAt(new_expired_at);

                match edit_key {
                    b"custom_script" => {
                        das_assert!(
                            flag == SubAccountConfigFlag::CustomScript,
                            SubAccountCellErrorCode::WitnessEditKeyInvalid,
                            "  witnesses[{:>2}] The flag is {}, so the 'custom_script' is not allowed in edit_key.",
                            i,
                            flag.to_string()
                        );

                        das_assert!(
                            edit_value_bytes.is_empty(),
                            SubAccountCellErrorCode::WitnessEditValueError,
                            "  witnesses[{:>2}] SubAccountMintSignWitness.edit_value_bytes should be empty.",
                            i
                        );
                    }
                    b"custom_rule" => {
                        das_assert!(
                            flag == SubAccountConfigFlag::CustomRule,
                            SubAccountCellErrorCode::WitnessEditKeyInvalid,
                            "  witnesses[{:>2}] The flag is {}, so the 'custom_rule' is not allowed in edit_key.",
                            i,
                            flag.to_string()
                        );

                        das_assert!(
                            edit_value_bytes.len() == 28 + 8,
                            SubAccountCellErrorCode::WitnessEditValueError,
                            "  witnesses[{:>2}] SubAccountMintSignWitness.edit_value_bytes should be 36 bytes.",
                            i
                        );
                    }
                    _ => {}
                }
            }
            SubAccountAction::Edit => {
                das_assert!(
                    sign_expired_at_bytes.len() == 8,
                    ErrorCode::WitnessStructureError,
                    "  witnesses[{:>2}] SubAccountMintSignWitness.expired_at_bytes should be 8 bytes.",
                    i
                );
                sign_expired_at = u64::from_le_bytes(sign_expired_at_bytes.try_into().unwrap());

                let lock_args_reader = sub_account.as_reader().lock().args();
                _lock_args = lock_args_reader.raw_data().to_vec();
                (sign_role, sign_type, sign_args) = Self::parse_sign_info(i, sign_role_byte, &_lock_args)?;

                // The actual type of the edit_value field is base what the edit_key field is.
                edit_value = match edit_key {
                    b"owner" => SubAccountEditValue::Owner(edit_value_bytes.to_vec()),
                    b"manager" => SubAccountEditValue::Manager(edit_value_bytes.to_vec()),
                    b"records" => {
                        let records = match Records::from_slice(edit_value_bytes) {
                            Ok(val) => val,
                            Err(e) => {
                                warn!(
                                    "  witnesses[{:>2}] Sub-account witness structure error, decoding records failed: {}",
                                    i, e
                                );
                                return Err(code_to_error!(ErrorCode::WitnessStructureError));
                            }
                        };

                        SubAccountEditValue::Records(records)
                    }
                    _ => SubAccountEditValue::None,
                };
            }
            SubAccountAction::Recycle => {
                edit_value = SubAccountEditValue::None;
            }
        }

        debug!(
            "  Sub-account witnesses[{:>2}]: {{ version: {}, signature: 0x{}, lock_args: 0x{}, sign_role: 0x{}, sign_exipired_at: {}, new_root: 0x{}, action: {}, sub_account: {}, edit_key: {}, sign_args: {} }}",
            i, version, util::hex_string(signature), util::hex_string(&_lock_args), util::hex_string(sign_role_byte), sign_expired_at, util::hex_string(new_root), action, sub_account.account().as_prettier(), String::from_utf8(edit_key.to_vec()).unwrap(), util::hex_string(&sign_args)
        );

        Ok(SubAccountWitness {
            index: i,
            version,
            signature: signature.to_vec(),
            sign_role,
            sign_type,
            sign_args,
            sign_expired_at,
            new_root: new_root.to_vec(),
            proof: proof.to_vec(),
            action,
            sub_account,
            edit_key: edit_key.to_vec(),
            edit_value,
            edit_value_bytes: edit_value_bytes.to_vec(),
        })
    }

    fn parse_field<'a>(
        field_name: &str,
        bytes: &'a [u8],
        start: usize,
    ) -> Result<(usize, &'a [u8]), Box<dyn ScriptError>> {
        // Every field is start with 4 bytes of uint32 as its length.
        let length = match bytes.get(start..(start + WITNESS_LENGTH_BYTES)) {
            Some(bytes) => {
                das_assert!(
                    bytes.len() == 4,
                    ErrorCode::WitnessStructureError,
                    "  [{}] Sub-account witness structure error, expect {}..{} to be bytes of LE uint32.",
                    field_name,
                    start,
                    start + WITNESS_LENGTH_BYTES
                );

                u32::from_le_bytes(bytes.try_into().unwrap()) as usize
            }
            None => {
                warn!(
                    "  [{}] Sub-account witness structure error, expect 4 bytes in {}..{} .",
                    field_name,
                    start,
                    start + WITNESS_LENGTH_BYTES
                );
                return Err(code_to_error!(ErrorCode::WitnessStructureError));
            }
        };

        // Slice the field base on the start and length.
        let from = start + WITNESS_LENGTH_BYTES;
        let to = from + length;
        let field_bytes = match bytes.get(from..to) {
            Some(bytes) => bytes,
            None => {
                warn!(
                    "  [{}] Sub-account witness structure error, expect {} bytes in {}..{} .",
                    field_name, length, from, to
                );
                return Err(code_to_error!(ErrorCode::WitnessStructureError));
            }
        };

        let new_start = start + WITNESS_LENGTH_BYTES + length;
        Ok((new_start, field_bytes))
    }

    fn parse_sign_info(
        index: usize,
        sign_role_byte: &[u8],
        lock_args: &[u8],
    ) -> Result<(Option<LockRole>, Option<DasLockType>, Vec<u8>), Box<dyn ScriptError>> {
        let sign_role_int = match sign_role_byte.try_into() {
            Ok(val) => u8::from_le_bytes(val),
            Err(e) => {
                warn!(
                    "  witnesses[{:>2}] Parsing 0x{} to u8 failed: {}",
                    index,
                    util::hex_string(sign_role_byte),
                    e
                );
                return Err(code_to_error!(ErrorCode::Encoding));
            }
        };
        let sign_type_int;
        let sign_args_ref;

        let sign_role;
        if sign_role_int == LockRole::Owner as u8 {
            sign_type_int = data_parser::das_lock_args::get_owner_type(lock_args);
            sign_args_ref = data_parser::das_lock_args::get_owner_lock_args(lock_args);
            sign_role = Some(LockRole::Owner);
        } else {
            sign_type_int = data_parser::das_lock_args::get_manager_type(lock_args);
            sign_args_ref = data_parser::das_lock_args::get_manager_lock_args(lock_args);
            sign_role = Some(LockRole::Manager);
        };

        let sign_type = DasLockType::try_from(sign_type_int).ok();
        let sign_args = sign_args_ref.to_vec();

        Ok((sign_role, sign_type, sign_args))
    }

    pub fn iter(&self) -> SubAccountWitnessesIter {
        SubAccountWitnessesIter {
            parser: self,
            current: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.indexes.len()
    }

    pub fn get_mint_sign(&self, lock_args: &[u8]) -> Option<Result<SubAccountMintSignWitness, Box<dyn ScriptError>>> {
        match self.mint_sign_index {
            Some(i) => match self.parse_mint_sign_witness(i, lock_args) {
                Ok(witness) => Some(Ok(witness)),
                Err(e) => Some(Err(e)),
            },
            _ => None,
        }
    }

    pub fn get_renew_sign(&self, lock_args: &[u8]) -> Option<Result<SubAccountMintSignWitness, Box<dyn ScriptError>>> {
        match self.renew_sign_index {
            Some(i) => match self.parse_mint_sign_witness(i, lock_args) {
                Ok(witness) => Some(Ok(witness)),
                Err(e) => Some(Err(e)),
            },
            _ => None,
        }
    }

    pub fn get_rules(
        &self,
        sub_account_cell_data: &[u8],
        data_type: DataType,
    ) -> Result<Option<Vec<ast_types::SubAccountRule>>, Box<dyn ScriptError>> {
        let (indexes, expected_hash) = match data_type {
            DataType::SubAccountPriceRule => (
                &self.price_rule_indexes,
                data_parser::sub_account_cell::get_price_rules_hash(&sub_account_cell_data),
            ),
            DataType::SubAccountPreservedRule => (
                &self.preserved_rule_indexes,
                data_parser::sub_account_cell::get_preserved_rules_hash(&sub_account_cell_data),
            ),
            _ => unreachable!(),
        };

        if indexes.is_empty() {
            if expected_hash.is_none() || expected_hash == Some(&[0u8; 10]) {
                return Ok(None);
            } else {
                warn!("The {:?} is required, but not found in witnesses.", data_type);
                return Err(code_to_error!(ErrorCode::WitnessEmpty));
            }
        }

        let (hash, rules) = self.parse_rule_witnesses(data_type)?;
        das_assert!(
            expected_hash == hash.get(0..10),
            SubAccountCellErrorCode::ConfigRulesHashMismatch,
            "The hash of {} is mismatched.(in_data: {:?}, calculated: {:?})",
            data_type.to_string(),
            expected_hash.map(|v| util::hex_string(v)),
            hash.get(0..10).map(|v| util::hex_string(v))
        );

        Ok(Some(rules))
    }

    pub fn get(&self, index: usize) -> Option<Result<SubAccountWitness, Box<dyn ScriptError>>> {
        match self.indexes.get(index) {
            None => return None,
            Some(&i) => Some(Self::parse_witness(self.flag, i)),
        }
    }

    pub fn only_contains_recycle(&self) -> bool {
        self.contains_recycle && !self.contains_creation && !self.contains_edition && !self.contains_renew
    }
}
