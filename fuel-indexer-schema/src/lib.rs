extern crate alloc;
use alloc::vec::Vec;
pub use fuel_indexer_database_types as sql_types;

use crate::sql_types::ColumnType;
use core::convert::{TryFrom, TryInto};
use graphql_parser::schema::{Definition, Document, TypeDefinition};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

pub const LOG_LEVEL_ERROR: u32 = 0;
pub const LOG_LEVEL_WARN: u32 = 1;
pub const LOG_LEVEL_INFO: u32 = 2;
pub const LOG_LEVEL_DEBUG: u32 = 3;
pub const LOG_LEVEL_TRACE: u32 = 4;

use sha2::{Digest, Sha256};

pub const BASE_SCHEMA: &str = include_str!("./base.graphql");

#[cfg(feature = "db-models")]
pub mod db;

pub use fuel_types::{Address, AssetId, Bytes32, Bytes4, Bytes8, ContractId, Salt, Word};
pub struct NativeHandlerResult(pub u64, pub Vec<FtColumn>);

pub type ID = u64;
pub type Int4 = i32;
pub type Int8 = i64;
pub type UInt4 = u32;
pub type UInt8 = u64;
pub type Timestamp = u64;

// serde_scale for now, can look at other options if necessary.
pub fn serialize(obj: &impl Serialize) -> Vec<u8> {
    serde_scale::to_vec(obj).expect("Serialize failed")
}

pub fn deserialize<'a, T: Deserialize<'a>>(bytes: &'a [u8]) -> T {
    serde_scale::from_slice(bytes).expect("Deserialize failed")
}

pub fn type_id(namespace: &str, type_name: &str) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&Sha256::digest(format!("{}:{}", namespace, type_name).as_bytes())[..8]);
    u64::from_le_bytes(bytes)
}

pub fn schema_version(schema: &str) -> String {
    format!("{:x}", Sha256::digest(schema.as_bytes()))
}

pub fn type_name(typ: &TypeDefinition<String>) -> String {
    match typ {
        TypeDefinition::Scalar(obj) => obj.name.clone(),
        TypeDefinition::Object(obj) => obj.name.clone(),
        TypeDefinition::Interface(obj) => obj.name.clone(),
        TypeDefinition::Union(obj) => obj.name.clone(),
        TypeDefinition::Enum(obj) => obj.name.clone(),
        TypeDefinition::InputObject(obj) => obj.name.clone(),
    }
}

pub fn get_schema_types(ast: &Document<String>) -> (HashSet<String>, HashSet<String>) {
    let types: HashSet<String> = ast
        .definitions
        .iter()
        .filter_map(|def| {
            if let Definition::TypeDefinition(typ) = def {
                Some(typ)
            } else {
                None
            }
        })
        .map(type_name)
        .collect();

    let directives = ast
        .definitions
        .iter()
        .filter_map(|def| {
            if let Definition::DirectiveDefinition(dir) = def {
                Some(dir.name.clone())
            } else {
                None
            }
        })
        .collect();

    (types, directives)
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub enum FtColumn {
    ID(u64),
    Address(Address),
    AssetId(AssetId),
    Bytes4(Bytes4),
    Bytes8(Bytes8),
    Bytes32(Bytes32),
    ContractId(ContractId),
    Int4(i32),
    Int8(i64),
    UInt4(u32),
    UInt8(u64),
    Timestamp(u64),
    Salt(Salt),
}

impl FtColumn {
    pub fn new(ty: ColumnType, size: usize, bytes: &[u8]) -> FtColumn {
        match ty {
            ColumnType::ID => {
                let ident =
                    u64::from_le_bytes(bytes[..size].try_into().expect("Invalid slice length"));
                FtColumn::ID(ident)
            }
            ColumnType::Address => {
                let address = Address::try_from(&bytes[..size]).expect("Invalid slice length");
                FtColumn::Address(address)
            }
            ColumnType::AssetId => {
                let asset_id = AssetId::try_from(&bytes[..size]).expect("Invalid slice length");
                FtColumn::AssetId(asset_id)
            }
            ColumnType::Bytes4 => {
                let bytes = Bytes4::try_from(&bytes[..size]).expect("Invalid slice length");
                FtColumn::Bytes4(bytes)
            }
            ColumnType::Bytes8 => {
                let bytes = Bytes8::try_from(&bytes[..size]).expect("Invalid slice length");
                FtColumn::Bytes8(bytes)
            }
            ColumnType::Bytes32 => {
                let bytes = Bytes32::try_from(&bytes[..size]).expect("Invalid slice length");
                FtColumn::Bytes32(bytes)
            }
            ColumnType::ContractId => {
                let contract_id =
                    ContractId::try_from(&bytes[..size]).expect("Invalid slice length");
                FtColumn::ContractId(contract_id)
            }
            ColumnType::Salt => {
                let salt = Salt::try_from(&bytes[..size]).expect("Invalid slice length");
                FtColumn::Salt(salt)
            }
            ColumnType::Int4 => {
                let int4 =
                    i32::from_le_bytes(bytes[..size].try_into().expect("Invalid slice length"));
                FtColumn::Int4(int4)
            }
            ColumnType::Int8 => {
                let int8 =
                    i64::from_le_bytes(bytes[..size].try_into().expect("Invalid slice length"));
                FtColumn::Int8(int8)
            }
            ColumnType::UInt4 => {
                let int4 =
                    u32::from_le_bytes(bytes[..size].try_into().expect("Invalid slice length"));
                FtColumn::UInt4(int4)
            }
            ColumnType::UInt8 => {
                let int8 =
                    u64::from_le_bytes(bytes[..size].try_into().expect("Invalid slice length"));
                FtColumn::UInt8(int8)
            }
            ColumnType::Timestamp => {
                let uint8 =
                    u64::from_le_bytes(bytes[..size].try_into().expect("Invalid slice length"));
                FtColumn::Timestamp(uint8)
            }
            ColumnType::Blob => {
                panic!("Blob not supported for FtColumn!");
            }
            ColumnType::ForeignKey => {
                panic!("ForeignKey not supported for FtColumn!");
            }
        }
    }

    pub fn query_fragment(&self) -> String {
        match self {
            FtColumn::ID(value) => {
                format!("{}", value)
            }
            FtColumn::Address(value) => {
                format!("'{:x}'", value)
            }
            FtColumn::AssetId(value) => {
                format!("'{:x}'", value)
            }
            FtColumn::Bytes4(value) => {
                format!("'{:x}'", value)
            }
            FtColumn::Bytes8(value) => {
                format!("'{:x}'", value)
            }
            FtColumn::Bytes32(value) => {
                format!("'{:x}'", value)
            }
            FtColumn::ContractId(value) => {
                format!("'{:x}'", value)
            }
            FtColumn::Int4(value) => {
                format!("{}", value)
            }
            FtColumn::Int8(value) => {
                format!("{}", value)
            }
            FtColumn::UInt4(value) => {
                format!("{}", value)
            }
            FtColumn::UInt8(value) => {
                format!("{}", value)
            }
            FtColumn::Timestamp(value) => {
                format!("{}", value)
            }
            FtColumn::Salt(value) => {
                format!("'{:x}'", value)
            }
        }
    }
}

mod tests {
    #[test]
    fn test_fragments() {
        use super::*;

        let id = FtColumn::ID(123456);
        let addr = FtColumn::Address(Address::try_from([0x12; 32]).expect("Bad bytes"));
        let asset_id = FtColumn::AssetId(AssetId::try_from([0xA5; 32]).expect("Bad bytes"));
        let bytes4 = FtColumn::Bytes4(Bytes4::try_from([0xF0; 4]).expect("Bad bytes"));
        let bytes8 = FtColumn::Bytes8(Bytes8::try_from([0x9D; 8]).expect("Bad bytes"));
        let bytes32 = FtColumn::Bytes32(Bytes32::try_from([0xEE; 32]).expect("Bad bytes"));
        let contractid = FtColumn::ContractId(ContractId::try_from([0x78; 32]).expect("Bad bytes"));
        let int4 = FtColumn::Int4(i32::from_le_bytes([0x78; 4]));
        let int8 = FtColumn::Int8(i64::from_le_bytes([0x78; 8]));
        let uint4 = FtColumn::UInt4(u32::from_le_bytes([0x78; 4]));
        let uint8 = FtColumn::UInt8(u64::from_le_bytes([0x78; 8]));
        let uint64 = FtColumn::Timestamp(u64::from_le_bytes([0x78; 8]));
        let salt = FtColumn::Salt(Salt::try_from([0x31; 32]).expect("Bad bytes"));

        insta::assert_yaml_snapshot!(id.query_fragment());
        insta::assert_yaml_snapshot!(addr.query_fragment());
        insta::assert_yaml_snapshot!(asset_id.query_fragment());
        insta::assert_yaml_snapshot!(bytes4.query_fragment());
        insta::assert_yaml_snapshot!(bytes8.query_fragment());
        insta::assert_yaml_snapshot!(bytes32.query_fragment());
        insta::assert_yaml_snapshot!(contractid.query_fragment());
        insta::assert_yaml_snapshot!(salt.query_fragment());
        insta::assert_yaml_snapshot!(int4.query_fragment());
        insta::assert_yaml_snapshot!(int8.query_fragment());
        insta::assert_yaml_snapshot!(uint4.query_fragment());
        insta::assert_yaml_snapshot!(uint8.query_fragment());
        insta::assert_yaml_snapshot!(uint64.query_fragment());
    }
}
